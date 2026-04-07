/*
** C variadic entry point for sqlite3_mprintf.
** Calls Rust's sqlite3_vmprintf with the extracted va_list.
*/

#include <stdarg.h>

/* Rust helper — sqlite3_vmprintf takes va_list */
extern char *sqlite3_vmprintf(const char *zFormat, va_list ap);

__attribute__((visibility("default")))
char *sqlite3_mprintf(const char *zFormat, ...) {
    va_list ap;
    va_start(ap, zFormat);
    char *result = sqlite3_vmprintf(zFormat, ap);
    va_end(ap);
    return result;
}
