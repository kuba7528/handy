# =============================================================================
# Handy - automatyczna instalacja zależności i budowanie (Windows)
# =============================================================================
# Uruchom w PowerShellu JAKO ADMINISTRATOR:
#   powershell -ExecutionPolicy Bypass -File .\build-windows.ps1
#
# Skrypt jest idempotentny — można go uruchamiać wielokrotnie. winget pomija
# już zainstalowane pakiety. Po udanym buildzie instalator MSI/EXE znajdzie się
# w: src-tauri\target\release\bundle\
# =============================================================================

$ErrorActionPreference = "Stop"
$ProjectRoot = $PSScriptRoot
Set-Location $ProjectRoot

function Write-Step($msg) { Write-Host "`n=== $msg ===" -ForegroundColor Cyan }

# --- 0. Sprawdzenia wstępne -------------------------------------------------
Write-Step "Sprawdzanie winget"
if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
    throw "winget nie jest dostepny. Zainstaluj 'App Installer' ze sklepu Microsoft Store i uruchom ponownie."
}

$wingetCommon = @("--accept-package-agreements", "--accept-source-agreements", "--disable-interactivity")

function Install-WingetPkg($id, $extraArgs = @()) {
    Write-Host "-> Instaluje $id ..." -ForegroundColor Yellow
    & winget install --id $id -e --source winget @wingetCommon @extraArgs
    if ($LASTEXITCODE -ne 0 -and $LASTEXITCODE -ne -1978335189) {
        # -1978335189 = "No applicable upgrade / already installed"
        Write-Host "   (kod wyjscia $LASTEXITCODE — kontynuuje, pakiet moze byc juz zainstalowany)" -ForegroundColor DarkYellow
    }
}

# --- 1. Narzędzia systemowe -------------------------------------------------
Write-Step "Instalacja Visual Studio Build Tools (C++), CMake, Vulkan SDK"
Install-WingetPkg "Microsoft.VisualStudio.2022.BuildTools" @(
    "--override",
    "--quiet --wait --norestart --add Microsoft.VisualStudio.Workload.VCTools --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows11SDK.22621 --add Microsoft.VisualStudio.Component.VC.CMake.Project --includeRecommended"
)
Install-WingetPkg "Kitware.CMake"
Install-WingetPkg "KhronosGroup.VulkanSDK"

Write-Step "Instalacja Rust (rustup) i Bun"
Install-WingetPkg "Rustlang.Rustup"
Install-WingetPkg "Oven-sh.Bun"

# --- 2. Odświeżenie PATH w bieżącej sesji -----------------------------------
Write-Step "Odswiezanie PATH"
$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" +
            [System.Environment]::GetEnvironmentVariable("Path", "User")
# Typowe lokalizacje, gdyby PATH nie zostal jeszcze zaktualizowany
$env:Path += ";$env:USERPROFILE\.cargo\bin;$env:USERPROFILE\.bun\bin"

# --- 3. Toolchain Rust MSVC -------------------------------------------------
Write-Step "Konfiguracja toolchaina Rust (stable-msvc)"
if (Get-Command rustup -ErrorAction SilentlyContinue) {
    rustup default stable-msvc
    rustup update
} else {
    throw "rustup nie jest w PATH. Zamknij i otworz nowy terminal (administrator), a nastepnie uruchom skrypt ponownie."
}

if (-not (Get-Command bun -ErrorAction SilentlyContinue)) {
    throw "bun nie jest w PATH. Zamknij i otworz nowy terminal (administrator), a nastepnie uruchom skrypt ponownie."
}

# --- 4. Model VAD (wymagany do dzialania i budowania) -----------------------
Write-Step "Pobieranie modelu Silero VAD"
$modelDir = Join-Path $ProjectRoot "src-tauri\resources\models"
$modelPath = Join-Path $modelDir "silero_vad_v4.onnx"
if (-not (Test-Path $modelPath)) {
    New-Item -ItemType Directory -Force -Path $modelDir | Out-Null
    Invoke-WebRequest -Uri "https://blob.handy.computer/silero_vad_v4.onnx" -OutFile $modelPath
    Write-Host "   Pobrano model do $modelPath" -ForegroundColor Green
} else {
    Write-Host "   Model juz istnieje — pomijam." -ForegroundColor Green
}

# --- 5. Zależności frontendu ------------------------------------------------
Write-Step "Instalacja zaleznosci JS (bun install)"
bun install

# --- 6. Budowanie produkcyjne ----------------------------------------------
Write-Step "Budowanie aplikacji (bun run tauri build) — to potrwa kilka-kilkanascie minut"
bun run tauri build

# --- 7. Wynik ---------------------------------------------------------------
Write-Step "Gotowe"
$bundleDir = Join-Path $ProjectRoot "src-tauri\target\release\bundle"
Write-Host "Instalatory/binarka znajduja sie w:" -ForegroundColor Green
Write-Host "  $bundleDir" -ForegroundColor Green
Get-ChildItem -Recurse $bundleDir -Include *.msi, *.exe -ErrorAction SilentlyContinue |
    Select-Object FullName | Format-Table -AutoSize
Write-Host "`nSurowa binarka (do szybkiego uruchomienia): src-tauri\target\release\handy.exe" -ForegroundColor Green
