# Specification

## Overview

Verify c_variadic feature isolation: only in printf_c_variadic.rs

## Table of Contents

- [Overview](#overview)
- [Features](#features)
    - [Feature: build_all](#build_all)
      - [constraint_build_all](#constraint_build_all)
    - [Feature: fts5_no_libc_global](#fts5_no_libc_global)
      - [fts5_no_libc_calls](#fts5_no_libc_calls)
    - [Feature: lib_exports](#lib_exports)
      - [cdylib_symbols](#cdylib_symbols)
      - [no_rust_variadic_exports](#no_rust_variadic_exports)
      - [rustfixture_symbols](#rustfixture_symbols)
      - [shell_symbols](#shell_symbols)
    - [Feature: toolchain_version](#toolchain_version)
      - [shell_nightly](#shell_nightly)
      - [stable_toolchain](#stable_toolchain)
      - [tclsqlite_nightly](#tclsqlite_nightly)

## Features

### Feature: build_all
**Check that our rust codebase is healthy**

#### constraint_build_all
**Description:** Ensure our rust codebase is healthy
**Command:** `cd $WORKSPACE_ROOT && ./build_all.sh`

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
**Command:** `cd $PROJECT_ROOT && cargo build --release --lib 2>/dev/null && SYMS=$(nm -D target/release/libsqlite_noamalgam.so) && for sym in sqlite3_str_appendf sqlite3_snprintf sqlite3_mprintf sqlite3_vsnprintf sqlite3_vmprintf sqlite3_test_control sqlite3_db_config sqlite3_config sqlite3_vtab_config sqlite3_log; do echo "$SYMS" | grep -q "T ${sym}$" || { echo "missing $sym in cdylib"; exit 1; }; done && grep -q "export_name.*sqlite3_test_control_args" $PROJECT_ROOT/src/printf_c_variadic.rs`

#### no_rust_variadic_exports
**Description:** Negative: none of the 10 exported symbols must be Rust variadic extern fns — they live in C only
**Command:** `! grep -qE "pub unsafe extern.*fn (sqlite3_str_appendf|sqlite3_snprintf|sqlite3_mprintf|sqlite3_vsnprintf|sqlite3_vmprintf|sqlite3_test_control|sqlite3_db_config|sqlite3_config|sqlite3_vtab_config|sqlite3_log)" $PROJECT_ROOT/src/src/printf.rs $PROJECT_ROOT/src/printf_c_variadic.rs 2>/dev/null`

#### rustfixture_symbols
**Description:** Behavioral: all 10 symbols must be global (T) in the rustfixture binary
**Command:** `SYMS=$(nm $PROJECT_ROOT/crust-tclsqlite/target/release/rustfixture) && for sym in sqlite3_str_appendf sqlite3_snprintf sqlite3_mprintf sqlite3_vsnprintf sqlite3_vmprintf sqlite3_test_control sqlite3_db_config sqlite3_config sqlite3_vtab_config sqlite3_log; do echo "$SYMS" | grep -q "T ${sym}$" || { echo "missing $sym in rustfixture"; exit 1; }; done && grep -q "export_name.*sqlite3_db_config_args" $PROJECT_ROOT/src/printf_c_variadic.rs`

#### shell_symbols
**Description:** Behavioral: all 10 symbols must be global (T) in the shell binary
**Command:** `SYMS=$(nm $PROJECT_ROOT/crust-sqlite-shell/target/release/sqlite3) && for sym in sqlite3_str_appendf sqlite3_snprintf sqlite3_mprintf sqlite3_vsnprintf sqlite3_vmprintf sqlite3_test_control sqlite3_db_config sqlite3_config sqlite3_vtab_config sqlite3_log; do echo "$SYMS" | grep -q "T ${sym}$" || { echo "missing $sym in shell"; exit 1; }; done && grep -q "export_name.*sqlite3_config_args" $PROJECT_ROOT/src/printf_c_variadic.rs`

### Feature: toolchain_version
**Enforce Rust toolchain version nightly-2026-03-26**

**Goals:**
- rust-toolchain.toml must specify channel nightly-2026-03-26

#### shell_nightly
**Description:** crust-sqlite-shell must use nightly toolchain (needs c_variadic, extern_types)
**Command:** `grep -q "^channel.*=.*\"nightly" "$PROJECT_ROOT/crust-sqlite-shell/rust-toolchain.toml" && exit 0 || exit 1`

#### stable_toolchain
**Description:** rust-toolchain.toml must use stable channel (c_variadic removed from lib crate)
**Command:** `grep -q "^channel.*=.*\"stable\"" "$PROJECT_ROOT/rust-toolchain.toml" && exit 0 || exit 1`

#### tclsqlite_nightly
**Description:** crust-tclsqlite must use nightly toolchain (needs c_variadic, extern_types)
**Command:** `grep -q "^channel.*=.*\"nightly" "$PROJECT_ROOT/crust-tclsqlite/rust-toolchain.toml" && exit 0 || exit 1`