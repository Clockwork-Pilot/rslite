#!/bin/bash

set -euo pipefail

export SQLITE_SRC="${SQLITE_SRC:-/sqlite}"
# Content-hash build cache: if src/ is identical to a previously passing
# build+test run, skip the expensive recompile and test execution.
_PROJ=$(cd "$(dirname "$0")" && pwd)
_BUILD_CACHE_DIR="/tmp/build_all_cache"
mkdir -p "$_BUILD_CACHE_DIR"

_calc_hash() {
    find "$_PROJ" -name "*.rs" -not -path "*/target/*" -print0 \
        | sort -z \
        | (xargs -0 sha256sum 2>/dev/null || true) \
        | sha256sum \
        | awk '{print $1}'
}

while true; do
    _SRC_HASH=$(_calc_hash)
    _CACHE_FILE="$_BUILD_CACHE_DIR/$_SRC_HASH"

    if [ -f "$_CACHE_FILE" ]; then
        echo "build_all: cache hit — build+tests previously passed for this src/ (${_SRC_HASH:0:8})"
        exit 0
    fi

    echo "build_all: starting build for src/ hash ${_SRC_HASH:0:8}"

    cd "$_PROJ"
    ./shell_build.sh

    ./testfixture_build.sh

    ( cd "$SQLITE_SRC" && ./rustfixture test/testrunner.tcl 2>&1 | tee /tmp/test_output.log )

    if grep -q "0 errors out of" /tmp/test_output.log; then
        _POST_HASH=$(_calc_hash)
        if [ "$_POST_HASH" != "$_SRC_HASH" ]; then
            echo "build_all: src/ changed during build (${_SRC_HASH:0:8} → ${_POST_HASH:0:8}), restarting..."
            continue
        fi
        echo "passed" > "$_CACHE_FILE"
        echo "Tests passed"
        exit 0
    else
        echo "FAILURE: Tests failed or regex '0 errors out of' not found."
        exit 1
    fi
done
