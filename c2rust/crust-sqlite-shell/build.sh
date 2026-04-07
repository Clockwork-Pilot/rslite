#!/usr/bin/env bash
# Build script for crust-sqlite-shell (c2rust transpiled SQLite shell)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJ_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
SHELL_SRC="$PROJ_DIR/sqlite-shell-src/shell.c"
C2RUST_CONFIG="$PROJ_DIR/c2rust-shell.toml"

echo "=== Building Crust SQLite Shell (C2Rust) ==="
echo "Project: $PROJ_DIR"
echo "Shell source: $SHELL_SRC"
echo "Config: $C2RUST_CONFIG"
echo ""

# Check if shell.c exists
if [ ! -f "$SHELL_SRC" ]; then
    echo "ERROR: shell.c not found at $SHELL_SRC"
    exit 1
fi

# Check if c2rust is installed
if ! command -v c2rust &> /dev/null; then
    echo "ERROR: c2rust not installed. Install with: cargo install c2rust"
    exit 1
fi

echo "Step 1: Transpile shell.c to Rust using c2rust..."
cd "$PROJ_DIR"
c2rust transpile "$C2RUST_CONFIG"

if [ $? -ne 0 ]; then
    echo "ERROR: c2rust transpilation failed"
    exit 1
fi

echo ""
echo "Step 2: Build Rust crate..."
cd "$SCRIPT_DIR"
cargo build --release

if [ $? -ne 0 ]; then
    echo "ERROR: Cargo build failed"
    exit 1
fi

echo ""
echo "=== Build Complete ==="
echo "Binary: $SCRIPT_DIR/target/release/sqlite3"
