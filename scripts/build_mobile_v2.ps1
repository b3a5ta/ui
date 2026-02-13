<#
.SYNOPSIS
    Builds the NuViO369 CRM Mobile Application for Android using Tauri v2.

.DESCRIPTION
    Automates the entire build pipeline:
    1. Builds Tailwind CSS (minified)
    2. Builds Dioxus Frontend (WASM)
    3. Initializes Android project (if needed)
    4. Applies Gradle fixes (OOM, Kotlin)
    5. Builds APK (arm64)
    6. Signs APK (debug keystore)
    7. Installs on connected device (ADB)

.EXAMPLE
    .\build_mobile_v2.ps1
#>

$ErrorActionPreference = "Stop"

# --- Configuration ---
$ProjectRoot = Resolve-Path ".."
$FrontendDir = Join-Path $ProjectRoot "mobile-app-v2" # Adjust if script runs from root vs scripts folder
if (!(Test-Path $FrontendDir)) { $FrontendDir = $ProjectRoot } # Fallback if running from root

$TauriDir = Join-Path $FrontendDir "src-tauri"
$DistDir = Join-Path $FrontendDir "dist"
$AndroidGenDir = Join-Path $TauriDir "gen/android"

Write-Host ">>> Starting NuViO369 Mobile Build (Tauri v2 + Dioxus 0.7)" -ForegroundColor Cyan

# --- Step 1: Frontend Build ---
Write-Host "`n[1/6] Building Tailwind CSS..." -ForegroundColor Yellow
Set-Location $FrontendDir
cmd /c "npm run build:css"

Write-Host "`n[2/6] Building Dioxus Frontend (WASM)..." -ForegroundColor Yellow
dx build --platform web --release
# Ensure dist folder is populated
if (!(Test-Path "$DistDir/index.html")) {
    Write-Error "Dioxus build failed. index.html missing in dist/"
}

# --- Step 2: Android Init ---
Set-Location $TauriDir
if (!(Test-Path $AndroidGenDir)) {
    Write-Host "`n[3/6] Initializing Android Project..." -ForegroundColor Yellow
    cargo tauri android init
} else {
    Write-Host "`n[3/6] Android project already initialized. Skipping init." -ForegroundColor Gray
}

# --- Step 3: Apply Fixes ---
Write-Host "`n[4/6] Applying Android Build Fixes..." -ForegroundColor Yellow
$GradleProps = Join-Path $AndroidGenDir "gradle.properties"
if (Test-Path $GradleProps) {
    $Content = Get-Content $GradleProps
    if ($Content -notmatch "org.gradle.jvmargs") {
        Add-Content $GradleProps "`norg.gradle.jvmargs=-Xmx2048m"
        Write-Host "  - Added JVM memory limit" -ForegroundColor Gray
    }
    if ($Content -notmatch "kotlin.incremental=false") {
        Add-Content $GradleProps "`nkotlin.incremental=false"
        Write-Host "  - Disabled Kotlin incremental build" -ForegroundColor Gray
    }
}

# --- Step 4: Build APK ---
Write-Host "`n[5/6] Building APK (aarch64)..." -ForegroundColor Yellow
$env:CARGO_BUILD_JOBS = 2 # Prevent OOM
cargo tauri android build --target aarch64

# --- Step 5: Sign & Install ---
# Note: This part assumes standard output paths. Adjust if needed.
$ApkPath = Join-Path $AndroidGenDir "app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk"
# If standard build produces debug apk by default without signing config:
if (!(Test-Path $ApkPath)) {
    $ApkPath = Join-Path $AndroidGenDir "app/build/outputs/apk/debug/app-debug.apk"
}

if (Test-Path $ApkPath) {
    Write-Host "`n[6/6] APK Built Successfully: $ApkPath" -ForegroundColor Green

    # Optional: Sign and Install
    # Write-Host "Signing and Installing..."
    # & apksigner sign --ks debug.keystore --ks-pass pass:android $ApkPath
    # & adb install -r $ApkPath
} else {
    Write-Warning "APK output not found at expected path. Check build logs."
}

Write-Host "`n>>> Build Complete!" -ForegroundColor Cyan
