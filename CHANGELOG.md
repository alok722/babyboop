# Changelog

All notable changes to this project are documented here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses [Semantic Versioning](https://semver.org/).

## [0.1.2] - 2026-11-17

### Changed

- The Windows download is less than half the size: about 69 MB for x64 (was 155 MB) and 65 MB for ARM (was 167 MB). It unpacks on launch, so the first start may take a moment longer.

### Known limits

- Windows builds pass CI but haven't been tested on real PCs yet.

## [0.1.1] - 2026-11-17

### Fixed

- macOS setup screen now names the permission list as macOS 27 does: Device Control and Data Access (Accessibility on earlier versions). The button says Open System Settings.
- The app's version number now matches the release. v0.1.0 on Mac reported 1.0.
- In the browser version, the exit hint said Esc 2s; it is Esc 5s.

### Known limits

- Windows builds pass CI but haven't been tested on real PCs yet.

## [0.1.0] - 2026-11-17

### Added

- macOS app (Intel and Apple silicon) that locks keys and trackpad gestures, with in-app Accessibility setup.
- Windows app (x64 and ARM) that locks keys and the mouse.
- Shared toy page: per-key emoji, tones, confetti, smash counter, shifting background, nursery melodies, idle pops and calm mode.
- Website that picks the right build for your computer, with a live keyboard demo, a browser-or-app comparison and a step-by-step setup guide.
- CI, tagged releases and a Pages workflow.

### Known limits

- Windows builds pass CI but haven't been tested on real PCs yet.
