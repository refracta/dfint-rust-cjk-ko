Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$RootDir = Resolve-Path (Join-Path $PSScriptRoot "..")

$OutDir = $env:OUT_DIR
if ([string]::IsNullOrWhiteSpace($OutDir)) {
  $OutDir = Join-Path $RootDir "dist"
}

$OutFile = $env:OUT_FILE
if ([string]::IsNullOrWhiteSpace($OutFile)) {
  $OutFile = Join-Path $OutDir "dfhooks_dfint_cjk_ko.dll"
}

New-Item -ItemType Directory -Force $OutDir | Out-Null

Push-Location $RootDir
try {
  cargo build --release
} finally {
  Pop-Location
}

$BuiltDll = Join-Path $RootDir "target\\release\\dfint_hook.dll"
Copy-Item -Force $BuiltDll $OutFile
Write-Host "Wrote: $OutFile"
