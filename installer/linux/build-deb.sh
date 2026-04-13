#!/bin/bash
set -euo pipefail

VERSION="${1:?Usage: build-deb.sh <version> <arch>}"
ARCH="${2:?Usage: build-deb.sh <version> <arch>}"

BINARY_NAME="ofx-viewer"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
OUTPUT_DIR="$PROJECT_DIR/target/installer"

case "$ARCH" in
  x86_64)
    TARGET="x86_64-unknown-linux-gnu"
    DEB_ARCH="amd64"
    ;;
  aarch64)
    TARGET="aarch64-unknown-linux-gnu"
    DEB_ARCH="arm64"
    ;;
  *) echo "Unknown arch: $ARCH"; exit 1 ;;
esac

BINARY="$PROJECT_DIR/target/$TARGET/release/$BINARY_NAME"

if [ ! -f "$BINARY" ]; then
  echo "Binary not found: $BINARY"
  exit 1
fi

# Build deb structure
DEB_DIR="$PROJECT_DIR/target/deb-build"
rm -rf "$DEB_DIR"
mkdir -p "$DEB_DIR/DEBIAN"
mkdir -p "$DEB_DIR/usr/bin"
mkdir -p "$DEB_DIR/usr/share/applications"
mkdir -p "$DEB_DIR/usr/share/icons/hicolor/256x256/apps"

cp "$BINARY" "$DEB_DIR/usr/bin/$BINARY_NAME"
chmod 755 "$DEB_DIR/usr/bin/$BINARY_NAME"

# Convert icon
if [ -f "$PROJECT_DIR/res/logo.png" ]; then
  if command -v convert &> /dev/null; then
    convert "$PROJECT_DIR/res/logo.png" -resize 256x256 "$DEB_DIR/usr/share/icons/hicolor/256x256/apps/$BINARY_NAME.png"
  else
    cp "$PROJECT_DIR/res/logo.png" "$DEB_DIR/usr/share/icons/hicolor/256x256/apps/$BINARY_NAME.png"
  fi
fi

# Create .desktop file
cat > "$DEB_DIR/usr/share/applications/$BINARY_NAME.desktop" << EOF
[Desktop Entry]
Name=OFX Viewer
Comment=View and analyze OFX financial files
Exec=$BINARY_NAME
Icon=$BINARY_NAME
Terminal=false
Type=Application
Categories=Office;Finance;
EOF

# Create control file
INSTALLED_SIZE=$(du -sk "$DEB_DIR/usr" | cut -f1)

cat > "$DEB_DIR/DEBIAN/control" << EOF
Package: $BINARY_NAME
Version: $VERSION
Section: office
Priority: optional
Architecture: $DEB_ARCH
Installed-Size: $INSTALLED_SIZE
Maintainer: zibo-chen
Description: OFX Viewer
 A desktop application for viewing and analyzing OFX financial files.
EOF

# Build the deb package
mkdir -p "$OUTPUT_DIR"
DEB_NAME="${BINARY_NAME}_${VERSION}_${DEB_ARCH}.deb"
dpkg-deb --build "$DEB_DIR" "$OUTPUT_DIR/$DEB_NAME"

echo "Created: $OUTPUT_DIR/$DEB_NAME"
