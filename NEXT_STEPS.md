# Next steps

Only what's still pending. Planned features are in the README under [Upcoming](README.md#upcoming).

## Windows validation (needs a PC)

CI proves the Windows app builds and its unit tests pass. Nobody has run it on real hardware yet.

- Run [apps/windows/TESTING.md](apps/windows/TESTING.md) on an **x64** PC and on a **Windows 11 ARM** PC, using the `.exe` from the latest release.
- Check the icon in the taskbar, Alt+Tab and Explorer at 100% and 150% scaling. If the small sizes look blurry, try BMP frames for 16, 32 and 48 px in the `.ico` (`scripts/make_icons.sh`).
- If a PC without the WebView2 runtime is available, confirm BabyBoop explains how to get it.

## After testing

- Add anything that escapes the lock and can't be fixed to **Known limits** in the README and to the Windows setup step in `site/index.html`.
- Replace "haven't been tried on a real PC yet" in the README, the site's Downloads section and `CHANGELOG.md` with what was tested.
- Open one GitHub issue per bug, fix what can be fixed, and release a patch version.

Prompt for a Claude Code session:

```text
Read CLAUDE.md, README.md and NEXT_STEPS.md. Here are my Windows test results for <x64 / ARM>: <paste results>.
Update Known limits, the site and the tested-by-hand notes, open an issue per bug, fix what you can with tests, and remove finished items from NEXT_STEPS.md. Commit and push straight to main.
```
