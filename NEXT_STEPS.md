# Next steps

Where BabyBoop stands and what's left before and after the first release. Planned features live in the README under [Upcoming](README.md#upcoming).

## Done

- Repo moved to [alok722/babyboop](https://github.com/alok722/babyboop); all links point there.
- CI is green: macOS build, Windows build (x64 and ARM) with 16 unit tests, and the site tests.
- Deferred review fixes: invariant-culture `--exit-after`, `ExitHold` cleanup and tests, per-hook error logging, macOS test mode quits without Accessibility, `build.sh` rejects unknown arguments, Windows-on-ARM needs Windows 11.
- Website live at [alok722.github.io/babyboop](https://alok722.github.io/babyboop/), deployed by `pages.yml` on every push that touches `site/` or `web/`. Written setup guide instead of screenshots.

## 1. Release v0.1.0

The Download buttons on the site and in the README point at `releases/latest/download/…` and return 404 until a release exists. Mark it as a normal release, not a pre-release, because `latest` skips pre-releases.

1. In `CHANGELOG.md`, change `## [0.1.0] - Unreleased` to today's date.
2. Commit, then `git tag v0.1.0 && git push origin v0.1.0`.
3. Watch `release.yml`. The release must have exactly `BabyBoop-macOS.dmg`, `BabyBoop-Windows-x64.exe` and `BabyBoop-Windows-arm64.exe`.
4. Click each Download button on the site and check it downloads.

## 2. macOS validation (from the release)

Test on a Mac using the DMG from the release, the way a new user would.

- [ ] Open the DMG and drag BabyBoop into Applications.
- [ ] First open shows "Apple could not verify…". **Open Anyway** in Privacy & Security works.
- [ ] The Accessibility prompt appears. After switching BabyBoop on, the onboarding panel hides within about 1 second.
- [ ] Press Start. Letters show their emoji with a tone. ⌘Tab, ⌘Q, ⌘Space, Mission Control, swipes and the Dock do nothing.
- [ ] Hold ⌥ Option + Shift + K for 2 s: exits. Hold Esc for 5 s: exits.
- [ ] Self-test: `open -W /Applications/BabyBoop.app --args --selftest`. In `~/Library/Logs/BabyBoop.log` expect `lock on`, then `lock off reason=exit combo` about 5 s after `Esc down (long)`.
- [ ] If an old BabySmash entry is in the Accessibility list, remove it.

## 3. Windows validation (pending, needs a PC)

CI proves the Windows app builds and the core tests pass. Nobody has run it on real hardware yet.

- Run [apps/windows/TESTING.md](apps/windows/TESTING.md) on an **x64** PC and on a **Windows 11 ARM** PC, using the `.exe` from the release.
- Check the icon in the taskbar, Alt+Tab and Explorer at 100% and 150% scaling. If the small sizes look blurry, try BMP frames for 16, 32 and 48 px in the `.ico` (`scripts/make_icons.sh`).
- Confirm the WebView2 message appears on a PC without the runtime, if one is available.

## 4. After testing: record what worked

For each platform, write down what passed, what escaped the lock and what broke. Then:

- **README → Known limits:** add anything that escapes the lock and can't be fixed (for example a specific touchpad gesture).
- **Website:** add the same limits to the setup section of `site/index.html`, so parents know before they hand over the keyboard. If Windows is still untested at release time, say so there.
- **Bugs to fix:** open one GitHub issue per problem with the bug report template, the device, the key or gesture, and the log.
- **Release a fix:** add a `## [0.1.1]` entry to `CHANGELOG.md` and tag `v0.1.1`.

## Prompt for a Claude Code session

```text
Read CLAUDE.md, README.md and NEXT_STEPS.md. Here are my test results for <macOS / Windows x64 / Windows ARM>: <paste results>.
Update README "Known limits" and the website setup section with what escaped the lock, open an issue for each bug, fix what you can with tests, and tick the done items in NEXT_STEPS.md. Commit and push straight to main.
```
