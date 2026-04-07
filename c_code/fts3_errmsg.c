/*
** C variadic wrapper for sqlite3Fts3ErrMsg.
** Provides the exported symbol for tclsqlite (fts3_term.rs).
*/

#include <stdarg.h>

extern char *sqlite3_vmprintf(const char *zFormat, va_list ap);
extern void sqlite3_free(void *p);

void sqlite3Fts3ErrMsg(char **pzErr, const char *zFormat, ...) {
    va_list ap;
    va_start(ap, zFormat);
    sqlite3_free(*pzErr);
    *pzErr = sqlite3_vmprintf(zFormat, ap);
    va_end(ap);
}
