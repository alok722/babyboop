//! Platform-free logic for the Windows host. Everything here builds and tests on any OS:
//! `cargo test` in `apps/windows-rs` runs on macOS too.

pub mod config;
pub mod exit_hold;
pub mod input;
pub mod keymap;
