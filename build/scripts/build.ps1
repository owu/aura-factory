#!/usr/bin/env pwsh

# Go to project root
$ProjectRoot = Resolve-Path "$PSScriptRoot/../.."
Set-Location $ProjectRoot

Write-Host "Building Windows executable for Aura Factory..." -ForegroundColor Green

# Ensure releases directory exists
$ReleaseDir = Join-Path $ProjectRoot "build/releases"
if (-not (Test-Path $ReleaseDir)) {
    New-Item -ItemType Directory -Path $ReleaseDir
}

# Clean previous builds
cargo clean

# Execute cargo build command
cargo build --release --target x86_64-pc-windows-msvc

# Check if build was successful
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

# Extract version from consts.rs
$constsContent = Get-Content -Path "./src/consts.rs"
$versionRegex = 'APP_VERSION: &str = "([^"]+)"'
$versionMatch = $constsContent | Select-String -Pattern $versionRegex
if ($versionMatch) {
    $version = $versionMatch.Matches.Groups[1].Value
}
else {
    $version = "unknown"
}

# Define source and destination paths
$sourcePath = "./target/x86_64-pc-windows-msvc/release/aura-factory.exe"
$destinationName = "AuraFactory.v$version.x86_64-windows.exe"
$destinationPath = Join-Path $ReleaseDir $destinationName

# Copy and rename the executable
Copy-Item -Path $sourcePath -Destination $destinationPath -Force

Write-Host "Build completed!" -ForegroundColor Green
Write-Host "Windows executable created at: $destinationPath" -ForegroundColor Cyan
