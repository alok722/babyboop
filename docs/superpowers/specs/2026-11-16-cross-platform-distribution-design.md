# BabyBoop: macOS + Windows apps and download site

## Intent

BabyBoop is a free, open-source toddler "smash the keyboard" app. Any parent who finds the site should be able to download it, install it and run it on a Mac or a Windows PC without technical help. The site picks the right download for the visitor's OS. One codebase, one repo.

## Goals

- Native macOS app (exists) and a new native Windows app, both locking input as tightly as the OS allows.
- One macOS download that runs on both Intel and Apple silicon.
- A static website that detects the visitor's OS and offers the matching download, with step-by-step first-run help.
- Automated builds: pushing a version tag builds both apps and publishes a GitHub Release.
- A README and repo hygiene like a normal open-source project.

## Non-goals

- Paid code signing or notarization (no Apple Developer ID, no Windows certificate).
- Linux, iOS, Android native apps.
- Auto-update.
- Accounts, analytics, telemetry.

## Anti-goals

- A parent gets stuck at a security warning or permission prompt with no guidance.
- The Mac and Windows apps drift apart in look, sound or behaviour.
- Anything that needs a server to run.

## Constraints (decided)

- Hosting: GitHub Releases for binaries, GitHub Pages for the site.
- Name: **BabyBoop** (renamed from BabySmash to avoid confusion with Scott Hanselman's BabySmash). Bundle ID `app.babyboop`, log `~/Library/Logs/BabyBoop.log`.
- Repo: `github.com/alok722/babyboop` (originally planned as `Alok-Raj_JLLT/BabyBoop`), starting from a fresh initial commit. It may go public later; build everything so that needs nothing but a visibility switch (or new remote) and turning on Pages.
- Commits go straight to `main`, no PRs.
- Unsigned apps, so the site and README must walk users through:
  - macOS Gatekeeper: "Apple could not verify…" → System Settings → Privacy & Security → **Open Anyway** (right-click → Open no longer bypasses it on macOS 15+).
  - macOS Accessibility permission for the event tap.
  - Windows SmartScreen: "Windows protected your PC" → **More info** → **Run anyway**.
- Windows testing on real hardware (x64 and ARM) is done by the owner; CI only builds. A `--selftest` flag on Windows mirrors macOS for quick checks.
- Exits stay the same on both OSes: hold ⌥/Alt + Shift + K for 2 s, or hold Esc for 5 s.
- Tones only, no speech or captions (existing user preference).

## Design

### Shared toy page

`index.html` (current `Resources/index.html`) is the single source of the toy UI, sound and animation. Both native apps bundle the same file; the site also hosts it as a "play in browser" fallback. The native bridge contract stays as is:

- Native → page: `nativeStart()`, `nativeStop()`, `nativeInput(key, x, y)`, `nativeExitProgress(p)`.
- Page → native: post `"start"` / `"quit"`.

macOS adds an onboarding pair (see Builder's calls). On macOS the bridge is `window.webkit.messageHandlers.native`. On Windows (WebView2) it is `window.chrome.webview.postMessage`. The page detects whichever exists. Page copy that names exit keys reads ⌥ on Mac and Alt on Windows.

### Repo layout

```
README.md  CLAUDE.md  LICENSE  CONTRIBUTING.md  CHANGELOG.md  .gitignore  .editorconfig
web/index.html                  shared toy page
apps/macos/src/*.swift          current Swift sources
apps/macos/Resources/           Info.plist, AppIcon.icns
apps/macos/build.sh             builds universal .app, ad-hoc signs, makes .dmg
apps/windows/                   .NET project (see below)
site/                           landing page + screenshots for install help
scripts/make_icon.swift         icon generator (also emits .ico for Windows)
.github/workflows/release.yml   tag → build both → GitHub Release
.github/workflows/pages.yml     main → deploy site/ (+ web/ as /play) to Pages
.github/ISSUE_TEMPLATE/
docs/
```

This replaces the root-level `src/`, `Resources/`, `scripts/` added earlier; their contents move under `apps/macos/` and `web/`.

### macOS app

- Universal binary: compile `src/*.swift` twice (`-target arm64-apple-macos13` and `-target x86_64-apple-macos13`), merge with `lipo`, ad-hoc sign the bundle.
- Package as `BabyBoop-macOS.dmg` (drag-to-Applications layout via `hdiutil`).
- First-run onboarding replaces today's alert-and-quit: a screen inside the app explaining Accessibility, a button that opens the right System Settings pane, and the app noticing when permission is granted (poll `AXIsProcessTrusted`) without needing a relaunch.
- Known cost: each new version has a new ad-hoc signature, so users re-grant Accessibility after updating. Onboarding must say so.

### Windows app

- C# / .NET 8, WPF window hosting WebView2, loading the bundled `index.html`.
- Fullscreen, topmost, borderless window on the primary display; shown only after Start, like macOS.
- Low-level keyboard hook (`WH_KEYBOARD_LL`) swallows keys while locked, including Win, Alt+Tab, Alt+F4, Ctrl+Esc and media keys, and forwards them to the page. Mouse and touchpad are locked too (see Builder's calls).
- Cannot block Ctrl+Alt+Del or Win+L (OS-reserved). Document this.
- Exit holds detected in the hook, same timings as macOS.
- Publish self-contained single-file builds for `win-x64` and `win-arm64`: `BabyBoop-Windows-x64.exe`, `BabyBoop-Windows-arm64.exe`. Portable exe, no installer.
- Requires the WebView2 Runtime (preinstalled on Windows 10 21H2+ and 11). If missing, show a message with the Microsoft download link.

### Download site

- Static HTML/CSS/JS in `site/`, no framework, deployed to GitHub Pages.
- Detects OS (`navigator.userAgentData.platform`, falling back to `navigator.userAgent`) and architecture on Windows where `userAgentData.getHighEntropyValues(['architecture'])` is available; default to x64.
- Primary button downloads the matching asset via `https://github.com/<owner>/BabyBoop/releases/latest/download/<asset>`, so links never need updating per release. Secondary links list all downloads.
- Unsupported OS (Linux, iOS, Android, ChromeOS): explain desktop-only and offer "Play in browser" (`/play`, the shared page).
- Below the button: illustrated first-run steps for the detected OS (security warning → permission → Start → how to exit).
- Owner/repo name lives in one config constant so the account migration is a one-line change.

### Releases

- `release.yml` runs on tags `v*`: macOS runner builds the DMG, Windows runner builds both exes, then one job creates the GitHub Release with all three assets and notes from `CHANGELOG.md`.
- Asset names are fixed (above) so `latest/download` links keep working.

### Open-source hygiene

- README in the usual open-source shape: name + one-liner, screenshot/GIF, download links, features, install help per OS, exits, build from source per OS, project layout, contributing, license.
- `LICENSE`, `CONTRIBUTING.md`, `CHANGELOG.md`, issue templates.

## Builder's calls (made here, open to change)

- License: MIT.
- Windows stack: .NET 8 + WPF + WebView2 (reuses the shared page; Tauri/Electron rejected as heavier or a second toolchain for little gain).
- macOS package: DMG rather than zip.
- Windows package: portable exe rather than an installer.
- Site: plain static files, no framework.
- Windows input parity with macOS: a `WH_MOUSE_LL` hook swallows mouse/touchpad input, the cursor is hidden and replaced by a virtual cursor, and `ClipCursor` keeps it on the primary display. Precision-touchpad gestures and touchscreen edge swipes that hooks can't see are researched during the build; whatever can't be blocked is documented as a known gap.
- Accessibility onboarding lives in the shared page, so it looks like the rest of the app. This adds bridge messages: page → native `"openSettings"`, native → page `nativePermission(granted)`. The browser and Windows builds never show that state. The copy tells users to remove the stale BabyBoop entry (−) and add it again (+) after an update.
- Workflows are written now, but nothing is tagged and Pages isn't switched on until the repo goes public.

## Left to the builder

- Exact WPF/WebView2 wiring, hook implementation details, file split under `apps/windows/`.
- Site visual design (should match the toy page's playful look).
- How screenshots for the install help are produced.
- Whether to also add `win-x86`.
