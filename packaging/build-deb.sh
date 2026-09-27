#!/usr/bin/env bash
#
# Build a Debian package (.deb) for open-keyboard using only dpkg-deb (no
# third-party cargo plugins). Ships the IBus engine, the `okb` CLI, and the IBus
# component descriptor.
#
# Usage:
#   packaging/build-deb.sh <version>        # e.g. 0.1.0 or v0.1.0
# Environment overrides:
#   BINDIR   directory holding the release binaries (default: target/release)
#   OUTDIR   where to write the .deb (default: dist)
set -euo pipefail

VERSION="${1:-${VERSION:-}}"
if [[ -z "$VERSION" ]]; then
  echo "usage: $0 <version>" >&2
  exit 1
fi
VERSION="${VERSION#v}" # strip a leading 'v' from a git tag

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BINDIR="${BINDIR:-$ROOT/target/release}"
OUTDIR="${OUTDIR:-$ROOT/dist}"
ARCH="$(dpkg --print-architecture)"
PKG="open-keyboard"
LIBEXEC="/usr/lib/ibus-open-keyboard"

for bin in okb okb-ibus-engine; do
  if [[ ! -x "$BINDIR/$bin" ]]; then
    echo "error: missing binary $BINDIR/$bin (build with: cargo build --release)" >&2
    exit 1
  fi
done

STAGE="$(mktemp -d)"
trap 'rm -rf "$STAGE"' EXIT

mkdir -p \
  "$STAGE/DEBIAN" \
  "$STAGE/usr/bin" \
  "$STAGE$LIBEXEC" \
  "$STAGE/usr/share/ibus/component" \
  "$STAGE/usr/share/doc/$PKG"

install -m755 "$BINDIR/okb" "$STAGE/usr/bin/okb"
install -m755 "$BINDIR/okb-ibus-engine" "$STAGE$LIBEXEC/okb-ibus-engine"

sed "s#@LIBEXEC@#$LIBEXEC#g" "$ROOT/packaging/ibus/open-keyboard.xml.in" \
  > "$STAGE/usr/share/ibus/component/open-keyboard.xml"

install -m644 "$ROOT/LICENSE" "$STAGE/usr/share/doc/$PKG/copyright"

cat > "$STAGE/DEBIAN/control" <<EOF
Package: $PKG
Version: $VERSION
Section: utils
Priority: optional
Architecture: $ARCH
Depends: ibus
Maintainer: Maifee Ul Asad <maifeeulasad@gmail.com>
Homepage: https://github.com/maifeeulasad/open-keyboard
Description: Bengali phonetic input method (open-keyboard)
 Type Bengali by typing phonetically in English. Provides an IBus engine
 for system-wide input plus a command-line transliteration tool (okb).
EOF

# Normalize directory permissions (mktemp -d is 0700; packaged dirs must be 0755).
find "$STAGE" -type d -exec chmod 0755 {} +

mkdir -p "$OUTDIR"
DEB="$OUTDIR/${PKG}_${VERSION}_${ARCH}.deb"
fakeroot dpkg-deb --build --root-owner-group "$STAGE" "$DEB"

echo "==> Built $DEB"
dpkg-deb --info "$DEB"
echo "---- contents ----"
dpkg-deb --contents "$DEB"
