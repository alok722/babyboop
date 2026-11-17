param(
    [Parameter(Mandatory)]
    [ValidateSet("win-x64", "win-arm64")]
    [string]$Rid,
    # Defaults to BABYBOOP_VERSION (set from the tag by release.yml), else the csproj <Version>
    [string]$Version = $env:BABYBOOP_VERSION
)
$ErrorActionPreference = "Stop"
$arch = $Rid.Substring(4)
$publish = Join-Path $PSScriptRoot "publish/$Rid"
$out = Join-Path $PSScriptRoot "out"
$versionArgs = @()
if ($Version) { $versionArgs = @("-p:Version=$Version") }

dotnet publish (Join-Path $PSScriptRoot "BabyBoop/BabyBoop.csproj") -c Release -r $Rid --self-contained `
    -p:PublishSingleFile=true -p:IncludeNativeLibrariesForSelfExtract=true -p:EnableCompressionInSingleFile=true @versionArgs -o $publish
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

New-Item -ItemType Directory -Force $out | Out-Null
Copy-Item (Join-Path $publish "BabyBoop.exe") (Join-Path $out "BabyBoop-Windows-$arch.exe") -Force
Write-Host "built $out/BabyBoop-Windows-$arch.exe"
