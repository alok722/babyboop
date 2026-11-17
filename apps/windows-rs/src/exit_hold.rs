//! The two parent exits: Esc held 5 s, or Alt+Shift+K held 2 s.
//! Time is passed in as seconds so tests control the clock.

use crate::keymap::{VK_ESCAPE, VK_K};

fn hold_seconds(vk: u32) -> f64 {
    if vk == VK_ESCAPE {
        5.0
    } else {
        2.0
    }
}

#[derive(Debug, Default)]
pub struct ExitHold {
    start: Option<f64>,
    key: Option<u32>,
}

impl ExitHold {
    pub fn key_down(&mut self, vk: u32, alt: bool, shift: bool, now: f64) {
        if self.start.is_some() {
            return;
        }
        if vk == VK_ESCAPE || (vk == VK_K && alt && shift) {
            self.start = Some(now);
            self.key = Some(vk);
        }
    }

    pub fn key_up(&mut self, vk: u32) {
        if self.key == Some(vk) {
            self.reset();
        }
    }

    pub fn modifiers_changed(&mut self, alt: bool, shift: bool) {
        if self.key == Some(VK_K) && !(alt && shift) {
            self.reset();
        }
    }

    pub fn progress(&self, now: f64) -> f64 {
        match (self.start, self.key) {
            (Some(s), Some(k)) => ((now - s) / hold_seconds(k)).clamp(0.0, 1.0),
            _ => 0.0,
        }
    }

    pub fn triggered(&self, now: f64) -> bool {
        self.progress(now) >= 1.0
    }

    pub fn reset(&mut self) {
        self.start = None;
        self.key = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hold() -> ExitHold {
        ExitHold::default()
    }

    #[test]
    fn esc_held_5s_triggers() {
        let mut h = hold();
        h.key_down(VK_ESCAPE, false, false, 0.0);
        assert!(h.triggered(5.0));
    }

    #[test]
    fn esc_held_4_9s_does_not() {
        let mut h = hold();
        h.key_down(VK_ESCAPE, false, false, 0.0);
        assert!(!h.triggered(4.9));
        assert!((h.progress(4.9) - 0.98).abs() < 1e-3);
    }

    #[test]
    fn release_resets() {
        let mut h = hold();
        h.key_down(VK_ESCAPE, false, false, 0.0);
        h.key_up(VK_ESCAPE);
        h.key_down(VK_ESCAPE, false, false, 2.0);
        assert!(!h.triggered(6.0));
        assert!(h.triggered(7.0));
    }

    #[test]
    fn autorepeat_does_not_restart() {
        let mut h = hold();
        h.key_down(VK_ESCAPE, false, false, 0.0);
        h.key_down(VK_ESCAPE, false, false, 3.0);
        assert!(h.triggered(5.0));
    }

    #[test]
    fn k_needs_alt_shift() {
        let mut h = hold();
        h.key_down(VK_K, false, true, 0.0);
        assert_eq!(h.progress(3.0), 0.0);
    }

    #[test]
    fn alt_shift_k_2s() {
        let mut h = hold();
        h.key_down(VK_K, true, true, 0.0);
        assert!(h.triggered(2.0));
    }

    #[test]
    fn reset_clears_in_progress_hold() {
        let mut h = hold();
        h.key_down(VK_ESCAPE, false, false, 0.0);
        assert!(h.triggered(5.0));
        h.reset();
        assert_eq!(h.progress(5.0), 0.0);
        h.key_down(VK_ESCAPE, false, false, 5.0);
        assert!(!h.triggered(6.0));
    }

    #[test]
    fn dropping_shift_resets_k() {
        let mut h = hold();
        h.key_down(VK_K, true, true, 0.0);
        h.modifiers_changed(true, false);
        assert!(!h.triggered(3.0));
    }

    #[test]
    fn other_key_up_does_not_reset_hold() {
        let mut h = hold();
        h.key_down(VK_ESCAPE, false, false, 0.0);
        h.key_up(VK_K);
        assert!(h.triggered(5.0));
    }

    #[test]
    fn esc_during_k_hold_is_ignored() {
        let mut h = hold();
        h.key_down(VK_K, true, true, 0.0);
        h.key_down(VK_ESCAPE, true, true, 1.0);
        h.key_up(VK_ESCAPE);
        assert!(h.triggered(2.0));
    }
}
