//! What the lock does with each swallowed keyboard and mouse message, minus the Win32 calls.
//! Methods that return a `String` hand back the JavaScript call to run in the page.

use std::collections::{BTreeMap, HashSet};

use crate::exit_hold::ExitHold;
use crate::keymap::{is_alt, is_modifier, is_shift, to_page_key};

/// Wheel distance (in Windows' 120-per-notch units) that counts as one boop.
pub const WHEEL_STEP: u32 = 120;

#[derive(Debug)]
pub struct LockInput {
    down: HashSet<u32>,
    exit: ExitHold,
    seen: BTreeMap<u32, u32>,
    park: (i32, i32),
    scale: f64,
    size: (f64, f64),
    x: f64,
    y: f64,
    wheel: u32,
}

impl LockInput {
    /// `screen_px` is the primary screen in physical pixels; `scale` is pixels per CSS pixel.
    pub fn new(screen_px: (i32, i32), scale: f64) -> Self {
        let scale = if scale > 0.0 { scale } else { 1.0 };
        let size = (screen_px.0 as f64 / scale, screen_px.1 as f64 / scale);
        LockInput {
            down: HashSet::new(),
            exit: ExitHold::default(),
            seen: BTreeMap::new(),
            park: (screen_px.0 / 2, screen_px.1 / 2),
            scale,
            size,
            x: size.0 / 2.0,
            y: size.1 / 2.0,
            wheel: 0,
        }
    }

    /// Where the real cursor is parked while locked, in physical pixels.
    pub fn park(&self) -> (i32, i32) {
        self.park
    }

    /// The page's virtual pointer, in CSS pixels.
    pub fn pointer(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn alt(&self) -> bool {
        self.down.iter().any(|&k| is_alt(k))
    }

    fn shift(&self) -> bool {
        self.down.iter().any(|&k| is_shift(k))
    }

    pub fn key_down(&mut self, vk: u32, now: f64) -> Option<String> {
        let repeat = !self.down.insert(vk);
        self.exit.key_down(vk, self.alt(), self.shift(), now);
        if is_modifier(vk) {
            self.exit.modifiers_changed(self.alt(), self.shift());
            None
        } else if repeat {
            None
        } else {
            Some(native_input(to_page_key(vk), None))
        }
    }

    pub fn key_up(&mut self, vk: u32) {
        self.down.remove(&vk);
        self.exit.key_up(vk);
        if is_modifier(vk) {
            self.exit.modifiers_changed(self.alt(), self.shift());
        }
    }

    /// `pt` is where Windows says the cursor went; the real cursor never leaves `park`.
    pub fn mouse_move(&mut self, pt: (i32, i32)) {
        self.x = (self.x + (pt.0 - self.park.0) as f64 / self.scale).clamp(0.0, self.size.0);
        self.y = (self.y + (pt.1 - self.park.1) as f64 / self.scale).clamp(0.0, self.size.1);
    }

    pub fn button(&self) -> String {
        native_input(None, Some((self.x, self.y)))
    }

    pub fn wheel(&mut self, delta: i16) -> Option<String> {
        self.wheel += delta.unsigned_abs() as u32;
        if self.wheel >= WHEEL_STEP {
            self.wheel = 0;
            Some(self.button())
        } else {
            None
        }
    }

    /// Counts a message for the log summary. True the first time this message is seen.
    pub fn note(&mut self, msg: u32) -> bool {
        let n = self.seen.entry(msg).or_insert(0);
        *n += 1;
        *n == 1
    }

    /// Windows can eat key-ups (Ctrl+Alt+Del, switching user), so drop what we think is held.
    pub fn forget_held(&mut self) {
        self.down.clear();
        self.exit.reset();
    }

    pub fn exit_progress(&self, now: f64) -> f64 {
        self.exit.progress(now)
    }

    /// "0x100:3 0x101:3" for the log line written when the lock ends.
    pub fn summary(&self) -> String {
        self.seen
            .iter()
            .map(|(m, n)| format!("0x{m:X}:{n}"))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

pub fn native_input(key: Option<char>, at: Option<(f64, f64)>) -> String {
    let k = key.map_or_else(|| "null".to_string(), |c| format!("\"{c}\""));
    match at {
        Some((x, y)) => format!("nativeInput({k}, {x}, {y})"),
        None => format!("nativeInput({k})"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keymap::{VK_ESCAPE, VK_K, VK_LMENU, VK_LSHIFT};

    fn lock() -> LockInput {
        LockInput::new((1920, 1080), 1.5)
    }

    #[test]
    fn letter_goes_to_page_once_per_press() {
        let mut l = lock();
        assert_eq!(l.key_down(0x41, 0.0).as_deref(), Some("nativeInput(\"A\")"));
        assert_eq!(l.key_down(0x41, 0.1), None, "autorepeat");
        l.key_up(0x41);
        assert!(l.key_down(0x41, 0.2).is_some());
    }

    #[test]
    fn other_keys_still_boop() {
        assert_eq!(lock().key_down(0x5B, 0.0).as_deref(), Some("nativeInput(null)"));
    }

    #[test]
    fn modifiers_alone_do_not_boop() {
        assert_eq!(lock().key_down(VK_LSHIFT, 0.0), None);
    }

    #[test]
    fn alt_shift_k_held_exits() {
        let mut l = lock();
        l.key_down(VK_LMENU, 0.0);
        l.key_down(VK_LSHIFT, 0.0);
        l.key_down(VK_K, 0.0);
        assert_eq!(l.exit_progress(2.0), 1.0);
    }

    #[test]
    fn releasing_alt_cancels_k() {
        let mut l = lock();
        l.key_down(VK_LMENU, 0.0);
        l.key_down(VK_LSHIFT, 0.0);
        l.key_down(VK_K, 0.0);
        l.key_up(VK_LMENU);
        assert_eq!(l.exit_progress(2.0), 0.0);
    }

    #[test]
    fn forget_held_drops_exit_and_keys() {
        let mut l = lock();
        l.key_down(VK_ESCAPE, 0.0);
        l.forget_held();
        assert_eq!(l.exit_progress(5.0), 0.0);
        assert!(l.key_down(VK_ESCAPE, 5.0).is_some(), "not treated as a repeat");
    }

    #[test]
    fn pointer_starts_centered_in_css_pixels() {
        let l = lock();
        assert_eq!(l.park(), (960, 540));
        assert_eq!(l.pointer(), (640.0, 360.0));
    }

    #[test]
    fn mouse_moves_scale_and_clamp() {
        let mut l = lock();
        l.mouse_move((960 + 30, 540 - 15));
        assert_eq!(l.pointer(), (660.0, 350.0));
        l.mouse_move((960 + 100_000, 540 + 100_000));
        assert_eq!(l.pointer(), (1280.0, 720.0));
        assert_eq!(l.button(), "nativeInput(null, 1280, 720)");
    }

    #[test]
    fn wheel_boops_every_notch() {
        let mut l = lock();
        assert_eq!(l.wheel(60), None);
        assert!(l.wheel(-60).is_some());
        assert!(l.wheel(i16::MIN).is_some());
    }

    #[test]
    fn summary_counts_messages_in_order() {
        let mut l = lock();
        assert!(l.note(0x101));
        assert!(l.note(0x100));
        assert!(!l.note(0x100));
        assert_eq!(l.summary(), "0x100:2 0x101:1");
    }

    #[test]
    fn zero_scale_falls_back_to_one() {
        assert_eq!(LockInput::new((800, 600), 0.0).pointer(), (400.0, 300.0));
    }
}
