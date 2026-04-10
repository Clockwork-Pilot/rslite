# Specification

## Overview

Verify c_variadic feature isolation: only in printf_c_variadic.rs

## Table of Contents

- [Overview](#overview)
- [Features](#features)
    - [Feature: build_all](#build_all)
      - [constraint_build_all](#constraint_build_all)
    - [Feature: clippy_checks](#clippy_checks)
      - [clippy_checks](#clippy_checks)
      - [no_forbidden_allows_robust](#no_forbidden_allows_robust)
    - [Feature: fts5_no_libc_global](#fts5_no_libc_global)
      - [fts5_no_libc_calls](#fts5_no_libc_calls)
    - [Feature: lib_exports](#lib_exports)
      - [cdylib_symbols](#cdylib_symbols)
      - [no_rust_variadic_exports](#no_rust_variadic_exports)
    - [Feature: pub_visibility](#pub_visibility)
      - [cdylib_default_exports](#cdylib_default_exports)
      - [rslite_uses_crust_core](#rslite_uses_crust_core)
      - [test_cdylib_superset](#test_cdylib_superset)
      - [test_tclsqlite_builds](#test_tclsqlite_builds)
    - [Feature: toolchain_version](#toolchain_version)
      - [c2rust_nightly](#c2rust_nightly)
      - [stable_toolchain_or_missing](#stable_toolchain_or_missing)

## Features

### Feature: build_all
**Check that our rust codebase is healthy**

#### constraint_build_all
**Description:** Ensure our rust codebase is healthy
**Command:** `cd $WORKSPACE_ROOT && make c-tcl-tests`

### Feature: clippy_checks
**Verify no clippy warnings in rust codebase**

**Goals:**
- Run cargo clippy on all packages
- Ensure all clippy lints pass with no warnings

#### clippy_checks
**Description:** Run cargo clippy and ensure no warnings reported
**Command:** `cd $PROJECT_ROOT && cargo clippy --all-targets --all-features -- -D warnings`

#### no_forbidden_allows_robust
**Description:** Structural: Robust whitespace-tolerant detection of forbidden clippy allows. Detects any formatting of allow() with forbidden lints, ignoring whitespace between allow and (.
**Command:** `! grep -rE "allow[[:space:]]*\\(.*clippy::(unnecessary_cast|missing_safety_doc|needless_return)|allow[[:space:]]*\\(.*warnings" "$PROJECT_ROOT/crates" --include="*.rs" 2>/dev/null`

### Feature: fts5_no_libc_global
**Ensure fts5.rs contains no libc:: calls (only external C FFI allowed)**

**Goals:**
- fts5.rs must not contain libc:: direct calls
- All C interop must go through safe wrappers or external C FFI only

#### fts5_no_libc_calls
**Description:** fts5.rs must not contain libc:: calls - use safe wrappers or external C FFI instead
**Command:** `grep -q "libc::" "$PROJECT_ROOT/src/fts5.rs" && exit 1 || exit 0`

### Feature: lib_exports
**All C FFI symbols correctly exported in rlib binaries and cdylib**

**Goals:**
- Verify all 10 exported C FFI symbols are present as global symbols in shell/rustfixture binaries and in the cdylib dynamic symbol table
- Each must have a C source file in c_code/
- Consolidates libs_appendf, libs_snprintf, libs_vsnprintf, libs_vmprintf

#### cdylib_symbols
**Description:** Behavioral: all 10 symbols must appear in dynamic symbol table (T) of the cdylib
**Command:** `SYMS=$(nm -D target/release/libsqlite_noamalgam.so) && for sym in sqlite3_str_appendf sqlite3_snprintf sqlite3_mprintf sqlite3_vsnprintf sqlite3_vmprintf sqlite3_test_control sqlite3_db_config sqlite3_config sqlite3_vtab_config sqlite3_log; do echo "$SYMS" | grep -q "T ${sym}$" || { echo "missing $sym in cdylib"; exit 1; }; done && grep -q "export_name.*sqlite3_test_control_args" $PROJECT_ROOT/crates/crust-core/src/printf_c_variadic.rs`

#### no_rust_variadic_exports
**Description:** Negative: none of the 10 exported symbols must be Rust variadic extern fns — they live in C only
**Command:** `! grep -qE "pub unsafe extern.*fn (sqlite3_str_appendf|sqlite3_snprintf|sqlite3_mprintf|sqlite3_vsnprintf|sqlite3_vmprintf|sqlite3_test_control|sqlite3_db_config|sqlite3_config|sqlite3_vtab_config|sqlite3_log)" $PROJECT_ROOT/src/src/printf.rs $PROJECT_ROOT/crates/crust-core/src/printf_c_variadic.rs 2>/dev/null`

### Feature: pub_visibility
**Non-test cdylib exports only stable C API; test feature re-exposes internals**

**Goals:**
- Only ~280 C API symbols exported from cdylib in default build
- test feature must not lose any default symbols from cdylib
- test feature must keep internals accessible for tclsqlite/testfixture

#### cdylib_default_exports
**Description:** Behavioral: non-test cdylib exports must match expected_exports.txt exactly (~280 stable C API symbols)
**Command:** `cd $PROJECT_ROOT && cargo build -p crust-core --features fts4,update_delete_limit 2>/dev/null && ACTUAL=$(nm -D --defined-only target/debug/libsqlite_noamalgam.so | grep " T " | awk "{print \$3}" | sort) && EXPECTED=$(sort $PROJECT_ROOT/crates/crust-core/expected_exports.txt) && diff <(echo "$ACTUAL") <(echo "$EXPECTED") || { echo "cdylib exports diverged from expected_exports.txt"; exit 1; }`

#### rslite_uses_crust_core
**Description:** Structural: rslite crate must not reference rslite-raw in its Cargo.toml or source files — it should use crust-core (sqlite_noamalgam) directly
**Command:** `cd $PROJECT_ROOT && ! grep -r 'rslite.raw\|rslite_raw' crates/rslite/Cargo.toml crates/rslite/src/ && echo 'rslite does not reference rslite-raw'`

#### test_cdylib_superset
**Description:** Behavioral: test cdylib must be a superset of default exports - all ~280 C API symbols still present
**Command:** `cd $PROJECT_ROOT && cargo build -p crust-core --features fts4,update_delete_limit,test 2>/dev/null && ACTUAL=$(nm -D --defined-only target/debug/libsqlite_noamalgam.so | grep " T " | awk "{print \$3}" | sort) && EXPECTED=$(sort $PROJECT_ROOT/crates/crust-core/expected_exports.txt) && MISSING=$(comm -23 <(echo "$EXPECTED") <(echo "$ACTUAL")) && test -z "$MISSING" || { echo "test build lost default symbols: $MISSING"; exit 1; }`

#### test_tclsqlite_builds
**Description:** Environmental: tclsqlite with test feature must compile, proving internal symbols remain accessible
**Command:** `cd $PROJECT_ROOT && cargo build -p crust-tclsqlite --features test 2>/dev/null`

### Feature: toolchain_version
**Enforce Rust toolchain versions. Stable for main code, nightly for shell and tests.**

**Goals:**
- rust-toolchain.toml must use stable channel
- c2rust rust-toolchain.toml must specify channel nightly

#### c2rust_nightly
**Description:** c2rust must use nightly toolchain (needs c_variadic, extern_types)
**Command:** `grep -q "^channel.*=.*\"nightly" "$PROJECT_ROOT/c2rust/rust-toolchain.toml" && exit 0 || exit 1`

#### stable_toolchain_or_missing
**Description:** rust-toolchain.toml must either be missing or use stable channel
**Command:** `test ! -f "$PROJECT_ROOT/rust-toolchain.toml" || grep -q "^channel.*=.*\"stable\"" "$PROJECT_ROOT/rust-toolchain.toml"`