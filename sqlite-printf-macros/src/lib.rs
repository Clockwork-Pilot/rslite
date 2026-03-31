use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Lit, Token, Expr, Error};

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
fn parse_format_specs(format: &str) -> Result<Vec<FormatSpec>, String> {
    let mut specs = Vec::new();
    let mut chars = format.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let Some(&next) = chars.peek() {
                match next {
                    '%' => {
                        chars.next(); // consume second %
                        specs.push(FormatSpec::Percent);
                    }
                    's' | 'S' => {
                        chars.next();
                        specs.push(FormatSpec::String);
                    }
                    'd' | 'i' => {
                        chars.next();
                        specs.push(FormatSpec::Integer);
                    }
                    'u' => {
                        chars.next();
                        specs.push(FormatSpec::Unsigned);
                    }
                    'x' | 'X' => {
                        chars.next();
                        specs.push(FormatSpec::Hex);
                    }
                    'p' => {
                        chars.next();
                        specs.push(FormatSpec::Pointer);
                    }
                    'f' | 'e' | 'E' | 'g' | 'G' => {
                        chars.next();
                        specs.push(FormatSpec::Float);
                    }
                    'c' => {
                        chars.next();
                        specs.push(FormatSpec::Char);
                    }
                    'z' => {
                        chars.next();
                        specs.push(FormatSpec::ZeroCopy);
                    }
                    'q' => {
                        chars.next();
                        specs.push(FormatSpec::SqliteQuote);
                    }
                    'Q' => {
                        chars.next();
                        specs.push(FormatSpec::SqliteQuotedString);
                    }
                    'w' => {
                        chars.next();
                        specs.push(FormatSpec::SqliteIdentifier);
                    }
                    'l' => {
                        chars.next();
                        // Check for 'lld', 'lli', 'llu'
                        if let Some(&second) = chars.peek() {
                            if second == 'l' {
                                chars.next();
                                if let Some(&third) = chars.peek() {
                                    match third {
                                        'd' | 'i' => {
                                            chars.next();
                                            specs.push(FormatSpec::Long64);
                                        }
                                        'u' => {
                                            chars.next();
                                            specs.push(FormatSpec::ULong64);
                                        }
                                        _ => {
                                            return Err(format!("Unknown format specifier: %ll{}", third));
                                        }
                                    }
                                }
                            } else if second == 'd' || second == 'i' {
                                chars.next();
                                specs.push(FormatSpec::Long);
                            }
                        }
                    }
                    '!' => {
                        chars.next();
                        // Check for !0.15g (SQLite float formatting)
                        if let Some(&second) = chars.peek() {
                            if second == '0' {
                                chars.next();
                                if let Some(&third) = chars.peek() {
                                    if third == '.' {
                                        chars.next();
                                        // Consume digits after '.'
                                        while let Some(&d) = chars.peek() {
                                            if d.is_ascii_digit() {
                                                chars.next();
                                            } else {
                                                break;
                                            }
                                        }
                                        if let Some(&final_char) = chars.peek() {
                                            if final_char == 'g' || final_char == 'G' {
                                                chars.next();
                                                specs.push(FormatSpec::FloatG15);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    '*' => {
                        // Check for .*s (raw bytes with length)
                        chars.next();
                        if let Some(&next2) = chars.peek() {
                            if next2 == 's' || next2 == 'S' {
                                chars.next();
                                specs.push(FormatSpec::RawBytes);
                            }
                        }
                    }
                    _ => {
                        return Err(format!("Unknown format specifier: %{}", next));
                    }
                }
            }
        }
    }

    Ok(specs)
}

#[derive(Debug, Clone)]
enum FormatSpec {
    Percent,
    String,
    Integer,
    Unsigned,
    Hex,
    Pointer,
    Float,
    Char,
    ZeroCopy,
    SqliteQuote,
    SqliteQuotedString,
    SqliteIdentifier,
    Long,
    Long64,
    ULong64,
    FloatG15,      // %!0.15g for json_printf
    RawBytes,      // %.*s for json_printf (consumes 2 args: len, ptr)
}

impl FormatSpec {
    fn rust_format(&self) -> &'static str {
        match self {
            FormatSpec::Percent => "%",
            FormatSpec::String | FormatSpec::ZeroCopy => "{}",
            FormatSpec::Integer | FormatSpec::Long | FormatSpec::Long64 => "{}",
            FormatSpec::Unsigned | FormatSpec::ULong64 => "{}",
            FormatSpec::Hex => "{:x}",
            FormatSpec::Pointer => "{:p}",
            FormatSpec::Float => "{}",
            FormatSpec::Char => "{}",
            FormatSpec::SqliteQuote => "{}",
            FormatSpec::SqliteQuotedString => "{}",
            FormatSpec::SqliteIdentifier => "{}",
            FormatSpec::FloatG15 => "{}",  // Not used in sqlite_printf, only json_printf
            FormatSpec::RawBytes => "{}",  // Not used in sqlite_printf, only json_printf
        }
    }

    fn is_argument_consuming(&self) -> bool {
        !matches!(self, FormatSpec::Percent)
    }

    fn arg_count(&self) -> usize {
        match self {
            FormatSpec::RawBytes => 2, // %.*s consumes 2 args: length and pointer
            FormatSpec::Percent => 0,
            _ => if self.is_argument_consuming() { 1 } else { 0 },
        }
    }
}

/// Generate code for json_printf! — formats and appends to JsonString buffer
fn gen_json_format(args: &[Expr], specs: &[FormatSpec]) -> Result<proc_macro2::TokenStream, String> {
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
                if #arg.is_null() {
                    String::new()
                } else {
                    std::ffi::CStr::from_ptr(#arg)
                        .to_str()
                        .unwrap_or("")
                        .to_string()
                }
            }}
        }
        // Numeric types - pass through for format! to handle
        FormatSpec::Integer | FormatSpec::Long | FormatSpec::Long64 => quote! { #arg },
        FormatSpec::Unsigned | FormatSpec::ULong64 => quote! { #arg },
        FormatSpec::Hex => quote! { #arg },
        FormatSpec::Pointer => quote! { #arg },
        FormatSpec::Float => quote! { #arg },
        FormatSpec::Char => quote! { #arg as u8 as char },
        FormatSpec::SqliteQuote => {
            // %q: escape single quotes, no wrapping
            quote! {{
                if #arg.is_null() {
                    String::new()
                } else {
                    let s = std::ffi::CStr::from_ptr(#arg)
                        .to_str()
                        .unwrap_or("");
                    s.replace('\'', "''")
                }
            }}
        }
        FormatSpec::SqliteQuotedString => {
            // %Q: escape single quotes AND wrap in single quotes; NULL -> "NULL"
            quote! {{
                if #arg.is_null() {
                    "NULL".to_string()
                } else {
                    let s = std::ffi::CStr::from_ptr(#arg)
                        .to_str()
                        .unwrap_or("");
                    format!("'{}'", s.replace('\'', "''"))
                }
            }}
        }
        FormatSpec::SqliteIdentifier => {
            // %w: double-quote SQL identifier, escape double quotes
            quote! {{
                if #arg.is_null() {
                    String::new()
                } else {
                    let s = std::ffi::CStr::from_ptr(#arg)
                        .to_str()
                        .unwrap_or("");
                    format!("\"{}\"", s.replace('"', "\"\""))
                }
            }}
        }
        FormatSpec::Percent => quote! {},
        FormatSpec::FloatG15 | FormatSpec::RawBytes => {
            // These are only used in json_printf, not sqlite_printf
            quote! { compile_error!("FloatG15 and RawBytes specifiers only supported in json_printf!") }
        }
    }
}

/// Convert SQLite format string to Rust format string
fn convert_format_string(format: &str, specs: &[FormatSpec]) -> String {
    let mut result = String::new();
    let mut chars = format.chars().peekable();
    let mut spec_iter = specs.iter();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let Some(&next) = chars.peek() {
                if next == '%' {
                    chars.next();
                    // %% -> consume the Percent spec and output a single %
                    if let Some(spec) = spec_iter.next() {
                        // For %%, we output a literal % (the spec is FormatSpec::Percent)
                        if spec.is_argument_consuming() {
                            result.push_str(spec.rust_format());
                        } else {
                            // Not consuming, output as literal
                            result.push('%');
                        }
                    }
                    continue;
                }

                // Skip past the format specifier
                chars.next();
                let mut spec_chars = String::from(next);

                // Handle multi-character specifiers like %lld
                if next == 'l' {
                    if let Some(&second) = chars.peek() {
                        if second == 'l' {
                            spec_chars.push(second);
                            chars.next();
                            if let Some(&third) = chars.peek() {
                                spec_chars.push(third);
                                chars.next();
                            }
                        } else if second == 'd' || second == 'i' {
                            spec_chars.push(second);
                            chars.next();
                        }
                    }
                }

                // Add Rust format specifier
                if let Some(spec) = spec_iter.next() {
                    if spec.is_argument_consuming() {
                        result.push_str(spec.rust_format());
                    } else {
                        result.push('%');
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}

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
    let SqlitePrintf { format_str, args } =
        parse_macro_input!(input as SqlitePrintf);

    // Compile-time validation
    let specs = match parse_format_specs(&format_str) {
        Ok(s) => s,
        Err(e) => {
            return syn::Error::new_spanned(&format_str, e)
                .to_compile_error()
                .into();
        }
    };

    // Count consuming arguments
    let arg_count = specs.iter().filter(|s| s.is_argument_consuming()).count();

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

    // Generate argument handlers (original approach for now)
    let mut arg_handlers = Vec::new();
    let mut arg_iter = args.iter();

    for spec in &specs {
        if spec.is_argument_consuming() {
            if let Some(arg) = arg_iter.next() {
                // Use the native gen_arg_handler (no overhead, direct type passing)
                arg_handlers.push(gen_arg_handler(arg, spec));
            }
        }
    }

    // Generate final code using format! (simpler approach)
    let expanded = quote! {
        {
            let result = format!(#rust_format, #(#arg_handlers),*);
            crate::safe_format::allocate_string(result)
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
            return syn::Error::new_spanned(&format_lit, e)
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
                total_args_needed,
                provided_args
            ),
        )
        .to_compile_error()
        .into();
    }

    // Parse remaining args as expressions
    let mut args = Vec::new();
    for i in 2..parts.len() {
        match syn::parse_str::<Expr>(parts[i].trim()) {
            Ok(expr) => args.push(expr),
            Err(_) => {
                return syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to parse argument: {}", parts[i]),
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
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                e,
            )
            .to_compile_error()
            .into();
        }
    };

    let target_tokens: proc_macro2::TokenStream = target.parse()
        .unwrap_or_else(|_| quote! { p });

    let expanded = quote! {
        unsafe {
            let s = #format_code;
            let bytes = s.as_bytes();
            crate::src::src::json::jsonAppendRaw(
                #target_tokens,
                bytes.as_ptr() as *const ::core::ffi::c_char,
                bytes.len() as crate::src::ext::rtree::rtree::u32_0
            );
        }
    };

    TokenStream::from(expanded)
}
