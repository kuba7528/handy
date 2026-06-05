# Deploy a portable Handy release folder (exe + DLLs + resources only).
# Default Cargo target: short path under Pobrane (not C:\ht on drive root).
# If MSVC/whisper hits MAX_PATH, use -TargetDir "C:\Users\Kuba\Pobrane\Moj_Handy\ht" instead.
param(
    [string]$TargetDir = "C:\Users\Kuba\Pobrane\Moj_Handy\h",
    [string]$DestDir = "C:\Users\Kuba\Pobrane\Moj_Handy\Handy\My_handy",
    [string]$ProjectRoot = $PSScriptRoot,
    [switch]$SkipIconRegen
)
$ErrorActionPreference = "Stop"
$release = Join-Path $TargetDir "release"
$exe = Join-Path $release "handy.exe"
$iconIco = Join-Path $ProjectRoot "src-tauri\icons\icon.ico"
$regenScript = Join-Path $ProjectRoot "scripts\regenerate_app_icon.ps1"

function Invoke-AccentIconRegen {
    if ($SkipIconRegen) { return }
    if (-not (Test-Path $regenScript)) {
        Write-Host "Skipping icon regen (script not found)." -ForegroundColor DarkYellow
        return
    }
    $portableSettings = Join-Path $DestDir "Data\settings_store.json"
    $settingsArg = @{}
    if (Test-Path $portableSettings) {
        $settingsArg["SettingsPath"] = $portableSettings
    }
    Write-Host "Regenerating bundle icons from accent settings..."
    & $regenScript @settingsArg -ProjectRoot $ProjectRoot
}

function Ensure-ReleaseBuild {
    $needsBuild = -not (Test-Path $exe)
    if (-not $needsBuild -and (Test-Path $iconIco)) {
        if ($iconIco.LastWriteTime -gt (Get-Item $exe).LastWriteTime) {
            Write-Host "icon.ico is newer than handy.exe — rebuilding..."
            $needsBuild = $true
        }
    }
    if (-not $needsBuild) { return }
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
    $sidecarSrc = Join-Path $release "handy-accent.ico"
    if (Test-Path $sidecarSrc) {
        Copy-Item $sidecarSrc (Join-Path $DestDir "handy-accent.ico") -Force
    }
}
Invoke-AccentIconRegen
Ensure-ReleaseBuild
Get-Process handy -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 1
Copy-PortableArtifacts
Write-Host "Deployed portable Handy to: $DestDir"
Write-Host "Run: $(Join-Path $DestDir 'handy.exe')"
Write-Host "Exe/taskbar icon color follows the last build; handy-accent.ico (if present) can be used for shortcuts."
