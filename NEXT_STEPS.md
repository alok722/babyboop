# Next steps

Prompt for a Claude Code session in this folder after the repo moves to its new account. Replace `NEW_OWNER` with the new account name.

```text
BabyBoop repo has moved to github.com/NEW_OWNER/BabyBoop. Finish what's left. Read CLAUDE.md, README.md, the spec docs/superpowers/specs/2026-11-16-cross-platform-distribution-design.md and the plan docs/superpowers/plans/2026-11-16-babyboop-cross-platform.md first. The local ledger .superpowers/sdd/2026-11-16-babyboop-cross-platform/progress.md (git-ignored) has every ruling and deferred item. Commit and push straight to main, no PRs.

1. Point everything at the new owner
   - git remote set-url origin https://github.com/NEW_OWNER/BabyBoop.git
   - Replace Alok-Raj_JLLT with NEW_OWNER in site/config.js (REPO), README.md (badges, download and release links), the spec, and CLAUDE.md. Then grep the whole repo for the old owner.
   - Make sure Actions is enabled on the repo (gh api repos/NEW_OWNER/BabyBoop/actions/permissions).

2. First CI run and fixes
   - Push, then watch .github/workflows/ci.yml with gh run watch.
   - The Windows code (apps/windows/**) has never been compiled; expect build errors. Fix them until the macos, windows and site jobs are green.
   - The new test ExitHoldTests.ResetClearsInProgressHold and the other 14 tests were never run; they must pass.

3. macOS self-test (needs me)
   - Ask me to grant Accessibility to ~/Applications/BabyBoop.app (bundle ID app.babyboop) and remove the old BabySmash entry. Check that the onboarding panel hides within about 1s of the grant (log line permission=true). Then run: open -W ~/Applications/BabyBoop.app --args --selftest
   - Expect "lock on" and then "lock off reason=exit combo" about 5s after "Esc down (long)" in ~/Library/Logs/BabyBoop.log.
   - Reinstalling changes the signature; remind me to re-add BabyBoop in Accessibility after every install.

4. Deferred minors from the final review. Fix each with a test where one is possible:
   - Config.cs: double.TryParse with CultureInfo.InvariantCulture
   - ExitHold: drop the unused isRepeat param, or use it
   - Tests: a key-up for a different key doesn't reset the hold; Esc pressed during a K hold is ignored
   - Hook-install log: report each hook's own error
   - Windows 10 on ARM: x64 emulation needs Windows 11. Say so on the site and in the README.
   - Workflows: call ./apps/windows/build.ps1
   - macOS --selftest and --exit-after should log and quit when Accessibility isn't granted
   - build.sh: an unknown argument should exit with an error, with an explicit install case
   - ICO: consider BMP frames for 16/32/48; check the taskbar and Explorer icons

5. Website
   - Replace the dashed .shot placeholders in site/index.html with real screenshots (I'll supply them), or remove them.
   - When the repo is public: enable Pages (Source: GitHub Actions), switch pages.yml to also run on push to main (paths: site/**, web/**), run it, then set the repo homepage to the Pages URL with gh repo edit --homepage.

6. Release v0.1.0
   - Ask me first. I need to have run apps/windows/TESTING.md on x64 and ARM PCs. Add anything that escaped the lock to the README's "Known limits".
   - Change CHANGELOG.md "## [0.1.0] - Unreleased" to today's date, commit, then git tag v0.1.0 && git push origin v0.1.0.
   - Watch release.yml. Check the release has exactly BabyBoop-macOS.dmg, BabyBoop-Windows-x64.exe and BabyBoop-Windows-arm64.exe, and that the site's Download button resolves through releases/latest/download.

7. Optional, ask before doing
   - Nursery rhymes: either play my own MP3s from a music folder, or a richer music-box version of the public-domain tunes.
   - Check whether the name "BabyBoop" is already taken by an app, trademark or domain before going public.
   - Make the repo public.
```

The local ledger and plan workspace under `.superpowers/` are not pushed, so run the follow-up from the original folder on the original Mac.
