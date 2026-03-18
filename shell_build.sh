#!/usr/bin/env bash
set -euo pipefail

PROJ=$(pwd)
SRC="$PROJ/sqlite-src-3510200"

mkdir -p "$PROJ/target"
mkdir -p "$PROJ/sqlite-shell"

CARGO_TARGET_DIR="$PROJ/sqlite-shell" cargo build --release

mapfile -t FLAGS < <(sed 's/\r//' "$PROJ/defines_shell.txt" | grep -v '^$')

cc -o "$SRC/sqlite3" \
    "$SRC/shell.c" \
    "${FLAGS[@]}" \
    -DSQLITE_API= \
    -I"$(readlink -f "$SRC")" \
    -L"$PROJ/sqlite-shell/release" -lsqlite_noamalgam \
    -Wl,-rpath,"$PROJ/sqlite-shell/release" \
    -lm -lpthread -ldl -lz
