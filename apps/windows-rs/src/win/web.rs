//! WebView2 setup. The page is `web/index.html`, compiled into the .exe and served from
//! https://app.babyboop/ so it gets a real origin (localStorage, audio autoplay rules).

use std::cell::RefCell;

use webview2_com::Microsoft::Web::WebView2::Win32::*;
use webview2_com::*;
use windows::core::{w, Interface, HSTRING, PCWSTR, PWSTR};
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::Shell::{SHCreateMemStream, ShellExecuteW};
use windows::Win32::UI::WindowsAndMessaging::{
    GetClientRect, MessageBoxW, IDYES, MB_ICONINFORMATION, MB_YESNO, SW_SHOWNORMAL,
};

use super::{log, on_page_message, page_loaded, BACKGROUND};

const ORIGIN: &str = "https://app.babyboop/";
const RUNTIME_URL: &str = "https://go.microsoft.com/fwlink/p/?LinkId=2124703";
const PAGE: &str = include_str!("../../../../web/index.html");

struct Web {
    controller: ICoreWebView2Controller,
    webview: ICoreWebView2,
}

thread_local! {
    static WEB: RefCell<Option<Web>> = const { RefCell::new(None) };
    static LOADED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn with_web(f: impl FnOnce(&Web)) {
    WEB.with(|w| {
        if let Ok(w) = w.try_borrow() {
            if let Some(web) = w.as_ref() {
                f(web);
            }
        }
    });
}

pub fn init(hwnd: HWND) {
    let options = CoreWebView2EnvironmentOptions::default();
    unsafe { options.set_additional_browser_arguments("--autoplay-policy=no-user-gesture-required".into()) };
    let options: ICoreWebView2EnvironmentOptions = options.into();
    let data = HSTRING::from(log::dir().join("WebView2").as_path());

    let handler = CreateCoreWebView2EnvironmentCompletedHandler::create(Box::new(move |result, env| {
        match (result, env) {
            (Ok(()), Some(env)) => create_controller(hwnd, env),
            (result, _) => runtime_missing(&format!("environment: {result:?}")),
        }
        Ok(())
    }));
    if let Err(e) = unsafe { CreateCoreWebView2EnvironmentWithOptions(PCWSTR::null(), &data, &options, &handler) } {
        runtime_missing(&format!("loader: {e}"));
    }
}

fn create_controller(hwnd: HWND, env: ICoreWebView2Environment) {
    let env2 = env.clone();
    let handler = CreateCoreWebView2ControllerCompletedHandler::create(Box::new(move |result, controller| {
        match (result, controller) {
            (Ok(()), Some(controller)) => {
                if let Err(e) = unsafe { setup(hwnd, env2, controller) } {
                    runtime_missing(&format!("setup: {e}"));
                }
            }
            (result, _) => runtime_missing(&format!("controller: {result:?}")),
        }
        Ok(())
    }));
    if let Err(e) = unsafe { env.CreateCoreWebView2Controller(hwnd, &handler) } {
        runtime_missing(&format!("controller: {e}"));
    }
}

unsafe fn setup(
    hwnd: HWND,
    env: ICoreWebView2Environment,
    controller: ICoreWebView2Controller,
) -> windows::core::Result<()> {
    let (r, g, b) = BACKGROUND;
    controller
        .cast::<ICoreWebView2Controller2>()?
        .SetDefaultBackgroundColor(COREWEBVIEW2_COLOR {
            A: 255,
            R: r,
            G: g,
            B: b,
        })?;
    let mut rect = RECT::default();
    GetClientRect(hwnd, &mut rect)?;
    controller.SetBounds(rect)?;

    let webview = controller.CoreWebView2()?;
    let settings = webview.Settings()?;
    settings.SetAreDefaultContextMenusEnabled(false)?;
    settings.SetAreDevToolsEnabled(false)?;
    settings.SetIsZoomControlEnabled(false)?;
    settings.SetIsStatusBarEnabled(false)?;
    settings
        .cast::<ICoreWebView2Settings3>()?
        .SetAreBrowserAcceleratorKeysEnabled(false)?;

    let mut token = 0i64;
    webview.AddWebResourceRequestedFilter(
        &HSTRING::from(format!("{ORIGIN}*")),
        COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL,
    )?;
    webview.add_WebResourceRequested(
        &WebResourceRequestedEventHandler::create(Box::new(move |_, args| {
            if let Some(args) = args {
                let stream = SHCreateMemStream(Some(PAGE.as_bytes()));
                let response = env.CreateWebResourceResponse(
                    stream.as_ref(),
                    200,
                    w!("OK"),
                    w!("Content-Type: text/html; charset=utf-8"),
                )?;
                args.SetResponse(&response)?;
            }
            Ok(())
        })),
        &mut token,
    )?;

    webview.add_WebMessageReceived(
        &WebMessageReceivedEventHandler::create(Box::new(|_, args| {
            if let Some(args) = args {
                let mut message = PWSTR::null();
                if args.TryGetWebMessageAsString(&mut message).is_ok() {
                    on_page_message(&take_pwstr(message));
                }
            }
            Ok(())
        })),
        &mut token,
    )?;

    webview.add_NavigationCompleted(
        &NavigationCompletedEventHandler::create(Box::new(|_, _| {
            if !LOADED.with(|l| l.replace(true)) {
                page_loaded();
            }
            Ok(())
        })),
        &mut token,
    )?;

    webview.Navigate(&HSTRING::from(format!("{ORIGIN}index.html")))?;
    WEB.with(|w| *w.borrow_mut() = Some(Web { controller, webview }));
    log::write("webview ready");
    Ok(())
}

pub fn resize(hwnd: HWND) {
    with_web(|web| unsafe {
        let mut rect = RECT::default();
        if GetClientRect(hwnd, &mut rect).is_ok() {
            let _ = web.controller.SetBounds(rect);
        }
    });
}

pub fn run_scripts(scripts: Vec<String>) {
    with_web(|web| {
        for script in scripts {
            unsafe {
                let _ = web.webview.ExecuteScript(
                    &HSTRING::from(script),
                    None::<&ICoreWebView2ExecuteScriptCompletedHandler>,
                );
            }
        }
    });
}

pub fn close() {
    WEB.with(|w| {
        if let Ok(mut w) = w.try_borrow_mut() {
            if let Some(web) = w.take() {
                unsafe {
                    let _ = web.controller.Close();
                }
            }
        }
    });
}

fn runtime_missing(detail: &str) {
    log::write(&format!("webview2 init failed: {detail}"));
    unsafe {
        let answer = MessageBoxW(
            None,
            w!("BabyBoop needs the Microsoft Edge WebView2 Runtime, and it couldn't be started on this PC.\n\nClick Yes to open Microsoft's download page. Install it, then open BabyBoop again."),
            w!("BabyBoop"),
            MB_YESNO | MB_ICONINFORMATION,
        );
        if answer == IDYES {
            ShellExecuteW(None, w!("open"), &HSTRING::from(RUNTIME_URL), None, None, SW_SHOWNORMAL);
        }
    }
    super::close();
}
