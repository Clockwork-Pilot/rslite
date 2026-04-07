/*
** C variadic entry point for sqlite3_snprintf.
** Calls Rust's sqlite3_vsnprintf with the extracted va_list.
*/

#include <stdarg.h>

/* Rust helper — sqlite3_vsnprintf takes va_list */
extern char *sqlite3_vsnprintf(int n, char *zBuf, const char *zFormat, va_list ap);

__attribute__((visibility("default")))
char *sqlite3_snprintf(int n, char *zBuf, const char *zFormat, ...) {
    va_list ap;
    va_start(ap, zFormat);
    char *result = sqlite3_vsnprintf(n, zBuf, zFormat, ap);
    va_end(ap);
    return result;
}
