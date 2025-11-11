#!/bin/bash
set -e

VERSION=$1
REPO="adi-family/cli-ports-manager"
TAP_REPO="adi-family/homebrew-ports-manager"

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

# Remove 'v' prefix if present
VERSION=${VERSION#v}

echo "Updating Homebrew formula for version $VERSION"

# Create temp directory for downloads
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

# Download all binaries
echo "Downloading binaries..."
curl -sL -o "$TEMP_DIR/ports-manager-macos-arm64" \
    "https://github.com/$REPO/releases/download/v$VERSION/ports-manager-macos-arm64"
curl -sL -o "$TEMP_DIR/ports-manager-macos-amd64" \
    "https://github.com/$REPO/releases/download/v$VERSION/ports-manager-macos-amd64"
curl -sL -o "$TEMP_DIR/ports-manager-linux-amd64" \
    "https://github.com/$REPO/releases/download/v$VERSION/ports-manager-linux-amd64"

# Calculate SHA256 checksums
echo "Calculating SHA256 checksums..."
if command -v shasum &> /dev/null; then
    ARM64_SHA=$(shasum -a 256 "$TEMP_DIR/ports-manager-macos-arm64" | awk '{print $1}')
    AMD64_SHA=$(shasum -a 256 "$TEMP_DIR/ports-manager-macos-amd64" | awk '{print $1}')
    LINUX_SHA=$(shasum -a 256 "$TEMP_DIR/ports-manager-linux-amd64" | awk '{print $1}')
else
    ARM64_SHA=$(sha256sum "$TEMP_DIR/ports-manager-macos-arm64" | awk '{print $1}')
    AMD64_SHA=$(sha256sum "$TEMP_DIR/ports-manager-macos-amd64" | awk '{print $1}')
    LINUX_SHA=$(sha256sum "$TEMP_DIR/ports-manager-linux-amd64" | awk '{print $1}')
fi

echo "ARM64 SHA: $ARM64_SHA"
echo "AMD64 SHA: $AMD64_SHA"
echo "Linux SHA: $LINUX_SHA"

# Clone the tap repository
echo "Cloning tap repository..."
git clone "https://github.com/$TAP_REPO.git" "$TEMP_DIR/tap"
cd "$TEMP_DIR/tap"

# Update the formula
FORMULA_PATH="Formula/ports-manager.rb"

echo "Updating formula..."

# Update version
sed -i.bak "s/version \".*\"/version \"$VERSION\"/" "$FORMULA_PATH"

# Update URLs
sed -i.bak "s|download/v[0-9.]\+/ports-manager-macos-arm64|download/v$VERSION/ports-manager-macos-arm64|g" "$FORMULA_PATH"
sed -i.bak "s|download/v[0-9.]\+/ports-manager-macos-amd64|download/v$VERSION/ports-manager-macos-amd64|g" "$FORMULA_PATH"
sed -i.bak "s|download/v[0-9.]\+/ports-manager-linux-amd64|download/v$VERSION/ports-manager-linux-amd64|g" "$FORMULA_PATH"

# Update SHA256 checksums
# Use a more robust sed pattern that works with both placeholder and actual checksums
sed -i.bak "/ports-manager-macos-arm64/,/sha256/ s/sha256 \".*\"/sha256 \"$ARM64_SHA\"/" "$FORMULA_PATH"
sed -i.bak "/ports-manager-macos-amd64/,/sha256/ s/sha256 \".*\"/sha256 \"$AMD64_SHA\"/" "$FORMULA_PATH"
sed -i.bak "/ports-manager-linux-amd64/,/sha256/ s/sha256 \".*\"/sha256 \"$LINUX_SHA\"/" "$FORMULA_PATH"

# Remove backup files
rm -f "$FORMULA_PATH.bak"

# Show the diff
echo "Changes made:"
git diff "$FORMULA_PATH"

# Commit and push if there are changes
if ! git diff --quiet "$FORMULA_PATH"; then
    git config user.name "github-actions[bot]"
    git config user.email "github-actions[bot]@users.noreply.github.com"

    git add "$FORMULA_PATH"
    git commit -m "🔄 Update ports-manager to $VERSION

- Update all binary URLs to v$VERSION
- Update SHA-256 checksums for all platforms
  - macOS ARM64: $ARM64_SHA
  - macOS AMD64: $AMD64_SHA
  - Linux AMD64: $LINUX_SHA"

    git push
    echo "✅ Formula updated successfully!"
else
    echo "ℹ️  No changes detected"
fi
