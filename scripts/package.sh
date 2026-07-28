#!/usr/bin/env bash
set -euo pipefail

# Extract version from Cargo.toml automatically
VERSION="v$(grep -m1 '^version =' Cargo.toml | cut -d '"' -f2)"
PROJECT="convlint"
DIST_DIR="dist"

echo "Building release packages for ${PROJECT} ${VERSION}..."

rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Helper function to package tarball
package_tar() {
    local target=$1
    local archive="${DIST_DIR}/${PROJECT}-${VERSION}-${target}.tar.gz"
    echo "Packaging ${archive}..."
    tar -czvf "$archive" \
        -C "target/${target}/release" "$PROJECT" \
        -C "../../../" README.md LICENSE.md
}

# Helper function to package zip
package_zip() {
    local target=$1
    local archive="${DIST_DIR}/${PROJECT}-${VERSION}-${target}.zip"
    echo "Packaging ${archive}..."
    zip -j "$archive" \
        "target/${target}/release/${PROJECT}.exe" README.md LICENSE Convlint.toml
}

# Package all built targets
package_tar "x86_64-unknown-linux-musl"
package_tar "aarch64-unknown-linux-musl"
package_tar "aarch64-apple-darwin"
package_zip "x86_64-pc-windows-gnu"

# Generate Checksums
(cd "$DIST_DIR" && sha256sum * > SHA256SUMS)

echo "Done! Release assets ready in ./${DIST_DIR}/"
