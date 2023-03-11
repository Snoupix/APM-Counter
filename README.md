# APM-Counter

A simple APM counter on the top-right screen corner written in Rust.
I'm using **eframe**, hence, **egui** for the Graphical User Interface (display), **rdev** for keyboard & mouse inputs, **tokio** for async tasks and Rust's standard library.

## Explanations

*Every keystroke will be taken when the App window isn't focused.*

Once started, it will display an APM counter that resets every minute and display the average Action Per Minute calculated in one minute every 100ms.

It only counts on a key/button release. Scroll wheel and mouse movement aren't used in the script.

For now, to shutdown the application, just press the **End** key.

## crates.io dependencies

 - [eframe](https://crates.io/crates/eframe) -> [egui](https://crates.io/crates/egui)
 - [rdev](https://crates.io/crates/rdev)
 - [tokio](https://crates.io/crates/tokio)
