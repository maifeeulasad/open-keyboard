#!/usr/bin/env bash
#
# Remove the open-keyboard IBus engine installed by packaging/install.sh.
set -euo pipefail

PREFIX="${PREFIX:-/usr}"
LIBEXECDIR="${LIBEXECDIR:-$PREFIX/lib/ibus-open-keyboard}"
COMPONENTDIR="${COMPONENTDIR:-$PREFIX/share/ibus/component}"

SUDO=""
if [[ $EUID -ne 0 ]]; then SUDO="sudo"; fi

echo "==> Removing IBus component"
$SUDO rm -f "$COMPONENTDIR/open-keyboard.xml"

echo "==> Removing engine binary"
$SUDO rm -rf "$LIBEXECDIR"

echo "==> Restarting IBus"
ibus restart || true

echo "Done. You may want to remove the input source from GNOME Settings."
