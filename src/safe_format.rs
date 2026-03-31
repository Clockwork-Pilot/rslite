/// Type-safe idiomatic Rust formatting for SQLite strings
/// Replaces variadic printf-style formatting with Rust's format! system

extern crate alloc;

use ::core::ffi::{c_char, c_void, CStr};
use alloc::string::String;
use alloc::vec::Vec;

/// Quote an identifier for SQL (double quotes, escape by doubling)
pub fn quote_identifier(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}

/// Quote a string literal for SQL (single quotes, escape by doubling)
pub fn quote_string(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

/// Convert a C string to Rust string, handling null pointers
pub unsafe fn c_str_to_rust(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        CStr::from_ptr(ptr)
            .to_str()
            .ok()
            .map(|s| s.to_string())
    }
}

/// Convert multiple C strings to Rust strings
pub unsafe fn c_strs_to_rust(ptrs: &[*const c_char]) -> Vec<String> {
    ptrs.iter()
        .filter_map(|&ptr| c_str_to_rust(ptr))
        .collect()
}

/// Allocate a C string from a Rust String (allocates with sqlite3_malloc64)
pub fn rust_to_c_string(s: String) -> *mut c_char {
    allocate_string(s)
}

/// Get length of a C string
pub unsafe fn cstr_len(ptr: *const c_char) -> usize {
    if ptr.is_null() {
        0
    } else {
        CStr::from_ptr(ptr).to_bytes().len()
    }
}

/// Allocate and copy bytes to SQLite-managed memory (with null termination)
pub unsafe fn allocate_and_copy(bytes: &[u8]) -> *mut c_char {
    let len = bytes.len();
    let ptr = crate::src::src::malloc::sqlite3_malloc64((len + 1) as u64) as *mut u8;
    if !ptr.is_null() {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, len);
        *ptr.add(len) = 0; // null terminate
    }
    ptr as *mut c_char
}

/// Allocate and copy a Rust string to SQLite-managed memory
pub fn allocate_string(s: String) -> *mut c_char {
    let bytes = s.into_bytes();
    unsafe { allocate_and_copy(&bytes) }
}

/// Format builder for SQL strings - type-safe alternative to printf
pub struct SqlFormat {
    parts: Vec<String>,
}

impl SqlFormat {
    pub fn new() -> Self {
        SqlFormat {
            parts: Vec::new(),
        }
    }

    /// Append a literal string
    pub fn literal(mut self, s: &str) -> Self {
        self.parts.push(s.to_string());
        self
    }

    /// Append a quoted identifier
    pub fn identifier(mut self, s: &str) -> Self {
        self.parts.push(quote_identifier(s));
        self
    }

    /// Append a quoted string literal
    pub fn string(mut self, s: &str) -> Self {
        self.parts.push(quote_string(s));
        self
    }

    /// Append an unquoted string value (use carefully!)
    pub fn raw(mut self, s: &str) -> Self {
        self.parts.push(s.to_string());
        self
    }

    /// Build the final SQL string
    pub fn build(self) -> String {
        self.parts.join("")
    }

    /// Build and allocate as C string
    pub fn build_c(self) -> *mut c_char {
        allocate_string(self.build())
    }
}

impl Default for SqlFormat {
    fn default() -> Self {
        Self::new()
    }
}

// Convenience helpers for common printf patterns found in SQLite code
pub unsafe fn format_s(format_ptr: *const c_char, arg: *const c_char) -> *mut c_char {
    if format_ptr.is_null() || arg.is_null() {
        return ::core::ptr::null_mut();
    }
    let fmt = match CStr::from_ptr(format_ptr).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };
    let arg_str = match CStr::from_ptr(arg).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return ::core::ptr::null_mut(),
    };
    let result = fmt.replace("%s", &arg_str);
    allocate_string(result)
}

pub unsafe fn format_q(format_ptr: *const c_char, arg: *const c_char) -> *mut c_char {
    if format_ptr.is_null() || arg.is_null() {
        return ::core::ptr::null_mut();
    }
    let fmt = match CStr::from_ptr(format_ptr).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };
    let arg_str = match CStr::from_ptr(arg).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };
    let result = fmt.replace("%q", &quote_string(arg_str));
    allocate_string(result)
}

pub unsafe fn format_Q(format_ptr: *const c_char, arg: *const c_char) -> *mut c_char {
    if format_ptr.is_null() || arg.is_null() {
        return ::core::ptr::null_mut();
    }
    let fmt = match CStr::from_ptr(format_ptr).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };
    let arg_str = match CStr::from_ptr(arg).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };
    let result = fmt.replace("%Q", &quote_identifier(arg_str));
    allocate_string(result)
}

pub unsafe fn format_z(format_ptr: *const c_char, arg: *const c_char) -> *mut c_char {
    if format_ptr.is_null() {
        return ::core::ptr::null_mut();
    }
    // %z is "zero-copy" string - just use as %s
    let fmt = match CStr::from_ptr(format_ptr).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };
    let arg_str = if arg.is_null() {
        String::new()
    } else {
        match CStr::from_ptr(arg).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ::core::ptr::null_mut(),
        }
    };
    let result = fmt.replace("%z", &arg_str);
    allocate_string(result)
}

// Proper SQLite formatting semantics replacements

/// Convert C string to Rust string, handling null safely with SQLite semantics
pub unsafe fn c_str_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        "NULL".to_string()
    } else {
        CStr::from_ptr(ptr)
            .to_str()
            .unwrap_or("")
            .to_string()
    }
}

/// SQLite %Q escaping: escape string for SQL + wrap in single quotes
/// Escapes single quotes by doubling them
pub fn sqlite_escape_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for ch in s.chars() {
        if ch == '\'' {
            out.push('\''); // double it
            out.push('\'');
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

/// SQLite %w identifier escaping: escape identifier with double quotes
pub fn sqlite_escape_identifier(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        if ch == '"' {
            out.push('"'); // double it
            out.push('"');
        } else {
            out.push(ch);
        }
    }
    out.push('"');
    out
}

/// SQLite-style %Q formatting: returns NULL or 'escaped string'
pub unsafe fn format_sqlite_Q(z: *const c_char) -> String {
    if z.is_null() {
        "NULL".to_string()
    } else {
        let s = c_str_to_string(z);
        sqlite_escape_string(&s)
    }
}

/// SQLite-style %w formatting: returns "escaped identifier"
pub unsafe fn format_sqlite_w(z: *const c_char) -> String {
    if z.is_null() {
        String::new()
    } else {
        let s = c_str_to_string(z);
        sqlite_escape_identifier(&s)
    }
}

// Specific helpers for memdb.rs

/// Format memdb VFSNAME: "memdb(%p,%lld)"
pub unsafe fn format_memdb_vfsname(
    pData: *mut ::core::ffi::c_uchar,
    sz: crate::src::headers::sqlite3_h::sqlite3_int64,
) -> *mut c_char {
    let result = format!("memdb({:p},{})", pData, sz);
    allocate_string(result)
}

/// Format PRAGMA page_count with identifier escaping
pub unsafe fn format_pragma_page_count(zSchema: *const c_char) -> *mut c_char {
    if zSchema.is_null() {
        return ::core::ptr::null_mut();
    }
    let schema_str = match CStr::from_ptr(zSchema).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };
    let quoted = sqlite_escape_identifier(schema_str);
    let result = format!("PRAGMA {}.page_count", quoted);
    allocate_string(result)
}

/// Format ATTACH with identifier escaping
pub unsafe fn format_attach_as(zSchema: *const c_char) -> *mut c_char {
    if zSchema.is_null() {
        return ::core::ptr::null_mut();
    }
    let schema_str = match CStr::from_ptr(zSchema).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };
    let quoted = sqlite_escape_identifier(schema_str);
    let result = format!("ATTACH x AS {}", quoted);
    allocate_string(result)
}
