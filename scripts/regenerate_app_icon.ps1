# Regenerate src-tauri/icons/icon.ico (and PNG bundle assets) using the current accent.
# Reads appearance_accent_color from settings_store.json when -Accent is not passed.
param(
    [string]$Accent,
    [string]$SettingsPath,
    [string]$ProjectRoot = (Split-Path $PSScriptRoot -Parent)
)
$ErrorActionPreference = "Stop"

function Resolve-SettingsStorePath {
    if ($SettingsPath -and (Test-Path $SettingsPath)) {
        return (Resolve-Path $SettingsPath).Path
    }
    $portable = Join-Path $ProjectRoot "Data\settings_store.json"
    if (Test-Path $portable) { return $portable }
    $appdata = Join-Path $env:APPDATA "com.pais.handy\settings_store.json"
    if (Test-Path $appdata) { return $appdata }
    return $null
}

$pyScript = Join-Path $ProjectRoot "scripts\generate_handy_icons.py"
if (-not (Test-Path $pyScript)) {
    throw "Missing $pyScript"
}

$args = @($pyScript)
if ($Accent) {
    $args += @("--accent", $Accent)
} else {
    $store = Resolve-SettingsStorePath
    if ($store) {
        Write-Host "Using accent from: $store"
        $args += @("--settings", $store)
    } else {
        Write-Host "No settings_store.json found - using default pink accent."
    }
}

$python = Get-Command python -ErrorAction SilentlyContinue
if (-not $python) {
    $python = Get-Command py -ErrorAction SilentlyContinue
    if ($python) {
        & py -3 @args
        exit $LASTEXITCODE
    }
    throw 'Python not found. Install Python 3 and Pillow: pip install Pillow'
}

& $python.Source @args
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host 'Bundle icons updated. Rebuild: .\deploy-handy.ps1 (uses subst W: -> Moj_Handy\h, no C:\ root folders).'
