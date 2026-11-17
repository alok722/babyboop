param(
    [Parameter(Mandatory)]
    [ValidateSet("win-x64", "win-arm64")]
    [string]$Rid
)
$ErrorActionPreference = "Stop"
$arch = $Rid.Substring(4)
$publish = Join-Path $PSScriptRoot "publish/$Rid"
$out = Join-Path $PSScriptRoot "out"

dotnet publish (Join-Path $PSScriptRoot "BabyBoop/BabyBoop.csproj") -c Release -r $Rid --self-contained `
    -p:PublishSingleFile=true -p:IncludeNativeLibrariesForSelfExtract=true -o $publish
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

New-Item -ItemType Directory -Force $out | Out-Null
Copy-Item (Join-Path $publish "BabyBoop.exe") (Join-Path $out "BabyBoop-Windows-$arch.exe") -Force
Write-Host "built $out/BabyBoop-Windows-$arch.exe"
