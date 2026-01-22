#!/usr/bin/env pwsh

Write-Host "Building Windows executable for Aura Factory..." -ForegroundColor Green

# Clean previous builds
cargo clean

# Execute cargo build command
cargo build --release --target x86_64-pc-windows-gnu

# Check if build was successful
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

# Extract version from consts.rs
$constsContent = Get-Content -Path "./src/consts.rs"
$versionRegex = 'APP_VERSION: &str = "([^"]+)"'
$versionMatch = $constsContent | Select-String -Pattern $versionRegex
$version = $versionMatch.Matches.Groups[1].Value

# Define source and destination paths
$sourcePath = "./target/x86_64-pc-windows-gnu/release/aura-factory.exe"
$destinationPath = "./AuraFactory.v$version.x86_64-windows.exe"

# Copy and rename the executable
Copy-Item -Path $sourcePath -Destination $destinationPath -Force

Write-Host "Build completed!" -ForegroundColor Green
Write-Host "Windows executable created at: $destinationPath" -ForegroundColor Cyan
