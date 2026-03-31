# Specification

## Overview

Verify c_variadic feature isolation: only in printf_c_variadic.rs

## Table of Contents

- [Overview](#overview)
- [Features](#features)
    - [Feature: build_all](#build_all)
      - [constraint_build_all](#constraint_build_all)
    - [Feature: memdb_c_variadic_migration](#memdb_c_variadic_migration)
      - [memdb_memdbFileControl_no_variadic](#memdb_memdbfilecontrol_no_variadic)
      - [memdb_no_feature](#memdb_no_feature)
      - [memdb_no_sqlite3_mprintf](#memdb_no_sqlite3_mprintf)
      - [memdb_no_sqlite3_mprintf_use](#memdb_no_sqlite3_mprintf_use)
      - [memdb_sqlite3_deserialize_no_variadic](#memdb_sqlite3_deserialize_no_variadic)
      - [memdb_sqlite3_serialize_no_variadic](#memdb_sqlite3_serialize_no_variadic)
    - [Feature: toolchain_version](#toolchain_version)
      - [c6](#c6)

## Features

### Feature: build_all
**Check that our rust codebase is healthy**

#### constraint_build_all
**Description:** Ensure our rust codebase is healthy
**Command:** `cd $WORKSPACE_ROOT && ./build_all.sh`

### Feature: memdb_c_variadic_migration
**Migrate memdb.rs and specific functions away from c_variadic feature**

**Goals:**
- Remove c_variadic feature usage from memdb.rs file
- Refactor memdbFileControl function to eliminate c_variadic
- Refactor sqlite3_serialize function to eliminate c_variadic
- Refactor sqlite3_deserialize function to eliminate c_variadic

#### memdb_memdbFileControl_no_variadic
**Description:** memdbFileControl function must not use c_variadic or variadic patterns
**Command:** `awk '/^[^/]*fn memdbFileControl/,/^}/' "$WORKSPACE_ROOT/src/src/memdb.rs" 2>/dev/null | grep -E '(va_list|VAList|VaListImpl|c_variadic|va_arg)' && exit 1 || exit 0`

#### memdb_no_c_variadic_feature
**Description:** memdb.rs must not have c_variadic feature declaration
**Command:** `grep -n '#!\[feature(c_variadic)\]' "$WORKSPACE_ROOT/src/src/memdb.rs" 2>/dev/null && exit 1 || exit 0`

#### memdb_no_sqlite3_mprintf
**Description:** memdb.rs must not use sqlite3_mprintf
**Command:** `grep -E "sqlite3_mprintf\(" "$WORKSPACE_ROOT/src/src/memdb.rs" 2>/dev/null && exit 1 || exit 0`

#### memdb_no_sqlite3_mprintf_use
**Description:** memdb.rs must not import sqlite3_mprintf
**Command:** `grep -E "use.*sqlite3_mprintf|pub use.*sqlite3_mprintf" "$WORKSPACE_ROOT/src/src/memdb.rs" 2>/dev/null && exit 1 || exit 0`

#### memdb_sqlite3_deserialize_no_variadic
**Description:** sqlite3_deserialize function must not use c_variadic or variadic patterns
**Command:** `awk '/^[^/]*fn sqlite3_deserialize/,/^}/' "$WORKSPACE_ROOT/src/src/memdb.rs" 2>/dev/null | grep -E '(va_list|VAList|VaListImpl|c_variadic|va_arg)' && exit 1 || exit 0`

#### memdb_sqlite3_serialize_no_variadic
**Description:** sqlite3_serialize function must not use c_variadic or variadic patterns
**Command:** `awk '/^[^/]*fn sqlite3_serialize/,/^}/' "$WORKSPACE_ROOT/src/src/memdb.rs" 2>/dev/null | grep -E '(va_list|VAList|VaListImpl|c_variadic|va_arg)' && exit 1 || exit 0`

### Feature: toolchain_version
**Enforce Rust toolchain version nightly-2026-03-26**

**Goals:**
- rust-toolchain.toml must specify channel nightly-2026-03-26

#### toolchain_nightly_version
**Description:** rust-toolchain.toml must use channel nightly-2026-03-26
**Command:** `grep -q "nightly-2026-03-26" "$PROJECT_ROOT/rust-toolchain.toml" && exit 0 || exit 1`