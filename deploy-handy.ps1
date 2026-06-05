# Deploy a portable Handy release folder (exe + DLLs + resources only).
# Cargo cache: Moj_Handy\h on disk. Release builds always use subst W: -> that folder (short CARGO_TARGET_DIR, avoids MAX_PATH).
param(
    [string]$TargetDir = "C:\Users\Kuba\Pobrane\Moj_Handy\h",
    [string]$DestDir = "C:\Users\Kuba\Pobrane\Moj_Handy\Handy\My_handy",
    [string]$ProjectRoot = $PSScriptRoot,
    [string]$SubstDrive = "W:",
    [switch]$SkipIconRegen
)
$ErrorActionPreference = "Stop"
$release = Join-Path $TargetDir "release"
$exe = Join-Path $release "handy.exe"
$iconIco = Join-Path $ProjectRoot "src-tauri\icons\icon.ico"
$regenScript = Join-Path $ProjectRoot "scripts\regenerate_app_icon.ps1"

function Ensure-SubstCargoRoot {
    param([string]$PhysicalRoot)
    if (-not (Test-Path -LiteralPath $PhysicalRoot)) {
        New-Item -ItemType Directory -Path $PhysicalRoot -Force | Out-Null
    }
    $physicalRoot = (Resolve-Path -LiteralPath $PhysicalRoot).Path
    $drive = $SubstDrive.TrimEnd('\')
    $driveLetter = $drive.TrimEnd(':')
    $lines = @(cmd /c "subst" 2>$null)
    foreach ($line in $lines) {
        if ($line -match "^$([regex]::Escape($driveLetter)):\\:\s*=>\s*(.+)$") {
            $mapped = $matches[1].Trim().TrimEnd('\')
            if ($mapped.ToUpperInvariant() -eq $physicalRoot.ToUpperInvariant()) {
                Write-Host "Using existing subst ${driveLetter}: -> $physicalRoot"
                return "${driveLetter}:"
            }
            Write-Host "Remapping subst ${driveLetter}: (was $mapped) -> $physicalRoot"
            cmd /c "subst ${driveLetter}: /D" | Out-Null
            break
        }
    }
    Write-Host "Mapping subst $drive -> $physicalRoot (CARGO_TARGET_DIR=$drive)"
    cmd /c "subst ${driveLetter}: `"$physicalRoot`""
    if ($LASTEXITCODE -ne 0) { throw "subst ${driveLetter}: failed for $physicalRoot" }
    return $drive
}

function Test-BuildOutputPathTooLong {
    param([string]$Output)
    return ($Output -match 'path too long|MAX_PATH|filename longer than 260|LNK1104|cannot open file')
}

function Clear-StaleCargoRootBuildCache {
    param([string]$PhysicalTarget)
    $buildRoot = Join-Path $PhysicalTarget "release\build"
    if (-not (Test-Path $buildRoot)) { return }
    Get-ChildItem $buildRoot -Directory -Filter "ferrous-opencc-*" -ErrorAction SilentlyContinue | ForEach-Object {
        $map = Join-Path $_.FullName "out\embedded_map.rs"
        if ((Test-Path $map) -and (Select-String -Path $map -Pattern 'include_bytes!\(r"C:/' -Quiet)) {
            Write-Host "Removing ferrous-opencc build cache tied to obsolete root paths..."
            Remove-Item $_.FullName -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
    Get-ChildItem $buildRoot -Directory -Filter "whisper-rs-sys-*" -ErrorAction SilentlyContinue | ForEach-Object {
        $marker = Join-Path $_.FullName "output"
        if ((Test-Path $marker) -and (Select-String -Path $marker -Pattern "C:/release/build" -Quiet)) {
            Write-Host "Removing whisper-rs-sys build cache tied to obsolete root paths..."
            Remove-Item $_.FullName -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
}

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

function Invoke-TauriReleaseBuild {
    Clear-StaleCargoRootBuildCache -PhysicalTarget $TargetDir
    $cargoRoot = Ensure-SubstCargoRoot -PhysicalRoot $TargetDir
    $env:CARGO_TARGET_DIR = $cargoRoot
    Push-Location $ProjectRoot
    try {
        $buildOutput = bun tauri build 2>&1 | Tee-Object -Variable buildLines
        $exit = $LASTEXITCODE
        if ($exit -ne 0) {
            $text = ($buildLines | Out-String)
            if (Test-BuildOutputPathTooLong $text) {
                throw "Build failed (possible MAX_PATH). subst $SubstDrive should point at $TargetDir. Output: $text"
            }
            throw "bun tauri build failed with exit code $exit"
        }
    } finally { Pop-Location }
}

function Ensure-ReleaseBuild {
    $needsBuild = -not (Test-Path $exe)
    if (-not $needsBuild -and (Test-Path $iconIco)) {
        if ((Get-Item $iconIco).LastWriteTime -gt (Get-Item $exe).LastWriteTime) {
            Write-Host "icon.ico is newer than handy.exe - rebuilding..."
            $needsBuild = $true
        }
    }
    if (-not $needsBuild) { return }
    Write-Host "Building Handy release (bun tauri build via subst $SubstDrive)..."
    Invoke-TauriReleaseBuild
    if (-not (Test-Path $exe)) { throw "Release binary not found at $exe" }
}

function Resolve-ReleaseArtifact($name) {
    $root = Join-Path $release $name
    if (Test-Path $root) { return $root }
    $deps = Join-Path $release "deps\$name"
    if (Test-Path $deps) { return $deps }
    return $null
}

function Copy-PortableArtifacts {
    $items = @("handy.exe", "DirectML.dll", "handy_app_lib.dll")
    if (-not (Test-Path $DestDir)) { New-Item -ItemType Directory -Path $DestDir -Force | Out-Null }
    foreach ($name in $items) {
        $src = Resolve-ReleaseArtifact $name
        if (-not $src) { throw "Missing release artifact: $name (looked in $release and deps\)" }
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
Write-Host "Cargo target on disk: $TargetDir (build uses subst $SubstDrive -> same folder)."
Write-Host "Exe/taskbar icon color follows the last build; handy-accent.ico (if present) can be used for shortcuts."
