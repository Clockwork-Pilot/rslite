/*
 * C variadic wrapper for sqlite3_log.
 * Formats the message using sqlite3_vsnprintf, then calls Rust sqlite3_log_formatted.
 */
#include <stdarg.h>
#include <stdint.h>

/* Rust entry point — checks xLog and invokes the callback with pre-formatted message */
extern void sqlite3_log_formatted(int iErrCode, const char *zMsg);

/* sqlite3_vsnprintf is already exported from Rust via c_code/printf_c.c */
extern char *sqlite3_vsnprintf(int n, char *zBuf, const char *zFormat, va_list ap);

void sqlite3_log(int iErrCode, const char *zFormat, ...) {
    va_list ap;
    char zMsg[700];
    va_start(ap, zFormat);
    sqlite3_vsnprintf(sizeof(zMsg), zMsg, zFormat, ap);
    va_end(ap);
    sqlite3_log_formatted(iErrCode, zMsg);
}
