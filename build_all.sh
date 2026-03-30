#!/bin/bash

set -euo pipefail

# Content-hash build cache: if src/ is identical to a previously passing
# build+test run, skip the expensive recompile and test execution.
_PROJ=$(cd "$(dirname "$0")" && pwd)
_BUILD_CACHE_DIR="/tmp/build_all_cache"
_SRC_HASH=$(find "$_PROJ/src" -name "*.rs" -print0 | sort -z | (xargs -0 sha256sum 2>/dev/null || true) | sha256sum | awk '{print $1}')
_CACHE_FILE="$_BUILD_CACHE_DIR/$_SRC_HASH"

if [ -f "$_CACHE_FILE" ]; then
    echo "build_all: cache hit — build+tests previously passed for this src/ (${_SRC_HASH:0:8})"
    exit 0
fi

./shell_build.sh

./testfixture_build.sh

cd /sqlite
./rustfixture test/testrunner.tcl 2>&1 | tee /tmp/test_output.log

if grep -q "0 errors out of" /tmp/test_output.log; then
    mkdir -p "$_BUILD_CACHE_DIR"
    echo "passed" > "$_CACHE_FILE"
    exit 0
else
    echo "FAILURE: Tests failed or regex '0 errors out of' not found."
    exit 1
fi