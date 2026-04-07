/// Format specification types and parsing logic for SQLite printf formats.

#[derive(Debug, Clone)]
pub enum FormatSpec {
    Percent,
    String,
    Integer,
    IntegerWidth(usize),
    IntegerZeroPad(usize),
    Unsigned,
    Hex,
    HexUpper,
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
    FloatG15,
    RawBytes,
    HexPrecision(usize),
    FloatPrecisionStripped,
    Ordinal,
    HexWidth { width: usize, zero_pad: bool, upper: bool },
    HexWidthLong { width: usize, zero_pad: bool, upper: bool },
    /// %.Nf / %.Ne / %.Ng — float with fixed precision N
    FloatFixed(usize),
}

impl FormatSpec {
    pub fn rust_format(&self) -> String {
        match self {
            FormatSpec::Percent => "%".to_string(),
            FormatSpec::String | FormatSpec::ZeroCopy => "{}".to_string(),
            FormatSpec::Integer | FormatSpec::Long | FormatSpec::Long64 => "{}".to_string(),
            FormatSpec::IntegerWidth(w) => format!("{{:>{}}}", w),
            FormatSpec::IntegerZeroPad(w) => format!("{{:0{}}}", w),
            FormatSpec::Unsigned | FormatSpec::ULong64 => "{}".to_string(),
            FormatSpec::Hex => "{:x}".to_string(),
            FormatSpec::HexUpper => "{:X}".to_string(),
            FormatSpec::Pointer => "{:p}".to_string(),
            FormatSpec::Float => "{}".to_string(),
            FormatSpec::Char => "{}".to_string(),
            FormatSpec::SqliteQuote => "{}".to_string(),
            FormatSpec::SqliteQuotedString => "{}".to_string(),
            FormatSpec::SqliteIdentifier => "{}".to_string(),
            FormatSpec::FloatG15 => "{}".to_string(),
            FormatSpec::RawBytes => "{}".to_string(),
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
            FormatSpec::FloatFixed(n) => format!("{{:.{}}}", n),
        }
    }

    pub fn is_argument_consuming(&self) -> bool {
        !matches!(self, FormatSpec::Percent)
    }

    pub fn arg_count(&self) -> usize {
        match self {
            FormatSpec::RawBytes => 2,
            FormatSpec::FloatPrecisionStripped => 2,
            FormatSpec::Percent => 0,
            _ => if self.is_argument_consuming() { 1 } else { 0 },
        }
    }
}

/// Parse SQLite format specifiers from a format string.
pub fn parse_format_specs(format: &str) -> Result<Vec<FormatSpec>, String> {
    let mut specs = Vec::new();
    let mut chars = format.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let Some(&next) = chars.peek() {
                match next {
                    '%' => {
                        chars.next();
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
                    'x' => {
                        chars.next();
                        specs.push(FormatSpec::Hex);
                    }
                    'X' => {
                        chars.next();
                        specs.push(FormatSpec::HexUpper);
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
                                chars.next();
                                if let Some(&third) = chars.peek() {
                                    if third == '.' {
                                        chars.next();
                                        while let Some(&d) = chars.peek() {
                                            if d.is_ascii_digit() { chars.next(); } else { break; }
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
                                    } else if third.is_ascii_digit() {
                                        while let Some(&d) = chars.peek() {
                                            if d.is_ascii_digit() { chars.next(); } else { break; }
                                        }
                                        if let Some(&final_char) = chars.peek() {
                                            if matches!(final_char, 'g' | 'G') {
                                                chars.next();
                                                specs.push(FormatSpec::FloatG15);
                                            } else {
                                                return Err(format!("Unknown format specifier: %!.N{}", final_char));
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
                                        'f' | 'F' | 'e' | 'E' | 'g' | 'G' => {
                                            chars.next();
                                            specs.push(FormatSpec::FloatFixed(prec));
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
                        let mut zero_pad = next == '0';
                        while let Some(&c) = chars.peek() {
                            if matches!(c, '-' | '+' | ' ' | '#') {
                                if c == '0' { zero_pad = true; }
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        let mut width_str = String::new();
                        if next.is_ascii_digit() { width_str.push(next); }
                        while let Some(&c) = chars.peek() {
                            if c.is_ascii_digit() { width_str.push(c); chars.next(); } else { break; }
                        }
                        let width: usize = width_str.parse().unwrap_or(0);
                        if chars.peek() == Some(&'.') {
                            chars.next();
                            while let Some(&c) = chars.peek() {
                                if c.is_ascii_digit() || c == '*' { chars.next(); } else { break; }
                            }
                        }
                        let ll_prefix = if chars.peek() == Some(&'l') {
                            chars.next();
                            if chars.peek() == Some(&'l') { chars.next(); true } else { false }
                        } else { false };
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
                                'd' | 'i' => {
                                    chars.next();
                                    if ll_prefix {
                                        specs.push(FormatSpec::Long64);
                                    } else if width > 0 {
                                        if zero_pad {
                                            specs.push(FormatSpec::IntegerZeroPad(width));
                                        } else {
                                            specs.push(FormatSpec::IntegerWidth(width));
                                        }
                                    } else {
                                        specs.push(FormatSpec::Integer);
                                    }
                                }
                                'u' => {
                                    chars.next();
                                    if ll_prefix {
                                        specs.push(FormatSpec::ULong64);
                                    } else if width > 0 {
                                        if zero_pad {
                                            specs.push(FormatSpec::IntegerZeroPad(width));
                                        } else {
                                            specs.push(FormatSpec::IntegerWidth(width));
                                        }
                                    } else {
                                        specs.push(FormatSpec::Unsigned);
                                    }
                                }
                                's' | 'S' => { chars.next(); specs.push(FormatSpec::String); }
                                'f' | 'F' | 'e' | 'E' | 'g' | 'G' => { chars.next(); specs.push(FormatSpec::Float); }
                                'q' => { chars.next(); specs.push(FormatSpec::SqliteQuote); }
                                'Q' => { chars.next(); specs.push(FormatSpec::SqliteQuotedString); }
                                'w' => { chars.next(); specs.push(FormatSpec::SqliteIdentifier); }
                                'z' => { chars.next(); specs.push(FormatSpec::ZeroCopy); }
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

/// Convert a SQLite format string to a Rust `format!`-compatible string.
pub fn convert_format_string(format: &str, specs: &[FormatSpec]) -> String {
    let mut result = String::new();
    let mut chars = format.chars().peekable();
    let mut spec_iter = specs.iter();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let Some(&next) = chars.peek() {
                if next == '%' {
                    chars.next();
                    if let Some(spec) = spec_iter.next() {
                        if spec.is_argument_consuming() {
                            result.push_str(&spec.rust_format());
                        } else {
                            result.push('%');
                        }
                    }
                    continue;
                }

                chars.next();

                if next == 'r' {
                } else if matches!(next, '-' | '+' | ' ' | '#' | '0'..='9') {
                    while let Some(&c) = chars.peek() {
                        if matches!(c, '-' | '+' | ' ' | '#' | '0'..='9') { chars.next(); } else { break; }
                    }
                    if chars.peek() == Some(&'l') {
                        chars.next();
                        if chars.peek() == Some(&'l') { chars.next(); }
                    }
                    if let Some(&type_ch) = chars.peek() {
                        if matches!(type_ch, 'x' | 'X' | 'd' | 'i' | 'u' | 's' | 'S' | 'f' | 'F' | 'e' | 'E' | 'g' | 'G' | 'q' | 'Q' | 'w' | 'z') { chars.next(); }
                    }
                } else if next == 'l' {
                    if let Some(&second) = chars.peek() {
                        if second == 'l' {
                            chars.next();
                            if let Some(&third) = chars.peek() {
                                if matches!(third, 'd' | 'i' | 'u') { chars.next(); }
                            }
                        } else if second == 'd' || second == 'i' { chars.next(); }
                    }
                } else if next == '.' {
                    if let Some(&second) = chars.peek() {
                        if second == '*' {
                            chars.next();
                            if let Some(&third) = chars.peek() {
                                if third == 's' || third == 'S' { chars.next(); }
                            }
                        } else if second.is_ascii_digit() {
                            while let Some(&d) = chars.peek() {
                                if d.is_ascii_digit() { chars.next(); } else { break; }
                            }
                            if let Some(&type_ch) = chars.peek() {
                                if matches!(type_ch, 'x' | 'X' | 'f' | 'F' | 'e' | 'E' | 'g' | 'G') { chars.next(); }
                            }
                        }
                    }
                } else if next == '!' {
                    if let Some(&second) = chars.peek() {
                        if second == '0' {
                            chars.next();
                            if let Some(&dot) = chars.peek() {
                                if dot == '.' {
                                    chars.next();
                                    while let Some(&d) = chars.peek() {
                                        if d.is_ascii_digit() { chars.next(); } else { break; }
                                    }
                                    if let Some(&g) = chars.peek() {
                                        if matches!(g, 'g' | 'G') { chars.next(); }
                                    }
                                }
                            }
                        } else if second == '.' {
                            chars.next();
                            if let Some(&star) = chars.peek() {
                                if star == '*' {
                                    chars.next();
                                    if let Some(&f) = chars.peek() {
                                        if matches!(f, 'f' | 'F') { chars.next(); }
                                    }
                                }
                            }
                        }
                    }
                }

                if let Some(spec) = spec_iter.next() {
                    if spec.is_argument_consuming() {
                        result.push_str(&spec.rust_format());
                    } else {
                        result.push('%');
                    }
                }
            }
        } else if ch == '{' {
            result.push_str("{{");
        } else if ch == '}' {
            result.push_str("}}");
        } else {
            result.push(ch);
        }
    }

    result
}
