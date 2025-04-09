#!/bin/bash
set -e

VERSION="0.1.0"
NAME="cacheclip"

# Create a dist directory
mkdir -p dist

# Detect host platform
HOST_OS=$(uname)
HOST_ARCH=$(uname -m)

# Define target architectures based on host platform
if [[ "$HOST_OS" == "Darwin" ]]; then
    # On macOS, only build for macOS targets
    # Linux cross-compilation requires X11 libraries which are complex to set up
    TARGETS=(
        "x86_64-apple-darwin"
        "aarch64-apple-darwin"
    )
elif [[ "$HOST_OS" == "Linux" ]]; then
    # On Linux, build for Linux and Windows
    TARGETS=(
        "x86_64-unknown-linux-gnu"
        "aarch64-unknown-linux-gnu"
        "x86_64-pc-windows-msvc"
        "aarch64-pc-windows-msvc"
    )
else
    echo "Unsupported host platform: $HOST_OS"
    exit 1
fi

# Install cross-compilation tools if needed
if [[ "$HOST_OS" == "Linux" ]]; then
    # For Windows builds on Linux
    if [[ " ${TARGETS[@]} " =~ " x86_64-pc-windows-msvc " ]] || [[ " ${TARGETS[@]} " =~ " aarch64-pc-windows-msvc " ]]; then
        if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
            echo "Installing Windows cross-compilation tools..."
            sudo apt-get update
            sudo apt-get install -y mingw-w64
        fi
    fi
fi

# Build for each target
for TARGET in "${TARGETS[@]}"; do
    echo "Building for $TARGET..."
    
    # Add appropriate target if not already installed
    rustup target add "$TARGET" || true
    
    # Build
    cargo build --release --target "$TARGET"
    
    # Get platform info from target
    if [[ "$TARGET" == *"linux"* ]]; then
        PLATFORM="linux"
        EXT=""
    elif [[ "$TARGET" == *"apple"* ]]; then
        PLATFORM="macos"
        EXT=""
    elif [[ "$TARGET" == *"windows"* ]]; then
        PLATFORM="windows"
        EXT=".exe"
    else
        echo "Unknown platform in target: $TARGET"
        continue
    fi
    
    # Get architecture from target
    if [[ "$TARGET" == "x86_64"* ]]; then
        ARCH="x86_64"
    elif [[ "$TARGET" == "aarch64"* ]]; then
        ARCH="arm64"
    else
        echo "Unknown architecture in target: $TARGET"
        continue
    fi
    
    # Create package name
    PACKAGE_NAME="${NAME}-v${VERSION}-${ARCH}-${PLATFORM}"
    
    # Set binary path
    BINARY_PATH="target/$TARGET/release/${NAME}${EXT}"
    
    # Create archive
    if [[ "$PLATFORM" == "windows" ]]; then
        if [[ "$HOST_OS" == "Darwin" ]] || [[ "$HOST_OS" == "Linux" ]]; then
            # On macOS or Linux creating a zip for Windows
            zip -j "dist/${PACKAGE_NAME}.zip" "$BINARY_PATH"
        else
            # On Windows
            powershell Compress-Archive -Path "$BINARY_PATH" -DestinationPath "dist/${PACKAGE_NAME}.zip"
        fi
    else
        # For Linux and macOS
        tar -czf "dist/${PACKAGE_NAME}.tar.gz" -C "$(dirname "$BINARY_PATH")" "$(basename "$BINARY_PATH")"
    fi
    
    echo "Created dist/${PACKAGE_NAME}.tar.gz"
done

echo "All builds complete!"