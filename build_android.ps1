#Requires -RunAsAdministrator
<#
.SYNOPSIS
    One-shot setup + build script for ALVR Android client.
    Installs: JDK 17, Android SDK (platform-32), Android NDK r26b, Rust stable.
    Then builds: alvr_client_android.apk
#>

$ErrorActionPreference = "Stop"
$ProgressPreference    = "SilentlyContinue"   # speeds up Invoke-WebRequest

# ── paths ───────────────────────────────────────────────────────────────────
$ROOT        = "C:\Projects\ALVR"
$DEPS_DIR    = "C:\android-deps"
$SDK_DIR     = "$DEPS_DIR\android-sdk"
$NDK_DIR     = "$DEPS_DIR\android-ndk-r26b"
$NDK_ZIP     = "$DEPS_DIR\android-ndk-r26b-windows.zip"
$CMDTOOLS_ZIP= "$DEPS_DIR\cmdlinetools.zip"
$CMDTOOLS_DIR= "$SDK_DIR\cmdline-tools\latest"
$JDK_DIR     = "$DEPS_DIR\jdk17"

New-Item -ItemType Directory -Force -Path $DEPS_DIR | Out-Null
New-Item -ItemType Directory -Force -Path $SDK_DIR  | Out-Null

function Log($msg) { Write-Host "==> $msg" -ForegroundColor Cyan }
function Fail($msg) { Write-Host "ERROR: $msg" -ForegroundColor Red; exit 1 }

# ────────────────────────────────────────────────────────────────────────────
# 1. JDK 17 (Eclipse Temurin)
# ────────────────────────────────────────────────────────────────────────────
if (-not (Test-Path "$JDK_DIR\bin\java.exe")) {
    Log "Downloading JDK 17 (Eclipse Temurin)..."
    $jdkUrl = "https://github.com/adoptium/temurin17-binaries/releases/download/jdk-17.0.12%2B7/OpenJDK17U-jdk_x64_windows_hotspot_17.0.12_7.zip"
    Invoke-WebRequest -Uri $jdkUrl -OutFile "$DEPS_DIR\jdk17.zip"
    Log "Extracting JDK..."
    Expand-Archive -Path "$DEPS_DIR\jdk17.zip" -DestinationPath "$DEPS_DIR\jdk17_tmp" -Force
    $inner = Get-ChildItem "$DEPS_DIR\jdk17_tmp" -Directory | Select-Object -First 1
    Move-Item $inner.FullName $JDK_DIR -Force
    Remove-Item "$DEPS_DIR\jdk17_tmp" -Recurse -Force -ErrorAction SilentlyContinue
    Remove-Item "$DEPS_DIR\jdk17.zip" -Force
    Log "JDK 17 installed at $JDK_DIR"
} else {
    Log "JDK 17 already present."
}
$env:JAVA_HOME = $JDK_DIR
$env:PATH = "$JDK_DIR\bin;$env:PATH"

# ────────────────────────────────────────────────────────────────────────────
# 2. Android SDK command-line tools + platform-32
# ────────────────────────────────────────────────────────────────────────────
if (-not (Test-Path "$CMDTOOLS_DIR\bin\sdkmanager.bat")) {
    Log "Downloading Android SDK command-line tools..."
    $ctUrl = "https://dl.google.com/android/repository/commandlinetools-win-11076708_latest.zip"
    Invoke-WebRequest -Uri $ctUrl -OutFile $CMDTOOLS_ZIP
    Log "Extracting command-line tools..."
    Expand-Archive -Path $CMDTOOLS_ZIP -DestinationPath "$DEPS_DIR\cmdtools_tmp" -Force
    New-Item -ItemType Directory -Force -Path $CMDTOOLS_DIR | Out-Null
    # Google zips the tools inside a "cmdline-tools" folder
    $inner = "$DEPS_DIR\cmdtools_tmp\cmdline-tools"
    if (Test-Path $inner) {
        Copy-Item "$inner\*" $CMDTOOLS_DIR -Recurse -Force
    } else {
        Copy-Item "$DEPS_DIR\cmdtools_tmp\*" $CMDTOOLS_DIR -Recurse -Force
    }
    Remove-Item "$DEPS_DIR\cmdtools_tmp" -Recurse -Force
    Remove-Item $CMDTOOLS_ZIP -Force
}
$env:ANDROID_HOME     = $SDK_DIR
$env:ANDROID_SDK_ROOT = $SDK_DIR
$env:PATH = "$CMDTOOLS_DIR\bin;$SDK_DIR\platform-tools;$env:PATH"

Log "Accepting Android SDK licenses..."
$yes = "y`ny`ny`ny`ny`ny`ny`n"
$yes | & "$CMDTOOLS_DIR\bin\sdkmanager.bat" --sdk_root=$SDK_DIR --licenses 2>&1 | Out-Null

$packages = @("platforms;android-32", "platform-tools", "build-tools;34.0.0")
foreach ($pkg in $packages) {
    Log "Installing SDK package: $pkg"
    & "$CMDTOOLS_DIR\bin\sdkmanager.bat" --sdk_root=$SDK_DIR $pkg | Out-Null
}

# ────────────────────────────────────────────────────────────────────────────
# 3. Android NDK r26b
# ────────────────────────────────────────────────────────────────────────────
if (-not (Test-Path "$NDK_DIR\ndk-build.cmd")) {
    Log "Downloading Android NDK r26b (~800 MB)..."
    $ndkUrl = "https://dl.google.com/android/repository/android-ndk-r26b-windows.zip"
    Invoke-WebRequest -Uri $ndkUrl -OutFile $NDK_ZIP
    Log "Extracting NDK (this may take a while)..."
    Expand-Archive -Path $NDK_ZIP -DestinationPath $DEPS_DIR -Force
    Remove-Item $NDK_ZIP -Force
    Log "NDK r26b installed at $NDK_DIR"
} else {
    Log "NDK r26b already present."
}
$env:ANDROID_NDK_HOME = $NDK_DIR

# ────────────────────────────────────────────────────────────────────────────
# 4. Rust + aarch64 target
# ────────────────────────────────────────────────────────────────────────────
$cargoExe = "$env:USERPROFILE\.cargo\bin\cargo.exe"
if (-not (Test-Path $cargoExe)) {
    Log "Installing Rust via rustup..."
    $rustupInit = "$DEPS_DIR\rustup-init.exe"
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustupInit
    & $rustupInit -y --default-toolchain stable --no-modify-path 2>&1
    Remove-Item $rustupInit -Force
}
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"

Log "Setting Rust stable as default..."
$ErrorActionPreference = "Continue"
& rustup default stable
$ErrorActionPreference = "Stop"

Log "Adding aarch64-linux-android target..."
$ErrorActionPreference = "Continue"
& rustup target add aarch64-linux-android
$ErrorActionPreference = "Stop"

# ────────────────────────────────────────────────────────────────────────────
# 5. cargo-apk, cargo-ndk, cbindgen
# ────────────────────────────────────────────────────────────────────────────
$tools = @{
    "cargo-ndk"  = "cargo-ndk"
    "cbindgen"   = "cbindgen"
}
$ErrorActionPreference = "Continue"
foreach ($bin in $tools.Keys) {
    if (-not (Test-Path "$env:USERPROFILE\.cargo\bin\$bin.exe")) {
        Log "Installing $bin..."
        & cargo install $tools[$bin]
        if ($LASTEXITCODE -ne 0) { $ErrorActionPreference = "Stop"; Fail "Failed to install $bin" }
    } else {
        Log "$bin already installed."
    }
}

if (-not (Test-Path "$env:USERPROFILE\.cargo\bin\cargo-apk.exe")) {
    Log "Installing cargo-apk (ALVR fork)..."
    & cargo install --git https://github.com/zarik5/cargo-apk cargo-apk
    if ($LASTEXITCODE -ne 0) { $ErrorActionPreference = "Stop"; Fail "Failed to install cargo-apk" }
} else {
    Log "cargo-apk already installed."
}
$ErrorActionPreference = "Stop"

# ────────────────────────────────────────────────────────────────────────────
# 6. ALVR prepare-deps (OpenXR loaders)
# ────────────────────────────────────────────────────────────────────────────
Set-Location $ROOT
# install unzip + llvm so xtask's download_and_extract_zip works on Windows
Log "Installing unzip and llvm via Chocolatey..."
$ErrorActionPreference = "Continue"
& choco install unzip llvm -y
$ErrorActionPreference = "Stop"

# Reload PATH so unzip.exe is visible in this session
$env:PATH = [System.Environment]::GetEnvironmentVariable("PATH","Machine") + ";" +
            [System.Environment]::GetEnvironmentVariable("PATH","User") + ";" +
            $env:PATH

Log "Running cargo xtask prepare-deps --platform android..."
& cargo xtask prepare-deps --platform android --ci
if ($LASTEXITCODE -ne 0) { Fail "prepare-deps failed" }

# ────────────────────────────────────────────────────────────────────────────
# 7. Build the APK
# ────────────────────────────────────────────────────────────────────────────
Log "Building ALVR Android client..."
$env:RUST_BACKTRACE = "1"
& cargo xtask build-client --release
if ($LASTEXITCODE -ne 0) { Fail "build-client failed" }

$apk = "$ROOT\build\alvr_client_android\alvr_client_android.apk"
if (Test-Path $apk) {
    Log "SUCCESS! APK: $apk"
} else {
    Fail "APK not found at expected path: $apk"
}
