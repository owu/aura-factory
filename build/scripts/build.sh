#!/bin/bash

# Get the script directory and then the project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/../.." &> /dev/null && pwd )"
cd "$PROJECT_ROOT"

# Ensure releases directory exists
mkdir -p build/releases

# Default target if not provided
TARGET=${1:-x86_64-unknown-linux-gnu}

echo "Building for target: $TARGET..."

# Clean previous builds (only the specific target output)
rm -rf "target/$TARGET/release/aura-factory"

# Build release version
cargo build --release --target "$TARGET"

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

# Extract version from consts.rs
VERSION=$(grep -oP 'APP_VERSION: &str = "\K[^"]+' src/consts.rs)
if [ -z "$VERSION" ]; then
    VERSION="unknown"
fi

# Determine packaging based on target OS
if [[ "$TARGET" == *"linux"* ]]; then
    # Create temporary build structure
    TEMP_BUILD="build/temp_linux"
    rm -rf "$TEMP_BUILD"
    mkdir -p "$TEMP_BUILD/aura-factory/usr/bin"
    mkdir -p "$TEMP_BUILD/aura-factory/usr/local/share/icons"
    mkdir -p "$TEMP_BUILD/aura-factory/usr/local/share/applications"

    # Copy executable
    cp "target/$TARGET/release/aura-factory" "$TEMP_BUILD/aura-factory/usr/bin/"

    # Copy icon (if exists)
    if [ -f "ui/statics/linux.png" ]; then
        cp ui/statics/linux.png "$TEMP_BUILD/aura-factory/usr/local/share/icons/aura-factory.png"
    fi

    # Create desktop file
    cat > "$TEMP_BUILD/aura-factory/usr/local/share/applications/aura-factory.desktop" << EOF
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

    # Create Makefile
    cat > "$TEMP_BUILD/aura-factory/Makefile" << EOF
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

    # Create tarball
    cd "$TEMP_BUILD/aura-factory"
    OUTPUT_NAME="AuraFactory.v${VERSION}.x86_64-linux.tar.xz"
    tar -cJf "../../releases/$OUTPUT_NAME" ./*
    cd "$PROJECT_ROOT"

    # Clean up temp
    rm -rf "$TEMP_BUILD"
    
    echo "Build completed! Package created at build/releases/$OUTPUT_NAME"

elif [[ "$TARGET" == *"apple-darwin"* ]]; then
    # macOS packaging (simple tar.gz for the binary)
    ARCH="x86_64"
    if [[ "$TARGET" == "aarch64"* ]]; then
        ARCH="aarch64"
    fi
    
    OUTPUT_NAME="AuraFactory.v${VERSION}.${ARCH}-macos.tar.gz"
    
    # Create temp dir for packaging
    TEMP_BUILD="build/temp_macos"
    rm -rf "$TEMP_BUILD"
    mkdir -p "$TEMP_BUILD"
    cp "target/$TARGET/release/aura-factory" "$TEMP_BUILD/"
    
    cd "$TEMP_BUILD"
    tar -czf "../releases/$OUTPUT_NAME" aura-factory
    cd "$PROJECT_ROOT"
    
    rm -rf "$TEMP_BUILD"
    echo "Build completed! Package created at build/releases/$OUTPUT_NAME"
fi
