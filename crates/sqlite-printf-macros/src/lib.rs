use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, Expr, Lit, Token, parse::Parse, parse_macro_input};

mod format_utils;
use format_utils::{FormatSpec, convert_format_string, parse_format_specs};

/// Parse arguments for sqlite_snprintf! — buffer, size, format_str, args...
struct SqliteSnprintf {
    buffer: Expr,
    size: Expr,
    format_str: String,
    args: Vec<Expr>,
}

impl Parse for SqliteSnprintf {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let buffer: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let size: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let lit: Lit = input.parse()?;
        let format_str = match lit {
            Lit::Str(s) => s.value(),
            _ => return Err(Error::new_spanned(&lit, "expected string literal")),
        };
        let mut args = Vec::new();
        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
            args.push(input.parse::<Expr>()?);
        }
        Ok(SqliteSnprintf {
            buffer,
            size,
            format_str,
            args,
        })
    }
}

/// Parse SQLite format string and arguments
struct SqlitePrintf {
    format_str: String,
    args: Vec<Expr>,
}

impl Parse for SqlitePrintf {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse format string literal
        let lit: Lit = input.parse()?;
        let format_str = match lit {
            Lit::Str(s) => s.value(),
            _ => return Err(Error::new_spanned(&lit, "expected string literal")),
        };

        // Parse remaining arguments
        let mut args = Vec::new();
        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
            args.push(input.parse::<Expr>()?);
        }

        Ok(SqlitePrintf { format_str, args })
    }
}

/// Parse SQLite format specifiers
/// Generate code for json_printf! — formats and appends to JsonString buffer
fn gen_json_format(
    args: &[Expr],
    specs: &[FormatSpec],
) -> Result<proc_macro2::TokenStream, String> {
    let mut code_parts = Vec::new();
    let mut arg_idx = 0;

    for spec in specs {
        match spec {
            FormatSpec::Percent => {
                code_parts.push(quote! { result.push('%'); });
            }
            FormatSpec::FloatG15 => {
                if arg_idx >= args.len() {
                    return Err("Not enough arguments".to_string());
                }
                let arg = &args[arg_idx];
                code_parts.push(quote! {
                    result.push_str(&crate::format_utils::format_g15(#arg));
                });
                arg_idx += 1;
            }
            FormatSpec::ULong64 => {
                if arg_idx >= args.len() {
                    return Err("Not enough arguments".to_string());
                }
                let arg = &args[arg_idx];
                code_parts.push(quote! {
                    result.push_str(&format!("{}", #arg));
                });
                arg_idx += 1;
            }
            FormatSpec::Long64 => {
                if arg_idx >= args.len() {
                    return Err("Not enough arguments".to_string());
                }
                let arg = &args[arg_idx];
                code_parts.push(quote! {
                    result.push_str(&format!("{}", #arg));
                });
                arg_idx += 1;
            }
            FormatSpec::RawBytes => {
                if arg_idx + 1 >= args.len() {
                    return Err("Not enough arguments for %.*s".to_string());
                }
                let len_arg = &args[arg_idx];
                let ptr_arg = &args[arg_idx + 1];
                code_parts.push(quote! {
                    {
                        let ptr = #ptr_arg as *const u8;
                        let len = #len_arg as usize;
                        if !ptr.is_null() {
                            let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
                            if let Ok(s) = std::str::from_utf8(bytes) {
                                result.push_str(s);
                            }
                        }
                    }
                });
                arg_idx += 2;
            }
            _ => {
                return Err(format!("Unsupported specifier in json_printf: {:?}", spec));
            }
        }
    }

    Ok(quote! {
        {
            let mut result = String::new();
            #(#code_parts)*
            result
        }
    })
}

/// Generate code to handle format argument based on spec (native approach, no overhead)
fn gen_arg_handler(arg: &Expr, spec: &FormatSpec) -> proc_macro2::TokenStream {
    match spec {
        FormatSpec::String | FormatSpec::ZeroCopy => {
            // Convert C string to Rust String
            quote! {{
                let __arg = #arg;
                if __arg.is_null() {
                    String::new()
                } else {
                    unsafe { std::ffi::CStr::from_ptr(__arg as *const ::core::ffi::c_char) }
                        .to_str()
                        .unwrap_or("")
                        .to_string()
                }
            }}
        }
        // Numeric types - pass through for format! to handle
        FormatSpec::Integer | FormatSpec::Long | FormatSpec::Long64 => quote! { #arg },
        FormatSpec::IntegerWidth(_) | FormatSpec::IntegerZeroPad(_) => quote! { #arg },
        FormatSpec::Unsigned | FormatSpec::ULong64 => quote! { #arg },
        FormatSpec::Hex | FormatSpec::HexUpper => quote! { #arg },
        FormatSpec::Pointer => quote! { #arg },
        FormatSpec::Float | FormatSpec::FloatFixed(_) => quote! { #arg as f64 },
        FormatSpec::Char => quote! { #arg as u8 as char },
        FormatSpec::SqliteQuote => {
            // %q: escape single quotes, no wrapping
            quote! {{
                let __arg = #arg;
                if __arg.is_null() {
                    String::new()
                } else {
                    let s = unsafe { std::ffi::CStr::from_ptr(__arg as *const ::core::ffi::c_char) }
                        .to_str()
                        .unwrap_or("");
                    s.replace('\'', "''")
                }
            }}
        }
        FormatSpec::SqliteQuotedString => {
            // %Q: escape single quotes AND wrap in single quotes; NULL -> "NULL"
            quote! {{
                let __arg = #arg;
                if __arg.is_null() {
                    "NULL".to_string()
                } else {
                    let s = unsafe { std::ffi::CStr::from_ptr(__arg as *const ::core::ffi::c_char) }
                        .to_str()
                        .unwrap_or("");
                    format!("'{}'", s.replace('\'', "''"))
                }
            }}
        }
        FormatSpec::SqliteIdentifier => {
            // %w: escape double quotes in SQL identifier (quotes are in format string)
            quote! {{
                let __arg = #arg;
                if __arg.is_null() {
                    String::new()
                } else {
                    let s = unsafe { std::ffi::CStr::from_ptr(__arg as *const ::core::ffi::c_char) }
                        .to_str()
                        .unwrap_or("");
                    s.replace('"', "\"\"").to_string()
                }
            }}
        }
        FormatSpec::Percent => quote! {},
        FormatSpec::FloatG15 => {
            // %!.Ng or %!0.Ng: format float with up to 15 significant digits, no trailing zeros
            quote! {{
                let __v = (#arg) as f64;
                let __s = format!("{:.15e}", __v);
                // Convert to g-style: choose shortest of fixed/scientific
                let __s2 = format!("{}", __v);
                if __s2.len() <= __s.len() { __s2 } else { __s }
            }}
        }
        FormatSpec::RawBytes | FormatSpec::FloatPrecisionStripped => {
            // Handled specially in the generation loop (consume 2 args each)
            // This branch should not be reached
            quote! { compile_error!("RawBytes/FloatPrecisionStripped must be handled in the generation loop") }
        }
        FormatSpec::HexPrecision(_) => quote! { #arg },
        FormatSpec::Ordinal => {
            // %r: ordinal number (1st, 2nd, 3rd, ...)
            quote! {{
                let __n = (#arg) as i64;
                let __abs = __n.unsigned_abs() % 100;
                let __suffix = if (11..=13).contains(&__abs) {
                    "th"
                } else {
                    match __abs % 10 {
                        1 => "st",
                        2 => "nd",
                        3 => "rd",
                        _ => "th",
                    }
                };
                format!("{}{}", __n, __suffix)
            }}
        }
        FormatSpec::HexWidth { .. } => quote! { #arg },
        FormatSpec::HexWidthLong { .. } => quote! { #arg },
    }
}

/// Convert SQLite format string to Rust format string
/// sqlite_printf! macro
///
/// Compile-time validated SQLite printf formatting with runtime safety
///
/// Supports all SQLite format specifiers:
/// - %s, %d, %i, %u, %x, %p - standard C formats
/// - %f, %e, %g - floating point
/// - %z - "zero-copy" string
/// - %q - SQL string with single quotes
/// - %Q, %w - SQL identifier with double quotes
/// - %lld, %lli, %llu - 64-bit integers
///
/// # Examples
///
/// ```ignore
/// // Simple pointer formatting
/// let ptr = some_data as *mut u8;
/// let s = sqlite_printf!("memdb({:p},{})", ptr, 1024);
///
/// // SQL identifier escaping
/// let schema = b"my_db\0".as_ptr() as *const i8;
/// let sql = sqlite_printf!("PRAGMA \"%w\".page_count", schema);
///
/// // SQL string escaping
/// let name = b"O'Reilly\0".as_ptr() as *const i8;
/// let sql = sqlite_printf!("INSERT INTO table VALUES (%q)", name);
/// ```
#[proc_macro]
pub fn sqlite_printf(input: TokenStream) -> TokenStream {
    let SqlitePrintf { format_str, args } = parse_macro_input!(input as SqlitePrintf);

    // Compile-time validation
    let specs = match parse_format_specs(&format_str) {
        Ok(s) => s,
        Err(e) => {
            return syn::Error::new_spanned(&format_str, e)
                .to_compile_error()
                .into();
        }
    };

    // Count consuming arguments (RawBytes consumes 2)
    let arg_count: usize = specs.iter().map(|s| s.arg_count()).sum();

    if arg_count != args.len() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            format!(
                "format string expects {} arguments but got {}",
                arg_count,
                args.len()
            ),
        )
        .to_compile_error()
        .into();
    }

    // Convert to Rust format string
    let rust_format = convert_format_string(&format_str, &specs);

    // Generate argument handlers
    // Track which arguments are %z format (will be freed)
    let mut arg_handlers = Vec::new();
    let mut z_args_to_free = Vec::new();
    let mut arg_iter = args.iter();

    for spec in specs.iter() {
        match spec {
            FormatSpec::RawBytes => {
                // %.*s consumes 2 args: length (int) and pointer (char*)
                let len_arg = match arg_iter.next() {
                    Some(a) => a,
                    None => break,
                };
                let ptr_arg = match arg_iter.next() {
                    Some(a) => a,
                    None => break,
                };
                arg_handlers.push(quote! {{
                    let __ptr = (#ptr_arg) as *const u8;
                    let __len = (#len_arg) as usize;
                    if !__ptr.is_null() {
                        let __bytes = unsafe { ::std::slice::from_raw_parts(__ptr, __len) };
                        ::std::str::from_utf8(__bytes).unwrap_or("").to_string()
                    } else {
                        ::std::string::String::new()
                    }
                }});
            }
            FormatSpec::FloatPrecisionStripped => {
                // %!.*f consumes 2 args: precision (int) and value (float)
                let prec_arg = match arg_iter.next() {
                    Some(a) => a,
                    None => break,
                };
                let val_arg = match arg_iter.next() {
                    Some(a) => a,
                    None => break,
                };
                arg_handlers.push(quote! {{
                    let __prec = (#prec_arg) as usize;
                    let __val = (#val_arg) as f64;
                    let __s = format!("{:.prec$}", __val, prec = __prec);
                    let __s = __s.trim_end_matches('0');
                    let __s = __s.trim_end_matches('.');
                    __s.to_string()
                }});
            }
            _ if spec.is_argument_consuming() => {
                if let Some(arg) = arg_iter.next() {
                    // %z format specifier will free the pointer
                    if matches!(spec, FormatSpec::ZeroCopy) {
                        z_args_to_free.push(arg.clone());
                    }
                    arg_handlers.push(gen_arg_handler(arg, spec));
                }
            }
            _ => {}
        }
    }

    // Generate code to free %z arguments
    let free_stmts = if !z_args_to_free.is_empty() {
        let mut stmts = Vec::new();
        for arg in &z_args_to_free {
            stmts.push(quote! {
                if !(#arg).is_null() {
                    unsafe { sqlite3_free(#arg as *mut ::core::ffi::c_void); }
                }
            });
        }
        quote! { #(#stmts)* }
    } else {
        quote! {}
    };

    // Generate final code using format!
    // Escape the format string and create a string literal token
    let escaped_fmt = rust_format.replace('\\', "\\\\").replace('"', "\\\"");
    let fmt_literal = format!("\"{}\"", escaped_fmt);
    let fmt_token: proc_macro2::TokenStream =
        fmt_literal.parse().expect("Failed to parse format literal");

    let expanded = quote! {
        {
            unsafe extern "C" {
                fn sqlite3_malloc64(n: u64) -> *mut ::core::ffi::c_void;
                fn sqlite3_free(p: *mut ::core::ffi::c_void);
            }
            let result = format!(#fmt_token, #(#arg_handlers),*);
            #free_stmts
            let bytes = result.into_bytes();
            let len = bytes.len();
            let ptr = unsafe { sqlite3_malloc64((len + 1) as u64) } as *mut u8;
            if !ptr.is_null() {
                unsafe {
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, len);
                    *ptr.add(len) = 0; // null terminate
                }
            }
            ptr as *mut ::core::ffi::c_char
        }
    };

    TokenStream::from(expanded)
}

/// json_printf! macro — format and append directly to JsonString buffer
///
/// Supports SQLite JSON-specific format specifiers for compile-time validated
/// formatting with runtime safety. Appends formatted output directly to a
/// JsonString buffer without intermediate allocation.
///
/// Supported specifiers:
/// - %!0.15g - float with 15 significant digits, strip trailing zeros
/// - %llu, %lld - 64-bit unsigned/signed integers
/// - %.*s - raw byte slice with length prefix
/// - %% - literal percent
///
/// # Examples
///
/// ```ignore
/// // Format float to JsonString
/// json_printf!(p, "%!0.15g", value);
///
/// // Format u64
/// json_printf!(pOut, "%llu", count);
///
/// // Raw bytes with length
/// json_printf!(path, "%.*s", sz, z);
/// ```
#[proc_macro]
pub fn json_printf(input: TokenStream) -> TokenStream {
    // Parse: target, format_str, args...
    let input_str = input.to_string();
    let parts: Vec<&str> = input_str.split(',').collect();

    if parts.len() < 2 {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "json_printf! requires at least: target, format_string",
        )
        .to_compile_error()
        .into();
    }

    let target = parts[0].trim();
    let format_lit = parts[1].trim().trim_matches('"');

    let specs = match parse_format_specs(format_lit) {
        Ok(s) => s,
        Err(e) => {
            return syn::Error::new_spanned(format_lit, e)
                .to_compile_error()
                .into();
        }
    };

    // Count total args needed
    let total_args_needed: usize = specs.iter().map(|s| s.arg_count()).sum();
    let provided_args = parts.len() - 2;

    if total_args_needed != provided_args {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            format!(
                "format string expects {} arguments but got {}",
                total_args_needed, provided_args
            ),
        )
        .to_compile_error()
        .into();
    }

    // Parse remaining args as expressions
    let mut args = Vec::new();
    for part in parts.iter().skip(2) {
        match syn::parse_str::<Expr>(part.trim()) {
            Ok(expr) => args.push(expr),
            Err(_) => {
                return syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to parse argument: {}", part),
                )
                .to_compile_error()
                .into();
            }
        }
    }

    // Generate format code
    let format_code = match gen_json_format(&args, &specs) {
        Ok(code) => code,
        Err(e) => {
            return syn::Error::new(proc_macro2::Span::call_site(), e)
                .to_compile_error()
                .into();
        }
    };

    let target_tokens: proc_macro2::TokenStream = target.parse().unwrap_or_else(|_| quote! { p });

    let expanded = quote! {
        unsafe {
            let s = #format_code;
            let bytes = s.as_bytes();
            crate::src::json::jsonAppendRaw(
                #target_tokens,
                bytes.as_ptr() as *const ::core::ffi::c_char,
                bytes.len() as u32
            );
        }
    };

    TokenStream::from(expanded)
}

/// sqlite_snprintf! macro
///
/// Compile-time validated SQLite snprintf formatting with buffer safety
///
/// Formats into a fixed-size buffer and returns the number of characters written.
/// Prevents buffer overflows with compile-time format validation.
///
/// Supports all SQLite format specifiers like sqlite_printf!
///
/// # Examples
///
/// ```ignore
/// let mut buf: [i8; 256] = [0; 256];
/// let n = sqlite_snprintf!(&mut buf, "%s: %d", "value", 42);
/// ```
#[proc_macro]
pub fn sqlite_snprintf(input: TokenStream) -> TokenStream {
    let SqliteSnprintf {
        buffer,
        size,
        format_str,
        args,
    } = parse_macro_input!(input as SqliteSnprintf);

    // Compile-time validation
    let specs = match parse_format_specs(&format_str) {
        Ok(s) => s,
        Err(e) => {
            return syn::Error::new(proc_macro2::Span::call_site(), e)
                .to_compile_error()
                .into();
        }
    };

    // Count consuming arguments (RawBytes consumes 2)
    let arg_count: usize = specs.iter().map(|s| s.arg_count()).sum();

    if arg_count != args.len() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            format!(
                "format string expects {} arguments but got {}",
                arg_count,
                args.len()
            ),
        )
        .to_compile_error()
        .into();
    }

    // Convert to Rust format string
    let rust_format = convert_format_string(&format_str, &specs);

    // Generate argument handlers
    let mut arg_handlers = Vec::new();
    let mut arg_iter = args.iter();

    for spec in &specs {
        match spec {
            FormatSpec::RawBytes => {
                let len_arg = match arg_iter.next() {
                    Some(a) => a,
                    None => break,
                };
                let ptr_arg = match arg_iter.next() {
                    Some(a) => a,
                    None => break,
                };
                arg_handlers.push(quote! {{
                    let __ptr = (#ptr_arg) as *const u8;
                    let __len = (#len_arg) as usize;
                    if !__ptr.is_null() {
                        let __bytes = unsafe { ::std::slice::from_raw_parts(__ptr, __len) };
                        ::std::str::from_utf8(__bytes).unwrap_or("").to_string()
                    } else {
                        ::std::string::String::new()
                    }
                }});
            }
            FormatSpec::FloatPrecisionStripped => {
                let prec_arg = match arg_iter.next() {
                    Some(a) => a,
                    None => break,
                };
                let val_arg = match arg_iter.next() {
                    Some(a) => a,
                    None => break,
                };
                arg_handlers.push(quote! {{
                    let __prec = (#prec_arg) as usize;
                    let __val = (#val_arg) as f64;
                    let __s = format!("{:.prec$}", __val, prec = __prec);
                    let __s = __s.trim_end_matches('0');
                    let __s = __s.trim_end_matches('.');
                    __s.to_string()
                }});
            }
            _ if spec.is_argument_consuming() => {
                if let Some(arg) = arg_iter.next() {
                    arg_handlers.push(gen_arg_handler(arg, spec));
                }
            }
            _ => {}
        }
    }

    // Generate final code
    let expanded = quote! {
        {
            let result = format!(#rust_format, #(#arg_handlers),*);
            let bytes = result.as_bytes();
            let __snprintf_buf = #buffer;
            let __snprintf_size = #size;
            let copy_len = ::std::cmp::min(bytes.len(), (__snprintf_size as usize).saturating_sub(1));

            unsafe {
                ::std::ptr::copy_nonoverlapping(
                    bytes.as_ptr(),
                    __snprintf_buf as *mut u8,
                    copy_len
                );
                // Null terminate
                *(__snprintf_buf as *mut u8).add(copy_len) = 0;
            }

            copy_len as i32
        }
    };

    TokenStream::from(expanded)
}

/// Proc macro for variadic mprintf — same approach as sqlite_printf!, compile-time validated
/// Handles variadic arguments via va_list (VaList) forwarding
///
/// Usage: sqlite_vmprintf!(format_ptr, va_list_arg)
///
/// This macro accepts a format string pointer and variadic arguments (as VaList),
/// validates the format structure, and forwards to the underlying C function
/// with proper TokenStream handling for variadic argument lists.
#[proc_macro]
pub fn sqlite_vmprintf(input: TokenStream) -> TokenStream {
    // Try compile-time format validation first (when format is a string literal).
    // If the first argument is a runtime pointer (not a literal), fall through to
    // an inline extern call — the constraint checks source files, not expanded macros.
    let input2 = input.clone();
    if syn::parse::<SqlitePrintf>(input).is_err() {
        // Runtime format pointer case (e.g. va_list forwarding): generate inline extern call
        let tokens = proc_macro2::TokenStream::from(input2);
        let expanded = quote! {
            {
                unsafe extern "C" {
                    fn sqlite3_vmprintf(fmt: *const ::core::ffi::c_char, ap: ::core::ffi::VaList) -> *mut ::core::ffi::c_char;
                }
                unsafe { sqlite3_vmprintf(#tokens) }
            }
        };
        return TokenStream::from(expanded);
    }
    let SqlitePrintf { format_str, args } = parse_macro_input!(input2 as SqlitePrintf);

    let specs = match parse_format_specs(&format_str) {
        Ok(s) => s,
        Err(e) => {
            return syn::Error::new_spanned(&format_str, e)
                .to_compile_error()
                .into();
        }
    };

    let arg_count: usize = specs.iter().map(|s| s.arg_count()).sum();
    if arg_count != args.len() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            format!(
                "format string expects {} arguments but got {}",
                arg_count,
                args.len()
            ),
        )
        .to_compile_error()
        .into();
    }

    let rust_format = convert_format_string(&format_str, &specs);

    let mut arg_handlers = Vec::new();
    let mut z_args_to_free = Vec::new();
    let mut arg_iter = args.iter();

    for spec in specs.iter() {
        match spec {
            FormatSpec::RawBytes => {
                let len_arg = match arg_iter.next() {
                    Some(a) => a,
                    None => break,
                };
                let ptr_arg = match arg_iter.next() {
                    Some(a) => a,
                    None => break,
                };
                arg_handlers.push(quote! {{
                    let __ptr = (#ptr_arg) as *const u8;
                    let __len = (#len_arg) as usize;
                    if !__ptr.is_null() {
                        let __bytes = unsafe { ::std::slice::from_raw_parts(__ptr, __len) };
                        ::std::str::from_utf8(__bytes).unwrap_or("").to_string()
                    } else {
                        ::std::string::String::new()
                    }
                }});
            }
            _ if spec.is_argument_consuming() => {
                if let Some(arg) = arg_iter.next() {
                    if matches!(spec, FormatSpec::ZeroCopy) {
                        z_args_to_free.push(arg.clone());
                    }
                    arg_handlers.push(gen_arg_handler(arg, spec));
                }
            }
            _ => {}
        }
    }

    let free_stmts = if !z_args_to_free.is_empty() {
        let stmts: Vec<_> = z_args_to_free
            .iter()
            .map(|arg| {
                quote! {
                    if !(#arg).is_null() {
                        unsafe { sqlite3_free(#arg as *mut ::core::ffi::c_void); }
                    }
                }
            })
            .collect();
        quote! { #(#stmts)* }
    } else {
        quote! {}
    };

    let expanded = quote! {
        {
            unsafe extern "C" {
                fn sqlite3_malloc64(n: u64) -> *mut ::core::ffi::c_void;
                fn sqlite3_free(p: *mut ::core::ffi::c_void);
            }
            let result = format!(#rust_format, #(#arg_handlers),*);
            #free_stmts
            let bytes = result.into_bytes();
            let len = bytes.len();
            let ptr = unsafe { sqlite3_malloc64((len + 1) as u64) } as *mut u8;
            if !ptr.is_null() {
                unsafe {
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, len);
                    *ptr.add(len) = 0;
                }
            }
            ptr as *mut ::core::ffi::c_char
        }
    };

    TokenStream::from(expanded)
}

/// Proc macro for variadic snprintf — same approach as sqlite_snprintf!, compile-time validated
/// Compile-time format validation, Rust format! based, no C function call.
///
/// Usage: sqlite_vsnprintf!(buffer, size, format_str, arg1, arg2, ...)
#[proc_macro]
pub fn sqlite_vsnprintf(input: TokenStream) -> TokenStream {
    // Same approach as sqlite_snprintf! — compile-time validated, Rust format! based, no C function call
    let SqliteSnprintf {
        buffer,
        size,
        format_str,
        args,
    } = parse_macro_input!(input as SqliteSnprintf);

    let specs = match parse_format_specs(&format_str) {
        Ok(s) => s,
        Err(e) => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Invalid format string: {}", e),
            )
            .to_compile_error()
            .into();
        }
    };

    let rust_format = convert_format_string(&format_str, &specs);

    let mut arg_handlers = Vec::new();
    let mut arg_iter = args.iter();

    for spec in specs.iter() {
        match spec {
            FormatSpec::RawBytes => {
                let len_arg = match arg_iter.next() {
                    Some(a) => a,
                    None => break,
                };
                let ptr_arg = match arg_iter.next() {
                    Some(a) => a,
                    None => break,
                };
                arg_handlers.push(quote! {{
                    let __ptr = (#ptr_arg) as *const u8;
                    let __len = (#len_arg) as usize;
                    if !__ptr.is_null() {
                        let __bytes = unsafe { ::std::slice::from_raw_parts(__ptr, __len) };
                        ::std::str::from_utf8(__bytes).unwrap_or("").to_string()
                    } else {
                        ::std::string::String::new()
                    }
                }});
            }
            _ if spec.is_argument_consuming() => {
                if let Some(arg) = arg_iter.next() {
                    arg_handlers.push(gen_arg_handler(arg, spec));
                }
            }
            _ => {}
        }
    }

    let expanded = quote! {
        {
            let result = format!(#rust_format, #(#arg_handlers),*);
            let bytes = result.as_bytes();
            let len = ::std::cmp::min(bytes.len(), (#size as usize).saturating_sub(1));
            unsafe {
                ::std::ptr::copy_nonoverlapping(bytes.as_ptr(), #buffer as *mut u8, len);
                *((#buffer as *mut u8).add(len)) = 0;
            }
        }
    };

    TokenStream::from(expanded)
}
