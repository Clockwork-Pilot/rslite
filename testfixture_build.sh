#!/usr/bin/env bash
set -euo pipefail

PROJ=$(pwd)
SRC="${SQLITE_SRC:-/sqlite}"

echo "Building rustfixture (crust-tclsqlite)..."
cargo build --release -q -p crust-tclsqlite --features crust-tclsqlite/test

cp "$PROJ/target/release/rustfixture" "$SRC/rustfixture"
chmod +x "$SRC/rustfixture"

echo "testfixture_build.sh complete: $SRC/rustfixture"
