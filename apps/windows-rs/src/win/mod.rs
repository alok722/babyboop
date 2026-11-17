//! The Windows app: one top-level window hosting WebView2, plus the input lock.
//!
//! Threads:
//! - The UI thread owns the window, WebView2 and all timers.
//! - While locked, a hook thread runs the low-level keyboard and mouse hooks (see `lock.rs`).
//!   It never touches WebView2: it queues JavaScript with `js()` and the UI thread runs it.

mod lock;
mod log;
mod selftest;
mod web;

use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use babyboop_core::config::Config;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{CreateSolidBrush, UpdateWindow};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::RemoteDesktop::{
    WTSRegisterSessionNotification, WTSUnRegisterSessionNotification, NOTIFY_FOR_THIS_SESSION,
};
use windows::Win32::UI::WindowsAndMessaging::*;

pub(crate) const BACKGROUND: (u8, u8, u8) = (0x1b, 0x14, 0x46);

const WM_APP_JS: u32 = WM_APP + 1;
const WM_APP_NUDGE: u32 = WM_APP + 2;

const TIMER_TICK: usize = 1;
const TIMER_EXIT_AFTER: usize = 2;
const TIMER_SELFTEST: usize = 100;

static HWND_RAW: AtomicIsize = AtomicIsize::new(0);
static CONFIG: OnceLock<Config> = OnceLock::new();
static START: OnceLock<Instant> = OnceLock::new();
static JS_QUEUE: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub(crate) fn config() -> &'static Config {
    CONFIG.get_or_init(Config::default)
}

/// Seconds since launch; the exit-hold clock.
pub(crate) fn now() -> f64 {
    START.get_or_init(Instant::now).elapsed().as_secs_f64()
}

pub(crate) fn hwnd() -> HWND {
    HWND(HWND_RAW.load(Ordering::SeqCst) as *mut _)
}

pub(crate) fn post(msg: u32) {
    unsafe {
        let _ = PostMessageW(Some(hwnd()), msg, WPARAM(0), LPARAM(0));
    }
}

/// Runs JavaScript in the page. Safe to call from any thread; scripts run in order on the UI thread.
pub(crate) fn js(script: impl Into<String>) {
    JS_QUEUE.lock().unwrap().push(script.into());
    post(WM_APP_JS);
}

pub(crate) fn close() {
    post(WM_CLOSE);
}

pub fn run() {
    log::reset();
    let args: Vec<String> = std::env::args().collect();
    let config = CONFIG.get_or_init(|| Config::parse(args.clone()));
    now();
    log::write(&format!(
        "BabyBoop {} starting, selftest={} exitAfter={:?}",
        env!("BABYBOOP_VERSION"),
        config.selftest,
        config.exit_after
    ));

    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let instance: HINSTANCE = GetModuleHandleW(None).expect("module handle").into();
        let icon_id = PCWSTR(1 as _); // winresource embeds the icon as resource 1
        let icon = |size: i32| {
            LoadImageW(Some(instance), icon_id, IMAGE_ICON, size, size, LR_DEFAULTCOLOR)
                .map(|h| HICON(h.0))
                .unwrap_or_default()
        };
        let (r, g, b) = BACKGROUND;
        let class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            hInstance: instance,
            hIcon: icon(GetSystemMetrics(SM_CXICON)),
            hIconSm: icon(GetSystemMetrics(SM_CXSMICON)),
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap_or_default(),
            hbrBackground: CreateSolidBrush(COLORREF(r as u32 | (g as u32) << 8 | (b as u32) << 16)),
            lpszClassName: w!("BabyBoopWindow"),
            ..Default::default()
        };
        RegisterClassExW(&class);

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("BabyBoopWindow"),
            w!("BabyBoop"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1200,
            800,
            None,
            None,
            Some(instance),
            None,
        )
        .expect("create window");
        HWND_RAW.store(hwnd.0 as isize, Ordering::SeqCst);
        let _ = WTSRegisterSessionNotification(hwnd, NOTIFY_FOR_THIS_SESSION);
        let _ = ShowWindow(hwnd, SW_SHOWMAXIMIZED);
        let _ = UpdateWindow(hwnd);

        web::init(hwnd);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).0 > 0 {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    log::write("exit");
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_SIZE => web::resize(hwnd),
        WM_APP_JS => {
            let scripts = std::mem::take(&mut *JS_QUEUE.lock().unwrap());
            web::run_scripts(scripts);
        }
        WM_APP_NUDGE => lock::nudge(),
        WM_TIMER => on_timer(hwnd, wparam.0),
        WM_ACTIVATE => {
            if (wparam.0 & 0xFFFF) as u32 != WA_INACTIVE {
                lock::forget_held("window activated");
            }
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
        WM_WTSSESSION_CHANGE => lock::forget_held("session switch"),
        WM_DPICHANGED => {
            // Per-monitor DPI: take the size Windows suggests, unless we're covering the screen.
            if !lock::locked() {
                let r = &*(lparam.0 as *const RECT);
                let _ = SetWindowPos(
                    hwnd,
                    None,
                    r.left,
                    r.top,
                    r.right - r.left,
                    r.bottom - r.top,
                    SWP_NOZORDER | SWP_NOACTIVATE,
                );
            }
        }
        WM_CLOSE => {
            let _ = DestroyWindow(hwnd);
        }
        WM_DESTROY => {
            if lock::locked() {
                lock::stop("window closed");
            }
            web::close();
            let _ = WTSUnRegisterSessionNotification(hwnd);
            PostQuitMessage(0);
        }
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
    LRESULT(0)
}

fn on_timer(hwnd: HWND, id: usize) {
    match id {
        TIMER_TICK => lock::tick(),
        TIMER_EXIT_AFTER => {
            unsafe {
                let _ = KillTimer(Some(hwnd), id);
            }
            if lock::locked() {
                let s = config().exit_after.unwrap_or_default();
                lock::stop(&format!("exit-after {s}s"));
            }
        }
        id if id >= TIMER_SELFTEST => {
            unsafe {
                let _ = KillTimer(Some(hwnd), id);
            }
            selftest::fire(id - TIMER_SELFTEST);
        }
        _ => {}
    }
}

/// Called once the page has loaded. Test modes lock straight away.
pub(crate) fn page_loaded() {
    log::write("page loaded");
    let config = config();
    if !config.test_mode() {
        return;
    }
    lock::start();
    unsafe {
        if let Some(s) = config.exit_after {
            SetTimer(Some(hwnd()), TIMER_EXIT_AFTER, (s * 1000.0).round() as u32, None);
        }
        if config.selftest {
            for (i, at) in selftest::schedule().enumerate() {
                SetTimer(Some(hwnd()), TIMER_SELFTEST + i, (at * 1000.0).round() as u32, None);
            }
        }
    }
}

pub(crate) fn on_page_message(message: &str) {
    log::write(&format!("page: {message}"));
    match message {
        "start" => lock::start(),
        "quit" => close(),
        _ => {}
    }
}
