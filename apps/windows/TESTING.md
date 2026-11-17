# Windows hand-test checklist

CI only proves the Windows app builds and the core tests pass. Run this list on a real PC before each release, once on **x64** and once on **ARM**.

The same list covers both Windows apps: the .NET one (the release `.exe`) and the Rust one (`apps/windows-rs/out/BabyBoop-Windows-<arch>.exe`). For the Rust app, run the `.exe` from that folder in the commands below.

The log is `%LOCALAPPDATA%\BabyBoop\BabyBoop.log`. Open it with:

```powershell
notepad $env:LOCALAPPDATA\BabyBoop\BabyBoop.log
```

## 1. Install

- [ ] Download `BabyBoop-Windows-x64.exe` (or `-arm64.exe`) from the release and run it.
- [ ] SmartScreen shows "Windows protected your PC". Click **More info**, then **Run anyway**. BabyBoop opens maximized with the Start screen.
- [ ] The exit text says **Alt** + Shift + K 2s or Esc 5s, and a **Quit** button is visible.

## 2. Self-test

```powershell
.\BabyBoop-Windows-x64.exe --selftest
```

- [ ] The app locks, then quits by itself after about 12 seconds.
- [ ] The log has `lock on`, then a `selftest posted …` line for each step, then `lock off reason=exit combo` about 5 s after `Esc down (long)`.
- [ ] No `lock off` between `Esc down (short)` and `Esc up after 1s`.
- [ ] Nothing happened outside the app (no Start menu, no app switch, no media playing).

## 3. Keys while locked (press Start first)

Each one should make an emoji pop and do nothing else.

- [ ] Letters and digits (A shows 🍎, digits show stars)
- [ ] **Win** key alone (Start menu must not open)
- [ ] **Alt+Tab**, **Win+Tab**
- [ ] **Alt+F4**
- [ ] **Ctrl+Esc**
- [ ] **Ctrl+Shift+Esc** (Task Manager must not open)
- [ ] **Win+D**, **Win+E**, **Win+R**
- [ ] Media keys (play/pause, volume) and laptop F-keys (brightness, airplane mode)
- [ ] Mashing many keys at once for 30 s: no stutter, no lost lock

Known to get through (Windows reserves them, no app can block them):

- **Ctrl+Alt+Del**
- **Win+L**

Note what each of these does, and that BabyBoop is still locked afterwards.

## 4. Mouse, touchpad, touchscreen

- [ ] The cursor is hidden. Clicking shows a pop where the virtual cursor is.
- [ ] Moving the mouse moves the pop position, and it never leaves the main screen.
- [ ] With a **second monitor**, the cursor can't reach it and clicks there do nothing.
- [ ] Scrolling (wheel, two-finger touchpad) makes pops.
- [ ] Right-click: no context menu.
- [ ] **3- and 4-finger touchpad swipes** (Task View, switching desktops, show desktop): record what happens.
- [ ] **Touchscreen edge swipes** (left, right, top), if the PC has a touchscreen: record what happens.

Anything in the last two that escapes the app is a known gap. Add it to the README's *Known limits*.

## 5. Exits

- [ ] Hold **Esc** for 5 s: the bar fills and the app unlocks. Releasing early resets it.
- [ ] Hold **Alt+Shift+K** for 2 s: the app unlocks. Letting go of Shift early resets it.
- [ ] After unlocking, the window is a normal maximized window. **Quit** closes it.
- [ ] Unlock with a hold, press **Start** again: it stays locked (repeat 3 times, with both exits).
- [ ] While locked, press **Ctrl+Alt+Del** and choose Cancel, or press **Win+L** and sign back in. BabyBoop is still locked, letters still pop, and **Shift+K** alone (no Alt) does not exit.
- [ ] Leave it locked for 10 minutes of mashing. Win and Alt+Tab are still blocked at the end (this checks the hooks were not dropped).

## 6. Sound and look

- [ ] Tones play on key press and the background music is quiet but audible.
- [ ] Calm mode is softer and slower.
- [ ] The app icon shows in the taskbar and in File Explorer.

## 7. WebView2 missing (optional, on a test VM)

- [ ] On a PC without the WebView2 Runtime, BabyBoop shows a message with the download link and closes instead of crashing.
