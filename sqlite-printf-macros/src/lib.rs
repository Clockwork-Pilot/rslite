use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Lit, Token, Expr, Error};

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
        Ok(SqliteSnprintf { buffer, size, format_str, args })
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
                                        'x' | 'X' => {
                                            chars.next();
                                            let upper = third == 'X';
                                            specs.push(FormatSpec::HexWidthLong { width: 0, zero_pad: false, upper });
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
                        if let Some(&second) = chars.peek() {
                            if second == '0' {
                                // Check for %!0.15g (SQLite float formatting for json_printf)
                                chars.next();
                                if let Some(&third) = chars.peek() {
                                    if third == '.' {
                                        chars.next();
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
                            } else if second == '.' {
                                // Check for %!.*f (float with dynamic precision, strip trailing zeros)
                                chars.next();
                                if let Some(&third) = chars.peek() {
                                    if third == '*' {
                                        chars.next();
                                        if let Some(&fourth) = chars.peek() {
                                            if matches!(fourth, 'f' | 'F') {
                                                chars.next();
                                                specs.push(FormatSpec::FloatPrecisionStripped);
                                            } else {
                                                return Err(format!("Unknown format specifier: %!.*{}", fourth));
                                            }
                                        }
                                    } else {
                                        return Err(format!("Unknown format specifier: %!.{}", third));
                                    }
                                }
                            } else {
                                return Err(format!("Unknown format specifier: %!{}", second));
                            }
                        }
                    }
                    '.' => {
                        // Check for %.*s (raw bytes with length prefix) or %.Nx (precision hex)
                        chars.next();
                        if let Some(&next2) = chars.peek() {
                            if next2 == '*' {
                                chars.next();
                                if let Some(&next3) = chars.peek() {
                                    if next3 == 's' || next3 == 'S' {
                                        chars.next();
                                        specs.push(FormatSpec::RawBytes);
                                    } else {
                                        return Err(format!("Unknown format specifier: %.*{}", next3));
                                    }
                                } else {
                                    return Err("Incomplete format specifier: %.*".to_string());
                                }
                            } else if next2.is_ascii_digit() {
                                // Parse numeric precision, e.g. %.3x or %.6x
                                let mut prec_str = String::new();
                                while let Some(&d) = chars.peek() {
                                    if d.is_ascii_digit() {
                                        prec_str.push(d);
                                        chars.next();
                                    } else {
                                        break;
                                    }
                                }
                                let prec: usize = prec_str.parse().unwrap_or(0);
                                if let Some(&type_ch) = chars.peek() {
                                    match type_ch {
                                        'x' | 'X' => {
                                            chars.next();
                                            specs.push(FormatSpec::HexPrecision(prec));
                                        }
                                        _ => {
                                            return Err(format!("Unknown format specifier: %.{}{}", prec_str, type_ch));
                                        }
                                    }
                                } else {
                                    return Err(format!("Incomplete format specifier: %.{}", prec_str));
                                }
                            } else {
                                return Err(format!("Unknown format specifier: %.{}", next2));
                            }
                        }
                    }
                    'r' => {
                        chars.next();
                        specs.push(FormatSpec::Ordinal);
                    }
                    '-' | '+' | ' ' | '#' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        // Parse optional flags then width digits
                        let mut zero_pad = next == '0';
                        // consume flags (-, +, space, #, 0)
                        while let Some(&c) = chars.peek() {
                            if matches!(c, '-' | '+' | ' ' | '#') {
                                if c == '0' { zero_pad = true; }
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        // consume width digits (already consumed `next` below)
                        let mut width_str = String::new();
                        if next.is_ascii_digit() {
                            width_str.push(next);
                        }
                        while let Some(&c) = chars.peek() {
                            if c.is_ascii_digit() {
                                width_str.push(c);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        let width: usize = width_str.parse().unwrap_or(0);
                        // Check for ll prefix
                        let ll_prefix = if chars.peek() == Some(&'l') {
                            chars.next(); // consume first 'l'
                            if chars.peek() == Some(&'l') {
                                chars.next(); // consume second 'l'
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        // Now get the type char
                        if let Some(&type_ch) = chars.peek() {
                            match type_ch {
                                'x' | 'X' => {
                                    chars.next();
                                    let upper = type_ch == 'X';
                                    if ll_prefix {
                                        specs.push(FormatSpec::HexWidthLong { width, zero_pad, upper });
                                    } else {
                                        specs.push(FormatSpec::HexWidth { width, zero_pad, upper });
                                    }
                                }
                                'd' | 'i' | 'u' => {
                                    chars.next();
                                    if ll_prefix {
                                        specs.push(FormatSpec::Long64);
                                    } else {
                                        specs.push(FormatSpec::Integer);
                                    }
                                }
                                's' | 'S' => {
                                    chars.next();
                                    specs.push(FormatSpec::String);
                                }
                                _ => {
                                    return Err(format!("Unknown format specifier: %{}{}{}", width_str, if ll_prefix { "ll" } else { "" }, type_ch));
                                }
                            }
                        } else {
                            return Err(format!("Incomplete format specifier after %{}", width_str));
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
    HexPrecision(usize), // %.Nx — zero-padded hex with width N
    FloatPrecisionStripped, // %!.*f — float with dynamic precision, strip trailing zeros (consumes 2 args: precision, value)
    Ordinal,                // %r — SQLite ordinal (1st, 2nd, 3rd, ...)
    HexWidth { width: usize, zero_pad: bool, upper: bool }, // %016llx, %06X, %02X etc.
    HexWidthLong { width: usize, zero_pad: bool, upper: bool }, // %016llx (ll prefix)
}

impl FormatSpec {
    fn rust_format(&self) -> String {
        match self {
            FormatSpec::Percent => "%".to_string(),
            FormatSpec::String | FormatSpec::ZeroCopy => "{}".to_string(),
            FormatSpec::Integer | FormatSpec::Long | FormatSpec::Long64 => "{}".to_string(),
            FormatSpec::Unsigned | FormatSpec::ULong64 => "{}".to_string(),
            FormatSpec::Hex => "{:x}".to_string(),
            FormatSpec::Pointer => "{:p}".to_string(),
            FormatSpec::Float => "{}".to_string(),
            FormatSpec::Char => "{}".to_string(),
            FormatSpec::SqliteQuote => "{}".to_string(),
            FormatSpec::SqliteQuotedString => "{}".to_string(),
            FormatSpec::SqliteIdentifier => "{}".to_string(),
            FormatSpec::FloatG15 => "{}".to_string(),  // Not used in sqlite_printf, only json_printf
            FormatSpec::RawBytes => "{}".to_string(),  // Not used in sqlite_printf, only json_printf
            FormatSpec::HexPrecision(n) => format!("{{:0{}x}}", n),
            FormatSpec::FloatPrecisionStripped => "{}".to_string(),
            FormatSpec::Ordinal => "{}".to_string(),
            FormatSpec::HexWidth { width, zero_pad, upper } => {
                let pad = if *zero_pad { "0" } else { "" };
                let case = if *upper { "X" } else { "x" };
                format!("{{:{}{}{}}}", pad, width, case)
            }
            FormatSpec::HexWidthLong { width, zero_pad, upper } => {
                let pad = if *zero_pad { "0" } else { "" };
                let case = if *upper { "X" } else { "x" };
                format!("{{:{}{}{}}}", pad, width, case)
            }
        }
    }

    fn is_argument_consuming(&self) -> bool {
        !matches!(self, FormatSpec::Percent)
    }

    fn arg_count(&self) -> usize {
        match self {
            FormatSpec::RawBytes => 2, // %.*s consumes 2 args: length and pointer
            FormatSpec::FloatPrecisionStripped => 2, // %!.*f consumes 2 args: precision and value
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
                let __arg = #arg;
                if __arg.is_null() {
                    String::new()
                } else {
                    unsafe { std::ffi::CStr::from_ptr(__arg) }
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
                let __arg = #arg;
                if __arg.is_null() {
                    String::new()
                } else {
                    let s = unsafe { std::ffi::CStr::from_ptr(__arg) }
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
                    let s = unsafe { std::ffi::CStr::from_ptr(__arg) }
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
                    let s = unsafe { std::ffi::CStr::from_ptr(__arg) }
                        .to_str()
                        .unwrap_or("");
                    s.replace('"', "\"\"").to_string()
                }
            }}
        }
        FormatSpec::Percent => quote! {},
        FormatSpec::FloatG15 => {
            // Only used in json_printf
            quote! { compile_error!("FloatG15 specifier only supported in json_printf!") }
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
                let __suffix = if __abs >= 11 && __abs <= 13 {
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
                            result.push_str(&spec.rust_format());
                        } else {
                            // Not consuming, output as literal
                            result.push('%');
                        }
                    }
                    continue;
                }

                // Skip past the format specifier
                chars.next();

                // Handle multi-character specifiers
                if next == 'r' {
                    // %r: single-char ordinal specifier, nothing extra to skip
                } else if matches!(next, '-' | '+' | ' ' | '#' | '0'..='9') {
                    // Width/flag prefix: skip remaining flag/digit chars, optional 'll', then type char
                    while let Some(&c) = chars.peek() {
                        if matches!(c, '-' | '+' | ' ' | '#' | '0'..='9') {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    // Optional 'll' prefix
                    if chars.peek() == Some(&'l') {
                        chars.next(); // consume first 'l'
                        if chars.peek() == Some(&'l') {
                            chars.next(); // consume second 'l'
                        }
                    }
                    // Consume type character (x, X, d, i, u, s)
                    if let Some(&type_ch) = chars.peek() {
                        if matches!(type_ch, 'x' | 'X' | 'd' | 'i' | 'u' | 's' | 'S') {
                            chars.next();
                        }
                    }
                } else if next == 'l' {
                    if let Some(&second) = chars.peek() {
                        if second == 'l' {
                            chars.next();
                            if let Some(&third) = chars.peek() {
                                if matches!(third, 'd' | 'i' | 'u') {
                                    chars.next();
                                }
                            }
                        } else if second == 'd' || second == 'i' {
                            chars.next();
                        }
                    }
                } else if next == '.' {
                    // %.*s or %.Nx: skip remaining specifier characters
                    if let Some(&second) = chars.peek() {
                        if second == '*' {
                            chars.next();
                            if let Some(&third) = chars.peek() {
                                if third == 's' || third == 'S' {
                                    chars.next();
                                }
                            }
                        } else if second.is_ascii_digit() {
                            // Skip digits then the type character (x/X)
                            while let Some(&d) = chars.peek() {
                                if d.is_ascii_digit() {
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                            if let Some(&type_ch) = chars.peek() {
                                if matches!(type_ch, 'x' | 'X') {
                                    chars.next();
                                }
                            }
                        }
                    }
                } else if next == '!' {
                    // %!0.15g or %!.*f: skip remaining specifier characters
                    if let Some(&second) = chars.peek() {
                        if second == '0' {
                            chars.next(); // consume '0'
                            if let Some(&dot) = chars.peek() {
                                if dot == '.' {
                                    chars.next(); // consume '.'
                                    while let Some(&d) = chars.peek() {
                                        if d.is_ascii_digit() { chars.next(); } else { break; }
                                    }
                                    if let Some(&g) = chars.peek() {
                                        if matches!(g, 'g' | 'G') { chars.next(); }
                                    }
                                }
                            }
                        } else if second == '.' {
                            chars.next(); // consume '.'
                            if let Some(&star) = chars.peek() {
                                if star == '*' {
                                    chars.next(); // consume '*'
                                    if let Some(&f) = chars.peek() {
                                        if matches!(f, 'f' | 'F') { chars.next(); }
                                    }
                                }
                            }
                        }
                    }
                }

                // Add Rust format specifier
                if let Some(spec) = spec_iter.next() {
                    if spec.is_argument_consuming() {
                        result.push_str(&spec.rust_format());
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
                    let __ptr = #ptr_arg as *const u8;
                    let __len = #len_arg as usize;
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
                    let __prec = #prec_arg as usize;
                    let __val = #val_arg as f64;
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
                    unsafe { crate::src::src::malloc::sqlite3_free(#arg as *mut ::core::ffi::c_void); }
                }
            });
        }
        quote! { #(#stmts)* }
    } else {
        quote! {}
    };

    // Generate final code using format! (simpler approach)
    let expanded = quote! {
        {
            let result = format!(#rust_format, #(#arg_handlers),*);
            #free_stmts
            let bytes = result.into_bytes();
            let len = bytes.len();
            let ptr = unsafe { crate::src::src::malloc::sqlite3_malloc64((len + 1) as u64) } as *mut u8;
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
    let SqliteSnprintf { buffer, size, format_str, args } =
        parse_macro_input!(input as SqliteSnprintf);

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
                    let __ptr = #ptr_arg as *const u8;
                    let __len = #len_arg as usize;
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
                    let __prec = #prec_arg as usize;
                    let __val = #val_arg as f64;
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
