#!/usr/bin/env bash
#
# Install the open-keyboard IBus engine on Ubuntu (or another IBus system).
#
# Steps:
#   1. build the engine in release mode,
#   2. copy the binary to LIBEXECDIR,
#   3. install the IBus component descriptor,
#   4. restart IBus so it picks up the new engine.
#
# Usage:
#   packaging/install.sh                # installs to /usr/lib/ibus-open-keyboard
#   sudo is invoked automatically for the system paths.
set -euo pipefail

PREFIX="${PREFIX:-/usr}"
LIBEXECDIR="${LIBEXECDIR:-$PREFIX/lib/ibus-open-keyboard}"
COMPONENTDIR="${COMPONENTDIR:-$PREFIX/share/ibus/component}"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

echo "==> Building okb-ibus-engine (release)"
cargo build --release -p okb-ibus

BIN="$REPO_ROOT/target/release/okb-ibus-engine"
if [[ ! -x "$BIN" ]]; then
  echo "error: built binary not found at $BIN" >&2
  exit 1
fi

echo "==> Checking for IBus"
if ! command -v ibus >/dev/null 2>&1; then
  echo "error: 'ibus' not found. Install it first:  sudo apt install ibus" >&2
  exit 1
fi

SUDO=""
if [[ $EUID -ne 0 ]]; then SUDO="sudo"; fi

echo "==> Installing binary to $LIBEXECDIR"
$SUDO install -Dm755 "$BIN" "$LIBEXECDIR/okb-ibus-engine"

echo "==> Installing IBus component to $COMPONENTDIR"
TMP_XML="$(mktemp)"
sed "s#@LIBEXEC@#$LIBEXECDIR#g" "$REPO_ROOT/packaging/ibus/open-keyboard.xml.in" > "$TMP_XML"
$SUDO install -Dm644 "$TMP_XML" "$COMPONENTDIR/open-keyboard.xml"
rm -f "$TMP_XML"

echo "==> Restarting IBus"
ibus restart || true

cat <<'EOF'

Done. Now enable the input source:

  GNOME Settings → Keyboard → Input Sources → (+) → Bengali →
    "Bengali (open-keyboard, phonetic)"

  or from a terminal:
    ibus engine open-keyboard-bn

Switch to it with Super+Space, then type e.g. "amar sonar bangla".
If it does not appear, log out and back in so GNOME re-reads input sources.
EOF
