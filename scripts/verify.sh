#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

command -v cargo >/dev/null || { echo "cargo is required" >&2; exit 2; }
command -v cmake >/dev/null || { echo "cmake is required" >&2; exit 2; }

cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --release

cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release
ctest --test-dir build --output-on-failure

echo "Verification complete: Rust workspace and C++ interoperability tests passed."
