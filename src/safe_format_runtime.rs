/// Runtime token-based formatting for SQLite strings
/// Feature-gated implementation for dynamic format string processing
///
/// This module provides runtime validation and formatting of SQLite printf-style
/// format strings using a token-based approach. This is kept separate from the
/// compile-time proc macro approach to maintain clear separation of concerns.

extern crate alloc;

use ::core::ffi::c_char;
use alloc::string::String;
use alloc::vec::Vec;

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

/// Process a single argument based on its format specifier
fn process_arg(arg_str: &str, spec: SqliteFormatSpec) -> String {
    match spec {
        SqliteFormatSpec::String => arg_str.to_string(),
        SqliteFormatSpec::SqliteQuote => escape_string_content(arg_str),
        SqliteFormatSpec::SqliteIdentifier => escape_identifier_content(arg_str),
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

// Runtime helpers for dynamic format strings

/// Helper for dynamic format strings with 1 argument (for fts3_write migration)
pub unsafe fn format_sql_1arg(fmt: *const c_char, arg1: *const c_char) -> *mut c_char {
    if fmt.is_null() {
        return ::core::ptr::null_mut();
    }
    let fmt_str = match ::core::ffi::CStr::from_ptr(fmt).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };
    let arg1_str = if arg1.is_null() {
        String::new()
    } else {
        match ::core::ffi::CStr::from_ptr(arg1).to_str() {
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
    crate::safe_format::allocate_string(result)
}

/// Helper for dynamic format strings with 2 arguments (for fts3_write migration)
pub unsafe fn format_sql_2args(fmt: *const c_char, arg1: *const c_char, arg2: *const c_char) -> *mut c_char {
    if fmt.is_null() {
        return ::core::ptr::null_mut();
    }
    let fmt_str = match ::core::ffi::CStr::from_ptr(fmt).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };

    let arg1_str = if arg1.is_null() {
        String::new()
    } else {
        match ::core::ffi::CStr::from_ptr(arg1).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ::core::ptr::null_mut(),
        }
    };

    let arg2_str = if arg2.is_null() {
        String::new()
    } else {
        match ::core::ffi::CStr::from_ptr(arg2).to_str() {
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
    crate::safe_format::allocate_string(result)
}

/// Helper for dynamic format strings with 3 arguments (for fts3_write migration)
pub unsafe fn format_sql_3args(fmt: *const c_char, arg1: *const c_char, arg2: *const c_char, arg3: *const c_char) -> *mut c_char {
    if fmt.is_null() {
        return ::core::ptr::null_mut();
    }
    let fmt_str = match ::core::ffi::CStr::from_ptr(fmt).to_str() {
        Ok(s) => s,
        Err(_) => return ::core::ptr::null_mut(),
    };

    let arg1_str = if arg1.is_null() {
        String::new()
    } else {
        match ::core::ffi::CStr::from_ptr(arg1).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ::core::ptr::null_mut(),
        }
    };

    let arg2_str = if arg2.is_null() {
        String::new()
    } else {
        match ::core::ffi::CStr::from_ptr(arg2).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ::core::ptr::null_mut(),
        }
    };

    let arg3_str = if arg3.is_null() {
        String::new()
    } else {
        match ::core::ffi::CStr::from_ptr(arg3).to_str() {
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
    crate::safe_format::allocate_string(result)
}
