/*
** C implementation of sqlite3_str_appendf.
**
** This is the variadic entry point — the `...` handling lives here in C,
** removing the need for Rust's unstable c_variadic feature for this function.
**
** Two paths:
**   SQLFUNC — extracts a PrintfArguments* from va_arg, calls Rust directly
**   Normal  — extracts va_args into uint64_t[], calls Rust with packed array
*/

#include <stdarg.h>
#include <stdint.h>
#include <string.h>

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

/* Rust helpers */
extern void sqlite3_str_vappendf_sqlfunc(
    sqlite3_str *p,
    const char  *zFormat,
    PrintfArguments *pArgList
);

extern void sqlite3_str_vappendf_packed(
    sqlite3_str *p,
    const char  *zFormat,
    const uint64_t *args,
    int nArgs
);

/*
 * Extract va_args guided by format string into uint64_t array.
 * Returns number of args extracted.
 * Doubles are bitcast via memcpy to preserve bits.
 */
int extract_va_to_packed(const char *fmt, va_list ap,
                         uint64_t *out, int maxArgs) {
    int n = 0;
    const char *p = fmt;
    while (*p && n < maxArgs) {
        if (*p != '%') {
            p = strchr(p, '%');
            if (!p) break;
        }
        p++; /* skip '%' */
        if (!*p) break;

        /* Skip flags */
        int flag_long = 0;
        for (;;) {
            switch (*p) {
            case '-': case '+': case ' ': case '0': case ',': case '!': case '#':
                p++; continue;
            default:
                break;
            }
            break;
        }

        /* Width */
        if (*p == '*') {
            int w = va_arg(ap, int);
            out[n++] = (uint64_t)(int64_t)w;
            if (n >= maxArgs) break;
            p++;
        } else {
            while (*p >= '0' && *p <= '9') p++;
        }

        /* Precision */
        if (*p == '.') {
            p++;
            if (*p == '*') {
                int pr = va_arg(ap, int);
                out[n++] = (uint64_t)(int64_t)pr;
                if (n >= maxArgs) break;
                p++;
            } else {
                while (*p >= '0' && *p <= '9') p++;
            }
        }

        /* Length modifier */
        if (*p == 'l') {
            flag_long = 1; p++;
            if (*p == 'l') { flag_long = 2; p++; }
        }

        if (!*p) break;

        /* Specifier — extract the appropriate type */
        switch (*p) {
        case 'd': case 'i':
            /* Signed integer */
            if (flag_long >= 2)
                out[n++] = (uint64_t)va_arg(ap, long long);
            else if (flag_long == 1)
                out[n++] = (uint64_t)va_arg(ap, long);
            else
                out[n++] = (uint64_t)(int64_t)va_arg(ap, int);
            break;
        case 'u': case 'o': case 'x': case 'X':
            /* Unsigned integer */
            if (flag_long >= 2)
                out[n++] = (uint64_t)va_arg(ap, unsigned long long);
            else if (flag_long == 1)
                out[n++] = (uint64_t)va_arg(ap, unsigned long);
            else
                out[n++] = (uint64_t)va_arg(ap, unsigned int);
            break;
        case 'f': case 'e': case 'E': case 'g': case 'G':
            /* Double — bitcast to uint64 */
            {
                double d = va_arg(ap, double);
                memcpy(&out[n], &d, sizeof(d));
                n++;
            }
            break;
        case 's': case 'z': case 'Q': case 'q': case 'w':
            /* String pointer */
            out[n++] = (uint64_t)(uintptr_t)va_arg(ap, char*);
            break;
        case 'c':
            /* Char (promoted to int in va_args) */
            out[n++] = (uint64_t)va_arg(ap, unsigned int);
            break;
        case 'p':
            /* Pointer */
            out[n++] = (uint64_t)(uintptr_t)va_arg(ap, void*);
            break;
        case 'n':
            /* Write-back pointer */
            out[n++] = (uint64_t)(uintptr_t)va_arg(ap, int*);
            break;
        case 'T':
            /* Token* or Expr* depending on # flag */
            out[n++] = (uint64_t)(uintptr_t)va_arg(ap, void*);
            break;
        case 'r':
            /* SrcItem* */
            out[n++] = (uint64_t)(uintptr_t)va_arg(ap, void*);
            break;
        case '%':
            /* Literal % — no arg consumed */
            break;
        default:
            /* Unknown specifier — skip */
            break;
        }
        p++;
    }
    return n;
}

__attribute__((visibility("default")))
void sqlite3_str_appendf(sqlite3_str *p, const char *zFormat, ...) {
    va_list ap;
    va_start(ap, zFormat);

    if (p->printfFlags & SQLITE_PRINTF_SQLFUNC) {
        /* SQLFUNC: first vararg is a PrintfArguments pointer */
        PrintfArguments *pArgList = va_arg(ap, PrintfArguments *);
        sqlite3_str_vappendf_sqlfunc(p, zFormat, pArgList);
    } else {
        /* Normal: extract va_args into packed array, call Rust */
        uint64_t packed[30];
        int nArgs = extract_va_to_packed(zFormat, ap, packed, 30);
        sqlite3_str_vappendf_packed(p, zFormat, packed, nArgs);
    }

    va_end(ap);
}
