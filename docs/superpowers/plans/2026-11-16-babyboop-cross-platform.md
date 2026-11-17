# BabyBoop Cross-Platform Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn the BabySmash macOS app into BabyBoop: a universal macOS app, a new Windows app, a download site that serves the right build per OS, and release automation, all in one repo.

**Architecture:** One shared toy page (`web/index.html`) is bundled by a Swift/AppKit app on macOS and a .NET 8 WPF + WebView2 app on Windows. Each native app swallows OS input while locked and forwards it to the page through a small bridge. A static site in `site/` detects the visitor's OS and links to fixed-name assets on GitHub Releases' `latest/download`.

**Tech Stack:** Swift 5 / AppKit / WebKit (macOS 13+), C# / .NET 8 / WPF / Microsoft.Web.WebView2 (Windows 10 21H2+), xUnit, plain HTML/CSS/JS, Node 24 `node --test`, GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-11-16-cross-platform-distribution-design.md`

## Global Constraints

- Product name **BabyBoop** everywhere; no "BabySmash" or "Baby Smash" left (`grep -ri "baby ?smash"` returns nothing outside `docs/`).
- macOS bundle ID `app.babyboop`; log `~/Library/Logs/BabyBoop.log`; macOS 13.0 minimum.
- Asset names, fixed: `BabyBoop-macOS.dmg`, `BabyBoop-Windows-x64.exe`, `BabyBoop-Windows-arm64.exe`.
- Exits on both OSes: hold ⌥/Alt + Shift + K for 2 s, or hold Esc for 5 s. Page copy says ⌥ Option on Mac, Alt on Windows.
- Tones only. No speech, no captions.
- Unsigned: macOS ad-hoc signed only; Windows unsigned.
- Repo `github.com/Alok-Raj_JLLT/BabyBoop`, private. Fresh history. Commit and push straight to `main`.
- Owner/repo string lives in one constant in `site/config.js`.
- Workflows are committed, but no tag is pushed and Pages is not enabled in this plan.
- No code comments unless logic can't explain itself.

## Review Focus

1. **Stale macOS Accessibility entry:** the switch shows on but `AXIsProcessTrusted()` is false after an update. Onboarding must keep polling and show the remove (−) / add (+) tip, never quit. (Task 3, Step 4.)
2. **iPad reporting as Mac:** iPadOS Safari sends a "Macintosh" UA. The site must treat Mac + `maxTouchPoints > 1` as unsupported, not offer the DMG. (Task 7 test `ipadIsUnsupported`.)
3. **Windows architecture unknown** (Firefox, older Chrome): default to x64, which runs on ARM under emulation. (Task 7 test `windowsDefaultsToX64`.)
4. **Exit hold with key autorepeat or a lost key-up:** autorepeat must not restart the timer, releasing must reset it, and dropping Shift or Alt mid K-hold must reset it. (Task 4 `ExitHoldTests`.)
5. **WebView2 Runtime missing on Windows:** show a message with the download link instead of crashing. (Task 5, Step 3.)

---

### Task 1: Fresh repo, new layout, rename to BabyBoop

**Files:**
- Move: `Resources/index.html` → `web/index.html`
- Move: `src/*.swift` → `apps/macos/src/`
- Move: `Resources/Info.plist`, `Resources/AppIcon.icns` → `apps/macos/Resources/`
- Move: `scripts/build.sh` → `apps/macos/build.sh`
- Keep: `scripts/make_icon.swift`
- Modify: `apps/macos/Resources/Info.plist`, `apps/macos/src/Log.swift`, `apps/macos/src/AppDelegate.swift`, `apps/macos/build.sh`, `web/index.html`, `.gitignore`, `CLAUDE.md`, `README.md`

**Interfaces:**
- Produces: `apps/macos/build.sh [--no-install]`, which builds `apps/macos/build/BabyBoop.app` and copies `web/index.html` into it. It installs to `~/Applications/BabyBoop.app` unless `--no-install` is given.

- [ ] **Step 1:** `rm -rf .git && git init -b main`, then do the moves above (plain `mv`; the history is fresh anyway).
- [ ] **Step 2:** Rename across the files above.
  - Info.plist: `CFBundleIdentifier` becomes `app.babyboop`, and `CFBundleName`/`CFBundleExecutable` become `BabyBoop`.
  - The log path becomes `BabyBoop.log`, and the alert, menu and page titles change to match.
  - In `index.html`, `<title>` and `<h1>` become `BabyBoop`.
  - In `build.sh`, `cd "$(dirname "$0")"`, then build from `src/*.swift` and `Resources/` and copy `../../web/index.html`.
  - In `.gitignore`, `build/` becomes `apps/macos/build/`.
- [ ] **Step 3:** Verify the build.
  - Run `apps/macos/build.sh --no-install && /usr/libexec/PlistBuddy -c "Print CFBundleIdentifier" apps/macos/build/BabyBoop.app/Contents/Info.plist`. Expected: `app.babyboop`.
  - Run `grep -riE "baby ?smash" --exclude-dir=docs --exclude-dir=.git .`. Expected: no output.
- [ ] **Step 4:** Commit `chore: BabyBoop repo layout and rename`. Add the remote `https://github.com/Alok-Raj_JLLT/BabyBoop.git` and push `main`.

### Task 2: Shared page bridge for both platforms + onboarding state

**Files:**
- Modify: `web/index.html` (the native hooks block, currently around lines 297–314)

**Interfaces:**
- Produces (page globals): `nativeStart()`, `nativeStop()`, `nativeInput(key: string|null, x?: number, y?: number)`, `nativeExitProgress(p: number)`, and the new `nativePermission(granted: boolean)`.
- Produces (page → native messages, plain strings): `"start"`, `"quit"`, `"openSettings"`, `"audio=<state> calm=<bool>"`.
- Bridge selection: `const host = window.webkit?.messageHandlers?.native ? "mac" : window.chrome?.webview ? "win" : null`, with `post(msg)` calling `webkit.messageHandlers.native.postMessage` or `chrome.webview.postMessage`.

- [ ] **Step 1:** Replace the `native` constant with `host` + `post(msg)`. Every existing `native.postMessage`/`native ?` use goes through these.
- [ ] **Step 2:** Exit copy per host.
  - `mac` keeps today's ⌥ Option text.
  - `win` reads "Parent exit: hold <kbd>Alt</kbd>+<kbd>Shift</kbd>+<kbd>K</kbd> 2s or <kbd>Esc</kbd> 5s". The hint says the same.
  - `null` (browser) keeps the 2s text.
- [ ] **Step 3:** Add an onboarding panel `#perm`, inside `#overlay` and hidden by default.
  - Title "One-time setup". Body explains that BabyBoop needs Accessibility to keep little fingers inside the app.
  - Button `#openSettings` posts `"openSettings"`.
  - Tip: "Just updated BabyBoop? Remove it from the list with −, then add it again with +."
  - `nativePermission(false)` shows `#perm` and disables `#start`. `nativePermission(true)` hides it and enables Start.
- [ ] **Step 4:** Verify in the browser pane with stubs.
  - Open `web/index.html?fresh=1`. Run `window.chrome = {webview: {postMessage: m => (window.__sent ||= []).push(m)}}`, then reload the inline module logic by re-running it. Or test with a copy that defines the stub before the script runs.
  - Expected: the exit text contains "Alt", and clicking Start pushes `"start"`.
  - Then run `nativePermission(false)`. Expected: `#perm` is visible and `#start.disabled === true`.
  - Then run `nativePermission(true)`. Expected: the reverse.
  - Plain browser (no stub): Start still starts a web session and the sound analyser peak is above 0. This guards the master-gain regression.
- [ ] **Step 5:** Commit `feat(web): cross-platform bridge and permission onboarding`.

### Task 3: macOS universal build, DMG, in-app onboarding

**Files:**
- Modify: `apps/macos/build.sh`, `apps/macos/src/AppDelegate.swift`
- Create: `apps/macos/src/Permission.swift`

**Interfaces:**
- Consumes: page `nativePermission(granted)` and message `"openSettings"` (Task 2).
- Produces: `apps/macos/build.sh --dmg`, which writes `apps/macos/build/BabyBoop-macOS.dmg`.
- Produces: `func startPermissionWatch()` and `func openAccessibilitySettings()` in `Permission.swift`.

- [ ] **Step 1:** Make `build.sh` universal.
  - Compile `src/*.swift` with `-target arm64-apple-macos13` and with `-target x86_64-apple-macos13` into `build/arm64` and `build/x86_64`.
  - Run `lipo -create` into `Contents/MacOS/BabyBoop`, then codesign as today.
  - Verify: run `lipo -archs apps/macos/build/BabyBoop.app/Contents/MacOS/BabyBoop`. Expected: `x86_64 arm64`.
- [ ] **Step 2:** Add `--dmg` to `build.sh`.
  - Stage the `.app` plus a symlink `Applications -> /Applications` in `build/dmg/`.
  - Run `hdiutil create -volname BabyBoop -srcfolder build/dmg -ov -format UDZO build/BabyBoop-macOS.dmg`.
  - `--dmg` implies `--no-install`.
  - Verify: `hdiutil verify apps/macos/build/BabyBoop-macOS.dmg` exits 0.
- [ ] **Step 3:** Replace the alert-and-quit in `AppDelegate.applicationDidFinishLaunching`.
  - Always build the window and web view.
  - On `didFinish`, call `startPermissionWatch()`. It polls `AXIsProcessTrusted()` every 1 s, calls `js("nativePermission(\(trusted))")` on each change, and logs `permission=<bool>`.
  - Prompt only when the user presses the button: `openAccessibilitySettings()` calls `AXIsProcessTrustedWithOptions` with prompt true, then opens `x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility`.
  - `Bridge` routes `"openSettings"` to `openAccessibilitySettings()`. `startLock()` refuses (logs `start refused: untrusted`) when `AXIsProcessTrusted()` is false.
- [ ] **Step 4:** Verify the untrusted path.
  - Install with `apps/macos/build.sh`. Before granting permission, open the app.
  - Expected: no alert and no quit, the onboarding panel shows, the log has `permission=false`, and pressing Start logs `start refused: untrusted`.
  - Grant permission in System Settings. Expected within about 1 s: the log has `permission=true` and the panel hides.
- [ ] **Step 5:** Run the regression self-test.
  - Run `open -W ~/Applications/BabyBoop.app --args --selftest`, then `grep -E "lock on|lock off reason=exit combo" ~/Library/Logs/BabyBoop.log`.
  - Expected: both lines are present, and `lock off` comes about 5 s after `Esc down (long)`.
- [ ] **Step 6:** Commit `feat(macos): universal binary, DMG, in-app Accessibility onboarding`.

### Task 4: Windows core logic library + tests (runs on macOS)

**Files:**
- Create: `apps/windows/BabyBoop.Core/BabyBoop.Core.csproj` (`net8.0`)
- Create: `apps/windows/BabyBoop.Core/ExitHold.cs`, `KeyMap.cs`
- Create: `apps/windows/BabyBoop.Tests/BabyBoop.Tests.csproj` (xUnit, `net8.0`), `ExitHoldTests.cs`, `KeyMapTests.cs`
- Create: `apps/windows/BabyBoop.sln`

**Interfaces:**
- Produces:
  ```csharp
  public sealed class ExitHold {
    public ExitHold(Func<DateTime> now);
    public void KeyDown(int vk, bool alt, bool shift, bool isRepeat);
    public void KeyUp(int vk);
    public void ModifiersChanged(bool alt, bool shift);
    public double Progress { get; }   // 0..1
    public bool Triggered { get; }    // Progress >= 1
  }
  public static class KeyMap {
    public const int VK_ESCAPE = 0x1B, VK_K = 0x4B;
    public static string? ToPageKey(int vk);  // "A".."Z", "0".."9", else null
  }
  ```
  Hold times: Esc 5 s, K (with Alt+Shift) 2 s.

- [ ] **Step 0:** `dotnet` is not installed on this Mac. Ask the user before running `brew install --cask dotnet-sdk`.
- [ ] **Step 1:** Write the failing tests in `ExitHoldTests` with a fake clock.
  - `EscHeld5sTriggers`: KeyDown Esc at t0, now = t0+5s → `Triggered`.
  - `EscHeld4_9sDoesNot`: t0+4.9s → `!Triggered`, and `Progress` is about 0.98.
  - `ReleaseResets`: Esc down, up at +1s, down at +2s, check at +6s → `!Triggered`. Check at +7s → `Triggered`.
  - `AutorepeatDoesNotRestart`: down at t0, repeat at +3s, check at +5s → `Triggered`.
  - `KNeedsAltShift`: K down with alt=false → `Progress == 0` at +3s.
  - `AltShiftK2s`: down with alt and shift, check at +2s → `Triggered`.
  - `DroppingShiftResetsK`: AltShiftK at t0, `ModifiersChanged(true,false)` at +1s, check at +3s → `!Triggered`.
- [ ] **Step 2:** Write `KeyMapTests`.
  - `0x41` → `"A"`, `0x5A` → `"Z"`, `0x30` → `"0"`, `0x39` → `"9"`, `0x1B` → `null`, `0x5B` (LWin) → `null`.
- [ ] **Step 3:** Add `apps/windows/**/bin/`, `apps/windows/**/obj/` and `apps/windows/out/` to `.gitignore`. Run `dotnet test apps/windows/BabyBoop.Tests`. Expected: FAIL (types missing).
- [ ] **Step 4:** Implement `ExitHold` and `KeyMap`, mirroring the macOS rules in `apps/macos/src/Lock.swift`.
- [ ] **Step 5:** Run `dotnet test apps/windows/BabyBoop.Tests`. Expected: all 13 pass.
- [ ] **Step 6:** Commit `feat(windows): exit-hold and key-map core with tests`.

### Task 5: Windows app (WPF + WebView2 + hooks)

**Files:**
- Create: `apps/windows/BabyBoop/BabyBoop.csproj`
  - `net8.0-windows`, `UseWPF`, `OutputType WinExe`
  - PackageReference `Microsoft.Web.WebView2`, ProjectReference Core
  - `ApplicationIcon` set to `Assets\AppIcon.ico`
  - `index.html` linked from `..\..\..\web\index.html`, copied to the output
- Create: `App.xaml(.cs)`, `MainWindow.xaml(.cs)`, `InputLock.cs` (hooks, ClipCursor, virtual cursor), `Native.cs` (P/Invoke), `SelfTest.cs`, `Log.cs`
- Create: `apps/windows/build.ps1`

**Interfaces:**
- Consumes: `ExitHold`, `KeyMap` (Task 4); page globals and messages (Task 2).
- Produces:
  - `apps/windows/build.ps1 -Rid win-x64|win-arm64` publishes a self-contained single file to `apps/windows/out/BabyBoop-Windows-<arch>.exe`.
  - Flags `--selftest` and `--exit-after N`.
  - Log at `%LOCALAPPDATA%\BabyBoop\BabyBoop.log`.

- [ ] **Step 1:** `MainWindow` hosts WebView2 and loads the bundled `index.html` via `SetVirtualHostNameToFolderMapping("app.babyboop", <exeDir>, Allow)` → `https://app.babyboop/index.html`.
  - `WebMessageReceived` routes `"start"` → `InputLock.Start()` and `"quit"` → `Shutdown()`, and logs everything else.
  - Before Start, the window is normal, maximized and closable.
- [ ] **Step 2:** `InputLock.Start()`:
  - Switches the window to borderless, topmost and fullscreen on the primary screen.
  - Installs `WH_KEYBOARD_LL` and `WH_MOUSE_LL`, hides the cursor and `ClipCursor`s to the primary bounds.
  - The keyboard hook feeds `ExitHold` and posts `nativeInput(KeyMap.ToPageKey(vk))` on non-repeat key-down via `ExecuteScriptAsync`, then returns 1 to swallow.
  - The mouse hook updates the virtual cursor from deltas and posts `nativeInput(null,x,y)` on button-down and on more than 120 wheel units, then swallows.
  - A 100 ms `DispatcherTimer` posts `nativeExitProgress(p)` on change and calls `Stop()` when triggered.
  - `Stop()` removes hooks, releases the clip, shows the cursor, restores the window and calls `nativeStop()`.
  - On test flags, start on load and exit after stop.
- [ ] **Step 3:** WebView2 missing. Catch `WebView2RuntimeNotFoundException` from `EnsureCoreWebView2Async` and show a MessageBox: "BabyBoop needs the Microsoft Edge WebView2 Runtime. Download it from https://go.microsoft.com/fwlink/p/?LinkId=2124703 and open BabyBoop again." Then exit.
- [ ] **Step 4:** `SelfTest.cs` uses `SendInput` to send a, LWin, Alt+Tab, Ctrl+Esc and media Play/Pause, then Esc for 1 s (no exit), then Esc held 6 s (exits at 5 s). It logs each step. Mirror `apps/macos/src/SelfTest.swift`.
- [ ] **Step 5:** `build.ps1` runs `dotnet publish apps/windows/BabyBoop -c Release -r $Rid --self-contained -p:PublishSingleFile=true -p:IncludeNativeLibrariesForSelfExtract=true` and renames the output to the fixed asset name.
- [ ] **Step 6:** Verify the build (macOS can cross-compile WPF with `-p:EnableWindowsTargeting=true`).
  - Run `dotnet build apps/windows/BabyBoop -r win-x64 -p:EnableWindowsTargeting=true`. Expected: Build succeeded.
- [ ] **Step 7:** Write `apps/windows/TESTING.md`, the hand-test checklist for the user on x64 and ARM.
  - Installation: SmartScreen flow, Start, and the `--selftest` log result.
  - Keys to try: Win, Alt+Tab, Alt+F4, Ctrl+Esc, media keys.
  - Gestures: 3/4-finger swipes and the touchscreen edge, with expected results.
  - Cursor confinement on a second monitor, both exits, and that Ctrl+Alt+Del is not blockable.
- [ ] **Step 8:** Commit `feat(windows): WPF + WebView2 app with input lock`.

### Task 6: Icons for both platforms

**Files:**
- Modify: `scripts/make_icon.swift`
- Create: `scripts/make_icons.sh`
- Output: `apps/macos/Resources/AppIcon.icns`, `apps/windows/BabyBoop/Assets/AppIcon.ico`, `site/assets/icon-512.png`

**Interfaces:**
- Produces: `scripts/make_icons.sh`, which regenerates all three outputs from `make_icon.swift`.

- [ ] **Step 1:** `make_icons.sh` renders 1024 px with `swift scripts/make_icon.swift`.
  - The `.icns` comes from an iconset (`sips` sizes 16–1024 at @1x/@2x) via `iconutil -c icns`.
  - The `.ico` packs 16/32/48/256 PNGs, using a small inline Python `struct` writer (ICO header plus PNG entries; no new dependency).
  - `icon-512.png` comes from `sips -z 512 512`.
- [ ] **Step 2:** Verify.
  - `file apps/windows/BabyBoop/Assets/AppIcon.ico` → `MS Windows icon resource - 4 icons`.
  - `apps/macos/build.sh --no-install` still succeeds.
- [ ] **Step 3:** Commit `feat: shared icon pipeline for macOS, Windows and site`.

### Task 7: Download site

**Files:**
- Create: `site/index.html`, `site/style.css`, `site/config.js`, `site/detect.js`, `site/main.js`
- Create: `site/detect.test.mjs`
- Create: `site/assets/` (icon from Task 6, install screenshots added by the user later; placeholders with alt text until then)

**Interfaces:**
- Produces:
  ```js
  // config.js
  export const REPO = "Alok-Raj_JLLT/BabyBoop";
  export const ASSETS = { mac: "BabyBoop-macOS.dmg", winX64: "BabyBoop-Windows-x64.exe", winArm64: "BabyBoop-Windows-arm64.exe" };
  export const downloadUrl = asset => `https://github.com/${REPO}/releases/latest/download/${asset}`;
  // detect.js
  export function pickDownload({ platform, userAgent, maxTouchPoints, architecture })
    // -> { os: "mac" | "windows" | "unsupported", asset: string | null, label: string }
  ```

- [ ] **Step 1:** Write the failing tests in `detect.test.mjs` (`node:test` + `assert`).
  - `macDesktop`: platform "macOS", touch 0 → `{os:"mac", asset:"BabyBoop-macOS.dmg"}`.
  - `macFromUAOnly`: platform undefined, UA containing "Macintosh", touch 0 → mac.
  - `ipadIsUnsupported`: UA "Macintosh", touch 5 → `os:"unsupported"`, `asset:null`.
  - `windowsArm`: platform "Windows", architecture "arm" → `BabyBoop-Windows-arm64.exe`.
  - `windowsDefaultsToX64`: platform "Windows", architecture undefined → x64.
  - `linuxUnsupported`, `androidUnsupported` (UA "Android"), `chromeOSUnsupported` (UA "CrOS") → unsupported.
- [ ] **Step 2:** Run `node --test site/`. Expected: FAIL (module missing).
- [ ] **Step 3:** Implement `pickDownload` in `site/detect.js`.
- [ ] **Step 4:** Run `node --test site/`. Expected: 8 pass.
- [ ] **Step 5:** Build the page.
  - `main.js` gathers inputs and awaits `navigator.userAgentData?.getHighEntropyValues(["architecture"])` in a try/catch.
  - It sets the primary button's href to `downloadUrl(asset)` and the label, e.g. "Download for Mac (Intel & Apple silicon)" or "Download for Windows (ARM)".
  - It always shows "Other downloads" links for all three assets.
  - For unsupported visitors: "BabyBoop runs on Mac and Windows computers", plus a "Play in browser" button to `play/`.
  - First-run steps per OS, as ordered lists with an image slot each.
    - Mac: open the DMG and drag BabyBoop to Applications. Open it, see "Apple could not verify…", then go to System Settings → Privacy & Security → Open Anyway. Turn on Accessibility (the in-app button), press Start, and exit with ⌥ Option+Shift+K 2 s or Esc 5 s.
    - Windows: run the exe, see "Windows protected your PC", click More info → Run anyway. Press Start, and exit with Alt+Shift+K 2 s or Esc 5 s. Note that Ctrl+Alt+Del always works.
  - Styling matches the toy page (gradient, rounded, big emoji).
- [ ] **Step 6:** Verify in the browser pane.
  - Serve with `npx --yes http-server site -p 4173` via preview, and copy `web/index.html` to `site/play/index.html` only for the local check (it is not committed).
  - Expected on this Mac: the button says Mac and its href ends with `BabyBoop-macOS.dmg`.
  - With `resize_window` mobile preset: unsupported copy plus Play in browser.
- [ ] **Step 7:** Commit `feat(site): OS-aware download page with install help`.

### Task 8: GitHub Actions

**Files:**
- Create: `.github/workflows/ci.yml`, `.github/workflows/release.yml`, `.github/workflows/pages.yml`

**Interfaces:**
- Consumes: `apps/macos/build.sh --dmg` (Task 3), `apps/windows/build.ps1 -Rid` (Task 5), `node --test site/` (Task 7), `dotnet test apps/windows/BabyBoop.Tests` (Task 4).

- [ ] **Step 1:** Write `ci.yml`, which runs on push to `main`.
  - Job `macos` (macos-latest): `apps/macos/build.sh --dmg`, then `lipo -archs` must print both archs.
  - Job `windows` (windows-latest): `dotnet test`, then `build.ps1` for both RIDs.
  - Job `site` (ubuntu-latest): `node --test site/`.
- [ ] **Step 2:** Write `release.yml`, which runs on tags `v*`.
  - The same build jobs upload artifacts. Job `release` (needs both) uses `gh release create "$GITHUB_REF_NAME"` with the three fixed-name assets.
  - Notes come from the matching `## [x.y.z]` section of `CHANGELOG.md`, extracted with `awk`.
  - `permissions: contents: write`.
- [ ] **Step 3:** Write `pages.yml`, triggered only by `workflow_dispatch` (Pages is off until the repo goes public).
  - It copies `site/` to `_site/` and `web/index.html` to `_site/play/index.html`, then uses `actions/upload-pages-artifact` and `actions/deploy-pages`.
- [ ] **Step 4:** Run `actionlint`. Expected: no output.
- [ ] **Step 5:** Commit `ci: build, release and pages workflows`, push, and check that `ci.yml` goes green with `gh run watch`.

### Task 9: Open-source docs and repo hygiene

**Files:**
- Create: `LICENSE` (MIT, copyright line `Copyright (c) 2026 BabyBoop contributors`), `CONTRIBUTING.md`, `CHANGELOG.md` (Keep a Changelog, `## [0.1.0] - Unreleased`), `.editorconfig`, `.github/ISSUE_TEMPLATE/bug_report.md`, `.github/ISSUE_TEMPLATE/feature_request.md`
- Modify: `README.md`, `CLAUDE.md`

- [ ] **Step 1:** Rewrite the README in open-source shape.
  - Name and one-liner, then a screenshot slot. Downloads link to the site and to Releases.
  - Features, then per-OS install help (same steps as the site), then exits and known limits.
  - Known limits: main display only; Ctrl+Alt+Del and Win+L can't be blocked; Accessibility must be re-granted after Mac updates; unsigned builds.
  - Build from source per OS, project layout, Contributing and License.
- [ ] **Step 2:** Update `CLAUDE.md` to the new layout.
  - Add the bridge contract including `openSettings`/`nativePermission`, the build and test commands for each part, and the existing rules.
- [ ] **Step 3:** Verify.
  - Every relative link in `README.md` resolves (`grep -o '](\./[^)]*' README.md` paths exist).
  - `grep -riE "baby ?smash" --exclude-dir=docs --exclude-dir=.git .` returns nothing.
- [ ] **Step 4:** Commit `docs: open-source README, license, contributing, changelog` and push `main`.
