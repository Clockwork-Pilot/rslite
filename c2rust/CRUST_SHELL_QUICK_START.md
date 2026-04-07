# C2Rust SQLite Shell - Quick Start Guide

This guide provides a fast path to get the C2Rust transpiled SQLite shell running and integrated into the build system.

## Current Status

✓ **Completed:**
- Project structure created (`crust-sqlite-shell/`)
- Configuration files ready (`c2rust-shell.toml`, `Cargo.toml`)
- Documentation complete
- Integration plan documented

⏳ **Blocked:** Awaiting LLVM installation for c2rust

---

## Phase 1: Install LLVM (Prerequisite)

C2Rust requires LLVM development headers. Choose one method:

### Method 1: System Package (Recommended)
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y llvm-dev libclang-dev clang

# Verify installation
llvm-config --version
```

### Method 2: Specific LLVM Version
```bash
# Ubuntu 22.04 - LLVM 15
sudo apt-get install -y llvm-15-dev

# Symlink to default location (if needed)
sudo ln -s /usr/bin/llvm-config-15 /usr/bin/llvm-config
```

### Method 3: LLVM APT Repository
```bash
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 15  # Installs LLVM 15

llvm-config --version  # Verify
```

**Verify Installation:**
```bash
llvm-config --version    # Shows version (e.g., "15.0.0")
which clang-15          # Should find clang
pkg-config --cflags libclang
```

---

## Phase 2: Install C2Rust

Once LLVM is installed:

```bash
cargo install c2rust
c2rust --version  # Should show version (e.g., "0.22.1")
```

**If installation fails:**
```bash
# Try with LLVM explicitly
export LLVM_CONFIG=/usr/bin/llvm-config-15
cargo install c2rust

# Or set library path
export LLVM_LIB_DIR=/usr/lib/llvm-15/lib
cargo install c2rust
```

---

## Phase 3: Transpile shell.c

From `/workspace`:

```bash
c2rust transpile c2rust-shell.toml
```

This generates Rust code in `crust-sqlite-shell/src/generated/`

**Expected output:**
```
✓ Analyzing shell.c
✓ Running Clang compiler
✓ Generating Rust code
✓ Writing to crust-sqlite-shell/src/generated/
```

---

## Phase 4: Build Rust Shell

```bash
cd /workspace/crust-sqlite-shell
cargo build --release
```

**Expected binary:** `target/release/sqlite3`

**Build time:** ~2-5 minutes (first build), ~10-30 seconds (incremental)

---

## Phase 5: Quick Test

```bash
# Quick smoke test
./target/release/sqlite3 :memory: "SELECT 1;"

# Should output:
# 1

# Test with actual database
./target/release/sqlite3 test.db "CREATE TABLE t(x); INSERT INTO t VALUES(42); SELECT * FROM t;"
```

---

## Phase 6: Integration with Build System

### Option A: Test Both (C and Rust)

Keep current setup, add Rust build in parallel:

```bash
# In /workspace:
./shell_build.sh        # Builds C shell (existing)
./crust-sqlite-shell/build.sh  # Builds Rust shell (new)
```

### Option B: Switch to Rust Only

Replace `shell_build.sh` with:

```bash
#!/usr/bin/env bash
set -euo pipefail

PROJ=$(pwd)
SRC="${SQLITE_SRC:-/sqlite}"

# Build Rust library
mkdir -p "$PROJ/target" "$PROJ/sqlite-shell"
CARGO_TARGET_DIR="$PROJ/sqlite-shell" cargo build -q --release

# Build Rust shell
cd "$PROJ/crust-sqlite-shell"
cargo build -q --release

# Install to test location
cp target/release/sqlite3 "$SRC/sqlite3"
echo "Installed: $SRC/sqlite3"
```

### Option C: Automatic Selection

Add to `build_all.sh`:

```bash
# Build library
CARGO_TARGET_DIR="$_PROJ/sqlite-shell" cargo build -q --release

# Use Rust shell if available, otherwise fall back to C
if [ -f "$_PROJ/crust-sqlite-shell/Cargo.toml" ] && command -v c2rust > /dev/null 2>&1; then
    echo "Building Rust shell..."
    cd "$_PROJ/crust-sqlite-shell" && cargo build -q --release
    cp target/release/sqlite3 "$SQLITE_SRC/sqlite3"
else
    echo "Using C shell (fallback)..."
    ./shell_build.sh
fi
```

---

## Verification Checklist

### After Transpilation
```bash
# Generated files exist
ls crust-sqlite-shell/src/generated/
✓ mod.rs and shell_*.rs files present

# Code compiles
cd crust-sqlite-shell && cargo build
✓ 0 errors, 0-N warnings
```

### After Build
```bash
# Binary exists and runs
./crust-sqlite-shell/target/release/sqlite3 --version
✓ SQLite version output

# Quick test
./crust-sqlite-shell/target/release/sqlite3 :memory: "SELECT 1;"
✓ Returns "1"
```

### Integration
```bash
# Full build with tests
./build_all.sh
✓ All tests pass
```

---

## Troubleshooting

### Problem: "c2rust command not found"

**Solution:** Install c2rust
```bash
cargo install c2rust
```

### Problem: "LLVM lib dir not found"

**Solution:** Install LLVM development libraries
```bash
# Ubuntu/Debian
sudo apt-get install llvm-dev libclang-dev

# Or specific version
sudo apt-get install llvm-15-dev
```

### Problem: Compilation errors after transpilation

**Expected:** 10-50 compilation errors are normal for first transpilation

**Fix:**
1. Review errors in build output
2. Common issues:
   - Variadic functions need wrappers
   - Global mutable state needs `lazy_static`
   - Unsafe pointer dereferencing
3. Fix iteratively: `cargo build` → fix → repeat

### Problem: Tests fail after building Rust shell

**Check:**
1. Binary location: `file /sqlite/sqlite3` (should be ELF)
2. Binary runs: `/sqlite/sqlite3 :memory: "SELECT 1;"`
3. Permissions: `chmod +x /sqlite/sqlite3`
4. Library linked: `ldd /sqlite/sqlite3` (check libsqlite)

---

## Development Workflow

### Making Changes to shell.c

1. Edit `sqlite-shell-src/shell.c`
2. Re-transpile:
   ```bash
   c2rust transpile c2rust-shell.toml
   ```
3. Rebuild:
   ```bash
   cd crust-sqlite-shell && cargo build --release
   ```
4. Test:
   ```bash
   ./target/release/sqlite3 :memory: "SELECT 1;"
   ```

### Debugging

```bash
# Build debug version with symbols
cd crust-sqlite-shell
cargo build  # Debug build, not --release

# Run with debugger
rust-gdb ./target/debug/sqlite3
```

---

## Performance

### Build Times (Approximate)

| Stage | First | Incremental |
|-------|-------|-------------|
| Rust lib | 10-30s | 1-5s |
| Rust shell | 30-60s | 5-15s |
| Tests | 3-5m | 3-5m |
| **Total** | **5-8m** | **4-6m** |

### Runtime Performance

| Metric | Target | Notes |
|--------|--------|-------|
| Startup | <15ms | C: ~10ms (5% tolerance) |
| Query execution | baseline | Within 2% |
| Memory | baseline | Within 10% |

---

## Files Created/Modified

### New Files
- `crust-sqlite-shell/` - Main project
- `c2rust-shell.toml` - Transpiler config
- `c2rust_shell_actions_log.md` - Detailed log
- `CRUST_SHELL_INTEGRATION.md` - Integration guide
- `build_all_updated.sh` - Updated build script
- `CRUST_SHELL_QUICK_START.md` - This file

### To Be Modified
- `build_all.sh` - Switch to Rust shell build
- `KNOWN_ISSUES.md` - Document migration status

---

## Next Steps

1. **Install LLVM:** Choose method from Phase 1
2. **Install c2rust:** Run `cargo install c2rust`
3. **Transpile:** Run `c2rust transpile c2rust-shell.toml`
4. **Build:** `cd crust-sqlite-shell && cargo build --release`
5. **Test:** `./target/release/sqlite3 :memory: "SELECT 1;"`
6. **Integrate:** Update `build_all.sh`

**Estimated time:** 30-60 minutes (excluding build time)

---

## Support

**For detailed information, see:**
- `/workspace/c2rust_shell_actions_log.md` - Complete action log
- `/workspace/CRUST_SHELL_INTEGRATION.md` - Integration details
- `/workspace/crust-sqlite-shell/README.md` - Project README
- `/workspace/SHELL_C_TO_RUST_MIGRATION.md` - Migration guide

**For help:**
1. Check logs for specific errors
2. Review troubleshooting section above
3. Consult C2Rust documentation: https://c2rust.com
4. Check compilation output for detailed error messages
