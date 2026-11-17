//! %LOCALAPPDATA%\BabyBoop\BabyBoop.log, emptied at each launch.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use windows::Win32::System::SystemInformation::GetLocalTime;

pub fn dir() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    base.join("BabyBoop")
}

fn file() -> PathBuf {
    dir().join("BabyBoop.log")
}

pub fn reset() {
    let _ = fs::create_dir_all(dir());
    let _ = fs::write(file(), "");
}

pub fn write(line: &str) {
    let t = unsafe { GetLocalTime() };
    let stamped = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03} {line}\n",
        t.wYear, t.wMonth, t.wDay, t.wHour, t.wMinute, t.wSecond, t.wMilliseconds
    );
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(file()) {
        let _ = f.write_all(stamped.as_bytes());
    }
}
