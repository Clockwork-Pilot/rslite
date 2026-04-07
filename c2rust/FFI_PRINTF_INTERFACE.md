# FFI Printf Interface Migration

## Overview

The SQLite shell and TCL bindings use a **C FFI interface** to call `sqlite3_str_appendf()`. This function has been transparently migrated to use the new **Rust printf_args! macro system** without requiring any changes to the client code.

## Architecture

### Call Flow

```
shell.rs / qpvtab.rs / csv.rs (FFI Clients)
    ↓ (FFI call with variadic C arguments)
sqlite3_str_appendf (extern "C" wrapper in printf_c_variadic.rs)
    ↓ (extract VaList/C variadic arguments)
extract_printf_args()
    ↓ (convert to Vec<PrintfArg>)
sqlite3_str_vappendf_args()
    ↓ (call new Rust formatter)
sqlite3_str_vappendf2() with printf_args! macro
```

---

## 1. FFI Declaration (CLIENT SIDE)

### crust-sqlite-shell/src/shell.rs (lines 622-626)

```rust
unsafe extern "C" {
    fn sqlite3_str_new(_: *mut sqlite3) -> *mut sqlite3_str;
    fn sqlite3_str_finish(_: *mut sqlite3_str) -> *mut ::core::ffi::c_char;
    fn sqlite3_str_appendf(
        _: *mut sqlite3_str,
        zFormat: *const ::core::ffi::c_char,
        ...          // ← C variadic arguments
    );
    fn sqlite3_str_append(
        _: *mut sqlite3_str,
        zIn: *const ::core::ffi::c_char,
        N: ::core::ffi::c_int,
    );
}
```

### crust-tclsqlite/src/src/ext/misc/qpvtab.rs (lines 41-45)

```rust
unsafe extern "C" {
    fn sqlite3_str_new(_: *mut sqlite3) -> *mut sqlite3_str;
    fn sqlite3_str_finish(_: *mut sqlite3_str) -> *mut ::core::ffi::c_char;
    fn sqlite3_str_appendf(
        _: *mut sqlite3_str,
        zFormat: *const ::core::ffi::c_char,
        ...          // ← C variadic arguments
    );
    fn sqlite3_str_append(
        _: *mut sqlite3_str,
        zIn: *const ::core::ffi::c_char,
        N: ::core::ffi::c_int,
    );
}
```

### crust-tclsqlite/src/src/ext/misc/csv.rs (lines 47-51)

```rust
unsafe extern "C" {
    fn sqlite3_str_new(_: *mut sqlite3) -> *mut sqlite3_str;
    fn sqlite3_str_finish(_: *mut sqlite3_str) -> *mut ::core::ffi::c_char;
    fn sqlite3_str_appendf(
        _: *mut sqlite3_str,
        zFormat: *const ::core::ffi::c_char,
        ...          // ← C variadic arguments
    );
}
```

---

## 2. CLIENT CALLS

### Example 1: crust-sqlite-shell/src/shell.rs (lines 37653-37658)

**2 format arguments:**

```rust
let mut pStr: *mut sqlite3_str = sqlite3_str_new(
    ::core::ptr::null_mut::<sqlite3>(),
);
sqlite3_str_appendf(
    pStr,
    b"%s, %s\0".as_ptr() as *const ::core::ffi::c_char,
    zPhase,
    sqlite3_errmsg(db),
);
```

### Example 2: crust-sqlite-shell/src/shell.rs (lines 37660-37664)

**1 format argument:**

```rust
if rc > 1 as ::core::ffi::c_int {
    sqlite3_str_appendf(
        pStr,
        b" (%d)\0".as_ptr() as *const ::core::ffi::c_char,
        rc,
    );
}
```

### Example 3: crust-tclsqlite/src/src/ext/misc/qpvtab.rs (lines 2182-2184)

**1 format argument (integer):**

```rust
sqlite3_str_appendf(
    pStr,
    b"%lld\0".as_ptr() as *const ::core::ffi::c_char,
    sqlite3_value_int64(pVal),
);
```

### Example 4: crust-tclsqlite/src/src/ext/misc/qpvtab.rs (lines 2189-2191)

**1 format argument (double):**

```rust
sqlite3_str_appendf(
    pStr,
    b"%!f\0".as_ptr() as *const ::core::ffi::c_char,
    sqlite3_value_double(pVal),
);
```

---

## 3. SERVER IMPLEMENTATION (WRAPPER)

### src/printf_c_variadic.rs (lines 66-84)

The **bridge function** that transparently routes C variadic calls to the new Rust infrastructure:

```rust
pub unsafe extern "C" fn sqlite3_str_appendf(
    mut p: *mut crate::src::headers::sqliteInt_h::StrAccum,
    mut zFormat: *const ::core::ffi::c_char,
    mut args: ...
) {
    if (*p).printfFlags as ::core::ffi::c_int & crate::src::headers::sqliteInt_h::SQLITE_PRINTF_SQLFUNC != 0 {
        // SQLFUNC path: read PrintfArguments pointer, then use direct mode
        let pArgList = args.arg::<*mut crate::src::headers::sqliteInt_h::PrintfArguments>();
        crate::src::src::printf::sqlite3_str_vappendf_sqlfunc(
            p as *mut crate::src::headers::sqliteInt_h::sqlite3_str,
            zFormat,
            pArgList,
        );
    } else {
        // Normal path: extract args from VaList, then format
        let (_s, a) = crate::src::src::printf::extract_printf_args(zFormat, args, false, ::core::ptr::null_mut());
        crate::src::src::printf::sqlite3_str_vappendf_args(p as *mut crate::src::headers::sqliteInt_h::sqlite3_str, zFormat, &a);
    }
}
```

---

## Key Points

1. **FFI Boundary Preservation**: The C interface signature remains unchanged
   - Clients continue to use variadic arguments (`...`)
   - No recompilation of client code needed

2. **Transparent Migration**: The wrapper function:
   - Extracts C variadic arguments using `args.arg::<T>()`
   - Converts them to `Vec<PrintfArg>` via `extract_printf_args()`
   - Routes to the new `sqlite3_str_vappendf_args()` which uses `printf_args!` macro internally

3. **Two Processing Paths**:
   - **SQLFUNC Path**: For SQL functions that pass `PrintfArguments*`
   - **Normal Path**: For standard variadic calls from shell/TCL

4. **No Client Changes Required**: 
   - shell.rs: 3 calls (unchanged)
   - qpvtab.rs: 13 calls (unchanged)
   - csv.rs: 5 calls (unchanged)

---

## Summary

The FFI interface maintains **100% backward compatibility** while routing all calls through the new **Rust printf_args! infrastructure**. The wrapper function in `printf_c_variadic.rs` acts as the translation layer between C variadic arguments and the Rust printf system.

This allows the main library (`src/`) to be fully modernized with the new printf architecture while client code (shell, TCL bindings) continues to use the existing C interface without modification.
