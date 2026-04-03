# Specification

## Overview

Verify c_variadic feature isolation: only in printf_c_variadic.rs

## Table of Contents

- [Overview](#overview)
- [Features](#features)
    - [Feature: auth_carray_dbpage_dbstat_func_main_c_variadic_migration](#auth_carray_dbpage_dbstat_func_main_c_variadic_migration)
      - [auth_no_sqlite3_mprintf](#auth_no_sqlite3_mprintf)
      - [carray_no_sqlite3_mprintf](#carray_no_sqlite3_mprintf)
      - [dbpage_no_sqlite3_mprintf](#dbpage_no_sqlite3_mprintf)
      - [dbstat_no_sqlite3_mprintf](#dbstat_no_sqlite3_mprintf)
      - [func_no_sqlite3_mprintf](#func_no_sqlite3_mprintf)
      - [main_no_sqlite3_mprintf](#main_no_sqlite3_mprintf)
      - [main_no_sqlite3_snprintf](#main_no_sqlite3_snprintf)
      - [no_sqlite3_mprintf_definition](#no_sqlite3_mprintf_definition)
      - [no_sqlite3_snprintf_definition](#no_sqlite3_snprintf_definition)
    - [Feature: build_all](#build_all)
      - [constraint_build_all](#constraint_build_all)
    - [Feature: fts3_aux_c_variadic_migration](#fts3_aux_c_variadic_migration)
      - [fts3_aux_no_sqlite3_mprintf](#fts3_aux_no_sqlite3_mprintf)
      - [fts3_aux_no_sqlite3_mprintf_use](#fts3_aux_no_sqlite3_mprintf_use)
      - [fts3_aux_uses_sqlite_printf_macro](#fts3_aux_uses_sqlite_printf_macro)
    - [Feature: fts3_c_variadic_migration](#fts3_c_variadic_migration)
      - [fts3_no_printf_c_variadic](#fts3_no_printf_c_variadic)
      - [fts3_no_sqlite3_mprintf](#fts3_no_sqlite3_mprintf)
      - [fts3_no_sqlite3_mprintf_use](#fts3_no_sqlite3_mprintf_use)
      - [fts3_uses_sqlite_printf_macro](#fts3_uses_sqlite_printf_macro)
    - [Feature: fts3_expr_c_variadic_migration](#fts3_expr_c_variadic_migration)
      - [fts3_expr_no_sqlite3_mprintf](#fts3_expr_no_sqlite3_mprintf)
      - [fts3_expr_no_sqlite3_mprintf_use](#fts3_expr_no_sqlite3_mprintf_use)
      - [fts3_expr_uses_sqlite_printf_macro](#fts3_expr_uses_sqlite_printf_macro)
    - [Feature: fts3_tokenizer_c_variadic_migration](#fts3_tokenizer_c_variadic_migration)
      - [fts3_tokenizer_no_sqlite3_mprintf](#fts3_tokenizer_no_sqlite3_mprintf)
      - [fts3_tokenizer_no_sqlite3_mprintf_use](#fts3_tokenizer_no_sqlite3_mprintf_use)
      - [fts3_tokenizer_uses_sqlite_printf_macro](#fts3_tokenizer_uses_sqlite_printf_macro)
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
    - [Feature: fts5_c_variadic_migration](#fts5_c_variadic_migration)
      - [fts5_no_sqlite3_mprintf](#fts5_no_sqlite3_mprintf)
      - [fts5_no_sqlite3_snprintf](#fts5_no_sqlite3_snprintf)
      - [fts5_uses_sqlite_printf_macro](#fts5_uses_sqlite_printf_macro)
    - [Feature: func_percentError_variadic_removal](#func_percenterror_variadic_removal)
      - [percentError_callsites_use_sqlite_printf](#percenterror_callsites_use_sqlite_printf)
      - [percentError_defined_nonvariadic_in_func](#percenterror_defined_nonvariadic_in_func)
      - [percentError_no_reexport](#percenterror_no_reexport)
      - [percentError_not_in_printf_c_variadic](#percenterror_not_in_printf_c_variadic)
    - [Feature: json_c_variadic_migration](#json_c_variadic_migration)
      - [json_no_forbidden_functions](#json_no_forbidden_functions)
      - [json_no_jsonPrintf_call](#json_no_jsonprintf_call)
      - [json_no_jsonPrintf_reexport](#json_no_jsonprintf_reexport)
      - [json_no_sqlite3_mprintf](#json_no_sqlite3_mprintf)
    - [Feature: memdb_c_variadic_migration](#memdb_c_variadic_migration)
      - [memdb_memdbFileControl_no_variadic](#memdb_memdbfilecontrol_no_variadic)
      - [memdb_no_feature](#memdb_no_feature)
      - [memdb_no_sqlite3_mprintf](#memdb_no_sqlite3_mprintf)
      - [memdb_no_sqlite3_mprintf_use](#memdb_no_sqlite3_mprintf_use)
      - [memdb_sqlite3_deserialize_no_variadic](#memdb_sqlite3_deserialize_no_variadic)
      - [memdb_sqlite3_serialize_no_variadic](#memdb_sqlite3_serialize_no_variadic)
    - [Feature: rtree_c_variadic_migration](#rtree_c_variadic_migration)
      - [rtree_no_sqlite3_mprintf](#rtree_no_sqlite3_mprintf)
      - [rtree_uses_sqlite_printf_macro](#rtree_uses_sqlite_printf_macro)
    - [Feature: sqlite3session_and_snprintf_migration](#sqlite3session_and_snprintf_migration)
      - [sqlite3session_no_printf_c_variadic](#sqlite3session_no_printf_c_variadic)
      - [sqlite3session_no_sqlite3_mprintf](#sqlite3session_no_sqlite3_mprintf)
      - [sqlite3session_no_sqlite3_snprintf](#sqlite3session_no_sqlite3_snprintf)
      - [sqlite_snprintf_exists](#sqlite_snprintf_exists)
      - [sqlite_snprintf_is_proc_macro](#sqlite_snprintf_is_proc_macro)
    - [Feature: sqlite3session_c_variadic_migration](#sqlite3session_c_variadic_migration)
      - [sqlite3session_no_printf_c_variadic](#sqlite3session_no_printf_c_variadic)
      - [sqlite3session_no_sqlite3_mprintf](#sqlite3session_no_sqlite3_mprintf)
    - [Feature: sqlite_snprintf_proc_macro](#sqlite_snprintf_proc_macro)
      - [sqlite3session_no_sqlite3_snprintf](#sqlite3session_no_sqlite3_snprintf)
      - [sqlite_snprintf_exists](#sqlite_snprintf_exists)
      - [sqlite_snprintf_is_proc_macro](#sqlite_snprintf_is_proc_macro)
    - [Feature: sqlite_vmprintf_vsnprintf_proc_macro](#sqlite_vmprintf_vsnprintf_proc_macro)
      - [macro_sqlite_vmprintf_exists](#macro_sqlite_vmprintf_exists)
      - [macro_sqlite_vsnprintf_exists](#macro_sqlite_vsnprintf_exists)
      - [macro_vmprintf_handles_variadic](#macro_vmprintf_handles_variadic)
      - [macro_vsnprintf_handles_variadic](#macro_vsnprintf_handles_variadic)
    - [Feature: toolchain_version](#toolchain_version)
      - [c6](#c6)
    - [Feature: vdbevtab_vdbeaux_pragma_table_os_unix_loadext_c_variadic_migration](#vdbevtab_vdbeaux_pragma_table_os_unix_loadext_c_variadic_migration)
      - [loadext_no_printf_c_variadic](#loadext_no_printf_c_variadic)
      - [loadext_no_sqlite3_mprintf](#loadext_no_sqlite3_mprintf)
      - [os_unix_no_printf_c_variadic](#os_unix_no_printf_c_variadic)
      - [os_unix_no_sqlite3_mprintf](#os_unix_no_sqlite3_mprintf)
      - [pragma_no_printf_c_variadic](#pragma_no_printf_c_variadic)
      - [pragma_no_sqlite3_mprintf](#pragma_no_sqlite3_mprintf)
      - [table_no_printf_c_variadic](#table_no_printf_c_variadic)
      - [table_no_sqlite3_mprintf](#table_no_sqlite3_mprintf)
      - [vdbeaux_no_printf_c_variadic](#vdbeaux_no_printf_c_variadic)
      - [vdbeaux_no_sqlite3_mprintf](#vdbeaux_no_sqlite3_mprintf)
      - [vdbevtab_no_printf_c_variadic](#vdbevtab_no_printf_c_variadic)
      - [vdbevtab_no_sqlite3_mprintf](#vdbevtab_no_sqlite3_mprintf)

## Features

### Feature: auth_carray_dbpage_dbstat_func_main_c_variadic_migration
**Migrate auth, carray, dbpage, dbstat, func, main away from sqlite3_mprintf/sqlite3_snprintf**

**Goals:**
- Replace all sqlite3_mprintf and sqlite3_snprintf calls in auth.rs, carray.rs, dbpage.rs, dbstat.rs, func.rs, main.rs with sqlite_printf! and sqlite_snprintf! macros
- Remove sqlite3_mprintf and sqlite3_snprintf function definitions from printf_c_variadic.rs

#### auth_no_sqlite3_mprintf
**Description:** auth.rs must not use sqlite3_mprintf
**Command:** `grep -E "\bsqlite3_mprintf\(" "$WORKSPACE_ROOT/src/src/auth.rs" 2>/dev/null && exit 1 || exit 0`

#### carray_no_sqlite3_mprintf
**Description:** carray.rs must not use sqlite3_mprintf
**Command:** `grep -E "\bsqlite3_mprintf\(" "$WORKSPACE_ROOT/src/src/carray.rs" 2>/dev/null && exit 1 || exit 0`

#### dbpage_no_sqlite3_mprintf
**Description:** dbpage.rs must not use sqlite3_mprintf
**Command:** `grep -E "\bsqlite3_mprintf\(" "$WORKSPACE_ROOT/src/src/dbpage.rs" 2>/dev/null && exit 1 || exit 0`

#### dbstat_no_sqlite3_mprintf
**Description:** dbstat.rs must not use sqlite3_mprintf
**Command:** `grep -E "\bsqlite3_mprintf\(" "$WORKSPACE_ROOT/src/src/dbstat.rs" 2>/dev/null && exit 1 || exit 0`

#### func_no_sqlite3_mprintf
**Description:** func.rs must not use sqlite3_mprintf
**Command:** `grep -E "\bsqlite3_mprintf\(" "$WORKSPACE_ROOT/src/src/func.rs" 2>/dev/null && exit 1 || exit 0`

#### main_no_sqlite3_mprintf
**Description:** main.rs must not use sqlite3_mprintf
**Command:** `grep -E "\bsqlite3_mprintf\(" "$WORKSPACE_ROOT/src/src/main.rs" 2>/dev/null && exit 1 || exit 0`

#### main_no_sqlite3_snprintf
**Description:** main.rs must not use sqlite3_snprintf
**Command:** `grep -E "\bsqlite3_snprintf\(" "$WORKSPACE_ROOT/src/src/main.rs" 2>/dev/null && exit 1 || exit 0`

#### no_sqlite3_mprintf_definition
**Description:** sqlite3_mprintf function must be removed from printf_c_variadic.rs
**Command:** `grep -E "^pub unsafe extern.*fn sqlite3_mprintf" "$WORKSPACE_ROOT/src/printf_c_variadic.rs" 2>/dev/null && exit 1 || exit 0`

#### no_sqlite3_snprintf_definition
**Description:** sqlite3_snprintf function must be removed from printf_c_variadic.rs
**Command:** `grep -E "^pub unsafe extern.*fn sqlite3_snprintf" "$WORKSPACE_ROOT/src/printf_c_variadic.rs" 2>/dev/null && exit 1 || exit 0`

### Feature: build_all
**Check that our rust codebase is healthy**

#### constraint_build_all
**Description:** Ensure our rust codebase is healthy
**Command:** `cd $WORKSPACE_ROOT && ./build_all.sh`

### Feature: fts3_aux_c_variadic_migration
**Migrate src/ext/fts3/fts3_aux.rs away from sqlite3_mprintf to sqlite_printf! proc macro**

**Goals:**
- Remove all sqlite3_mprintf calls from src/ext/fts3/fts3_aux.rs
- Remove sqlite3_mprintf import from src/ext/fts3/fts3_aux.rs
- Replace with compile-time validated sqlite_printf! macro calls

#### fts3_aux_no_sqlite3_mprintf
**Description:** fts3_aux.rs must not use sqlite3_mprintf function calls
**Command:** `grep -E "\bsqlite3_mprintf\(" "$WORKSPACE_ROOT/src/ext/fts3/fts3_aux.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_aux_no_sqlite3_mprintf_use
**Description:** fts3_aux.rs must not import sqlite3_mprintf
**Command:** `grep -E "use.*sqlite3_mprintf|pub use.*sqlite3_mprintf" "$WORKSPACE_ROOT/src/ext/fts3/fts3_aux.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_aux_uses_sqlite_printf_macro
**Description:** fts3_aux.rs must use sqlite_printf! macro for string formatting
**Command:** `grep -c "sqlite_printf!" "$WORKSPACE_ROOT/src/ext/fts3/fts3_aux.rs" 2>/dev/null | grep -qE "^[1-9]" && exit 0 || exit 1`

### Feature: fts3_c_variadic_migration
**Migrate src/ext/fts3/fts3.rs away from sqlite3_mprintf and printf_c_variadic variadic helpers**

**Goals:**
- Remove all sqlite3_mprintf calls from src/ext/fts3/fts3.rs
- Remove sqlite3_mprintf import from src/ext/fts3/fts3.rs
- Remove re-exports of fts3DbExec, fts3Appendf, sqlite3Fts3ErrMsg from printf_c_variadic
- Replace with compile-time validated sqlite_printf! macro calls

#### fts3_no_printf_c_variadic
**Description:** fts3.rs must not import from printf_c_variadic (fts3DbExec, fts3Appendf, sqlite3Fts3ErrMsg must be moved out)
**Command:** `grep -n "printf_c_variadic" "$WORKSPACE_ROOT/src/ext/fts3/fts3.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_no_sqlite3_mprintf
**Description:** fts3.rs must not use sqlite3_mprintf function calls
**Command:** `grep -E "\bsqlite3_mprintf\(" "$WORKSPACE_ROOT/src/ext/fts3/fts3.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_no_sqlite3_mprintf_use
**Description:** fts3.rs must not import sqlite3_mprintf
**Command:** `grep -E "use.*sqlite3_mprintf|pub use.*sqlite3_mprintf" "$WORKSPACE_ROOT/src/ext/fts3/fts3.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_uses_sqlite_printf_macro
**Description:** fts3.rs must use sqlite_printf! macro for string formatting
**Command:** `grep -c "sqlite_printf!" "$WORKSPACE_ROOT/src/ext/fts3/fts3.rs" 2>/dev/null | grep -qE "^[1-9]" && exit 0 || exit 1`

### Feature: fts3_expr_c_variadic_migration
**Migrate src/ext/fts3/fts3_expr.rs away from sqlite3_mprintf to sqlite_printf! proc macro**

**Goals:**
- Remove all sqlite3_mprintf calls from src/ext/fts3/fts3_expr.rs
- Remove sqlite3_mprintf import from src/ext/fts3/fts3_expr.rs
- Replace with compile-time validated sqlite_printf! macro calls

#### fts3_expr_no_sqlite3_mprintf
**Description:** fts3_expr.rs must not use sqlite3_mprintf function calls
**Command:** `grep -E "\bsqlite3_mprintf\(" "$WORKSPACE_ROOT/src/ext/fts3/fts3_expr.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_expr_no_sqlite3_mprintf_use
**Description:** fts3_expr.rs must not import sqlite3_mprintf
**Command:** `grep -E "use.*sqlite3_mprintf|pub use.*sqlite3_mprintf" "$WORKSPACE_ROOT/src/ext/fts3/fts3_expr.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_expr_uses_sqlite_printf_macro
**Description:** fts3_expr.rs must use sqlite_printf! macro for string formatting
**Command:** `grep -c "sqlite_printf!" "$WORKSPACE_ROOT/src/ext/fts3/fts3_expr.rs" 2>/dev/null | grep -qE "^[1-9]" && exit 0 || exit 1`

### Feature: fts3_tokenizer_c_variadic_migration
**Migrate src/ext/fts3/fts3_tokenizer.rs away from sqlite3_mprintf to sqlite_printf! proc macro**

**Goals:**
- Remove all sqlite3_mprintf calls from src/ext/fts3/fts3_tokenizer.rs
- Remove sqlite3_mprintf import from src/ext/fts3/fts3_tokenizer.rs
- Replace with compile-time validated sqlite_printf! macro calls

#### fts3_tokenizer_no_sqlite3_mprintf
**Description:** fts3_tokenizer.rs must not use sqlite3_mprintf function calls
**Command:** `grep -E "\bsqlite3_mprintf\(" "$WORKSPACE_ROOT/src/ext/fts3/fts3_tokenizer.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_tokenizer_no_sqlite3_mprintf_use
**Description:** fts3_tokenizer.rs must not import sqlite3_mprintf
**Command:** `grep -E "use.*sqlite3_mprintf|pub use.*sqlite3_mprintf" "$WORKSPACE_ROOT/src/ext/fts3/fts3_tokenizer.rs" 2>/dev/null && exit 1 || exit 0`

#### fts3_tokenizer_uses_sqlite_printf_macro
**Description:** fts3_tokenizer.rs must use sqlite_printf! macro for string formatting
**Command:** `grep -c "sqlite_printf!" "$WORKSPACE_ROOT/src/ext/fts3/fts3_tokenizer.rs" 2>/dev/null | grep -qE "^[1-9]" && exit 0 || exit 1`

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

### Feature: fts5_c_variadic_migration
**Migrate src/fts5.rs away from sqlite3_mprintf/sqlite3_snprintf to sqlite_printf! proc macro**

**Goals:**
- Remove all sqlite3_mprintf and sqlite3_snprintf calls from src/fts5.rs
- Replace with compile-time validated sqlite_printf! macro calls
- Ensure all tests pass after migration

#### fts5_no_sqlite3_mprintf
**Description:** fts5.rs must not use sqlite3_mprintf function calls
**Command:** `grep -E "\bsqlite3_mprintf\(" "$WORKSPACE_ROOT/src/fts5.rs" 2>/dev/null && exit 1 || exit 0`

#### fts5_no_sqlite3_snprintf
**Description:** fts5.rs must not use sqlite3_snprintf function calls
**Command:** `grep -E "\bsqlite3_snprintf\(" "$WORKSPACE_ROOT/src/fts5.rs" 2>/dev/null && exit 1 || exit 0`

#### fts5_uses_sqlite_printf_macro
**Description:** fts5.rs must use sqlite_printf! macro for string formatting
**Command:** `grep -c "sqlite_printf!" "$WORKSPACE_ROOT/src/fts5.rs" 2>/dev/null | grep -qE "^[1-9]" && exit 0 || exit 1`

### Feature: func_percentError_variadic_removal
**Remove variadic percentError from printf_c_variadic; redefine non-variadic in func.rs using sqlite_printf!**

**Goals:**
- percentError in src/printf_c_variadic.rs is a variadic C-ABI function that calls sqlite3_vmprintf then does a manual %s substitution for the function name.
- All 4 call sites are in func.rs. Delete percentError from printf_c_variadic.rs and remove its pub use re-export from func.rs.
- Define a private non-variadic helper unsafe fn percentError(pCtx, zMsg: *mut c_char) directly in func.rs.
- Each call site builds zMsg via sqlite_printf! combining the function name (from sqlite3VdbeFuncName) and any extra args inline.

#### percentError_callsites_use_sqlite_printf
**Description:** Behavioral: func.rs must have >= 5 sqlite_printf! uses (1 existing + 4 new call sites replacing variadic percentError calls)
**Command:** `[ $(grep -c "sqlite_printf!" "${CLAUDE_PROJECT_ROOT}/src/src/func.rs") -ge 5 ]`

#### percentError_defined_nonvariadic_in_func
**Description:** Structural: percentError must be defined directly in func.rs with a non-variadic signature
**Command:** `grep -q "fn percentError" "${CLAUDE_PROJECT_ROOT}/src/src/func.rs" && ! grep -qE "fn percentError.*\.\.\." "${CLAUDE_PROJECT_ROOT}/src/src/func.rs"`

#### percentError_no_reexport
**Description:** Structural: no pub use re-export of percentError from printf_c_variadic in func.rs
**Command:** `! grep -qE "pub use.*percentError" "${CLAUDE_PROJECT_ROOT}/src/src/func.rs"`

#### percentError_not_in_printf_c_variadic
**Description:** Structural: percentError function must be removed from printf_c_variadic.rs
**Command:** `! grep -q "fn percentError" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

### Feature: json_c_variadic_migration
**Migrate src/src/json.rs away from c_variadic functions (jsonPrintf) and sqlite3_mprintf**

**Goals:**
- Remove jsonPrintf c_variadic usage from json.rs
- Replace all jsonPrintf call sites with safe Rust alternatives
- Replace all sqlite3_mprintf calls with compile-time validated sqlite_printf! macro
- Remove re-export of jsonPrintf from printf_c_variadic module

#### json_no_forbidden_functions
**Description:** printf_c_variadic.rs and json.rs must not contain jsonPrintf or any json_append_* helper functions
**Command:** `grep -En "\bjsonPrintf\b|\bjson_append_u64_or_overflow\b|\bjson_append_array_index\b|\bjson_append_double\b|\bjson_append_key_quoted\b|\bjson_append_key_unquoted\b" "$WORKSPACE_ROOT/src/printf_c_variadic.rs" "$WORKSPACE_ROOT/src/src/json.rs" 2>/dev/null && exit 1 || exit 0`

#### json_no_jsonPrintf_call
**Description:** json.rs must not call jsonPrintf
**Command:** `grep -n "jsonPrintf(" "$WORKSPACE_ROOT/src/src/json.rs" 2>/dev/null && exit 1 || exit 0`

#### json_no_jsonPrintf_reexport
**Description:** json.rs must not re-export jsonPrintf from printf_c_variadic
**Command:** `grep -n "printf_c_variadic::jsonPrintf" "$WORKSPACE_ROOT/src/src/json.rs" 2>/dev/null && exit 1 || exit 0`

#### json_no_sqlite3_mprintf
**Description:** json.rs must not use sqlite3_mprintf function calls
**Command:** `grep -n "sqlite3_mprintf(" "$WORKSPACE_ROOT/src/src/json.rs" 2>/dev/null && exit 1 || exit 0`

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

### Feature: rtree_c_variadic_migration
**Migrate src/ext/rtree/rtree.rs away from sqlite3_mprintf to sqlite_printf! proc macro**

**Goals:**
- Remove all sqlite3_mprintf calls from src/ext/rtree/rtree.rs
- Replace with compile-time validated sqlite_printf! macro calls
- Ensure all 394649 tests pass after migration

#### rtree_no_sqlite3_mprintf
**Description:** rtree.rs must not use sqlite3_mprintf function calls
**Command:** `grep -E "\bsqlite3_mprintf\(" "$WORKSPACE_ROOT/src/ext/rtree/rtree.rs" 2>/dev/null && exit 1 || exit 0`

#### rtree_uses_sqlite_printf_macro
**Description:** rtree.rs must use sqlite_printf! macro for string formatting
**Command:** `grep -c "sqlite_printf!" "$WORKSPACE_ROOT/src/ext/rtree/rtree.rs" 2>/dev/null | grep -qE "^[1-9]" && exit 0 || exit 1`

### Feature: sqlite3session_and_snprintf_migration
**Combined migration of src/ext/session/sqlite3session.rs away from c_variadic functions and implementation of sqlite_snprintf! proc macro**

**Goals:**
- Remove sqlite3_mprintf usage from src/ext/session/sqlite3session.rs
- Remove sessionAppendPrintf usage and re-export from printf_c_variadic module
- Implement sqlite_snprintf! proc macro in sqlite-printf-macros
- Support buffer-safe formatting with compile-time validation
- Replace all format string calls with sqlite_printf! and sqlite_snprintf! macros

#### sqlite3session_no_printf_c_variadic
**Description:** sqlite3session.rs must not import from printf_c_variadic or use sessionAppendPrintf
**Command:** `grep -n "printf_c_variadic\|sessionAppendPrintf" "$WORKSPACE_ROOT/src/ext/session/sqlite3session.rs" 2>/dev/null && exit 1 || exit 0`

#### sqlite3session_no_sqlite3_mprintf
**Description:** sqlite3session.rs must not use sqlite3_mprintf
**Command:** `grep -n "sqlite3_mprintf" "$WORKSPACE_ROOT/src/ext/session/sqlite3session.rs" 2>/dev/null && exit 1 || exit 0`

#### sqlite3session_no_sqlite3_snprintf
**Description:** sqlite3session.rs must not use sqlite3_snprintf
**Command:** `grep -n "sqlite3_snprintf" "$WORKSPACE_ROOT/src/ext/session/sqlite3session.rs" 2>/dev/null && exit 1 || exit 0`

#### sqlite_snprintf_exists
**Description:** sqlite_snprintf! macro must exist in sqlite-printf-macros/src/lib.rs
**Command:** `grep -n "pub fn sqlite_snprintf" "$WORKSPACE_ROOT/sqlite-printf-macros/src/lib.rs" 2>/dev/null && exit 0 || exit 1`

#### sqlite_snprintf_is_proc_macro
**Description:** sqlite_snprintf must be a proc_macro (have #[proc_macro] attribute)
**Command:** `grep -B 1 "pub fn sqlite_snprintf" "$WORKSPACE_ROOT/sqlite-printf-macros/src/lib.rs" 2>/dev/null | grep -q "#\[proc_macro\]" && exit 0 || exit 1`

### Feature: sqlite3session_c_variadic_migration
**Migrate src/ext/session/sqlite3session.rs away from c_variadic functions and sqlite3_mprintf**

**Goals:**
- Remove sqlite3_mprintf usage from src/ext/session/sqlite3session.rs
- Remove sessionAppendPrintf usage and re-export from printf_c_variadic module
- Replace all format string calls with manual string allocation using sqlite3_malloc and libc string functions

#### sqlite3session_no_printf_c_variadic
**Description:** sqlite3session.rs must not import from printf_c_variadic or use sessionAppendPrintf
**Command:** `grep -n "printf_c_variadic\|sessionAppendPrintf" "$WORKSPACE_ROOT/src/ext/session/sqlite3session.rs" 2>/dev/null && exit 1 || exit 0`

#### sqlite3session_no_sqlite3_mprintf
**Description:** sqlite3session.rs must not use sqlite3_mprintf
**Command:** `grep -n "sqlite3_mprintf" "$WORKSPACE_ROOT/src/ext/session/sqlite3session.rs" 2>/dev/null && exit 1 || exit 0`

### Feature: sqlite_snprintf_proc_macro
**Add sqlite_snprintf proc macro to sqlite-printf-macros**

**Goals:**
- Implement sqlite_snprintf! proc macro in sqlite-printf-macros
- Support buffer-safe formatting with compile-time validation
- Remove all sqlite3_snprintf calls from sqlite3session.rs

#### sqlite3session_no_sqlite3_snprintf
**Description:** sqlite3session.rs must not use sqlite3_snprintf
**Command:** `grep -n "sqlite3_snprintf" "$WORKSPACE_ROOT/src/ext/session/sqlite3session.rs" 2>/dev/null && exit 1 || exit 0`

#### sqlite_snprintf_exists
**Description:** sqlite_snprintf! macro must exist in sqlite-printf-macros/src/lib.rs
**Command:** `grep -n "pub fn sqlite_snprintf" "$WORKSPACE_ROOT/sqlite-printf-macros/src/lib.rs" 2>/dev/null && exit 0 || exit 1`

#### sqlite_snprintf_is_proc_macro
**Description:** sqlite_snprintf must be a proc_macro (have #[proc_macro] attribute)
**Command:** `grep -B 1 "pub fn sqlite_snprintf" "$WORKSPACE_ROOT/sqlite-printf-macros/src/lib.rs" 2>/dev/null | grep -q "#\[proc_macro\]" && exit 0 || exit 1`

### Feature: sqlite_vmprintf_vsnprintf_proc_macro
**Add sqlite_vmprintf and sqlite_vsnprintf proc macros to sqlite-printf-macros**

**Goals:**
- Implement sqlite_vmprintf! and sqlite_vsnprintf! proc macros in sqlite-printf-macros
- Support variadic argument formatting with va_list compatibility
- Support compile-time format string validation
- Enforce function-only usage in printf.rs and printf_c_variadic.rs
- Migrate all sqlite3_vmprintf/sqlite3_vsnprintf calls from other files to use macros

#### macro_sqlite_vmprintf_exists
**Description:** Structural: sqlite_vmprintf! proc macro must exist and be exported in sqlite-printf-macros/src/lib.rs
**Command:** `grep -q "#\\[proc_macro\\]" "$PROJECT_ROOT/sqlite-printf-macros/src/lib.rs" && grep -q "pub fn sqlite_vmprintf" "$PROJECT_ROOT/sqlite-printf-macros/src/lib.rs" || exit 1`

#### macro_sqlite_vsnprintf_exists
**Description:** Structural: sqlite_vsnprintf! proc macro must exist and be exported in sqlite-printf-macros/src/lib.rs
**Command:** `grep -q "#\\[proc_macro\\]" "$PROJECT_ROOT/sqlite-printf-macros/src/lib.rs" && grep -q "pub fn sqlite_vsnprintf" "$PROJECT_ROOT/sqlite-printf-macros/src/lib.rs" || exit 1`

#### macro_vmprintf_handles_variadic
**Description:** Behavioral: sqlite_vmprintf! macro must handle variadic arguments (TokenStream, va_list reference, or Vec of expressions)
**Command:** `grep -A 30 "pub fn sqlite_vmprintf" "$PROJECT_ROOT/sqlite-printf-macros/src/lib.rs" | grep -qE "va_list|variadic|TokenStream" || exit 1`

#### macro_vsnprintf_handles_variadic
**Description:** Behavioral: sqlite_vsnprintf! macro must handle variadic arguments (TokenStream, va_list reference, or Vec of expressions)
**Command:** `grep -A 30 "pub fn sqlite_vsnprintf" "$PROJECT_ROOT/sqlite-printf-macros/src/lib.rs" | grep -qE "va_list|variadic|TokenStream" || exit 1`

### Feature: toolchain_version
**Enforce Rust toolchain version nightly-2026-03-26**

**Goals:**
- rust-toolchain.toml must specify channel nightly-2026-03-26

#### toolchain_nightly_version
**Description:** rust-toolchain.toml must use channel nightly-2026-03-26
**Command:** `grep -q "nightly-2026-03-26" "$PROJECT_ROOT/rust-toolchain.toml" && exit 0 || exit 1`

### Feature: vdbevtab_vdbeaux_pragma_table_os_unix_loadext_c_variadic_migration
**Migrate vdbevtab.rs, vdbeaux.rs, table.rs, pragma.rs, os_unix.rs, loadext.rs away from c_variadic functions and sqlite3_mprintf**

**Goals:**
- Remove c_variadic and sqlite3_mprintf usage from src/src/vdbevtab.rs
- Remove c_variadic and sqlite3_mprintf usage from src/src/vdbeaux.rs
- Remove c_variadic and sqlite3_mprintf usage from src/src/table.rs
- Remove c_variadic and sqlite3_mprintf usage from src/src/pragma.rs
- Remove c_variadic and sqlite3_mprintf usage from src/src/os_unix.rs
- Remove c_variadic and sqlite3_mprintf usage from src/src/loadext.rs
- Replace all format strings with compile-time validated sqlite_printf! or json_printf! macros

#### loadext_no_printf_c_variadic
**Description:** loadext.rs must not import from printf_c_variadic
**Command:** `grep -n "printf_c_variadic" "$WORKSPACE_ROOT/src/src/loadext.rs" 2>/dev/null && exit 1 || exit 0`

#### loadext_no_sqlite3_mprintf
**Description:** loadext.rs must not use sqlite3_mprintf
**Command:** `grep -n "sqlite3_mprintf" "$WORKSPACE_ROOT/src/src/loadext.rs" 2>/dev/null && exit 1 || exit 0`

#### os_unix_no_printf_c_variadic
**Description:** os_unix.rs must not import from printf_c_variadic
**Command:** `grep -n "printf_c_variadic" "$WORKSPACE_ROOT/src/src/os_unix.rs" 2>/dev/null && exit 1 || exit 0`

#### os_unix_no_sqlite3_mprintf
**Description:** os_unix.rs must not use sqlite3_mprintf
**Command:** `grep -n "sqlite3_mprintf" "$WORKSPACE_ROOT/src/src/os_unix.rs" 2>/dev/null && exit 1 || exit 0`

#### pragma_no_printf_c_variadic
**Description:** pragma.rs must not import from printf_c_variadic
**Command:** `grep -n "printf_c_variadic" "$WORKSPACE_ROOT/src/src/pragma.rs" 2>/dev/null && exit 1 || exit 0`

#### pragma_no_sqlite3_mprintf
**Description:** pragma.rs must not use sqlite3_mprintf
**Command:** `grep -n "sqlite3_mprintf" "$WORKSPACE_ROOT/src/src/pragma.rs" 2>/dev/null && exit 1 || exit 0`

#### table_no_printf_c_variadic
**Description:** table.rs must not import from printf_c_variadic
**Command:** `grep -n "printf_c_variadic" "$WORKSPACE_ROOT/src/src/table.rs" 2>/dev/null && exit 1 || exit 0`

#### table_no_sqlite3_mprintf
**Description:** table.rs must not use sqlite3_mprintf
**Command:** `grep -n "sqlite3_mprintf" "$WORKSPACE_ROOT/src/src/table.rs" 2>/dev/null && exit 1 || exit 0`

#### vdbeaux_no_printf_c_variadic
**Description:** vdbeaux.rs must not import from printf_c_variadic
**Command:** `grep -n "printf_c_variadic" "$WORKSPACE_ROOT/src/src/vdbeaux.rs" 2>/dev/null && exit 1 || exit 0`

#### vdbeaux_no_sqlite3_mprintf
**Description:** vdbeaux.rs must not use sqlite3_mprintf
**Command:** `grep -n "sqlite3_mprintf" "$WORKSPACE_ROOT/src/src/vdbeaux.rs" 2>/dev/null && exit 1 || exit 0`

#### vdbevtab_no_printf_c_variadic
**Description:** vdbevtab.rs must not import from printf_c_variadic
**Command:** `grep -n "printf_c_variadic" "$WORKSPACE_ROOT/src/src/vdbevtab.rs" 2>/dev/null && exit 1 || exit 0`

#### vdbevtab_no_sqlite3_mprintf
**Description:** vdbevtab.rs must not use sqlite3_mprintf
**Command:** `grep -n "sqlite3_mprintf" "$WORKSPACE_ROOT/src/src/vdbevtab.rs" 2>/dev/null && exit 1 || exit 0`