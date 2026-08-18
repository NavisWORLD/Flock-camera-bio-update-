#!/usr/bin/env bash
set -euo pipefail

PREFIX="${FLOCK_SIGNAL_PREFIX:-/opt/flock-signal}"
BIN_DIR="${PREFIX}/bin"
ENV_DIR="${PREFIX}/etc"

command -v cargo >/dev/null 2>&1 || { echo "cargo is required" >&2; exit 1; }
command -v cmake >/dev/null 2>&1 || { echo "cmake is required" >&2; exit 1; }

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

cargo build --release --workspace
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --parallel 2
ctest --test-dir build --output-on-failure

sudo install -d -m 0755 "$BIN_DIR" "$ENV_DIR"
sudo install -m 0755 target/release/flock-signal-gateway "$BIN_DIR/flock-signal-gateway"

if [[ ! -f "$ENV_DIR/flock-signal.env" ]]; then
  sudo sh -c "umask 077; printf '%s\n' 'FLOCK_SIGNAL_API_TOKEN=REPLACE_ME' 'FLOCK_SIGNAL_BIND=127.0.0.1:8080' > '$ENV_DIR/flock-signal.env'"
  echo "Created $ENV_DIR/flock-signal.env. Replace REPLACE_ME before starting the service."
fi

sudo install -m 0644 install/linux/flock-signal-gateway.service /etc/systemd/system/flock-signal-gateway.service

echo "Installed to $PREFIX. Configure $ENV_DIR/flock-signal.env, then run:"
echo "  sudo systemctl daemon-reload"
echo "  sudo systemctl enable --now flock-signal-gateway"
