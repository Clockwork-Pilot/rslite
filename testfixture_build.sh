#!/usr/bin/env bash
set -euo pipefail

PROJ=$(cd "$(dirname "$0")" && pwd)
SRC="${SQLITE_SRC:-/sqlite}"

echo "Building rustfixture (crust-tclsqlite)..."
cargo +nightly build --release -q --manifest-path "$PROJ/c2rust/Cargo.toml" -p crust-tclsqlite --features crust-tclsqlite/test

cp "$PROJ/c2rust/target/release/rustfixture" "$SRC/rustfixture"
chmod +x "$SRC/rustfixture"

echo "testfixture_build.sh complete: $SRC/rustfixture"
