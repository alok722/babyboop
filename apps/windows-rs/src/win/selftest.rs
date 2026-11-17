//! `--selftest`: injects keys a toddler (or the OS) might use to escape, then a short and a
//! long Esc. The log should show them swallowed and the lock ending only after the long Esc.

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP,
    VIRTUAL_KEY,
};

use super::log;

const VK_A: u16 = 0x41;
const VK_LWIN: u16 = 0x5B;
const VK_MENU: u16 = 0x12;
const VK_TAB: u16 = 0x09;
const VK_CONTROL: u16 = 0x11;
const VK_ESCAPE: u16 = 0x1B;
const VK_MEDIA_PLAY_PAUSE: u16 = 0xB3;

const D: bool = false; // key down
const U: bool = true; // key up

type Step = (f64, &'static str, &'static [(u16, bool)]);

const STEPS: &[Step] = &[
    (1.0, "a", &[(VK_A, D), (VK_A, U)]),
    (1.7, "LWin", &[(VK_LWIN, D), (VK_LWIN, U)]),
    (2.4, "Alt+Tab", &[(VK_MENU, D), (VK_TAB, D), (VK_TAB, U), (VK_MENU, U)]),
    (
        3.1,
        "Ctrl+Esc",
        &[(VK_CONTROL, D), (VK_ESCAPE, D), (VK_ESCAPE, U), (VK_CONTROL, U)],
    ),
    (
        3.8,
        "Media Play/Pause",
        &[(VK_MEDIA_PLAY_PAUSE, D), (VK_MEDIA_PLAY_PAUSE, U)],
    ),
    (4.5, "Esc down (short)", &[(VK_ESCAPE, D)]),
    (5.5, "Esc up after 1s", &[(VK_ESCAPE, U)]),
    (6.5, "Esc down (long)", &[(VK_ESCAPE, D)]),
    (12.5, "Esc up after 6s", &[(VK_ESCAPE, U)]),
];

/// Seconds after the lock starts at which each step fires.
pub fn schedule() -> impl Iterator<Item = f64> {
    STEPS.iter().map(|s| s.0)
}

pub fn fire(index: usize) {
    let Some(&(_, name, keys)) = STEPS.get(index) else {
        return;
    };
    let inputs: Vec<INPUT> = keys
        .iter()
        .map(|&(vk, up)| {
            let mut flags = KEYBD_EVENT_FLAGS(0);
            if up {
                flags |= KEYEVENTF_KEYUP;
            }
            if vk == VK_LWIN || vk == VK_MEDIA_PLAY_PAUSE {
                flags |= KEYEVENTF_EXTENDEDKEY;
            }
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(vk),
                        wScan: 0,
                        dwFlags: flags,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            }
        })
        .collect();
    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
    log::write(&format!("selftest posted {name}"));
}
