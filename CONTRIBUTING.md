# Contributing to BabyBoop

Thanks for helping make BabyBoop better for little ones.

## Reporting bugs

Open an issue with the bug report template. Include your OS version, Mac or PC model (Intel, Apple silicon, x64 or ARM), and what you pressed or did. The log helps a lot:

- Mac: `~/Library/Logs/BabyBoop.log`
- Windows: `%LOCALAPPDATA%\BabyBoop\BabyBoop.log`

If a key or gesture escapes the lock, say exactly which one.

## Making changes

1. Fork and clone the repo.
2. Build and test the part you change (see *Build from source* in the README).
3. Keep changes small and focused. Match the existing style; `.editorconfig` covers the basics.
4. Open a pull request describing what changed and how you tested it.

## Ground rules

- `web/index.html` is shared by both apps. A change there must keep working on Mac, Windows and in a plain browser.
- The page and the native apps talk through a small bridge: `nativeStart`, `nativeStop`, `nativeInput`, `nativeExitProgress` and `nativePermission` from native, and the messages `start`, `quit` and `openSettings` from the page. Change both sides together.
- The exits (hold ⌥/Alt + Shift + K for 2 s, or Esc for 5 s) are part of the product. Open an issue before changing them.
- Sounds stay gentle tones. No speech.
- Nothing may need a server, an account or tracking.

## Releasing

Maintainers add a `## [X.Y.Z]` section to `CHANGELOG.md`, then push a tag `vX.Y.Z`. The release workflow stamps that version into both apps, builds the Mac DMG and both Windows executables, and publishes the GitHub Release with the changelog section as its notes. Don't edit version numbers by hand; the tag is the only place a version lives.
