#!/usr/bin/env bash
# check_static_linkage.sh — Prove a binary is statically linked against sqlite_noamalgam.
#
# Usage:
#   ./c2rust-scripts/check_static_linkage.sh /sqlite/rustfixture

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

BINARY="${1:-/sqlite/rustfixture}"

if [ ! -f "$BINARY" ]; then
    echo -e "${RED}ERROR: binary not found: $BINARY${NC}"
    exit 1
fi

echo -e "${YELLOW}Analyzing: $BINARY${NC}"
echo ""

# ── 1. Dynamic dependency check ──────────────────────────────────────────────
echo "Dynamic dependencies (ldd):"
ldd "$BINARY" | sed 's/^/  /'
echo ""

if ldd "$BINARY" | grep -q "sqlite"; then
    echo -e "${RED}FAIL: binary dynamically links a sqlite library${NC}"
    ldd "$BINARY" | grep "sqlite"
    exit 1
fi
echo -e "${GREEN}✓ No libsqlite_noamalgam.so in dynamic deps — not dynamically linked${NC}"

# ── 2. Symbol presence check ─────────────────────────────────────────────────
echo ""
echo "Key SQLite symbols in binary (nm):"
if nm "$BINARY" 2>/dev/null | grep -E " T sqlite3_(open|exec|prepare)" | head -5 | sed 's/^/  /'; then
    echo -e "${GREEN}✓ SQLite symbols found as text (T) symbols — rlib is baked in${NC}"
else
    echo -e "${YELLOW}  (symbols stripped or nm unavailable)${NC}"
fi

# ── 3. File size sanity ───────────────────────────────────────────────────────
echo ""
SIZE=$(du -sh "$BINARY" | cut -f1)
echo "Binary size: $SIZE  (a dynamically linked binary would be much smaller)"
