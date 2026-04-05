/*
** C implementation of sqlite3_str_appendf.
**
** This is the variadic entry point — the `...` handling lives here in C,
** removing the need for Rust's unstable c_variadic feature for this function.
**
** Two paths:
**   SQLFUNC — extracts a PrintfArguments* from va_arg, calls Rust directly
**   Normal  — passes va_list to Rust for format-string-driven extraction
*/

#include <stdarg.h>
#include <stdint.h>

#define SQLITE_PRINTF_SQLFUNC 0x02

/* sqlite3_str / StrAccum layout — must match Rust's #[repr(C)] struct */
typedef struct sqlite3_str {
    void     *db;          /* sqlite3*          */
    char     *zText;
    uint32_t  nAlloc;
    uint32_t  mxAlloc;
    uint32_t  nChar;
    uint8_t   accError;
    uint8_t   printfFlags;
} sqlite3_str;

/* Opaque — Rust owns the definition */
typedef struct PrintfArguments PrintfArguments;

/* Rust helpers (no variadic / VaList in the SQLFUNC path) */
extern void sqlite3_str_vappendf_sqlfunc(
    sqlite3_str *p,
    const char  *zFormat,
    PrintfArguments *pArgList
);

/* Rust helper — normal path, takes va_list */
extern void sqlite3_str_vappendf_va(
    sqlite3_str *p,
    const char  *zFormat,
    va_list      ap
);

__attribute__((visibility("default")))
void sqlite3_str_appendf(sqlite3_str *p, const char *zFormat, ...) {
    va_list ap;
    va_start(ap, zFormat);

    if (p->printfFlags & SQLITE_PRINTF_SQLFUNC) {
        /* SQLFUNC: first vararg is a PrintfArguments pointer */
        PrintfArguments *pArgList = va_arg(ap, PrintfArguments *);
        sqlite3_str_vappendf_sqlfunc(p, zFormat, pArgList);
    } else {
        /* Normal: forward va_list to Rust for extraction + formatting */
        sqlite3_str_vappendf_va(p, zFormat, ap);
    }

    va_end(ap);
}
