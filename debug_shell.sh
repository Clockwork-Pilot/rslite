#!/usr/bin/env bash
set -euo pipefail

PROJ=$(cd "$(dirname "$0")" && pwd)
# get from env var or use default
SRC="${SQLITE_SRC:-/sqlite}"

echo "Building crust-sqlite-shell (C2Rust transpiled shell)..."
cargo +nightly build -q --manifest-path "$PROJ/c2rust/Cargo.toml" -p crust-sqlite-shell

# TODO: testrunner.tcl hardcodes /sqlite/sqlite3 as the shell path, so we copy
# the binary there for now. Ideally testrunner.tcl should be updated to use the
# binary directly from $PROJ/c2rust/target/release/sqlite3 and this copy removed.
cp "$PROJ/c2rust/target/debug/sqlite3" "$SRC/sqlite3"
chmod +x "$SRC/sqlite3"

echo "shell_build.sh complete: $SRC/sqlite3"
