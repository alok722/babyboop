<p align="center">
  <img src="site/assets/icon-512.png" alt="BabyBoop icon" width="120" height="120">
</p>

<h1 align="center">BabyBoop</h1>

<p align="center">
  <b>A free, open-source toddler keyboard game for Mac and Windows.</b><br>
  Every key, click and tap makes a big emoji, a happy tone and a burst of confetti,<br>
  while the computer stays locked so little hands can't wander off.
</p>

<p align="center">
  <a href="https://alok722.github.io/babyboop/"><b>Website</b></a> ·
  <a href="https://alok722.github.io/babyboop/play/"><b>Try it in your browser</b></a> ·
  <a href="#download"><b>Download</b></a> ·
  <a href="#contributing"><b>Contribute</b></a>
</p>

<p align="center">
  <a href="https://github.com/alok722/babyboop/actions/workflows/ci.yml"><img alt="CI" src="https://github.com/alok722/babyboop/actions/workflows/ci.yml/badge.svg"></a>
  <a href="https://github.com/alok722/babyboop/releases/latest"><img alt="Latest release" src="https://img.shields.io/github/v/release/alok722/babyboop?include_prereleases&label=release&color=ff5a36"></a>
  <a href="LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-ff5a36.svg"></a>
</p>

## Why an app and not just a web page?

You can [play BabyBoop in any browser](https://alok722.github.io/babyboop/play/), but a browser isn't allowed to block system keys. One swipe, <kbd>⌘</kbd>+<kbd>Tab</kbd> or <kbd>⌘</kbd>+<kbd>Q</kbd> and your toddler is in your email. The native app runs the same game full screen and swallows those keys and gestures until a grown-up holds the exit combo.

| | Browser | Mac / Windows app |
|---|---|---|
| Install | None | One small download |
| ⌘Tab / Alt+Tab, Windows key, ⌘Q | Leave or close the game | Blocked |
| Trackpad swipes, Mission Control, Dock | Work as usual | Blocked |
| Phones, tablets, Linux | Works | Not available |

## Download

| Platform | Download |
|---|---|
| Mac (Intel & Apple silicon, macOS 13+) | [BabyBoop-macOS.dmg](https://github.com/alok722/babyboop/releases/latest/download/BabyBoop-macOS.dmg) |
| Windows 10/11 (most PCs, Intel/AMD) | [BabyBoop-Windows-x64.exe](https://github.com/alok722/babyboop/releases/latest/download/BabyBoop-Windows-x64.exe) |
| Windows 11 on ARM (Windows 10 on ARM isn't supported) | [BabyBoop-Windows-arm64.exe](https://github.com/alok722/babyboop/releases/latest/download/BabyBoop-Windows-arm64.exe) |

All versions are on the [Releases](https://github.com/alok722/babyboop/releases) page. The [website](https://alok722.github.io/babyboop/) picks the right file for your computer and has a step-by-step setup guide.

## Features

- Same emoji for the same letter (A 🍎, B 🐻 …), stars for digits, random emoji for everything else
- Tones on every press, confetti, and a little celebration every 25 smashes
- Smash counter per session
- Slowly shifting background colours
- Quiet public-domain nursery melodies and surprise pops when things go quiet
- Calm mode: softer volume, slower animations, fewer things on screen
- System keys and trackpad gestures are blocked while playing

## Install

BabyBoop is free and isn't signed with a paid Apple or Microsoft certificate, so both systems show a warning the first time. That's expected.

### Mac

1. Open `BabyBoop-macOS.dmg` and drag **BabyBoop** into **Applications**.
2. Open BabyBoop. macOS says *"Apple could not verify BabyBoop"*. Click **Done**, open **System Settings → Privacy & Security**, scroll down and click **Open Anyway**.
3. BabyBoop asks for **Accessibility** access. Click **Open Accessibility settings** and turn BabyBoop on. This is what lets it block system keys and gestures.
4. Press **Start**.

After updating BabyBoop, macOS forgets the permission. In Accessibility, remove BabyBoop with **−** and add it again with **+**.

### Windows

1. Run the `.exe`. Windows says *"Windows protected your PC"*. Click **More info**, then **Run anyway**.
2. Press **Start**.

BabyBoop needs the Microsoft Edge WebView2 Runtime, which comes with Windows 10 (21H2 or later) and Windows 11. If it's missing, BabyBoop tells you where to get it.

## Exit

| | Mac | Windows |
|---|---|---|
| Hold 2 seconds | <kbd>⌥ Option</kbd> + <kbd>Shift</kbd> + <kbd>K</kbd> | <kbd>Alt</kbd> + <kbd>Shift</kbd> + <kbd>K</kbd> |
| Hold 5 seconds | <kbd>Esc</kbd> | <kbd>Esc</kbd> |

## Known limits

- The Windows builds pass CI but haven't been tested on real PCs yet. If something escapes the lock, please [open an issue](https://github.com/alok722/babyboop/issues/new/choose).
- Only the main display is covered.
- Windows always lets <kbd>Ctrl</kbd>+<kbd>Alt</kbd>+<kbd>Del</kbd> and <kbd>Win</kbd>+<kbd>L</kbd> through; no app can block them.
- Some Windows touchpad and touchscreen edge gestures may get through. See [apps/windows/TESTING.md](apps/windows/TESTING.md).
- On a phone, tablet or Linux, use [Play in browser](https://alok722.github.io/babyboop/play/); keys and gestures can't be locked there.

## Build from source

### Mac

Needs Xcode Command Line Tools (`xcode-select --install`).

```bash
apps/macos/build.sh              # build universal app, install to ~/Applications
apps/macos/build.sh --no-install # build only, into apps/macos/build/
apps/macos/build.sh --dmg        # build apps/macos/build/BabyBoop-macOS.dmg
```

Test flags: `--selftest` (posts fake system keys and an Esc hold, then exits) and `--exit-after N`. The log is `~/Library/Logs/BabyBoop.log`.

### Windows

Needs the .NET 8 SDK.

```powershell
dotnet test apps/windows/BabyBoop.Tests
apps/windows/build.ps1 -Rid win-x64     # or win-arm64; output in apps/windows/out/
```

Test flags: `--selftest` and `--exit-after N`. The log is `%LOCALAPPDATA%\BabyBoop\BabyBoop.log`.

### Website

```bash
node --test site/*.test.mjs
python3 -m http.server -d site 4173   # open http://localhost:4173
```

### Icons

```bash
scripts/make_icons.sh   # regenerates the macOS .icns, Windows .ico and site icon
```

## Project layout

```
web/index.html          the toy itself (UI, sound, animation), shared by both apps
apps/macos/             Swift/AppKit app hosting the page in WKWebView
apps/windows/           .NET 8 WPF app hosting the page in WebView2
site/                   website (GitHub Pages): https://alok722.github.io/babyboop/
scripts/                icon generation
.github/workflows/      CI, releases on v* tags, Pages deploy
```

## Upcoming

Ideas planned for a future release. Want to help with one? [Open an issue](https://github.com/alok722/babyboop/issues/new/choose) to talk it through first.

- **Nursery rhymes.** Either play your own MP3s from a music folder you choose, or a richer music-box version of the public-domain tunes BabyBoop already hums.

## Contributing

Bug reports, ideas and pull requests are welcome.

- Found a bug or a key that escapes the lock? [Open an issue](https://github.com/alok722/babyboop/issues/new/choose).
- Want to change something? Read [CONTRIBUTING.md](CONTRIBUTING.md) first. It covers bug reports, ground rules and how the page talks to the native apps.
- The game itself is a single file, [`web/index.html`](web/index.html), so most ideas start there.

## License

[MIT](LICENSE)
