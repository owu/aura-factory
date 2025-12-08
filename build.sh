#!/bin/bash

# Build script for Linux package

echo "Building Linux package for Aura Factory..."

# Clean previous builds
rm -rf target/release/aura-factory
rm -rf build

# Clean previous builds
cargo clean

# Build release version
cargo build --release --target x86_64-unknown-linux-gnu

# Create build directory structure
mkdir -p build/aura-factory/usr/bin
mkdir -p build/aura-factory/usr/local/share/icons
mkdir -p build/aura-factory/usr/local/share/applications

# Copy executable
cp target/x86_64-unknown-linux-gnu/release/aura-factory build/aura-factory/usr/bin/

# Copy icon
cp ui/statics/logo.png build/aura-factory/usr/local/share/icons/aura-factory.png

# Create desktop file
cat > build/aura-factory/usr/local/share/applications/aura-factory.desktop << EOF
[Desktop Entry]
Type=Application
Name=Aura Factory
Comment=Video processing application
Exec=aura-factory
Icon=aura-factory
Terminal=false
Categories=Video;Graphics;
StartupNotify=true
EOF

# Create Makefile for installation/uninstallation
cat > build/aura-factory/Makefile << EOF
PREFIX = /usr

install:
	mkdir -p \$(DESTDIR)\$(PREFIX)/bin
	mkdir -p \$(DESTDIR)\$(PREFIX)/local/share/icons
	mkdir -p \$(DESTDIR)\$(PREFIX)/local/share/applications
	cp usr/bin/aura-factory \$(DESTDIR)\$(PREFIX)/bin/
	cp usr/local/share/icons/aura-factory.png \$(DESTDIR)\$(PREFIX)/local/share/icons/
	cp usr/local/share/applications/aura-factory.desktop \$(DESTDIR)\$(PREFIX)/local/share/applications/

uninstall:
	rm -f \$(DESTDIR)\$(PREFIX)/bin/aura-factory
	rm -f \$(DESTDIR)\$(PREFIX)/local/share/icons/aura-factory.png
	rm -f \$(DESTDIR)\$(PREFIX)/local/share/applications/aura-factory.desktop
EOF

# Extract version from consts.rs
VERSION=$(grep -oP 'APP_VERSION: &str = "\K[^"]+' src/consts.rs)

# Create tarball
cd build/aura-factory
tar -cJf ../AuraFactory.v${VERSION}.x86_64-linux.tar.xz ./*
cd ../..

# Move tarball to project root
mv build/AuraFactory.v${VERSION}.x86_64-linux.tar.xz .

# Clean up build directory
rm -rf build

echo "Build completed! Linux package created at ./AuraFactory.v${VERSION}.x86_64-linux.tar.xz"
