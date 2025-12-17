#!/usr/bin/env bash
set -e

PROJECT_NAME="synth"
BUILD_DIR="build"
mkdir -p "$BUILD_DIR"

# Timestamped folder for final artifacts
TIMESTAMP=$(date +"%Y%m%d-%H%M%S")
FINAL_DIR="$BUILD_DIR/$TIMESTAMP"
mkdir -p "$FINAL_DIR"

# Force cross to use x86_64 Docker images on Apple Silicon
export CROSS_RUNNER_EXTRA_FLAGS="--platform linux/amd64"

# ---------------- Functions ----------------
build_macos() {
  local TARGET=$1
  local ARCH=$2

  echo "Building macOS target $TARGET..."
  cargo build --release --target "$TARGET"

  BIN_PATH="target/$TARGET/release/$PROJECT_NAME"
  FINAL_NAME="$FINAL_DIR/${PROJECT_NAME}-cli_${ARCH}"
  cp "$BIN_PATH" "$FINAL_NAME"
}

build_linux() {
  local TARGET=$1
  local ARCH=$2

  # Skip aarch64 Linux on Apple Silicon
  if [[ "$(uname -m)" == "arm64" && "$TARGET" == "aarch64-unknown-linux-gnu" ]]; then
    echo "Skipping $TARGET on Apple Silicon due to missing Docker image."
    return
  fi

  echo "Building Linux target $TARGET..."
  cross build --release --target "$TARGET"

  BIN_PATH="target/$TARGET/release/$PROJECT_NAME"
  FINAL_NAME="$FINAL_DIR/${PROJECT_NAME}-cli_${ARCH}"
  cp "$BIN_PATH" "$FINAL_NAME"
}

build_windows() {
  local TARGET=$1
  local ARCH=$2

  echo "Building Windows target $TARGET..."
  cross build --release --target "$TARGET"

  BIN_PATH="target/$TARGET/release/$PROJECT_NAME.exe"
  FINAL_NAME="$FINAL_DIR/${PROJECT_NAME}-cli_${ARCH}.exe"
  cp "$BIN_PATH" "$FINAL_NAME"
}

# ---------------- Build ----------------
# macOS
build_macos x86_64-apple-darwin "x86_64-macos"
build_macos aarch64-apple-darwin "arm64-macos"

# Linux
build_linux x86_64-unknown-linux-gnu "x86_64-linux"
build_linux aarch64-unknown-linux-gnu "arm64-linux"  # Will skip on Apple Silicon

# Windows
if [[ "$(uname -m)" == "arm64" ]]; then
  echo "Skipping Windows build on Apple Silicon due to missing Docker image."
else
  build_windows x86_64-pc-windows-gnu "x86_64-windows"
fi


echo "All builds completed. Artifacts are in $FINAL_DIR"
