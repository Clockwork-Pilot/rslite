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
    - [Feature: fts3_dead_code_removal](#fts3_dead_code_removal)
      - [fts3Appendf_removed](#fts3appendf_removed)
      - [fts3DbExec_removed](#fts3dbexec_removed)
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
    - [Feature: fts5IndexPrepareStmt_safety](#fts5indexpreparestmt_safety)
      - [fts5IndexPrepareStmt_no_unsafe_extern_c](#fts5indexpreparestmt_no_unsafe_extern_c)
      - [fts5IndexPrepareStmt_no_unsafe_keyword](#fts5indexpreparestmt_no_unsafe_keyword)
      - [fts5IndexPrepareStmt_safe_wrapper_no_unsafe_keyword](#fts5indexpreparestmt_safe_wrapper_no_unsafe_keyword)
    - [Feature: fts5WriteDlidxGrow_safety](#fts5writedlidxgrow_safety)
      - [fts5WriteDlidxGrow_safe_wrapper_no_unsafe_keyword](#fts5writedlidxgrow_safe_wrapper_no_unsafe_keyword)
    - [Feature: fts5WriteInit_safety](#fts5writeinit_safety)
      - [fts5WriteInit_no_libc](#fts5writeinit_no_libc)
      - [fts5WriteInit_no_unsafe_extern_c](#fts5writeinit_no_unsafe_extern_c)
      - [fts5WriteInit_no_unsafe_wrapping](#fts5writeinit_no_unsafe_wrapping)
    - [Feature: fts5_BufferAppendPrintf_variadic_removal](#fts5_bufferappendprintf_variadic_removal)
      - [prev_sqlite3Fts5Mprintf_complete](#prev_sqlite3fts5mprintf_complete)
      - [sqlite3Fts5BufferAppendPrintf_callsites_use_sqlite_printf](#sqlite3fts5bufferappendprintf_callsites_use_sqlite_printf)
      - [sqlite3Fts5BufferAppendPrintf_defined_in_fts5](#sqlite3fts5bufferappendprintf_defined_in_fts5)
      - [sqlite3Fts5BufferAppendPrintf_not_in_variadic](#sqlite3fts5bufferappendprintf_not_in_variadic)
    - [Feature: fts5_ConfigErrmsg_variadic_removal](#fts5_configerrmsg_variadic_removal)
      - [ConfigErrmsg_callsites_use_sqlite_printf](#configerrmsg_callsites_use_sqlite_printf)
      - [ConfigErrmsg_defined_nonvariadic_in_fts5](#configerrmsg_defined_nonvariadic_in_fts5)
      - [ConfigErrmsg_no_reexport](#configerrmsg_no_reexport)
      - [ConfigErrmsg_not_in_printf_c_variadic](#configerrmsg_not_in_printf_c_variadic)
    - [Feature: fts5_ExecPrintf_variadic_removal](#fts5_execprintf_variadic_removal)
      - [fts5ExecPrintf_callsites_use_sqlite_printf](#fts5execprintf_callsites_use_sqlite_printf)
      - [fts5ExecPrintf_defined_in_fts5](#fts5execprintf_defined_in_fts5)
      - [fts5ExecPrintf_not_in_variadic](#fts5execprintf_not_in_variadic)
      - [prev_fts3_dead_code_complete](#prev_fts3_dead_code_complete)
    - [Feature: fts5_Mprintf_variadic_removal](#fts5_mprintf_variadic_removal)
      - [prev_sqlite3Fts5ParseError_complete](#prev_sqlite3fts5parseerror_complete)
      - [sqlite3Fts5Mprintf_callsites_use_sqlite_printf](#sqlite3fts5mprintf_callsites_use_sqlite_printf)
      - [sqlite3Fts5Mprintf_defined_in_fts5](#sqlite3fts5mprintf_defined_in_fts5)
      - [sqlite3Fts5Mprintf_not_in_variadic](#sqlite3fts5mprintf_not_in_variadic)
    - [Feature: fts5_ParseError_variadic_removal](#fts5_parseerror_variadic_removal)
      - [prev_fts5ExecPrintf_complete](#prev_fts5execprintf_complete)
      - [sqlite3Fts5ParseError_callsites_use_sqlite_printf](#sqlite3fts5parseerror_callsites_use_sqlite_printf)
      - [sqlite3Fts5ParseError_defined_in_fts5](#sqlite3fts5parseerror_defined_in_fts5)
      - [sqlite3Fts5ParseError_not_in_variadic](#sqlite3fts5parseerror_not_in_variadic)
    - [Feature: fts5_PrepareStatement_variadic_removal](#fts5_preparestatement_variadic_removal)
      - [PrepareStatement_callsites_use_sqlite_printf](#preparestatement_callsites_use_sqlite_printf)
      - [PrepareStatement_defined_nonvariadic_in_fts5](#preparestatement_defined_nonvariadic_in_fts5)
      - [PrepareStatement_no_reexport](#preparestatement_no_reexport)
      - [PrepareStatement_not_in_printf_c_variadic](#preparestatement_not_in_printf_c_variadic)
    - [Feature: fts5_PrintfAppend_variadic_removal](#fts5_printfappend_variadic_removal)
      - [fts5PrintfAppend_callsites_use_sqlite_printf](#fts5printfappend_callsites_use_sqlite_printf)
      - [fts5PrintfAppend_defined_nonvariadic_in_fts5](#fts5printfappend_defined_nonvariadic_in_fts5)
      - [fts5PrintfAppend_no_reexport](#fts5printfappend_no_reexport)
      - [fts5PrintfAppend_not_in_printf_c_variadic](#fts5printfappend_not_in_printf_c_variadic)
    - [Feature: fts5_SetVtabError_variadic_removal](#fts5_setvtaberror_variadic_removal)
      - [fts5SetVtabError_callsites_use_sqlite_printf](#fts5setvtaberror_callsites_use_sqlite_printf)
      - [fts5SetVtabError_defined_nonvariadic_in_fts5](#fts5setvtaberror_defined_nonvariadic_in_fts5)
      - [fts5SetVtabError_no_reexport](#fts5setvtaberror_no_reexport)
      - [fts5SetVtabError_not_in_printf_c_variadic](#fts5setvtaberror_not_in_printf_c_variadic)
    - [Feature: fts5_c_variadic_migration](#fts5_c_variadic_migration)
      - [fts5_no_sqlite3_mprintf](#fts5_no_sqlite3_mprintf)
      - [fts5_no_sqlite3_snprintf](#fts5_no_sqlite3_snprintf)
      - [fts5_uses_sqlite_printf_macro](#fts5_uses_sqlite_printf_macro)
    - [Feature: fts5_no_libc_global](#fts5_no_libc_global)
      - [fts5_no_libc_calls](#fts5_no_libc_calls)
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
<<<<<<< HEAD
=======
    - [Feature: sqlite3Fts5BufferSize_safety](#sqlite3fts5buffersize_safety)
      - [sqlite3Fts5BufferSize_no_unsafe_extern_c](#sqlite3fts5buffersize_no_unsafe_extern_c)
      - [sqlite3Fts5BufferSize_no_unsafe_keyword](#sqlite3fts5buffersize_no_unsafe_keyword)
      - [sqlite3Fts5BufferSize_safe_wrapper_no_unsafe_keyword](#sqlite3fts5buffersize_safe_wrapper_no_unsafe_keyword)
>>>>>>> origin/master-no-abi
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

### Feature: fts3_dead_code_removal
**Remove dead variadic fts3DbExec and fts3Appendf from printf_c_variadic.rs — zero callers**

**Goals:**
- fts3DbExec and fts3Appendf in printf_c_variadic.rs have no callers and no re-exports anywhere in the codebase.
- Simply delete both functions from printf_c_variadic.rs.

#### fts3Appendf_removed
**Description:** Structural: fts3Appendf must be deleted from printf_c_variadic.rs (dead code, zero callers)
**Command:** `! grep -q "fn fts3Appendf" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

#### fts3DbExec_removed
**Description:** Structural: fts3DbExec must be deleted from printf_c_variadic.rs (dead code, zero callers)
**Command:** `! grep -q "fn fts3DbExec" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

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

### Feature: fts5IndexPrepareStmt_safety
**Refactor fts5IndexPrepareStmt to safe references**

**Goals:**
- Remove unsafe extern C from fts5IndexPrepareStmt
- Replace raw pointer parameters with safe mutable references

#### fts5IndexPrepareStmt_no_unsafe_extern_c
**Description:** fts5IndexPrepareStmt must not have unsafe extern C signature
**Command:** `awk '/^[^/]*fn fts5IndexPrepareStmt/,/^}/' "$PROJECT_ROOT/src/fts5.rs" 2>/dev/null | grep -E 'unsafe\s+extern' && exit 1 || exit 0`

#### fts5IndexPrepareStmt_no_unsafe_keyword
**Description:** fts5IndexPrepareStmt must be completely safe - no unsafe keyword anywhere
**Command:** `awk '/^[^/]*fn fts5IndexPrepareStmt/,/^}/' "$PROJECT_ROOT/src/fts5.rs" 2>/dev/null | grep -E '\bunsafe\b' && exit 1 || exit 0`

#### fts5IndexPrepareStmt_safe_wrapper_no_unsafe_keyword
**Description:** _fts5IndexPrepareStmt_safe_wrapper must not have unsafe keyword in signature
**Command:** `awk '/^fn _fts5IndexPrepareStmt_safe_wrapper/,/^}/' "$PROJECT_ROOT/src/fts5.rs" 2>/dev/null | grep -E '^fn ' | grep -E 'unsafe' && exit 1 || exit 0`

### Feature: fts5WriteDlidxGrow_safety
**Refactor fts5WriteDlidxGrow core logic to safe function via wrapper**

**Goals:**
- Create _fts5WriteDlidxGrow_safe_wrapper function with no unsafe keyword
- Ensure all unsafe operations are wrapped in minimal unsafe blocks

#### fts5WriteDlidxGrow_safe_wrapper_no_unsafe_keyword
**Description:** _fts5WriteDlidxGrow_safe_wrapper must not have unsafe keyword in signature
**Command:** `awk '/^fn _fts5WriteDlidxGrow_safe_wrapper/,/^}/' "$PROJECT_ROOT/src/fts5.rs" 2>/dev/null | grep -E '^fn ' | grep -E 'unsafe' && exit 1 || exit 0`

### Feature: fts5WriteInit_safety
**Ensure fts5WriteInit contains no unsafe extern "C" or unsafe blocks**

**Goals:**
- Remove all unsafe extern "C" declarations from fts5WriteInit
- Remove all unsafe {} blocks from fts5WriteInit

#### fts5WriteInit_no_libc
**Description:** fts5WriteInit must not use libc functions
**Command:** `awk '/^[^/]*fn fts5WriteInit/,/^}/' "$PROJECT_ROOT/src/fts5.rs" 2>/dev/null | grep -E 'libc::' && exit 1 || exit 0`

#### fts5WriteInit_no_unsafe_extern_c
**Description:** fts5WriteInit must not contain unsafe extern "C"
**Command:** `awk '/^[^/]*fn fts5WriteInit/,/^}/' "$PROJECT_ROOT/src/fts5.rs" 2>/dev/null | grep -E 'unsafe\s+extern\s+"C"' && exit 1 || exit 0`

#### fts5WriteInit_no_unsafe_wrapping
**Description:** fts5WriteInit body must not be wrapped in unsafe block - entire body must be safe
**Command:** `awk '/^[^/]*fn fts5WriteInit/,/^}/' "$PROJECT_ROOT/src/fts5.rs" 2>/dev/null | grep -E '^\s*unsafe\s*$' && exit 1 || exit 0`

### Feature: fts5_BufferAppendPrintf_variadic_removal
**Remove variadic sqlite3Fts5BufferAppendPrintf from printf_c_variadic; redefine non-variadic in fts5.rs using sqlite_printf!**

**Goals:**
- sqlite3Fts5BufferAppendPrintf in printf_c_variadic.rs: if *pRc==OK, formats via sqlite3_vmprintf then appends to Fts5Buffer. 26 call sites in fts5.rs.
- Remove from printf_c_variadic.rs and pub use re-export. Define private non-variadic fn in fts5.rs taking pre-formatted *mut c_char.
- Each call site passes sqlite_printf!(fmt, args...) as the string arg. Depends on fts5_Mprintf_variadic_removal being complete.

#### prev_sqlite3Fts5Mprintf_complete
**Description:** Dependency: fts5_Mprintf_variadic_removal must be complete first
**Command:** `! grep -q "fn sqlite3Fts5Mprintf" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

#### sqlite3Fts5BufferAppendPrintf_callsites_use_sqlite_printf
**Description:** Behavioral: fts5.rs must have at least 159 sqlite_printf! calls (133 + 26 sqlite3Fts5BufferAppendPrintf call sites)
**Command:** `[ $(grep -c "sqlite_printf!" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs") -ge 159 ]`

#### sqlite3Fts5BufferAppendPrintf_defined_in_fts5
**Description:** Structural: sqlite3Fts5BufferAppendPrintf must be defined non-variadic in fts5.rs
**Command:** `grep -q "fn sqlite3Fts5BufferAppendPrintf" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs" && ! grep -qE "fn sqlite3Fts5BufferAppendPrintf.*\.\.\." "${CLAUDE_PROJECT_ROOT}/src/fts5.rs"`

#### sqlite3Fts5BufferAppendPrintf_not_in_variadic
**Description:** Structural: sqlite3Fts5BufferAppendPrintf must be removed from printf_c_variadic.rs
**Command:** `! grep -q "fn sqlite3Fts5BufferAppendPrintf" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

### Feature: fts5_ConfigErrmsg_variadic_removal
**Remove variadic sqlite3Fts5ConfigErrmsg from printf_c_variadic; redefine pub(crate) non-variadic in fts5.rs**

**Goals:**
- sqlite3Fts5ConfigErrmsg in printf_c_variadic.rs is variadic: formats message via sqlite3_vmprintf and stores in pConfig.pzErrmsg if non-null.
- 9 call sites in fts5.rs, 1 call site inside fts5PrepareStatement in printf_c_variadic.rs. Remove from printf_c_variadic.rs and pub use re-export from fts5.rs.
- Redefine as pub(crate) non-variadic unsafe fn sqlite3Fts5ConfigErrmsg(p: *mut Fts5Config, zMsg: *mut c_char) in fts5.rs.
- Each fts5.rs call site passes sqlite_printf!(fmt, args...) as zMsg. The fts5PrepareStatement call in printf_c_variadic.rs calls crate::src::fts5::sqlite3Fts5ConfigErrmsg with sqlite_printf!.
- One call site (21741) has a runtime-selected format string requiring two sqlite_printf! branches in an if/else.

#### ConfigErrmsg_callsites_use_sqlite_printf
**Description:** Behavioral: fts5.rs must have >= 99 sqlite_printf! uses (89 existing + 9 call sites + 1 extra for runtime-selected format branch)
**Command:** `[ $(grep -c "sqlite_printf!" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs") -ge 99 ]`

#### ConfigErrmsg_defined_nonvariadic_in_fts5
**Description:** Structural: sqlite3Fts5ConfigErrmsg must be defined in fts5.rs with non-variadic signature
**Command:** `grep -q "fn sqlite3Fts5ConfigErrmsg" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs" && ! grep -qE "fn sqlite3Fts5ConfigErrmsg.*\.\.\." "${CLAUDE_PROJECT_ROOT}/src/fts5.rs"`

#### ConfigErrmsg_no_reexport
**Description:** Structural: no pub use re-export of sqlite3Fts5ConfigErrmsg in fts5.rs
**Command:** `! grep -qE "pub use.*sqlite3Fts5ConfigErrmsg" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs"`

#### ConfigErrmsg_not_in_printf_c_variadic
**Description:** Structural: sqlite3Fts5ConfigErrmsg function must be removed from printf_c_variadic.rs
**Command:** `! grep -q "fn sqlite3Fts5ConfigErrmsg" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

### Feature: fts5_ExecPrintf_variadic_removal
**Remove variadic fts5ExecPrintf from printf_c_variadic; redefine non-variadic in fts5.rs using sqlite_printf!**

**Goals:**
- fts5ExecPrintf in printf_c_variadic.rs: formats SQL via sqlite3_vmprintf, executes via sqlite3_exec, reports error in *pzErr. 8 call sites in fts5.rs.
- Remove from printf_c_variadic.rs and pub use re-export. Define private non-variadic fn fts5ExecPrintf(db, pzErr, zSql: *mut c_char) in fts5.rs.
- Each call site passes sqlite_printf!(fmt, args...) as zSql. Depends on fts3_dead_code_removal being complete.

#### fts5ExecPrintf_callsites_use_sqlite_printf
**Description:** Behavioral: fts5.rs must have at least 109 sqlite_printf! calls (101 existing + 8 fts5ExecPrintf call sites)
**Command:** `[ $(grep -c "sqlite_printf!" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs") -ge 109 ]`

#### fts5ExecPrintf_defined_in_fts5
**Description:** Structural: fts5ExecPrintf must be defined non-variadic in fts5.rs
**Command:** `grep -q "fn fts5ExecPrintf" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs" && ! grep -qE "fn fts5ExecPrintf.*\.\.\." "${CLAUDE_PROJECT_ROOT}/src/fts5.rs"`

#### fts5ExecPrintf_not_in_variadic
**Description:** Structural: fts5ExecPrintf must be removed from printf_c_variadic.rs
**Command:** `! grep -q "fn fts5ExecPrintf" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

#### prev_fts3_dead_code_complete
**Description:** Dependency: fts3_dead_code_removal must be complete first (fts3DbExec and fts3Appendf gone)
**Command:** `! grep -qE "fn fts3DbExec|fn fts3Appendf" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

### Feature: fts5_Mprintf_variadic_removal
**Remove variadic sqlite3Fts5Mprintf from printf_c_variadic; redefine non-variadic in fts5.rs using sqlite_printf!**

**Goals:**
- sqlite3Fts5Mprintf in printf_c_variadic.rs: if *pRc==OK, returns sqlite3_vmprintf(zFmt,args) or sets *pRc=NOMEM. 14 call sites in fts5.rs.
- Remove from printf_c_variadic.rs and pub use re-export. Define private non-variadic fn returning *mut c_char in fts5.rs.
- Each call site passes sqlite_printf!(fmt, args...) directly. Depends on fts5_ParseError_variadic_removal being complete.

#### prev_sqlite3Fts5ParseError_complete
**Description:** Dependency: fts5_ParseError_variadic_removal must be complete first
**Command:** `! grep -q "fn sqlite3Fts5ParseError" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

#### sqlite3Fts5Mprintf_callsites_use_sqlite_printf
**Description:** Behavioral: fts5.rs must have at least 133 sqlite_printf! calls (119 + 14 sqlite3Fts5Mprintf call sites)
**Command:** `[ $(grep -c "sqlite_printf!" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs") -ge 133 ]`

#### sqlite3Fts5Mprintf_defined_in_fts5
**Description:** Structural: sqlite3Fts5Mprintf must be defined non-variadic in fts5.rs
**Command:** `grep -q "fn sqlite3Fts5Mprintf" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs" && ! grep -qE "fn sqlite3Fts5Mprintf.*\.\.\." "${CLAUDE_PROJECT_ROOT}/src/fts5.rs"`

#### sqlite3Fts5Mprintf_not_in_variadic
**Description:** Structural: sqlite3Fts5Mprintf must be removed from printf_c_variadic.rs
**Command:** `! grep -q "fn sqlite3Fts5Mprintf" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

### Feature: fts5_ParseError_variadic_removal
**Remove variadic sqlite3Fts5ParseError from printf_c_variadic; redefine non-variadic in fts5.rs using sqlite_printf!**

**Goals:**
- sqlite3Fts5ParseError in printf_c_variadic.rs: if pParse.rc==OK, sets pParse.zErr=sqlite3_vmprintf(zFmt,args) and pParse.rc=SQLITE_ERROR. 10 call sites in fts5.rs.
- Remove from printf_c_variadic.rs and pub use re-export. Define private non-variadic fn in fts5.rs taking *mut c_char.
- Each call site passes sqlite_printf!(fmt, args...) as zErr. Depends on fts5_ExecPrintf_variadic_removal being complete.

#### prev_fts5ExecPrintf_complete
**Description:** Dependency: fts5_ExecPrintf_variadic_removal must be complete first
**Command:** `! grep -q "fn fts5ExecPrintf" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

#### sqlite3Fts5ParseError_callsites_use_sqlite_printf
**Description:** Behavioral: fts5.rs must have at least 119 sqlite_printf! calls (109 + 10 sqlite3Fts5ParseError call sites)
**Command:** `[ $(grep -c "sqlite_printf!" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs") -ge 119 ]`

#### sqlite3Fts5ParseError_defined_in_fts5
**Description:** Structural: sqlite3Fts5ParseError must be defined non-variadic in fts5.rs
**Command:** `grep -q "fn sqlite3Fts5ParseError" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs" && ! grep -qE "fn sqlite3Fts5ParseError.*\.\.\." "${CLAUDE_PROJECT_ROOT}/src/fts5.rs"`

#### sqlite3Fts5ParseError_not_in_variadic
**Description:** Structural: sqlite3Fts5ParseError must be removed from printf_c_variadic.rs
**Command:** `! grep -q "fn sqlite3Fts5ParseError" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

### Feature: fts5_PrepareStatement_variadic_removal
**Remove variadic fts5PrepareStatement from printf_c_variadic; redefine non-variadic in fts5.rs using sqlite_printf!**

**Goals:**
- fts5PrepareStatement in printf_c_variadic.rs is variadic: formats SQL via sqlite3_vmprintf, prepares statement, reports error via sqlite3Fts5ConfigErrmsg.
- Single call site in fts5.rs at line 17473 with format %Q.%Q ORDER BY %s("%w"%s%s) %s and 7 args.
- Remove from printf_c_variadic.rs and pub use re-export from fts5.rs. Define private non-variadic unsafe fn fts5PrepareStatement(ppStmt, pConfig, zSql: *mut c_char) in fts5.rs.
- Call site passes sqlite_printf!(fmt, ...) as zSql. sqlite3Fts5ConfigErrmsg can be downgraded from pub(crate) to private fn.

#### PrepareStatement_callsites_use_sqlite_printf
**Description:** Behavioral: fts5.rs must have >= 101 sqlite_printf! uses (99 existing + 1 call site SQL + 1 errmsg in function body)
**Command:** `[ $(grep -c "sqlite_printf!" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs") -ge 101 ]`

#### PrepareStatement_defined_nonvariadic_in_fts5
**Description:** Structural: fts5PrepareStatement must be defined in fts5.rs with non-variadic signature
**Command:** `grep -q "fn fts5PrepareStatement" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs" && ! grep -qE "fn fts5PrepareStatement.*\.\.\." "${CLAUDE_PROJECT_ROOT}/src/fts5.rs"`

#### PrepareStatement_no_reexport
**Description:** Structural: no pub use re-export of fts5PrepareStatement in fts5.rs
**Command:** `! grep -qE "pub use.*fts5PrepareStatement" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs"`

#### PrepareStatement_not_in_printf_c_variadic
**Description:** Structural: fts5PrepareStatement must be removed from printf_c_variadic.rs
**Command:** `! grep -q "fn fts5PrepareStatement" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

### Feature: fts5_PrintfAppend_variadic_removal
**Remove variadic fts5PrintfAppend from printf_c_variadic; redefine non-variadic in fts5.rs using sqlite_printf!**

**Goals:**
- fts5PrintfAppend in src/printf_c_variadic.rs is a variadic C-ABI function: formats new piece via sqlite3_vmprintf, concatenates with existing zApp string, frees both intermediates, returns result.
- There are 20 call sites in fts5.rs. Delete fts5PrintfAppend from printf_c_variadic.rs and remove its pub use re-export from fts5.rs.
- Redefine as private non-variadic unsafe fn fts5PrintfAppend(zApp: *mut c_char, zNew: *mut c_char) -> *mut c_char directly in fts5.rs.
- Each call site passes sqlite_printf!(fmt, args...) as the zNew argument instead of a raw format string with variadic args.

#### fts5PrintfAppend_callsites_use_sqlite_printf
**Description:** Behavioral: fts5.rs must have >= 80 sqlite_printf! uses (60 existing + 20 call sites replacing variadic fts5PrintfAppend calls)
**Command:** `[ $(grep -c "sqlite_printf!" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs") -ge 80 ]`

#### fts5PrintfAppend_defined_nonvariadic_in_fts5
**Description:** Structural: fts5PrintfAppend must be defined in fts5.rs with a non-variadic signature
**Command:** `grep -q "fn fts5PrintfAppend" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs" && ! grep -qE "fn fts5PrintfAppend.*\.\.\." "${CLAUDE_PROJECT_ROOT}/src/fts5.rs"`

#### fts5PrintfAppend_no_reexport
**Description:** Structural: no pub use re-export of fts5PrintfAppend from printf_c_variadic in fts5.rs
**Command:** `! grep -qE "pub use.*fts5PrintfAppend" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs"`

#### fts5PrintfAppend_not_in_printf_c_variadic
**Description:** Structural: fts5PrintfAppend function must be removed from printf_c_variadic.rs
**Command:** `! grep -q "fn fts5PrintfAppend" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

### Feature: fts5_SetVtabError_variadic_removal
**Remove variadic fts5SetVtabError from printf_c_variadic; redefine non-variadic in fts5.rs using sqlite_printf!**

**Goals:**
- fts5SetVtabError in src/printf_c_variadic.rs is variadic: frees p.p.base.zErrMsg and sets it to sqlite3_vmprintf(zFormat, args).
- There are 8 call sites in fts5.rs. Delete from printf_c_variadic.rs and remove pub use re-export from fts5.rs.
- Redefine as private non-variadic unsafe fn fts5SetVtabError(p: *mut Fts5FullTable, zMsg: *mut c_char) in fts5.rs.
- Each call site passes sqlite_printf!(fmt, args...) as zMsg.

#### fts5SetVtabError_callsites_use_sqlite_printf
**Description:** Behavioral: fts5.rs must have >= 89 sqlite_printf! uses (81 existing + 8 call sites)
**Command:** `[ $(grep -c "sqlite_printf!" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs") -ge 89 ]`

#### fts5SetVtabError_defined_nonvariadic_in_fts5
**Description:** Structural: fts5SetVtabError must be defined in fts5.rs with non-variadic signature
**Command:** `grep -q "fn fts5SetVtabError" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs" && ! grep -qE "fn fts5SetVtabError.*\.\.\." "${CLAUDE_PROJECT_ROOT}/src/fts5.rs"`

#### fts5SetVtabError_no_reexport
**Description:** Structural: no pub use re-export of fts5SetVtabError in fts5.rs
**Command:** `! grep -qE "pub use.*fts5SetVtabError" "${CLAUDE_PROJECT_ROOT}/src/fts5.rs"`

#### fts5SetVtabError_not_in_printf_c_variadic
**Description:** Structural: fts5SetVtabError must be removed from printf_c_variadic.rs
**Command:** `! grep -q "fn fts5SetVtabError" "${CLAUDE_PROJECT_ROOT}/src/printf_c_variadic.rs"`

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

### Feature: fts5_no_libc_global
**Ensure fts5.rs contains no libc:: calls (only external C FFI allowed)**

**Goals:**
- fts5.rs must not contain libc:: direct calls
- All C interop must go through safe wrappers or external C FFI only

#### fts5_no_libc_calls
**Description:** fts5.rs must not contain libc:: calls - use safe wrappers or external C FFI instead
**Command:** `grep -q "libc::" "$PROJECT_ROOT/src/fts5.rs" && exit 1 || exit 0`

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

<<<<<<< HEAD
=======
### Feature: sqlite3Fts5BufferSize_safety
**Refactor sqlite3Fts5BufferSize to safe references**

**Goals:**
- Remove unsafe extern C from sqlite3Fts5BufferSize
- Replace raw pointer parameters with safe mutable references

#### sqlite3Fts5BufferSize_no_unsafe_extern_c
**Description:** sqlite3Fts5BufferSize must not have unsafe extern C signature
**Command:** `awk '/^[^/]*fn sqlite3Fts5BufferSize/,/^}/' "$PROJECT_ROOT/src/fts5.rs" 2>/dev/null | grep -E 'unsafe\s+extern' && exit 1 || exit 0`

#### sqlite3Fts5BufferSize_no_unsafe_keyword
**Description:** sqlite3Fts5BufferSize must be completely safe - no unsafe keyword anywhere
**Command:** `awk '/^[^/]*fn sqlite3Fts5BufferSize/,/^}/' "$PROJECT_ROOT/src/fts5.rs" 2>/dev/null | grep -E '\bunsafe\b' && exit 1 || exit 0`

#### sqlite3Fts5BufferSize_safe_wrapper_no_unsafe_keyword
**Description:** _sqlite3Fts5BufferSize_safe_wrapper must not have unsafe keyword in signature
**Command:** `awk '/^fn _sqlite3Fts5BufferSize_safe_wrapper/,/^}/' "$PROJECT_ROOT/src/fts5.rs" 2>/dev/null | grep -E '^fn ' | grep -E 'unsafe' && exit 1 || exit 0`

>>>>>>> origin/master-no-abi
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