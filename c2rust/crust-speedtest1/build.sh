#!/usr/bin/env bash
# Build script for crust-speedtest1 (c2rust transpiled SQLite speedtest1)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=== Building Crust SQLite Speedtest1 (C2Rust) ==="
echo "Crate: $SCRIPT_DIR"
echo ""

cargo +nightly build --release

echo ""
echo "=== Build Complete ==="
echo "Binary: $SCRIPT_DIR/../target/release/speedtest1"
