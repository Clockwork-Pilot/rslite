/*
** C implementation of sqlite3_vsnprintf.
** Receives va_list from C callers (e.g. sqlite3_snprintf),
** calls Rust helpers for StrAccum init and formatting.
*/

#include <stdarg.h>
#include <stdint.h>
#include <string.h>

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

/* Rust helpers */
extern void sqlite3StrAccumInit(sqlite3_str *p, void *db, char *zBase, int n, int mx);
extern void sqlite3_str_vappendf_va(sqlite3_str *p, const char *zFormat, va_list ap);

__attribute__((visibility("default")))
char *sqlite3_vsnprintf(int n, char *zBuf, const char *zFormat, va_list ap) {
    sqlite3_str acc;
    if (n <= 0) {
        return zBuf;
    }
    memset(&acc, 0, sizeof(acc));
    sqlite3StrAccumInit(&acc, (void*)0, zBuf, n, 0);
    sqlite3_str_vappendf_va(&acc, zFormat, ap);
    zBuf[acc.nChar] = 0;
    return zBuf;
}
