# BabyBoop

Toddler keyboard game for macOS and Windows. Both native apps host the same page, `web/index.html`, swallow OS input while locked and forward it to the page.

## Layout

- `web/index.html`: all UI, sound and animation. Works in a plain browser too.
- `apps/macos/`: Swift/AppKit + WKWebView.
  - `src/main.swift`: entry point only.
  - `src/AppDelegate.swift`: window, web view, exit-hold timer, JS bridge.
  - `src/Lock.swift`: lock state, `startLock`/`endLock`, CGEventTap callback.
  - `src/Permission.swift`: Accessibility polling and opening Settings.
  - `src/Config.swift`, `src/SelfTest.swift`, `src/Log.swift`.
- `apps/windows/`: .NET 8 WPF + WebView2. Releases ship this one for now.
  - `BabyBoop.Core`: platform-free `ExitHold` and `KeyMap`; tests in `BabyBoop.Tests`.
  - `BabyBoop`: the app, including the keyboard/mouse hooks in `InputLock.cs`.
- `apps/windows-rs/`: Rust rewrite of the Windows app (Win32 + WebView2 via the `windows` and `webview2-com` crates). It replaces `apps/windows/` once it passes `TESTING.md` on a real PC; see NEXT_STEPS.md.
  - `src/exit_hold.rs`, `src/keymap.rs`, `src/input.rs`, `src/config.rs`: platform-free logic with unit tests.
  - `src/win/`: the app. `lock.rs` has the hooks; `web.rs` has WebView2.
- `site/`: static download site. `config.js` holds the repo name; `detect.js` picks the asset.
- `scripts/make_icons.sh`: regenerates the `.icns`, `.ico` and site icon from `make_icon.swift`.

## Native ↔ page contract

- Native calls `nativeStart()`, `nativeStop()`, `nativeInput(key, x, y)`, `nativeExitProgress(p)` and `nativePermission(granted)` (macOS only).
- The page posts `"start"`, `"quit"`, `"openSettings"` and `"audio=… calm=…"`.
  - On macOS it posts through `window.webkit.messageHandlers.native`.
  - On Windows it posts through `window.chrome.webview`.
- Keep all three sides in sync.

## Build and verify

- **macOS:**
  - `apps/macos/build.sh --no-install` compile-checks without touching the installed app.
  - After any local Mac build check (`--no-install`, `--dmg`), run `rm -rf apps/macos/build`. Otherwise Spotlight finds the leftover `BabyBoop.app` and shows two BabyBoop entries next to the installed one.
  - `--dmg` makes the release DMG. `lipo -archs` should print `x86_64 arm64`.
  - Installing changes the ad-hoc signature, so the user must re-add BabyBoop in Accessibility (named "Device Control and Data Access" on macOS 27+). Say so after every install.
  - `--selftest` and `--exit-after N` exercise the lock; check `~/Library/Logs/BabyBoop.log`.
- **Windows:**
  - `dotnet test apps/windows/BabyBoop.Tests` runs on macOS too.
  - Compile the app on macOS with `dotnet build apps/windows/BabyBoop -r win-x64 -p:EnableWindowsTargeting=true`.
  - The user hand-tests on real PCs; the checklist is `apps/windows/TESTING.md`.
- **Windows (Rust):** in `apps/windows-rs`:
  - `cargo test` runs the logic tests on macOS.
  - `cargo clippy --target x86_64-pc-windows-msvc --all-targets -- -D warnings` type-checks the Win32 code on macOS. Use `aarch64-pc-windows-msvc` for ARM.
  - Linking needs Windows. CI builds both `.exe` files and uploads them as the `BabyBoop-Windows-rust` artifact.
  - Rust comes from Homebrew's keg-only `rustup`, so put `/opt/homebrew/opt/rustup/bin` on `PATH` first.
- **Site:** run `node --test site/*.test.mjs`.
- **Browser preview:** `.claude/launch.json` serves `site/` on port 4173 (the `site` preview). Add a query string (`?fresh=1`) to force a reload. Pages under `file://` can't be scripted; serve them over http.

## Rules

- Exits: ⌥/Alt+Shift+K held 2s, Esc held 5s. Don't change without asking.
- Audio goes through the `master` gain node to `audio.destination`; never rewrite that line with a blanket replace.
- No speech or captions; tones only (user preference).
- Asset names are fixed: `BabyBoop-macOS.dmg`, `BabyBoop-Windows-x64.exe`, `BabyBoop-Windows-arm64.exe`. The site links to `releases/latest/download/<asset>`.
- Commit and push straight to `main` (personal repo, no PRs).
