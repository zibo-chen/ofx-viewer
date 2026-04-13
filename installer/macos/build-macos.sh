#!/bin/bash
set -euo pipefail

VERSION="${1:?Usage: build-macos.sh <version> <arch>}"
ARCH="${2:?Usage: build-macos.sh <version> <arch>}"

APP_NAME="OFX Viewer"
BINARY_NAME="ofx-viewer"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
OUTPUT_DIR="$PROJECT_DIR/target/installer"

case "$ARCH" in
  x86_64)  TARGET="x86_64-apple-darwin" ;;
  aarch64) TARGET="aarch64-apple-darwin" ;;
  *) echo "Unknown arch: $ARCH"; exit 1 ;;
esac

BINARY="$PROJECT_DIR/target/$TARGET/release/$BINARY_NAME"

if [ ! -f "$BINARY" ]; then
  echo "Binary not found: $BINARY"
  exit 1
fi

# Create .app bundle
APP_DIR="$PROJECT_DIR/target/$BINARY_NAME-app"
BUNDLE="$APP_DIR/$APP_NAME.app"

rm -rf "$APP_DIR"
mkdir -p "$BUNDLE/Contents/MacOS"
mkdir -p "$BUNDLE/Contents/Resources"

cp "$BINARY" "$BUNDLE/Contents/MacOS/$BINARY_NAME"

# Create Info.plist
cat > "$BUNDLE/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$BINARY_NAME</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>com.ofxviewer.app</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

# Convert icon if sips is available
if [ -f "$PROJECT_DIR/res/logo.png" ]; then
  ICONSET="$APP_DIR/AppIcon.iconset"
  mkdir -p "$ICONSET"
  for SIZE in 16 32 64 128 256 512; do
    sips -z $SIZE $SIZE "$PROJECT_DIR/res/logo.png" --out "$ICONSET/icon_${SIZE}x${SIZE}.png" 2>/dev/null || true
    DOUBLE=$((SIZE * 2))
    if [ $DOUBLE -le 1024 ]; then
      sips -z $DOUBLE $DOUBLE "$PROJECT_DIR/res/logo.png" --out "$ICONSET/icon_${SIZE}x${SIZE}@2x.png" 2>/dev/null || true
    fi
  done
  iconutil -c icns -o "$BUNDLE/Contents/Resources/AppIcon.icns" "$ICONSET" 2>/dev/null || true
fi

# Create DMG
mkdir -p "$OUTPUT_DIR"
DMG_NAME="$BINARY_NAME-${VERSION}-macos-${ARCH}.dmg"
DMG_PATH="$OUTPUT_DIR/$DMG_NAME"

rm -f "$DMG_PATH"

# Create a temporary DMG directory
DMG_DIR="$APP_DIR/dmg-contents"
mkdir -p "$DMG_DIR"
cp -R "$BUNDLE" "$DMG_DIR/"
ln -s /Applications "$DMG_DIR/Applications"

hdiutil create -volname "$APP_NAME" \
  -srcfolder "$DMG_DIR" \
  -ov -format UDZO \
  "$DMG_PATH"

echo "Created: $DMG_PATH"
