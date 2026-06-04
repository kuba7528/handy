# Deploy a portable Handy release folder (exe + DLLs + resources only).
param(
    [string]$TargetDir = "C:\ht",
    [string]$DestDir = "C:\Users\Kuba\Pobrane\Moj_Handy\Handy\My_handy",
    [string]$ProjectRoot = $PSScriptRoot
)
$ErrorActionPreference = "Stop"
$release = Join-Path $TargetDir "release"
$exe = Join-Path $release "handy.exe"
function Ensure-ReleaseBuild {
    if (Test-Path $exe) { return }
    Write-Host "Building Handy release (bun tauri build)..."
    $env:CARGO_TARGET_DIR = $TargetDir
    Push-Location $ProjectRoot
    try { bun tauri build } finally { Pop-Location }
    if (-not (Test-Path $exe)) { throw "Release binary not found at $exe" }
}
function Copy-PortableArtifacts {
    $items = @("handy.exe", "DirectML.dll", "handy_app_lib.dll")
    if (-not (Test-Path $DestDir)) { New-Item -ItemType Directory -Path $DestDir -Force | Out-Null }
    foreach ($name in $items) {
        $src = Join-Path $release $name
        if (-not (Test-Path $src)) { throw "Missing release artifact: $src" }
        Copy-Item $src (Join-Path $DestDir $name) -Force
    }
    $resSrc = Join-Path $release "resources"
    $resDst = Join-Path $DestDir "resources"
    if (-not (Test-Path $resSrc)) { throw "Missing resources folder: $resSrc" }
    if (Test-Path $resDst) { Remove-Item $resDst -Recurse -Force }
    Copy-Item $resSrc $resDst -Recurse -Force
}
Ensure-ReleaseBuild
Get-Process handy -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 1
Copy-PortableArtifacts
Write-Host "Deployed portable Handy to: $DestDir"
Write-Host "Run: $(Join-Path $DestDir 'handy.exe')"
