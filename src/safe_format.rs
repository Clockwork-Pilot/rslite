/// Type-safe idiomatic Rust formatting for SQLite strings
/// Replaces variadic printf-style formatting with Rust's format! system

extern crate alloc;

use ::core::ffi::{c_char, CStr};
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

/// SQLite %q escaping: escape string content (wrap in quotes)
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

/// SQLite %Q escaping: escape string content for identifier (wrap in quotes)
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

/// Escape string content only, without adding quotes (for format strings that have quotes)
fn escape_string_content(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if ch == '\'' {
            out.push('\''); // double it
            out.push('\'');
        } else {
            out.push(ch);
        }
    }
    out
}

/// Escape identifier content only, without adding quotes (for format strings that have quotes)
fn escape_identifier_content(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if ch == '"' {
            out.push('"'); // double it
            out.push('"');
        } else {
            out.push(ch);
        }
    }
    out
}

/// SQLite format specifier type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteFormatSpec {
    String,
    SqliteQuote,      // %q - escape quotes, no wrapping
    SqliteIdentifier, // %Q or %w - escape and wrap in quotes
}

/// Token representation of a parsed format string
#[derive(Debug, Clone)]
pub enum FormatToken {
    Literal(String),
    Spec {
        spec: SqliteFormatSpec,
        arg_index: usize
    },
}

/// Tokenize a SQLite format string into literal and specifier tokens
pub fn tokenize_format(format: &str) -> Result<Vec<FormatToken>, String> {
    let mut tokens = Vec::new();
    let mut literal = String::new();
    let mut chars = format.chars().peekable();
    let mut arg_index = 0;

    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let Some(&next) = chars.peek() {
                match next {
                    '%' => {
                        chars.next();
                        literal.push('%');
                    }
                    's' | 'S' => {
                        chars.next();
                        if !literal.is_empty() {
                            tokens.push(FormatToken::Literal(literal.clone()));
                            literal.clear();
                        }
                        tokens.push(FormatToken::Spec {
                            spec: SqliteFormatSpec::String,
                            arg_index,
                        });
                        arg_index += 1;
                    }
                    'q' => {
                        chars.next();
                        if !literal.is_empty() {
                            tokens.push(FormatToken::Literal(literal.clone()));
                            literal.clear();
                        }
                        tokens.push(FormatToken::Spec {
                            spec: SqliteFormatSpec::SqliteQuote,
                            arg_index,
                        });
                        arg_index += 1;
                    }
                    'Q' | 'w' => {
                        chars.next();
                        if !literal.is_empty() {
                            tokens.push(FormatToken::Literal(literal.clone()));
                            literal.clear();
                        }
                        tokens.push(FormatToken::Spec {
                            spec: SqliteFormatSpec::SqliteIdentifier,
                            arg_index,
                        });
                        arg_index += 1;
                    }
                    _ => {
                        // Unknown specifier - treat as literal
                        literal.push('%');
                        chars.next();
                        literal.push(next);
                    }
                }
            }
        } else {
            literal.push(ch);
        }
    }

    if !literal.is_empty() {
        tokens.push(FormatToken::Literal(literal));
    }

    Ok(tokens)
}

/// Process a single argument based on its format specifier
fn process_arg(arg_str: &str, spec: SqliteFormatSpec) -> String {
    match spec {
        SqliteFormatSpec::String => arg_str.to_string(),
        SqliteFormatSpec::SqliteQuote => escape_string_content(arg_str),
        SqliteFormatSpec::SqliteIdentifier => sqlite_escape_string(arg_str),
    }
}

/// Apply tokenized format with runtime arguments
pub fn apply_tokens(tokens: &[FormatToken], args: &[&str]) -> String {
    let mut result = String::new();

    for token in tokens {
        match token {
            FormatToken::Literal(s) => result.push_str(s),
            FormatToken::Spec { spec, arg_index } => {
                if *arg_index < args.len() {
                    result.push_str(&process_arg(args[*arg_index], *spec));
                }
            }
        }
    }

    result
}
// Helper for dynamic format strings with 1 argument (for fts3_write migration)
pub unsafe fn format_sql_1arg(fmt: *const c_char, arg1: *const c_char) -> *mut c_char {
    if fmt.is_null() {
        return ::core::ptr::null_mut();
    }
    let fmt_str = match CStr::from_ptr(fmt).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };
    let arg1_str = if arg1.is_null() {
        String::new()
    } else {
        match CStr::from_ptr(arg1).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ::core::ptr::null_mut(),
        }
    };

    // Tokenize and apply (same logic as proc macro)
    let tokens = match tokenize_format(fmt_str) {
        Ok(t) => t,
        Err(_) => return ::core::ptr::null_mut(),
    };
    let result = apply_tokens(&tokens, &[&arg1_str]);
    allocate_string(result)
}

// Helper for dynamic format strings with 2 arguments (for fts3_write migration)
pub unsafe fn format_sql_2args(fmt: *const c_char, arg1: *const c_char, arg2: *const c_char) -> *mut c_char {
    if fmt.is_null() {
        return ::core::ptr::null_mut();
    }
    let fmt_str = match CStr::from_ptr(fmt).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };

    let arg1_str = if arg1.is_null() {
        String::new()
    } else {
        match CStr::from_ptr(arg1).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ::core::ptr::null_mut(),
        }
    };

    let arg2_str = if arg2.is_null() {
        String::new()
    } else {
        match CStr::from_ptr(arg2).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ::core::ptr::null_mut(),
        }
    };

    // Tokenize and apply (same logic as proc macro)
    let tokens = match tokenize_format(fmt_str) {
        Ok(t) => t,
        Err(_) => return ::core::ptr::null_mut(),
    };
    let result = apply_tokens(&tokens, &[&arg1_str, &arg2_str]);
    allocate_string(result)
}

// Helper for dynamic format strings with 3 arguments (for fts3_write migration)
pub unsafe fn format_sql_3args(fmt: *const c_char, arg1: *const c_char, arg2: *const c_char, arg3: *const c_char) -> *mut c_char {
    if fmt.is_null() {
        return ::core::ptr::null_mut();
    }
    let fmt_str = match CStr::from_ptr(fmt).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };

    let arg1_str = if arg1.is_null() {
        String::new()
    } else {
        match CStr::from_ptr(arg1).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ::core::ptr::null_mut(),
        }
    };

    let arg2_str = if arg2.is_null() {
        String::new()
    } else {
        match CStr::from_ptr(arg2).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ::core::ptr::null_mut(),
        }
    };

    let arg3_str = if arg3.is_null() {
        String::new()
    } else {
        match CStr::from_ptr(arg3).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ::core::ptr::null_mut(),
        }
    };

    // Tokenize and apply (same logic as proc macro)
    let tokens = match tokenize_format(fmt_str) {
        Ok(t) => t,
        Err(_) => return ::core::ptr::null_mut(),
    };
    let result = apply_tokens(&tokens, &[&arg1_str, &arg2_str, &arg3_str]);
    allocate_string(result)
}
