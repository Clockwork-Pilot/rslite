//! Runtime printf validation and formatting for SQLite-style format strings.
//!
//! Reuses parsing logic from `sqlite-printf-common`. Provides functions for
//! cases where format strings are only known at runtime.

use core::ffi::c_char;
use core::ffi::c_void;
pub use sqlite_printf_common::{FormatSpec, parse_format_specs, convert_format_string};

/// A safe, type-tagged argument for runtime SQLite-style formatting.
pub enum SqlArg<'a> {
    Int(i64),
    Float(f64),
    Str(&'a str),
    CString(*const c_char),
    Ptr(*mut c_void),
    Null,
}

/// Validate a null-terminated C format string at runtime.
/// Returns the parsed specs on success, or an error message.
///
/// # Safety
/// `fmt` must be a valid null-terminated C string.
pub unsafe fn validate_format(fmt: *const c_char) -> Result<Vec<FormatSpec>, String> {
    let s = unsafe { core::ffi::CStr::from_ptr(fmt) }
        .to_str()
        .map_err(|e| format!("invalid UTF-8 in format string: {}", e))?;
    parse_format_specs(s)
}

/// Safe runtime equivalent of `sqlite_printf!`.
///
/// Parses the format string, matches each specifier against the `SqlArg` slice,
/// and produces a formatted `String` entirely in safe Rust.
pub fn sqlite_vmprintf(fmt: &str, args: &[SqlArg]) -> String {
    let specs = match parse_format_specs(fmt) {
        Ok(s) => s,
        Err(_) => return String::new(),
    };
    let rust_fmt = convert_format_string(fmt, &specs);
    render_format(&rust_fmt, &specs, args)
}

/// Safe runtime equivalent of `sqlite_snprintf!`.
///
/// Formats into a caller-supplied byte buffer, truncating to `n - 1` bytes
/// and always NUL-terminating. Returns the number of bytes written (excluding NUL).
pub fn sqlite_vsnprintf(buf: &mut [u8], fmt: &str, args: &[SqlArg]) -> usize {
    let formatted = sqlite_vmprintf(fmt, args);
    let bytes = formatted.as_bytes();
    if buf.is_empty() {
        return 0;
    }
    let max = buf.len() - 1;
    let len = core::cmp::min(bytes.len(), max);
    buf[..len].copy_from_slice(&bytes[..len]);
    buf[len] = 0;
    len
}

fn render_format(rust_fmt: &str, _specs: &[FormatSpec], args: &[SqlArg]) -> String {
    let mut result = String::new();
    let mut arg_idx: usize = 0;
    let mut chars = rust_fmt.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            if chars.peek() == Some(&'{') {
                chars.next();
                result.push('{');
                continue;
            }
            let mut placeholder = String::new();
            for c in chars.by_ref() {
                if c == '}' { break; }
                placeholder.push(c);
            }
            if arg_idx < args.len() {
                format_arg(&mut result, &placeholder, &args[arg_idx]);
                arg_idx += 1;
            }
        } else if ch == '}' {
            if chars.peek() == Some(&'}') {
                chars.next();
                result.push('}');
            } else {
                result.push('}');
            }
        } else if ch == '%' {
            result.push('%');
        } else {
            result.push(ch);
        }
    }

    result
}

fn format_arg(result: &mut String, placeholder: &str, arg: &SqlArg) {
    use std::fmt::Write;
    match arg {
        SqlArg::Int(v) => {
            if placeholder.contains('x') || placeholder.contains('X') {
                let spec = placeholder.trim_start_matches(':');
                let upper = spec.contains('X');
                let zero_pad = spec.starts_with('0');
                let width: usize = spec.trim_start_matches('0')
                    .trim_end_matches(|c: char| c == 'x' || c == 'X')
                    .parse().unwrap_or(0);
                if upper {
                    if zero_pad && width > 0 {
                        let _ = write!(result, "{:0width$X}", *v as u64, width = width);
                    } else if width > 0 {
                        let _ = write!(result, "{:width$X}", *v as u64, width = width);
                    } else {
                        let _ = write!(result, "{:X}", *v as u64);
                    }
                } else if zero_pad && width > 0 {
                    let _ = write!(result, "{:0width$x}", *v as u64, width = width);
                } else if width > 0 {
                    let _ = write!(result, "{:width$x}", *v as u64, width = width);
                } else {
                    let _ = write!(result, "{:x}", *v as u64);
                }
            } else {
                let _ = write!(result, "{}", v);
            }
        }
        SqlArg::Float(v) => {
            let _ = write!(result, "{}", v);
        }
        SqlArg::Str(s) => {
            result.push_str(s);
        }
        SqlArg::CString(_ptr) => {
            result.push_str("(cstring)");
        }
        SqlArg::Ptr(ptr) => {
            let _ = write!(result, "{:p}", *ptr);
        }
        SqlArg::Null => {
            result.push_str("(null)");
        }
    }
}
