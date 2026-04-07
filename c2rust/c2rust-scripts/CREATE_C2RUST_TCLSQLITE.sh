#!/usr/bin/env bash
################################################################################
# CREATE_C2RUST_TCLSQLITE.sh
#
# Automates the creation of a C2Rust transpiled SQLite testfixture (rustfixture)
# from tclsqlite.c and all test_*.c files.
#
# Usage:
#   ./CREATE_C2RUST_TCLSQLITE.sh [sqlite_src] [output_dir] [defines_testfixture_path]
#
# Example:
#   ./CREATE_C2RUST_TCLSQLITE.sh /sqlite ./crust-tclsqlite ./defines_testfixture.txt
#
# Prerequisites:
#   - C2Rust binary: /c2rust/target/release/c2rust
#   - Cargo (Rust compiler)
#   - TCL development headers (pkg-config tcl)
#   - Existing sqlite_noamalgam Rust library
#
################################################################################

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

C2RUST_BIN="${C2RUST_BIN:-/c2rust/target/release/c2rust}"
SRC="${1:-/sqlite}"
OUTPUT_DIR="${2:-./crust-tclsqlite}"
DEFINES_FILE="${3:-$(cd "$(dirname "$0")/.." && pwd)/defines_testfixture.txt}"
PROJ_DIR="$(cd "$(dirname "$0")/.." && pwd)"

echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}C2Rust SQLite Testfixture Creation Script${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""

# ============================================================================
# Step 1: Verify Prerequisites
# ============================================================================
echo -e "${YELLOW}[1/7] Verifying prerequisites...${NC}"

if [ ! -f "$C2RUST_BIN" ]; then
    echo -e "${RED}ERROR: C2Rust binary not found at $C2RUST_BIN${NC}"
    echo "Build c2rust with: cd /c2rust && cargo build --release"
    exit 1
fi
echo "  ✓ C2Rust found: $C2RUST_BIN"

if [ ! -f "$SRC/src/tclsqlite.c" ]; then
    echo -e "${RED}ERROR: tclsqlite.c not found at $SRC/src/tclsqlite.c${NC}"
    exit 1
fi
echo "  ✓ SQLite source found: $SRC"

if [ ! -f "$DEFINES_FILE" ]; then
    echo -e "${RED}ERROR: defines_testfixture.txt not found at $DEFINES_FILE${NC}"
    exit 1
fi
echo "  ✓ defines_testfixture.txt found: $DEFINES_FILE"

if ! pkg-config --exists tcl 2>/dev/null; then
    echo -e "${RED}ERROR: TCL development headers not found (pkg-config tcl)${NC}"
    exit 1
fi
TCL_VERSION=$(pkg-config --modversion tcl)
echo "  ✓ TCL found: $TCL_VERSION"

if ! which cargo > /dev/null 2>&1; then
    echo -e "${RED}ERROR: Cargo not found${NC}"
    exit 1
fi
echo "  ✓ Cargo available"

echo ""

# ============================================================================
# Step 2: Create Directory Structure
# ============================================================================
echo -e "${YELLOW}[2/7] Creating project structure...${NC}"

mkdir -p "$OUTPUT_DIR/src"
echo "  ✓ Created $OUTPUT_DIR"

echo ""

# ============================================================================
# Step 3: Generate C2Rust Configuration
# ============================================================================
echo -e "${YELLOW}[3/7] Generating C2Rust configuration...${NC}"

mapfile -t FLAGS < <(sed 's/\r//' "$DEFINES_FILE" | grep -v '^$')
TCL_CFLAGS=$(pkg-config --cflags tcl)

# List of all source files to transpile
SOURCE_FILES=(
    "$SRC/src/tclsqlite.c"
    "$SRC/src/test1.c"
    "$SRC/src/test2.c"
    "$SRC/src/test3.c"
    "$SRC/src/test4.c"
    "$SRC/src/test5.c"
    "$SRC/src/test6.c"
    "$SRC/src/test8.c"
    "$SRC/src/test9.c"
    "$SRC/src/test_autoext.c"
    "$SRC/src/test_backup.c"
    "$SRC/src/test_bestindex.c"
    "$SRC/src/test_blob.c"
    "$SRC/src/test_btree.c"
    "$SRC/src/test_config.c"
    "$SRC/src/test_delete.c"
    "$SRC/src/test_demovfs.c"
    "$SRC/src/test_devsym.c"
    "$SRC/src/test_fs.c"
    "$SRC/src/test_func.c"
    "$SRC/src/test_hexio.c"
    "$SRC/src/test_init.c"
    "$SRC/src/test_intarray.c"
    "$SRC/src/test_journal.c"
    "$SRC/src/test_malloc.c"
    "$SRC/src/test_md5.c"
    "$SRC/src/test_multiplex.c"
    "$SRC/src/test_mutex.c"
    "$SRC/src/test_onefile.c"
    "$SRC/src/test_osinst.c"
    "$SRC/src/test_pcache.c"
    "$SRC/src/test_quota.c"
    "$SRC/src/test_rtree.c"
    "$SRC/src/test_schema.c"
    "$SRC/src/test_superlock.c"
    "$SRC/src/test_syscall.c"
    "$SRC/src/test_tclsh.c"
    "$SRC/src/test_tclvar.c"
    "$SRC/src/test_thread.c"
    "$SRC/src/test_vdbecov.c"
    "$SRC/src/test_vfs.c"
    "$SRC/src/test_window.c"
    "$SRC/src/test_wsd.c"
    "$SRC/ext/fts3/fts3_term.c"
    "$SRC/ext/fts3/fts3_test.c"
    "$SRC/ext/session/test_session.c"
    "$SRC/ext/recover/sqlite3recover.c"
    "$SRC/ext/recover/dbdata.c"
    "$SRC/ext/recover/test_recover.c"
    "$SRC/ext/intck/test_intck.c"
    "$SRC/ext/intck/sqlite3intck.c"
    "$SRC/ext/rbu/test_rbu.c"
    "$SRC/ext/expert/sqlite3expert.c"
    "$SRC/ext/expert/test_expert.c"
    "$SRC/ext/misc/amatch.c"
    "$SRC/ext/misc/appendvfs.c"
    "$SRC/ext/misc/basexx.c"
    "$SRC/ext/misc/cksumvfs.c"
    "$SRC/ext/misc/closure.c"
    "$SRC/ext/misc/csv.c"
    "$SRC/ext/misc/decimal.c"
    "$SRC/ext/misc/eval.c"
    "$SRC/ext/misc/explain.c"
    "$SRC/ext/misc/fileio.c"
    "$SRC/ext/misc/fuzzer.c"
    "$SRC/ext/fts5/fts5_tcl.c"
    "$SRC/ext/fts5/fts5_test_mi.c"
    "$SRC/ext/fts5/fts5_test_tok.c"
    "$SRC/ext/misc/ieee754.c"
    "$SRC/ext/misc/mmapwarm.c"
    "$SRC/ext/misc/nextchar.c"
    "$SRC/ext/misc/normalize.c"
    "$SRC/ext/misc/prefixes.c"
    "$SRC/ext/misc/qpvtab.c"
    "$SRC/ext/misc/randomjson.c"
    "$SRC/ext/misc/regexp.c"
    "$SRC/ext/misc/remember.c"
    "$SRC/ext/misc/series.c"
    "$SRC/ext/misc/spellfix.c"
    "$SRC/ext/misc/stmtrand.c"
    "$SRC/ext/misc/totype.c"
    "$SRC/ext/misc/unionvtab.c"
    "$SRC/ext/misc/wholenumber.c"
    "$SRC/ext/misc/zipfile.c"
    "$SRC/ext/rtree/test_rtreedoc.c"
)

INCLUDES=(
    "-I$SRC"
    "-I$SRC/src"
    "-I$SRC/ext/rtree"
    "-I$SRC/ext/icu"
    "-I$SRC/ext/fts3"
    "-I$SRC/ext/session"
    "-I$SRC/ext/misc"
)

# Generate c2rust config
cat > "$OUTPUT_DIR/c2rust-tclsqlite.toml" << TOML
[transpilation]
emit_modules = true
emit_extern_crates = true
use_std = true
output_dir = "src/"

[c2rust.options]
simplify_extern_crates = true
preserve_comments = true
emit_panic_on_fail = true
TOML

echo "  ✓ Generated c2rust-tclsqlite.toml"
echo ""

# ============================================================================
# Step 4: Generate compile_commands.json and Transpile
# ============================================================================
echo -e "${YELLOW}[4/7] Generating compile_commands.json...${NC}"

COMPILE_COMMANDS="$OUTPUT_DIR/compile_commands.json"

# Parse TCL_CFLAGS into array elements
read -ra TCL_CFLAGS_ARRAY <<< "$TCL_CFLAGS"

# Use Python to generate valid JSON (handles special chars like -DFOO="" correctly)
EXISTING_FILES=()
for src_file in "${SOURCE_FILES[@]}"; do
    if [ ! -f "$src_file" ]; then
        echo "  WARNING: $src_file not found, skipping"
    else
        EXISTING_FILES+=("$src_file")
    fi
done

python3 - "$SRC" "$COMPILE_COMMANDS" "${FLAGS[@]}" "${INCLUDES[@]}" \
    "${TCL_CFLAGS_ARRAY[@]}" "-DTCLSH_INIT_PROC=sqlite3TestInit" \
    "---files---" "${EXISTING_FILES[@]}" << 'PYEOF'
import json, sys

args = sys.argv[1:]
src_dir = args[0]
output_path = args[1]
rest = args[2:]

sep = rest.index("---files---")
compile_flags = rest[:sep]
source_files = rest[sep+1:]

entries = []
for f in source_files:
    entries.append({
        "directory": src_dir,
        "arguments": ["cc"] + compile_flags + ["-c", f],
        "file": f,
    })

with open(output_path, "w") as fp:
    json.dump(entries, fp, indent=2)
PYEOF

echo "  ✓ Generated compile_commands.json (${#EXISTING_FILES[@]} files)"

echo ""
echo -e "${YELLOW}[4/7] Transpiling ${#SOURCE_FILES[@]} C files to Rust...${NC}"

if "$C2RUST_BIN" transpile "$COMPILE_COMMANDS" \
    --emit-modules \
    --disable-rustfmt \
    --output-dir "$OUTPUT_DIR/src" > /tmp/c2rust-tclsqlite.log 2>&1; then
    echo "  ✓ Transpilation successful"
    RUST_FILES=$(find "$OUTPUT_DIR/src" -name "*.rs" | wc -l)
    echo "  ✓ Generated $RUST_FILES Rust files"
else
    echo -e "${RED}ERROR: Transpilation failed${NC}"
    cat /tmp/c2rust-tclsqlite.log
    exit 1
fi

echo ""

# ============================================================================
# Step 5: Create Cargo Configuration
# ============================================================================
echo -e "${YELLOW}[5/7] Creating Cargo configuration...${NC}"

cat > "$OUTPUT_DIR/Cargo.toml" << 'TOML'
[package]
name = "crust-tclsqlite"
version = "0.1.0"
edition = "2024"
authors = ["C2Rust Automated Migration"]
build = "build.rs"

[[bin]]
name = "rustfixture"
path = "src/main.rs"

[dependencies]
sqlite_noamalgam = { path = "../" }
libc = "0.2"

[build-dependencies]
pkg-config = "0.3"

[features]
default = []
test = ["sqlite_noamalgam/test"]

[profile.release]
lto = true
codegen-units = 1
opt-level = 3
TOML

echo "  ✓ Created $OUTPUT_DIR/Cargo.toml"

cat > "$OUTPUT_DIR/build.rs" << 'RUST'
fn main() {
    // TCL library — required for the TCL interpreter embedded in rustfixture
    let tcl = pkg_config::probe_library("tcl").expect("TCL not found via pkg-config");
    for path in &tcl.link_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
    }
    for lib in &tcl.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    // System libraries
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=c");
}
RUST

echo "  ✓ Created $OUTPUT_DIR/build.rs"

# Generate mod.rs files for the nested directory structure
# (C2Rust mirrors source paths: src/ → src/src/src/, ext/ → src/src/ext/)
echo -e "${YELLOW}[5/7] Generating module hierarchy (mod.rs files)...${NC}"

python3 - "$OUTPUT_DIR/src" << 'PYEOF'
import os, sys

base = sys.argv[1]
for dirpath, dirnames, filenames in os.walk(base):
    if dirpath == base:
        continue
    rs_files = sorted(f[:-3] for f in filenames if f.endswith(".rs") and f != "mod.rs")
    subdirs = sorted(dirnames)
    lines = [f"pub mod {sub};" for sub in subdirs] + [f"pub mod {rs};" for rs in rs_files]
    with open(os.path.join(dirpath, "mod.rs"), "w") as f:
        f.write("\n".join(lines) + "\n")
PYEOF
echo "  ✓ Generated mod.rs files for module hierarchy"

# Generate main.rs — the binary entry point
cat > "$OUTPUT_DIR/src/main.rs" << 'RUST'
#![feature(extern_types, c_variadic)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(warnings)]
#![allow(static_mut_refs)]

// Force cargo to link sqlite_noamalgam — C FFI calls, no Rust type imports yet.
// Remove once modules use `use sqlite_noamalgam::...` directly.
extern crate sqlite_noamalgam;
#[macro_use]
extern crate c2rust_bitfields;

pub mod src;

fn main() {
    // tclsqlite is at src/src/src/tclsqlite.rs → path: src::src::tclsqlite
    src::src::tclsqlite::main();
}
RUST

echo "  ✓ Created $OUTPUT_DIR/src/main.rs"
echo ""

# ============================================================================
# Step 6: Fix Generated Code Issues
# ============================================================================
echo -e "${YELLOW}[6/7] Fixing generated code issues...${NC}"

# Fix 1: #[no_mangle] → #[unsafe(no_mangle)] for edition 2024
find "$OUTPUT_DIR/src/src" -name "*.rs" -exec \
    sed -i 's/^#\[no_mangle\]$/#[unsafe(no_mangle)]/g' {} \;
find "$OUTPUT_DIR/src/src" -name "*.rs" -exec \
    sed -i 's/\(^\s*\)#\[no_mangle\]$/\1#[unsafe(no_mangle)]/g' {} \;
echo "  ✓ Fixed #[no_mangle] → #[unsafe(no_mangle)] for edition 2024"

# Fix 2: extern "C" { → unsafe extern "C" { for edition 2024
find "$OUTPUT_DIR/src/src" -name "*.rs" -exec \
    sed -i 's/\(^\s*\)extern "\([^"]*\)" {$/\1unsafe extern "\2" {/g' {} \;
echo "  ✓ Fixed extern blocks → unsafe extern"

# Fix 3: link_section attribute needs unsafe(...) wrapper in edition 2024
find "$OUTPUT_DIR/src/src" -name "*.rs" -exec \
    sed -i 's/link_section = "\([^"]*\)")/unsafe(link_section = "\1"))/g' {} \;
echo "  ✓ Fixed link_section → unsafe(link_section)"

# Fix 4: Remove VaListImpl declarations and .as_va_list() calls
python3 - "$OUTPUT_DIR/src/src" << 'PYEOF'
import re, os, sys

base = sys.argv[1]
for dirpath, _, filenames in os.walk(base):
    for fname in filenames:
        if not fname.endswith(".rs"):
            continue
        fpath = os.path.join(dirpath, fname)
        with open(fpath) as f:
            content = f.read()
        if "VaListImpl" not in content and "as_va_list" not in content:
            continue
        # Remove VaListImpl type declarations
        new = re.sub(r'\s*let mut (\w+): ::core::ffi::VaListImpl;\n', '\n', content)
        # Convert bare assignment to let binding
        new = re.sub(r'\b(\w+) = (c2rust_args|[\w_]+)\.clone\(\);', r'let mut \1 = \2.clone();', new)
        # Fix double let mut from repeated passes
        new = new.replace('let mut let mut ', 'let mut ')
        # Remove .as_va_list() calls
        new = new.replace('.as_va_list()', '')
        if new != content:
            with open(fpath, 'w') as f:
                f.write(new)
PYEOF
echo "  ✓ Fixed VaListImpl → VaList (nightly API change)"

echo ""

# ============================================================================
# Step 7: Integration Setup
# ============================================================================
echo -e "${YELLOW}[7/7] Setting up integration...${NC}"

if ! grep -q "crust-tclsqlite" "$PROJ_DIR/Cargo.toml"; then
    sed -i '/members = \[/a\    "crust-tclsqlite",' "$PROJ_DIR/Cargo.toml"
    echo "  ✓ Added crust-tclsqlite to workspace"
fi

echo ""

# ============================================================================
# Summary
# ============================================================================
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ C2Rust Testfixture Creation Complete!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo "Created Files:"
echo "  ✓ $OUTPUT_DIR/Cargo.toml"
echo "  ✓ $OUTPUT_DIR/build.rs"
echo "  ✓ $OUTPUT_DIR/c2rust-tclsqlite.toml"
echo "  ✓ $OUTPUT_DIR/src/main.rs"
echo "  ✓ $OUTPUT_DIR/src/ ($RUST_FILES transpiled Rust files)"
echo ""
echo "Next Steps:"
echo "  1. Build the testfixture:"
echo "     cargo build --release -p crust-tclsqlite"
echo ""
echo "  2. Update testfixture_build.sh to use:"
echo "     cargo build --release -p crust-tclsqlite"
echo ""
echo "  3. Run the full test suite:"
echo "     ./build_all.sh"
echo ""
