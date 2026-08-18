#!/usr/bin/env bash
set -euo pipefail

PREFIX="${FLOCK_SIGNAL_PREFIX:-/opt/flock-signal}"
BIN_DIR="${PREFIX}/bin"
LIB_DIR="${PREFIX}/lib"
INCLUDE_DIR="${PREFIX}/include"
ENV_DIR="${PREFIX}/etc"
SERVICE_USER="flocksignal"

command -v cargo >/dev/null 2>&1 || { echo "cargo is required" >&2; exit 1; }
command -v cmake >/dev/null 2>&1 || { echo "cmake is required" >&2; exit 1; }

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

cargo build --release --workspace
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --parallel 2
ctest --test-dir build --output-on-failure

if ! id -u "$SERVICE_USER" >/dev/null 2>&1; then
  sudo useradd --system --user-group --home-dir "$PREFIX" --shell /usr/sbin/nologin "$SERVICE_USER"
fi

sudo install -d -o root -g "$SERVICE_USER" -m 0755 "$PREFIX" "$BIN_DIR"
sudo install -d -o root -g root -m 0755 "$LIB_DIR" "$INCLUDE_DIR" "$INCLUDE_DIR/flock_signal"
sudo install -d -o root -g "$SERVICE_USER" -m 0750 "$ENV_DIR"
sudo install -o root -g root -m 0755 target/release/flock-signal-gateway "$BIN_DIR/flock-signal-gateway"
sudo install -o root -g root -m 0755 target/release/libflock_signal_ffi.so "$LIB_DIR/libflock_signal_ffi.so"
sudo install -o root -g root -m 0644 cpp/include/flock_signal/flock_signal.h "$INCLUDE_DIR/flock_signal/flock_signal.h"
sudo install -o root -g root -m 0644 cpp/include/flock_signal/flock_signal.hpp "$INCLUDE_DIR/flock_signal/flock_signal.hpp"

if [[ ! -f "$ENV_DIR/flock-signal.env" ]]; then
  tmp_env="$(mktemp)"
  trap 'rm -f "$tmp_env"' EXIT
  printf '%s\n' \
    'FLOCK_SIGNAL_API_TOKEN=REPLACE_ME' \
    'FLOCK_SIGNAL_BIND=127.0.0.1:8080' > "$tmp_env"
  sudo install -o root -g "$SERVICE_USER" -m 0640 "$tmp_env" "$ENV_DIR/flock-signal.env"
  rm -f "$tmp_env"
  trap - EXIT
  echo "Created $ENV_DIR/flock-signal.env. Replace REPLACE_ME before starting the service."
else
  sudo chown root:"$SERVICE_USER" "$ENV_DIR/flock-signal.env"
  sudo chmod 0640 "$ENV_DIR/flock-signal.env"
fi

sudo install -o root -g root -m 0644 install/linux/flock-signal-gateway.service /etc/systemd/system/flock-signal-gateway.service

echo "Installed runtime and SDK to $PREFIX."
echo "  Gateway: $BIN_DIR/flock-signal-gateway"
echo "  C/C++ headers: $INCLUDE_DIR/flock_signal"
echo "  Shared library: $LIB_DIR/libflock_signal_ffi.so"
echo "Configure $ENV_DIR/flock-signal.env, then run:"
echo "  sudo systemctl daemon-reload"
echo "  sudo systemctl enable --now flock-signal-gateway"
