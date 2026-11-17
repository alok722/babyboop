# Next steps

Only what's still pending. Planned features are in the README under [Upcoming](README.md#upcoming).

## Windows: test on a real PC, then switch to the Rust app

There are two Windows apps, and nobody has run either on real hardware:

- **.NET** (`apps/windows/`): what releases ship today. About 65–69 MB.
- **Rust** (`apps/windows-rs/`): a rewrite with the same behavior, about 0.5 MB. CI builds and tests it, but releases don't ship it yet.

The plan: test .NET first as the baseline, then Rust with the same checklist. If Rust is at least as good, delete .NET, move Rust into its place and release it.

### Prompt for a Claude Code session on the Windows PC

```text
Read CLAUDE.md and NEXT_STEPS.md, then do "Windows: test on a real PC" on this PC, one step at a time.
Set the PC up, then test the .NET app, then the Rust app, with apps/windows/TESTING.md.
Run the automatic parts yourself (builds, tests, --selftest, reading the log). For the parts only a person
can check, walk me through one TESTING.md section at a time and wait for my pass/fail before moving on.
If Rust is as good as .NET, do step 5 (switch to Rust and delete .NET), rebuild and recheck everything end to end.
Ask me before tagging a release. Commit and push straight to main.
```

### 1. Set up the PC

In PowerShell:

```powershell
winget install --id Git.Git -e
winget install --id GitHub.cli -e
winget install --id Microsoft.DotNet.SDK.8 -e
winget install --id Rustlang.Rustup -e
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--quiet --wait --add Microsoft.VisualStudio.Workload.VCTools --add Microsoft.VisualStudio.Component.VC.Tools.ARM64 --includeRecommended"
```

- Open a new terminal so the new tools are on `PATH`. Then `gh auth login` (as `alok722`), `git clone https://github.com/alok722/babyboop.git` and `cd babyboop`.
- The Build Tools give Rust its MSVC linker. The ARM64 component lets an x64 PC build the ARM `.exe` too.
- Check the PC type with `$env:PROCESSOR_ARCHITECTURE` (`AMD64` is x64, `ARM64` is ARM). Test the type you have. The other one stays on this list.
- Keep results in `apps/windows/TEST-RESULTS.md`: one row per TESTING.md item, with columns for .NET and Rust, plus notes. Commit it with the results.

### 2. Test the .NET app (the baseline)

```powershell
dotnet test apps/windows/BabyBoop.Tests
gh release download --pattern "BabyBoop-Windows-x64.exe" --dir $env:TEMP\bb   # or -arm64
```

- Run [apps/windows/TESTING.md](apps/windows/TESTING.md) with the release `.exe`.
- Anything that gets through here is a Windows limit or a shared bug. Expect the Rust app to behave the same.

### 3. Test the Rust app

```powershell
cd apps/windows-rs
cargo test
./build.ps1 -Arch x64   # or arm64; output in apps/windows-rs/out/
```

- Run the same TESTING.md with `apps/windows-rs/out/BabyBoop-Windows-<arch>.exe`. Both apps write the same log file, so compare the logs side by side.
- Also check its size, how fast it starts, and the icon at 100% and 150% scaling (taskbar, Alt+Tab, Explorer).
- Fix Rust bugs here, with tests in `src/input.rs` or `src/exit_hold.rs` where the logic allows.

### 4. Decide

- **Rust passes everything .NET passes:** do step 5.
- **Rust fails something .NET passes, and it can't be fixed:** keep .NET. Note why here, and open an issue.
- **Both fail the same item:** it's a Windows limit. It goes under Known limits (step 6), and it doesn't block the switch.

### 5. Switch to Rust and delete .NET

1. `git rm -r` the .NET app: `apps/windows/BabyBoop*` and `apps/windows/build.ps1`. Keep `TESTING.md` and `TEST-RESULTS.md`.
2. `git mv` everything in `apps/windows-rs/` into `apps/windows/`. Paths inside the crate keep working, because the folder depth is the same.
3. `scripts/make_icons.sh`: write the `.ico` straight to `apps/windows/assets/AppIcon.ico`.
4. `.gitignore`: drop the .NET entries and point the `target/` and `out/` entries at `apps/windows/`.
5. `ci.yml`: delete the .NET `windows` job, rename `windows-rs` to `windows` and fix its paths.
6. `release.yml`: replace `dotnet test` and `build.ps1 -Rid …` with `cargo test` and `./build.ps1 -Arch x64` / `-Arch arm64` (working directory `apps/windows`). Upload `apps/windows/out/*.exe`. The asset names stay the same, so the site and README links don't change.
7. Docs:
   - CLAUDE.md: the layout and the Windows build lines.
   - README: Build (Rust instead of the .NET SDK) and Project layout.
   - CONTRIBUTING.md.
   - `site/index.html`: the "small .NET one" line in Open source.
   - CHANGELOG: a new minor version. Say the Windows app is now Rust, give its download size, and note that nothing changes for players.
8. Rebuild end to end: `cargo test`, both `build.ps1` targets, CI green on `main`, then `--selftest` plus the exits section of TESTING.md on the freshly built `.exe`.
9. Ask before tagging the release. After it publishes, download the release `.exe` and repeat the self-test once.

### 6. Record the results

- Add anything that gets through the lock and can't be fixed to **Known limits** in the README and to the Windows setup step in `site/index.html`.
- Replace "haven't been tried on a real PC yet" in the README, the site's Downloads section and `CHANGELOG.md` with what was tested, and on which PC type.
- Open one GitHub issue per bug, fix what can be fixed, and release a patch.
- Remove the finished parts of this section. Keep whichever PC type (x64 or ARM) is still untested.
