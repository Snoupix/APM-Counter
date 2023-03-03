use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use eframe::egui::{self, RichText};
use eframe::epaint::{Color32, Pos2, Vec2};
use eframe::NativeOptions;
use rdev::{listen, Event, EventType, Key};
use tokio::spawn;

const DEFAULT_SCREEN_WIDTH: f32 = 1920.;
const WINDOW_SIZE: Vec2 = Vec2 { x: 200., y: 50. };
const WINDOW_POS: Pos2 = Pos2 {
    x: DEFAULT_SCREEN_WIDTH - WINDOW_SIZE.x,
    y: 0.,
};

#[derive(Debug, Default)]
struct State {
    shutdown: bool,
    actions: u64,
    // 100ms elapsed
    elapsed: u64,
    apm: u64,
    checked_screen_size: bool,
}

struct Window {
    state: Arc<Mutex<State>>,
}

impl Window {
    fn new(_cc: &eframe::CreationContext<'_>, state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let _state = Arc::clone(&self.state);
        let mut state_guard = _state.lock().unwrap();
        let monitor_size = frame.info().window_info.monitor_size;

        // These IF statements are ugly but needed to satisfy Clippy.
        if !state_guard.checked_screen_size {
            if let Some(size) = monitor_size {
                if size.x != DEFAULT_SCREEN_WIDTH {
                    state_guard.checked_screen_size = true;
                    frame.set_window_pos(Pos2 {
                        x: size.x - WINDOW_SIZE.x,
                        ..WINDOW_POS
                    });
                }
            }
        }

        if state_guard.shutdown {
            frame.close();
            std::process::exit(0);
        }

        let stroke = egui::Stroke {
            width: WINDOW_SIZE.x,
            color: Color32::TRANSPARENT,
        };
        let frame = egui::Frame::default()
            .fill(Color32::TRANSPARENT)
            .stroke(stroke);
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(
                    RichText::new(format!("{} APM", state_guard.apm))
                        .color(Color32::GREEN)
                        .size(25.)
                        .strong(),
                );
            });
        });

        drop(state_guard);
        sleep(Duration::from_millis(500));
        ctx.request_repaint();
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(State::default()));

    let mut _state = Arc::clone(&state);
    spawn(async move {
        if let Err(error) = listen(move |e| callback(e, &mut _state)) {
            eprintln!("Error: {:?}", error);
        }
    });

    let _state = Arc::clone(&state);
    spawn(async move {
        loop {
            let mut state_guard = _state.lock().unwrap();

            if state_guard.shutdown {
                break;
            }

            if state_guard.elapsed == 600 {
                state_guard.apm = state_guard.actions;
                state_guard.actions = 0;
                state_guard.elapsed = 0;
            } else {
                state_guard.elapsed += 1;
                state_guard.apm = state_guard.actions * (600 / state_guard.elapsed);
            }

            drop(state_guard);
            sleep(Duration::from_millis(100));
        }
    });

    let _state = Arc::clone(&state);
    let native_options = NativeOptions {
        always_on_top: true,
        maximized: false,
        decorated: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: Some(WINDOW_POS),
        initial_window_size: Some(WINDOW_SIZE),
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: true,
        mouse_passthrough: false,
        vsync: true,
        ..NativeOptions::default()
    };

    eframe::run_native(
        "APM Counter",
        native_options,
        Box::new(|cc| Box::new(Window::new(cc, _state))),
    )
    .expect("Eframe window failed to run");
}

fn callback(event: Event, state: &mut Arc<Mutex<State>>) {
    match event.event_type {
        EventType::KeyRelease(_) | EventType::ButtonRelease(_) => {
            let mut state = state.lock().unwrap();

            if let EventType::KeyRelease(key) = event.event_type {
                if key == Key::Escape {
                    state.shutdown = true;
                    return;
                }
            }

            state.actions += 1;
        }
        _ => (),
    }
}
