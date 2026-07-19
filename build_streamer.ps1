#Requires -RunAsAdministrator
<#
.SYNOPSIS
    Build ALVR streamer (PC side / SteamVR plugin) including server_core changes.
    Installs: LLVM, Vulkan SDK, pkgconfiglite, x264, FFmpeg via prepare-deps.
#>

$ErrorActionPreference = "Stop"
$ProgressPreference    = "SilentlyContinue"

$ROOT = "C:\Projects\ALVR"

function Log($msg)  { Write-Host "==> $msg" -ForegroundColor Cyan }
function Fail($msg) { Write-Host "ERROR: $msg" -ForegroundColor Red; exit 1 }

# ── make cargo / rustup available ───────────────────────────────────────────
$env:PATH = "$env:USERPROFILE\.cargo\bin;" +
            [System.Environment]::GetEnvironmentVariable("PATH","Machine") + ";" +
            [System.Environment]::GetEnvironmentVariable("PATH","User")

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Fail "cargo not found. Run build_android.ps1 first to install Rust."
}

# ── 1. choco deps (LLVM, Vulkan SDK, pkgconfiglite, zip/unzip) ──────────────
Log "Installing build tools via Chocolatey..."
$ErrorActionPreference = "Continue"
& choco install llvm vulkan-sdk pkgconfiglite zip unzip -y
$ErrorActionPreference = "Stop"

# Reload PATH so new tools are visible
$env:PATH = "$env:USERPROFILE\.cargo\bin;" +
            [System.Environment]::GetEnvironmentVariable("PATH","Machine") + ";" +
            [System.Environment]::GetEnvironmentVariable("PATH","User")

# ── 2. prepare-deps: download x264 + FFmpeg ─────────────────────────────────
Set-Location $ROOT
Log "Running cargo xtask prepare-deps --platform windows..."
& cargo xtask prepare-deps --platform windows --ci
if ($LASTEXITCODE -ne 0) { Fail "prepare-deps failed (exit $LASTEXITCODE)" }

# ── 3. build streamer ────────────────────────────────────────────────────────
Log "Building ALVR streamer (release)..."
$env:RUST_BACKTRACE = "1"
& cargo xtask build-streamer --release
if ($LASTEXITCODE -ne 0) { Fail "build-streamer failed (exit $LASTEXITCODE)" }

# ── 4. also build launcher ───────────────────────────────────────────────────
Log "Building ALVR launcher (release)..."
& cargo xtask build-launcher --release
if ($LASTEXITCODE -ne 0) { Fail "build-launcher failed (exit $LASTEXITCODE)" }

# ── 5. sign executables ──────────────────────────────────────────────────────
Log "Signing executables..."
$certThumb = "D8FE6893DDF5B8EEAE02CFBC8AB4BB71B8F7BFD8"
$cert = Get-Item "Cert:\CurrentUser\My\$certThumb" -ErrorAction SilentlyContinue

if ($cert) {
    $streamerDir = "$ROOT\build\alvr_streamer_windows"
    $launcherDir = "$ROOT\build\alvr_launcher_windows"
    foreach ($exe in (Get-ChildItem "$streamerDir\*.exe", "$launcherDir\*.exe")) {
        $result = Set-AuthenticodeSignature -FilePath $exe.FullName -Certificate $cert -TimestampServer "http://timestamp.digicert.com"
        Write-Host "  $($exe.Name): $($result.Status)" -ForegroundColor $(if ($result.Status -eq "Valid") { "Green" } else { "Yellow" })
    }
} else {
    Write-Host "  [SKIP] Certificate not found (Thumbprint: $certThumb)" -ForegroundColor Yellow
}

# ── done ─────────────────────────────────────────────────────────────────────
$streamerDir = "$ROOT\build\alvr_streamer_windows"
$launcherDir = "$ROOT\build\alvr_launcher_windows"
Log "Done!"
Write-Host "  Streamer : $streamerDir" -ForegroundColor Green
Write-Host "  Launcher : $launcherDir" -ForegroundColor Green
