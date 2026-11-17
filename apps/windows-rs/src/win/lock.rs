//! The input lock: low-level keyboard and mouse hooks that swallow everything while a game runs.
//!
//! The hooks run on their own thread with its own message loop, so a busy UI thread
//! (WebView2, timers) can never make Windows time the hooks out and let keys through.

use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Mutex};
use std::thread::{self, JoinHandle};

use babyboop_core::input::LockInput;
use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::Input::KeyboardAndMouse::SetFocus;
use windows::Win32::UI::WindowsAndMessaging::*;

use super::{config, hwnd, js, log, now, post, TIMER_TICK, WM_APP_NUDGE};

static LOCKED: AtomicBool = AtomicBool::new(false);
static INPUT: Mutex<Option<LockInput>> = Mutex::new(None);

struct HookThread {
    handle: JoinHandle<()>,
    id: u32,
}

thread_local! {
    static HOOKS: RefCell<Option<HookThread>> = const { RefCell::new(None) };
    static LAST_PROGRESS: Cell<f64> = const { Cell::new(-1.0) };
}

pub fn locked() -> bool {
    LOCKED.load(Ordering::SeqCst)
}

pub fn start() {
    if locked() {
        return;
    }
    let hwnd = hwnd();
    let (w, h) = unsafe { (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)) };
    unsafe {
        let _ = ShowWindow(hwnd, SW_RESTORE);
        SetWindowLongPtrW(hwnd, GWL_STYLE, (WS_POPUP | WS_VISIBLE).0 as isize);
        let _ = SetWindowPos(hwnd, Some(HWND_TOPMOST), 0, 0, w, h, SWP_FRAMECHANGED | SWP_SHOWWINDOW);
        let _ = SetForegroundWindow(hwnd);
        let _ = SetFocus(Some(hwnd));
    }
    let scale = unsafe { GetDpiForWindow(hwnd) } as f64 / 96.0;
    let input = LockInput::new((w, h), scale);
    let (px, py) = input.park();
    unsafe {
        let _ = SetCursorPos(px, py);
        let _ = ClipCursor(Some(&RECT {
            left: 0,
            top: 0,
            right: w,
            bottom: h,
        }));
    }
    *INPUT.lock().unwrap() = Some(input);
    LAST_PROGRESS.with(|p| p.set(-1.0));
    LOCKED.store(true, Ordering::SeqCst);

    match install_hooks() {
        Ok(hooks) => HOOKS.with(|h| *h.borrow_mut() = Some(hooks)),
        Err(failed) => {
            stop(&format!("no hook ({failed})"));
            return;
        }
    }
    unsafe {
        SetTimer(Some(hwnd), TIMER_TICK, 100, None);
    }
    js("nativeStart()");
    post(WM_APP_NUDGE);
    let c = config();
    log::write(&format!(
        "lock on, screen={w}x{h} scale={scale} exitAfter={} selftest={}",
        c.exit_after.map_or("none".into(), |s| s.to_string()),
        c.selftest
    ));
}

pub fn stop(reason: &str) {
    LOCKED.store(false, Ordering::SeqCst);
    let hwnd = hwnd();
    unsafe {
        let _ = KillTimer(Some(hwnd), TIMER_TICK);
    }
    remove_hooks();
    let summary = INPUT.lock().unwrap().take().map(|i| i.summary()).unwrap_or_default();
    LAST_PROGRESS.with(|p| p.set(-1.0));
    log::write(&format!("lock off reason={reason} messagesSeen=[{summary}]"));
    unsafe {
        let _ = ClipCursor(None);
        SetWindowLongPtrW(hwnd, GWL_STYLE, (WS_OVERLAPPEDWINDOW | WS_VISIBLE).0 as isize);
        let _ = SetWindowPos(
            hwnd,
            Some(HWND_NOTOPMOST),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED | SWP_SHOWWINDOW,
        );
        let _ = ShowWindow(hwnd, SW_MAXIMIZE);
    }
    js("nativeStop()");
    if config().test_mode() {
        super::close();
    }
}

/// Every 100 ms while locked: report the exit-hold progress and end the lock when it's full.
pub fn tick() {
    let p = INPUT.lock().unwrap().as_ref().map_or(0.0, |i| i.exit_progress(now()));
    if LAST_PROGRESS.with(|last| last.replace(p)) != p {
        js(format!("nativeExitProgress({p})"));
    }
    if p >= 1.0 {
        stop("exit combo");
    }
}

/// Wiggle the parked cursor once so Windows hides it over the page right away.
pub fn nudge() {
    let park = INPUT.lock().unwrap().as_ref().map(|i| i.park());
    if let Some((x, y)) = park {
        unsafe {
            let _ = SetCursorPos(x + 1, y);
            let _ = SetCursorPos(x, y);
        }
    }
}

pub fn forget_held(why: &str) {
    if !locked() {
        return;
    }
    if let Some(i) = INPUT.lock().unwrap().as_mut() {
        i.forget_held();
    }
    log::write(&format!("held keys cleared ({why})"));
}

fn install_hooks() -> Result<HookThread, String> {
    let (tx, rx) = mpsc::channel::<Result<u32, String>>();
    let handle = thread::Builder::new()
        .name("BabyBoop input hooks".into())
        .spawn(move || unsafe {
            let mut msg = MSG::default();
            // Create this thread's message queue before anyone posts WM_QUIT to it.
            let _ = PeekMessageW(&mut msg, None, WM_USER, WM_USER, PM_NOREMOVE);
            let module: Option<HINSTANCE> = GetModuleHandleW(None).ok().map(Into::into);
            let keyboard = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), module, 0);
            let mouse = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), module, 0);
            let mut failed = Vec::new();
            if let Err(e) = &keyboard {
                log::write(&format!("keyboard hook FAILED error={e}"));
                failed.push("keyboard");
            }
            if let Err(e) = &mouse {
                log::write(&format!("mouse hook FAILED error={e}"));
                failed.push("mouse");
            }
            let ok = failed.is_empty();
            let _ = tx.send(if ok {
                Ok(GetCurrentThreadId())
            } else {
                Err(failed.join(","))
            });
            if ok {
                while GetMessageW(&mut msg, None, 0, 0).0 > 0 {}
            }
            if let Ok(h) = keyboard {
                let _ = UnhookWindowsHookEx(h);
            }
            if let Ok(h) = mouse {
                let _ = UnhookWindowsHookEx(h);
            }
        })
        .map_err(|e| format!("thread: {e}"))?;
    match rx.recv() {
        Ok(Ok(id)) => Ok(HookThread { handle, id }),
        Ok(Err(failed)) => {
            let _ = handle.join();
            Err(failed)
        }
        Err(_) => Err("hook thread died".into()),
    }
}

fn remove_hooks() {
    if let Some(hooks) = HOOKS.with(|h| h.borrow_mut().take()) {
        unsafe {
            let _ = PostThreadMessageW(hooks.id, WM_QUIT, WPARAM(0), LPARAM(0));
        }
        let _ = hooks.handle.join();
    }
}

/// Counts the message, and logs the first of each kind so a test run shows what arrived.
fn note(input: &mut LockInput, msg: u32) {
    if input.note(msg) {
        log::write(&format!("first message 0x{msg:X}"));
    }
}

unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < 0 || !locked() {
        return CallNextHookEx(None, code, wparam, lparam);
    }
    let key = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
    let msg = wparam.0 as u32;
    let script = {
        let mut guard = INPUT.lock().unwrap();
        let Some(input) = guard.as_mut() else { return LRESULT(1) };
        note(input, msg);
        match msg {
            WM_KEYDOWN | WM_SYSKEYDOWN => input.key_down(key.vkCode, now()),
            WM_KEYUP | WM_SYSKEYUP => {
                input.key_up(key.vkCode);
                None
            }
            _ => None,
        }
    };
    if let Some(script) = script {
        js(script);
    }
    LRESULT(1)
}

unsafe extern "system" fn mouse_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < 0 || !locked() {
        return CallNextHookEx(None, code, wparam, lparam);
    }
    let mouse = &*(lparam.0 as *const MSLLHOOKSTRUCT);
    let msg = wparam.0 as u32;
    let script = {
        let mut guard = INPUT.lock().unwrap();
        let Some(input) = guard.as_mut() else { return LRESULT(1) };
        note(input, msg);
        match msg {
            WM_MOUSEMOVE => {
                input.mouse_move((mouse.pt.x, mouse.pt.y));
                None
            }
            WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => Some(input.button()),
            WM_MOUSEWHEEL | WM_MOUSEHWHEEL => input.wheel((mouse.mouseData >> 16) as u16 as i16),
            _ => None,
        }
    };
    if let Some(script) = script {
        js(script);
    }
    LRESULT(1)
}
