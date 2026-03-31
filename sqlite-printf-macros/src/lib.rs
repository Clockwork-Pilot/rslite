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
                        specs.push(FormatSpec::SqliteIdentifier);
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
    SqliteIdentifier,
    Long,
    Long64,
    ULong64,
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
            FormatSpec::SqliteIdentifier => "{}",
        }
    }

    fn is_argument_consuming(&self) -> bool {
        !matches!(self, FormatSpec::Percent)
    }
}

/// Generate code to handle format argument based on spec
fn gen_arg_handler(arg: &Expr, spec: &FormatSpec) -> proc_macro2::TokenStream {
    match spec {
        FormatSpec::String => quote! { #arg },
        FormatSpec::Integer | FormatSpec::Long | FormatSpec::Long64 => quote! { #arg },
        FormatSpec::Unsigned | FormatSpec::ULong64 => quote! { #arg },
        FormatSpec::Hex => quote! { #arg },
        FormatSpec::Pointer => quote! { #arg },
        FormatSpec::Float => quote! { #arg },
        FormatSpec::Char => quote! { #arg as u8 as char },
        FormatSpec::ZeroCopy => quote! { #arg },
        FormatSpec::SqliteQuote => {
            // %q: SQL string literal - just the content, quotes in format string
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
        FormatSpec::SqliteIdentifier => {
            // %Q or %w: SQL identifier/string - escape for SQL, quotes in format string
            quote! {{
                if #arg.is_null() {
                    String::new()
                } else {
                    let s = std::ffi::CStr::from_ptr(#arg)
                        .to_str()
                        .unwrap_or("");
                    s.replace('"', "\"\"")
                }
            }}
        }
        FormatSpec::Percent => quote! {},
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
                    result.push('%');
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

    // Generate argument handlers
    let mut arg_handlers = Vec::new();
    let mut arg_iter = args.iter();

    for spec in &specs {
        if spec.is_argument_consuming() {
            if let Some(arg) = arg_iter.next() {
                arg_handlers.push(gen_arg_handler(arg, spec));
            }
        }
    }

    // Generate final code
    let expanded = quote! {
        {
            let result = format!(#rust_format, #(#arg_handlers),*);
            crate::safe_format::allocate_string(result)
        }
    };

    TokenStream::from(expanded)
}
