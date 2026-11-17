param(
    [Parameter(Mandatory)]
    [ValidateSet("x64", "arm64")]
    [string]$Arch,
    # Defaults to BABYBOOP_VERSION (set from the tag by release.yml), else Cargo.toml's version
    [string]$Version = $env:BABYBOOP_VERSION
)
$ErrorActionPreference = "Stop"
$target = @{ x64 = "x86_64-pc-windows-msvc"; arm64 = "aarch64-pc-windows-msvc" }[$Arch]
if ($Version) { $env:BABYBOOP_VERSION = $Version }

rustup target add $target
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
cargo build --release --locked --target $target --manifest-path (Join-Path $PSScriptRoot "Cargo.toml")
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$out = Join-Path $PSScriptRoot "out"
New-Item -ItemType Directory -Force $out | Out-Null
Copy-Item (Join-Path $PSScriptRoot "target/$target/release/BabyBoop.exe") (Join-Path $out "BabyBoop-Windows-$Arch.exe") -Force
Write-Host "built $out/BabyBoop-Windows-$Arch.exe"
