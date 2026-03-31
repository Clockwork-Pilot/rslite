# Specification

## Overview

Verify c_variadic feature isolation: only in printf_c_variadic.rs

## Table of Contents

- [Overview](#overview)
- [Features](#features)
    - [Feature: build_all](#build_all)
      - [constraint_build_all](#constraint_build_all)
    - [Feature: fts3_write_c_variadic_migration](#fts3_write_c_variadic_migration)
      - [fts3_write_fts3SqlExec_no_variadic](#fts3_write_fts3sqlexec_no_variadic)
      - [fts3_write_fts3SqlStmt_no_variadic](#fts3_write_fts3sqlstmt_no_variadic)
      - [fts3_write_no_feature](#fts3_write_no_feature)
      - [fts3_write_no_sqlite3_mprintf](#fts3_write_no_sqlite3_mprintf)
      - [fts3_write_no_sqlite3_mprintf_use](#fts3_write_no_sqlite3_mprintf_use)
    - [Feature: fts3_write_enum_constants](#fts3_write_enum_constants)
      - [fts3_write_fts_stat_enum_defined](#fts3_write_fts_stat_enum_defined)
      - [fts3_write_no_fts_stat_incrmergehint_const](#fts3_write_no_fts_stat_incrmergehint_const)
      - [fts3_write_no_sql_delete_all_segdir_const](#fts3_write_no_sql_delete_all_segdir_const)
      - [fts3_write_sql_const_enum_defined](#fts3_write_sql_const_enum_defined)
    - [Feature: fts3_write_sqlite_printf_migration](#fts3_write_sqlite_printf_migration)
      - [fts3_write_no_format_sql_1arg](#fts3_write_no_format_sql_1arg)
      - [fts3_write_no_format_sql_2args](#fts3_write_no_format_sql_2args)
      - [fts3_write_no_format_sql_3args](#fts3_write_no_format_sql_3args)
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

### Feature: fts3_write_c_variadic_migration
**Migrate fts3_write.rs and specific functions away from c_variadic feature**

**Goals:**
- Remove c_variadic feature usage from fts3_write.rs file
- Refactor fts3SqlStmt function to eliminate c_variadic
- Refactor fts3SqlExec function to eliminate c_variadic
- Replace all sqlite3_mprintf calls with compile-time validated macros

#### fts3_write_fts3SqlExec_no_variadic
**Description:** fts3SqlExec function must not use c_variadic or variadic patterns
**Command:** `awk "/^[^/]*fn fts3SqlExec/,/^}/" "$WORKSPACE_ROOT/src/ext/fts3/fts3_write.rs" 2>/dev/null | grep -E "(va_list|VAList|VaListImpl|c_variadic|va_arg)" && exit 1 || exit 0`

#### fts3_write_fts3SqlStmt_no_variadic
**Description:** fts3SqlStmt function must not use c_variadic or variadic patterns
**Command:** `awk "/^[^/]*fn fts3SqlStmt/,/^}/" "$WORKSPACE_ROOT/src/ext/fts3/fts3_write.rs" 2>/dev/null | grep -E "(va_list|VAList|VaListImpl|c_variadic|va_arg)" && exit 1 || exit 0`

#### fts3_write_no_c_variadic_feature
**Description:** fts3_write.rs must not have c_variadic feature declaration
**Command:** `grep -n "#!\[feature(c_variadic)\]" "$WORKSPACE_ROOT/src/ext/fts3/fts3_write.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_write_no_sqlite3_mprintf
**Description:** fts3_write.rs must not use sqlite3_mprintf function calls
**Command:** `grep -E "sqlite3_mprintf\(" "$WORKSPACE_ROOT/src/ext/fts3/fts3_write.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_write_no_sqlite3_mprintf_use
**Description:** fts3_write.rs must not import sqlite3_mprintf
**Command:** `grep -E "use.*sqlite3_mprintf|pub use.*sqlite3_mprintf" "$WORKSPACE_ROOT/src/ext/fts3/fts3_write.rs" 2>/dev/null && exit 1 || exit 0`

### Feature: fts3_write_enum_constants
**Migrate FTS3 and SQL constants from individual const definitions to enum variants**

**Goals:**
- Replace individual const SQL_* definitions with enum SqlConstant variants
- Replace individual const FTS_STAT_* definitions with enum FtsStatConstant variants
- Define enums with proper c_int assigned values
- Maintain all constant values and usage compatibility
- Verify all 394649 tests still pass

#### fts3_write_fts_stat_enum_defined
**Description:** fts3_write.rs must define FtsStatConstant enum or similar with FTS_STAT values
**Command:** `grep -n "enum.*FtsStatConstant\|enum.*FTS_STAT" "$WORKSPACE_ROOT/src/ext/fts3/fts3_write.rs" 2>/dev/null || exit 1`

#### fts3_write_no_fts_stat_incrmergehint_const
**Description:** fts3_write.rs must not define FTS_STAT_INCRMERGEHINT as const (should be enum variant)
**Command:** `grep -n "pub const FTS_STAT_INCRMERGEHINT" "$WORKSPACE_ROOT/src/ext/fts3/fts3_write.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_write_no_sql_delete_all_segdir_const
**Description:** fts3_write.rs must not define SQL_DELETE_ALL_SEGDIR as const (should be enum variant)
**Command:** `grep -n "pub const SQL_DELETE_ALL_SEGDIR" "$WORKSPACE_ROOT/src/ext/fts3/fts3_write.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_write_sql_const_enum_defined
**Description:** fts3_write.rs must define SqlConstant enum or similar with SQL_* values
**Command:** `grep -n "enum.*SqlConstant\|enum.*SQL_" "$WORKSPACE_ROOT/src/ext/fts3/fts3_write.rs" 2>/dev/null || exit 1`

### Feature: fts3_write_sqlite_printf_migration
**Migrate fts3_write.rs from format_sql_* helpers to sqlite_printf! proc macro**

**Goals:**
- Replace all safe_format::format_sql_1arg calls with sqlite_printf! macro
- Replace all safe_format::format_sql_2args calls with sqlite_printf! macro
- Replace all safe_format::format_sql_3args calls with sqlite_printf! macro
- Verify no format_sql_* functions are used in fts3_write.rs
- All 394649 tests must pass after migration

#### fts3_write_no_format_sql_1arg
**Description:** fts3_write.rs must not use format_sql_1arg function
**Command:** `grep -n "format_sql_1arg" "$WORKSPACE_ROOT/src/ext/fts3/fts3_write.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_write_no_format_sql_2args
**Description:** fts3_write.rs must not use format_sql_2args function
**Command:** `grep -n "format_sql_2args" "$WORKSPACE_ROOT/src/ext/fts3/fts3_write.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_write_no_format_sql_3args
**Description:** fts3_write.rs must not use format_sql_3args function
**Command:** `grep -n "format_sql_3args" "$WORKSPACE_ROOT/src/ext/fts3/fts3_write.rs" 2>/dev/null && exit 1 || exit 0`

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