/*
** C implementation of sqlite3_vmprintf.
** Receives va_list from C callers (e.g. sqlite3_mprintf),
** extracts args into packed uint64_t array, calls Rust for formatting.
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
extern int sqlite3_initialize(void);
extern void sqlite3StrAccumInit(sqlite3_str *p, void *db, char *zBase, int n, int mx);
extern void sqlite3_str_vappendf_packed(sqlite3_str *p, const char *zFormat,
                                        const uint64_t *args, int nArgs);
extern char *sqlite3StrAccumFinish(sqlite3_str *p);

/* SQLITE_MAX_LENGTH — must match sqliteLimit.h */
#ifndef SQLITE_MAX_LENGTH
# define SQLITE_MAX_LENGTH 1000000000
#endif

/* Defined in printf_c.c */
int extract_va_to_packed(const char *fmt, va_list ap, uint64_t *out, int maxArgs);

__attribute__((visibility("default")))
char *sqlite3_vmprintf(const char *zFormat, va_list ap) {
    char *z;
    char zBase[70];
    sqlite3_str acc;

    if (sqlite3_initialize()) {
        return (char*)0;
    }
    memset(&acc, 0, sizeof(acc));
    sqlite3StrAccumInit(&acc, (void*)0, zBase, (int)sizeof(zBase), SQLITE_MAX_LENGTH);

    uint64_t packed[30];
    int nArgs = extract_va_to_packed(zFormat, ap, packed, 30);
    sqlite3_str_vappendf_packed(&acc, zFormat, packed, nArgs);

    z = sqlite3StrAccumFinish(&acc);
    return z;
}
