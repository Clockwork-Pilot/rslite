//! Compare sqlite3_mprintf (SQLite's C implementation via sqlite3_str_vappendf)
//! against our sqlite_printf! macro (Rust proc-macro implementation).
//!
//! Format string patterns are collected from the codebase with:
//!   grep -roh '"[^"]*%[^"]*"' src/src/ --include="*.rs"
//!
//! Each test calls both implementations with the same arguments and asserts
//! outputs match.  Failures indicate gaps in our Rust implementation.
//!
//! Patterns NOT included here because our parser does not support them yet:
//!   %!0.20e  — only g/G handled for %!0.N flag
//!   %#Q      — # alternate-form flag not supported
//!   %#T      — T specifier (Token) not supported outside db context
//!   %!S      — unknown !S combination
//!   %.18s    — precision-truncated strings (%.Ns) not supported
//!   %.4c     — precision-truncated chars  (%.Nc) not supported
//!   %.*z     — raw-bytes with %z sink not supported
//!   %S       — SrcItem specifier requires live db object

use core::ffi::{c_char, c_void};
use std::ffi::CStr;

use sqlite_noamalgam::sqlite_printf;

// sqlite3_mprintf is #[no_mangle] pub extern "C" in sqlite_noamalgam.
// Declare it here so we can call it directly from the test binary.
unsafe extern "C" {
    fn sqlite3_mprintf(fmt: *const c_char, ...) -> *mut c_char;
    fn sqlite3_free(p: *mut c_void);
}

/// Consume a `*mut c_char` returned by `sqlite3_mprintf`: copy to String, free.
unsafe fn take_m(s: *mut c_char) -> String {
    if s.is_null() {
        return "(null)".to_string();
    }
    let r = CStr::from_ptr(s).to_string_lossy().into_owned();
    sqlite3_free(s as *mut c_void);
    r
}

/// Consume a `*mut c_char` returned by `sqlite_printf!`: copy to String, free.
unsafe fn take_p(s: *mut c_char) -> String {
    if s.is_null() {
        return "(null)".to_string();
    }
    let r = CStr::from_ptr(s).to_string_lossy().into_owned();
    sqlite3_free(s as *mut c_void);
    r
}

// ─── %d  basic integer ────────────────────────────────────────────────────

#[test]
fn d_basic() {
    unsafe {
        for &v in &[0i32, 1, -1, 42, -42, i32::MAX, i32::MIN] {
            let m = take_m(sqlite3_mprintf(b"%d\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%d", v));
            assert_eq!(m, p, "%d({v})");
        }
    }
}

// ─── %d  width / zero-padding ─────────────────────────────────────────────
// NOTE: our parser maps %Nd and %0Nd to plain Integer (width is lost),
// so these tests are expected to FAIL until integer-width support is added.

#[test]
fn d_zero_pad_02() {
    // "%02d" — from: "%02d:%02d", "%02d:%02d:%02d", "%04d-%02d-%02d"
    unsafe {
        for &v in &[0i32, 5, 9, 10, 99] {
            let m = take_m(sqlite3_mprintf(b"%02d\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%02d", v));
            assert_eq!(m, p, "%02d({v})");
        }
    }
}

#[test]
fn d_zero_pad_03() {
    // "%03d" — from: "%03d"
    unsafe {
        for &v in &[0i32, 7, 42, 999] {
            let m = take_m(sqlite3_mprintf(b"%03d\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%03d", v));
            assert_eq!(m, p, "%03d({v})");
        }
    }
}

#[test]
fn d_zero_pad_04() {
    // "%04d" — from: "%04d-%02d-%02d", "%04d"
    unsafe {
        for &v in &[0i32, 5, 42, 2024] {
            let m = take_m(sqlite3_mprintf(b"%04d\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%04d", v));
            assert_eq!(m, p, "%04d({v})");
        }
    }
}

#[test]
#[ignore] // TODO: Right-aligned integers without zero-padding not working correctly
fn d_right_align_2() {
    // "%2d" — from: "%2d"
    unsafe {
        for &v in &[0i32, 5, 10, 99] {
            let m = take_m(sqlite3_mprintf(b"%2d\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%2d", v));
            assert_eq!(m, p, "%2d({v})");
        }
    }
}

// ─── Date / time composite patterns ──────────────────────────────────────

#[test]
fn date_time_hhmm() {
    // "%02d:%02d" — from codebase
    unsafe {
        let m = take_m(sqlite3_mprintf(b"%02d:%02d\0".as_ptr() as _, 3i32, 5i32));
        let p = take_p(sqlite_printf!("%02d:%02d", 3i32, 5i32));
        assert_eq!(m, p, "%02d:%02d(3,5)");

        let m = take_m(sqlite3_mprintf(b"%02d:%02d\0".as_ptr() as _, 23i32, 59i32));
        let p = take_p(sqlite_printf!("%02d:%02d", 23i32, 59i32));
        assert_eq!(m, p, "%02d:%02d(23,59)");
    }
}

#[test]
fn date_time_hhmmss() {
    // "%02d:%02d:%02d" — from codebase
    unsafe {
        let m = take_m(sqlite3_mprintf(b"%02d:%02d:%02d\0".as_ptr() as _, 12i32, 30i32, 5i32));
        let p = take_p(sqlite_printf!("%02d:%02d:%02d", 12i32, 30i32, 5i32));
        assert_eq!(m, p, "%02d:%02d:%02d(12,30,5)");
    }
}

#[test]
fn date_yyyymmdd() {
    // "%04d-%02d-%02d" — from codebase
    unsafe {
        let m = take_m(sqlite3_mprintf(b"%04d-%02d-%02d\0".as_ptr() as _, 2024i32, 1i32, 5i32));
        let p = take_p(sqlite_printf!("%04d-%02d-%02d", 2024i32, 1i32, 5i32));
        assert_eq!(m, p, "%04d-%02d-%02d(2024,1,5)");
    }
}

#[test]
fn date_with_char_prefix() {
    // "%c%04d-%02d-%02d" — from codebase (sign + date)
    unsafe {
        let m = take_m(sqlite3_mprintf(b"%c%04d-%02d-%02d\0".as_ptr() as _, b'+' as i32, 2024i32, 1i32, 5i32));
        let p = take_p(sqlite_printf!("%c%04d-%02d-%02d", b'+' as i32, 2024i32, 1i32, 5i32));
        assert_eq!(m, p, "%c%04d-%02d-%02d");
    }
}

#[test]
#[ignore] // TODO: floating point width/precision not yet implemented
fn datetime_full() {
    // "%c%04d-%02d-%02d %02d:%02d:%06.3f" — from codebase
    unsafe {
        let m = take_m(sqlite3_mprintf(
            b"%c%04d-%02d-%02d %02d:%02d:%06.3f\0".as_ptr() as _,
            b'+' as i32, 2024i32, 3i32, 15i32, 10i32, 30i32, 5.123f64,
        ));
        let p = take_p(sqlite_printf!(
            "%c%04d-%02d-%02d %02d:%02d:%06.3f",
            b'+' as i32, 2024i32, 3i32, 15i32, 10i32, 30i32, 5.123f64
        ));
        assert_eq!(m, p, "%c%04d-%02d-%02d %02d:%02d:%06.3f");
    }
}

// ─── %s  string ──────────────────────────────────────────────────────────

#[test]
fn s_basic() {
    let hello: *const c_char = b"hello\0".as_ptr() as _;
    let world: *const c_char = b"world\0".as_ptr() as _;
    let empty: *const c_char = b"\0".as_ptr() as _;

    unsafe {
        // "%s"
        let m = take_m(sqlite3_mprintf(b"%s\0".as_ptr() as _, hello));
        let p = take_p(sqlite_printf!("%s", hello));
        assert_eq!(m, p, "%s");

        // "%s%s" — from codebase
        let m = take_m(sqlite3_mprintf(b"%s%s\0".as_ptr() as _, hello, world));
        let p = take_p(sqlite_printf!("%s%s", hello, world));
        assert_eq!(m, p, "%s%s");

        // "%s.%s" — from codebase  (e.g. "db.table")
        let m = take_m(sqlite3_mprintf(b"%s.%s\0".as_ptr() as _, hello, world));
        let p = take_p(sqlite_printf!("%s.%s", hello, world));
        assert_eq!(m, p, "%s.%s");

        // "%s.%s.%s" — from codebase  (e.g. "db.schema.table")
        let foo: *const c_char = b"foo\0".as_ptr() as _;
        let m = take_m(sqlite3_mprintf(b"%s.%s.%s\0".as_ptr() as _, hello, world, foo));
        let p = take_p(sqlite_printf!("%s.%s.%s", hello, world, foo));
        assert_eq!(m, p, "%s.%s.%s");

        // "%s%s" with empty first operand
        let m = take_m(sqlite3_mprintf(b"%s%s\0".as_ptr() as _, empty, hello));
        let p = take_p(sqlite_printf!("%s%s", empty, hello));
        assert_eq!(m, p, "%s%s (empty+hello)");

        // "%s=?" — from codebase
        let col: *const c_char = b"rowid\0".as_ptr() as _;
        let m = take_m(sqlite3_mprintf(b"%s=?\0".as_ptr() as _, col));
        let p = take_p(sqlite_printf!("%s=?", col));
        assert_eq!(m, p, "%s=?");

        // "%s(%d)" — from codebase
        let m = take_m(sqlite3_mprintf(b"%s(%d)\0".as_ptr() as _, hello, 3i32));
        let p = take_p(sqlite_printf!("%s(%d)", hello, 3i32));
        assert_eq!(m, p, "%s(%d)");

        // "%s constraint failed" — from codebase
        let kind: *const c_char = b"UNIQUE\0".as_ptr() as _;
        let m = take_m(sqlite3_mprintf(b"%s constraint failed\0".as_ptr() as _, kind));
        let p = take_p(sqlite_printf!("%s constraint failed", kind));
        assert_eq!(m, p, "%s constraint failed");

        // "%d:%s" — from codebase
        let m = take_m(sqlite3_mprintf(b"%d:%s\0".as_ptr() as _, 42i32, hello));
        let p = take_p(sqlite_printf!("%d:%s", 42i32, hello));
        assert_eq!(m, p, "%d:%s");

        // "%d values for %d columns" — from codebase
        let m = take_m(sqlite3_mprintf(b"%d values for %d columns\0".as_ptr() as _, 3i32, 2i32));
        let p = take_p(sqlite_printf!("%d values for %d columns", 3i32, 2i32));
        assert_eq!(m, p, "%d values for %d columns");

        // "%d columns assigned %d values" — from codebase
        let m = take_m(sqlite3_mprintf(b"%d columns assigned %d values\0".as_ptr() as _, 2i32, 3i32));
        let p = take_p(sqlite_printf!("%d columns assigned %d values", 2i32, 3i32));
        assert_eq!(m, p, "%d columns assigned %d values");

        // "%s is %u but should be %u" — from codebase
        let field: *const c_char = b"nHeight\0".as_ptr() as _;
        let m = take_m(sqlite3_mprintf(b"%s is %u but should be %u\0".as_ptr() as _, field, 1u32, 2u32));
        let p = take_p(sqlite_printf!("%s is %u but should be %u", field, 1u32, 2u32));
        assert_eq!(m, p, "%s is %u but should be %u");
    }
}

// ─── %.*s  raw bytes (length + pointer) ──────────────────────────────────

#[test]
fn raw_bytes() {
    let data: *const c_char = b"hello world\0".as_ptr() as _;
    let suffix: *const c_char = b"_suffix\0".as_ptr() as _;

    unsafe {
        // "%.*s" — from codebase
        for &n in &[0i32, 1, 3, 5, 11] {
            let m = take_m(sqlite3_mprintf(b"%.*s\0".as_ptr() as _, n, data));
            let p = take_p(sqlite_printf!("%.*s", n, data));
            assert_eq!(m, p, "%.*s(n={n})");
        }
        // "%.*s%s" — from codebase
        let m = take_m(sqlite3_mprintf(b"%.*s%s\0".as_ptr() as _, 5i32, data, suffix));
        let p = take_p(sqlite_printf!("%.*s%s", 5i32, data, suffix));
        assert_eq!(m, p, "%.*s%s");
    }
}

// ─── %q / %Q  SQL string escaping ────────────────────────────────────────

#[test]
fn sql_q() {
    let simple:     *const c_char = b"hello\0".as_ptr() as _;
    let apostrophe: *const c_char = b"it's\0".as_ptr() as _;
    let multi:      *const c_char = b"O'Reilly's\0".as_ptr() as _;
    let null_ptr:   *const c_char = core::ptr::null();

    unsafe {
        // %q — simple string (no apostrophes)
        let m = take_m(sqlite3_mprintf(b"%q\0".as_ptr() as _, simple));
        let p = take_p(sqlite_printf!("%q", simple));
        assert_eq!(m, p, "%q simple");

        // %q — string with apostrophe → double the quote
        let m = take_m(sqlite3_mprintf(b"%q\0".as_ptr() as _, apostrophe));
        let p = take_p(sqlite_printf!("%q", apostrophe));
        assert_eq!(m, p, "%q apostrophe");

        // %q — multiple apostrophes
        let m = take_m(sqlite3_mprintf(b"%q\0".as_ptr() as _, multi));
        let p = take_p(sqlite_printf!("%q", multi));
        assert_eq!(m, p, "%q multi-apostrophe");

        // %Q — wraps in single quotes
        let m = take_m(sqlite3_mprintf(b"%Q\0".as_ptr() as _, simple));
        let p = take_p(sqlite_printf!("%Q", simple));
        assert_eq!(m, p, "%Q simple");

        // %Q — with apostrophe inside
        let m = take_m(sqlite3_mprintf(b"%Q\0".as_ptr() as _, apostrophe));
        let p = take_p(sqlite_printf!("%Q", apostrophe));
        assert_eq!(m, p, "%Q apostrophe");

        // %Q — NULL ptr → literal "NULL"
        let m = take_m(sqlite3_mprintf(b"%Q\0".as_ptr() as _, null_ptr));
        let p = take_p(sqlite_printf!("%Q", null_ptr));
        assert_eq!(m, p, "%Q NULL ptr");

        // "%Q.%Q" — from codebase  (e.g. schema.table SQL fragment)
        let db:  *const c_char = b"main\0".as_ptr() as _;
        let tbl: *const c_char = b"my_table\0".as_ptr() as _;
        let m = take_m(sqlite3_mprintf(b"%Q.%Q\0".as_ptr() as _, db, tbl));
        let p = take_p(sqlite_printf!("%Q.%Q", db, tbl));
        assert_eq!(m, p, "%Q.%Q");
    }
}

// ─── %w  SQL identifier escaping ─────────────────────────────────────────

#[test]
fn sql_w() {
    let simple:     *const c_char = b"mytable\0".as_ptr() as _;
    let with_quote: *const c_char = b"my\"table\0".as_ptr() as _;

    unsafe {
        // %w — simple identifier (no double quotes)
        let m = take_m(sqlite3_mprintf(b"%w\0".as_ptr() as _, simple));
        let p = take_p(sqlite_printf!("%w", simple));
        assert_eq!(m, p, "%w simple");

        // %w — identifier with embedded double quote → doubled
        let m = take_m(sqlite3_mprintf(b"%w\0".as_ptr() as _, with_quote));
        let p = take_p(sqlite_printf!("%w", with_quote));
        assert_eq!(m, p, "%w with double-quote");

        // "%w_%s" — from codebase  (e.g. "fts5_content")
        let name:   *const c_char = b"fts5\0".as_ptr() as _;
        let suffix: *const c_char = b"content\0".as_ptr() as _;
        let m = take_m(sqlite3_mprintf(b"%w_%s\0".as_ptr() as _, name, suffix));
        let p = take_p(sqlite_printf!("%w_%s", name, suffix));
        assert_eq!(m, p, "%w_%s");
    }
}

// ─── %lld / %llu  64-bit integers ────────────────────────────────────────

#[test]
fn lld_llu() {
    unsafe {
        // "%lld" — from codebase
        for &v in &[0i64, 1, -1, i64::MAX, i64::MIN] {
            let m = take_m(sqlite3_mprintf(b"%lld\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%lld", v));
            assert_eq!(m, p, "%lld({v})");
        }

        // "%llu" — from: " %llu", "%llu", "%.*z:%u"
        for &v in &[0u64, 1, u64::MAX] {
            let m = take_m(sqlite3_mprintf(b"%llu\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%llu", v));
            assert_eq!(m, p, "%llu({v})");
        }

        // " %llu" — from codebase (leading space literal)
        let m = take_m(sqlite3_mprintf(b" %llu\0".as_ptr() as _, 42u64));
        let p = take_p(sqlite_printf!(" %llu", 42u64));
        assert_eq!(m, p, "\" %llu\"");
    }
}

// ─── %x / %X  hexadecimal ─────────────────────────────────────────────────

#[test]
fn hex_basic() {
    unsafe {
        // "%x" lowercase — from codebase
        for &v in &[0u32, 1, 0xAB, 0xDEAD, u32::MAX] {
            let m = take_m(sqlite3_mprintf(b"%x\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%x", v));
            assert_eq!(m, p, "%x({v:#x})");
        }

        // "%X" uppercase — NOTE: our plain %X maps to FormatSpec::Hex → "{:x}" (lowercase bug)
        for &v in &[0u32, 0xDEAD] {
            let m = take_m(sqlite3_mprintf(b"%X\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%X", v));
            assert_eq!(m, p, "%X({v:#x})");
        }
    }
}

#[test]
fn hex_zero_padded() {
    unsafe {
        // "%02x" — from codebase
        for &v in &[0u32, 5, 0x0F, 0xFF] {
            let m = take_m(sqlite3_mprintf(b"%02x\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%02x", v));
            assert_eq!(m, p, "%02x({v:#x})");
        }
    }
}

#[test]
fn hex_precision() {
    unsafe {
        // "%.3x+%.6x" — from codebase  (e.g. page/offset formatting)
        let m = take_m(sqlite3_mprintf(b"%.3x+%.6x\0".as_ptr() as _, 0xABu32, 0x123456u32));
        let p = take_p(sqlite_printf!("%.3x+%.6x", 0xABu32, 0x123456u32));
        assert_eq!(m, p, "%.3x+%.6x");

        // "%s%.3x+%.6x" — from codebase  (prefix string + two hex values)
        let prefix: *const c_char = b"pg:\0".as_ptr() as _;
        let m = take_m(sqlite3_mprintf(b"%s%.3x+%.6x\0".as_ptr() as _, prefix, 0xABu32, 0x123456u32));
        let p = take_p(sqlite_printf!("%s%.3x+%.6x", prefix, 0xABu32, 0x123456u32));
        assert_eq!(m, p, "%s%.3x+%.6x");
    }
}

// ─── %f / %g / %.Nf  floating point ──────────────────────────────────────

#[test]
#[ignore] // TODO: g-format uses significant digits not decimal places; width not implemented
fn float_fixed_precision() {
    unsafe {
        // "%.3f" — from: "%.3f", "%06.3f"
        for &v in &[0.0f64, 3.14159, -1.5, 1000.0] {
            let m = take_m(sqlite3_mprintf(b"%.3f\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%.3f", v));
            assert_eq!(m, p, "%.3f({v})");
        }

        // "%.16g" — from codebase (high-precision float)
        // NOTE: our parser maps %.16g → FloatFixed(16) → "{:.16}" (decimal places, not sig digits)
        for &v in &[1.0f64, 3.14159265358979, 0.1 + 0.2] {
            let m = take_m(sqlite3_mprintf(b"%.16g\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%.16g", v));
            assert_eq!(m, p, "%.16g({v})");
        }

        // "%06.3f" — from codebase (zero-padded + precision float)
        // NOTE: our %06.3f → FormatSpec::Float → "{}" — loses both width and precision
        for &v in &[3.14159f64, 0.0, -1.5] {
            let m = take_m(sqlite3_mprintf(b"%06.3f\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%06.3f", v));
            assert_eq!(m, p, "%06.3f({v})");
        }
    }
}

// ─── %!0.15g  SQLite special 15-significant-digit float ──────────────────

#[test]
#[ignore] // TODO: %!0.15g uses different algorithm than our implementation
fn float_g15() {
    // "%!0.15g" — from codebase (canonical SQLite float serialization)
    // Our impl uses a different algorithm than SQLite's FpDecode;
    // mismatches indicate gaps to fix.
    unsafe {
        for &v in &[
            0.0f64,
            1.0,
            -1.0,
            0.5,
            1.0 / 3.0,
            3.14159265358979323846,
            1.23456789012345678,
            1e10,
            1e-10,
            1e100,
            1e-100,
            f64::MAX,
            f64::MIN_POSITIVE,
        ] {
            let m = take_m(sqlite3_mprintf(b"%!0.15g\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%!0.15g", v));
            assert_eq!(m, p, "%!0.15g({v})");
        }

        // "%!.15g" — variant from codebase
        for &v in &[1.0f64, 3.14159265358979, 1.0 / 3.0] {
            let m = take_m(sqlite3_mprintf(b"%!.15g\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%!.15g", v));
            assert_eq!(m, p, "%!.15g({v})");
        }
    }
}

// ─── %!.*f  float with dynamic precision + trailing-zero stripping ────────

#[test]
#[ignore] // TODO: %!.*f edge cases with trailing zero stripping
fn float_precision_stripped() {
    // "%!.*f" — from codebase
    unsafe {
        for &(prec, val) in &[
            (0i32, 3.14159f64),
            (3i32, 3.14159f64),
            (6i32, 1.0f64),
            (2i32, 0.0f64),
            (3i32, -1.5f64),
            (10i32, 1.0 / 3.0),
        ] {
            let m = take_m(sqlite3_mprintf(b"%!.*f\0".as_ptr() as _, prec, val));
            let p = take_p(sqlite_printf!("%!.*f", prec, val));
            assert_eq!(m, p, "%!.*f(prec={prec}, val={val})");
        }
    }
}

// ─── %c  character ────────────────────────────────────────────────────────

#[test]
fn char_basic() {
    unsafe {
        // "%c" — from: "%c%04d-%02d-%02d", "%c?)"
        for &ch in &[b'A', b'z', b'0', b'+', b'-', b' ', b'\t'] {
            let m = take_m(sqlite3_mprintf(b"%c\0".as_ptr() as _, ch as i32));
            let p = take_p(sqlite_printf!("%c", ch as i32));
            assert_eq!(m, p, "%c({ch:?})");
        }
    }
}

// ─── %r  ordinal numbers ──────────────────────────────────────────────────

#[test]
fn ordinal() {
    // "%r" — from: "%r ORDER BY term does not match any column",
    //              "%r %s BY term out of range"
    unsafe {
        for &n in &[0i32, 1, 2, 3, 4, 5, 11, 12, 13, 21, 22, 23, 100, 101, 111, 112, 113] {
            let m = take_m(sqlite3_mprintf(b"%r\0".as_ptr() as _, n));
            let p = take_p(sqlite_printf!("%r", n));
            assert_eq!(m, p, "%r({n})");
        }

        // "%r %s" — from codebase
        let msg: *const c_char = b"ORDER BY\0".as_ptr() as _;
        let m = take_m(sqlite3_mprintf(b"%r %s\0".as_ptr() as _, 1i32, msg));
        let p = take_p(sqlite_printf!("%r %s", 1i32, msg));
        assert_eq!(m, p, "%r %s");
    }
}

// ─── %%  literal percent ──────────────────────────────────────────────────

#[test]
fn literal_percent() {
    unsafe {
        let m = take_m(sqlite3_mprintf(b"100%%\0".as_ptr() as _));
        let p = take_p(sqlite_printf!("100%%"));
        assert_eq!(m, p, "100%%");

        let m = take_m(sqlite3_mprintf(b"%%d\0".as_ptr() as _));
        let p = take_p(sqlite_printf!("%%d"));
        assert_eq!(m, p, "%%d");
    }
}

// ─── %u  unsigned integer ─────────────────────────────────────────────────

#[test]
fn unsigned_int() {
    unsafe {
        for &v in &[0u32, 1, 42, u32::MAX] {
            let m = take_m(sqlite3_mprintf(b"%u\0".as_ptr() as _, v));
            let p = take_p(sqlite_printf!("%u", v));
            assert_eq!(m, p, "%u({v})");
        }
    }
}

// ─── %c%u  char + unsigned (from codebase) ────────────────────────────────

#[test]
fn char_unsigned() {
    // "%c%u" — from codebase
    unsafe {
        let m = take_m(sqlite3_mprintf(b"%c%u\0".as_ptr() as _, b'+' as i32, 42u32));
        let p = take_p(sqlite_printf!("%c%u", b'+' as i32, 42u32));
        assert_eq!(m, p, "%c%u");
    }
}
