#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(windows)]
mod win;

#[cfg(windows)]
fn main() {
    win::run();
}

#[cfg(not(windows))]
fn main() {
    eprintln!("BabyBoop's Windows host only runs on Windows. Run `cargo test` for the shared logic.");
    std::process::exit(1);
}
