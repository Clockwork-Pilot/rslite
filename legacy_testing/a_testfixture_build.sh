#!/usr/bin/env bash
set -euo pipefail

PROJ=$(pwd)

mkdir -p "$PROJ/sqlite-testfixture" 


CARGO_TARGET_DIR="$PROJ/sqlite-testfixture" cargo build -q --features "test  fts3 fts4"

bash a_testfixture_swap_optimized.sh