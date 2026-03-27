#!/bin/bash

# Content-hash build cache: if src/ is identical to a previously passing
# build+test run, skip the expensive recompile and test execution.
_PROJ=$(cd "$(dirname "$0")" && pwd)
_BUILD_CACHE_DIR="/tmp/build_all_cache"
_SRC_HASH=$(find "$_PROJ/src" -name "*.rs" | sort | xargs sha256sum 2>/dev/null | sha256sum | awk '{print $1}')
_CACHE_FILE="$_BUILD_CACHE_DIR/$_SRC_HASH"

if [ -f "$_CACHE_FILE" ]; then
    echo "build_all: cache hit — build+tests previously passed for this src/ (${_SRC_HASH:0:8})"
    exit 0
fi

./shell_build.sh

./testfixture_build.sh

cd /sqlite
./rustfixture test/testrunner.tcl
_RC=$?

if [ $_RC -eq 0 ]; then
    mkdir -p "$_BUILD_CACHE_DIR"
    echo "passed" > "$_CACHE_FILE"
fi

exit $_RC
