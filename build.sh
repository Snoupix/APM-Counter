#!/usr/bin/env bash

# Personal build script for my WSL

set -e

cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/apm_display.exe /mnt/c/Users/samuf/Desktop/APM.exe
