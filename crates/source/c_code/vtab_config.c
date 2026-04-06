/*
** C variadic entry point for sqlite3_vtab_config.
** Extracts va_args per op code and packs into uint64_t[] for Rust.
*/

#include <stdarg.h>
#include <stdint.h>

/* Rust non-variadic implementation */
extern int sqlite3_vtab_config_args(void *db, int op, const uint64_t *args);

#define SQLITE_VTAB_CONSTRAINT_SUPPORT  1

__attribute__((visibility("default")))
int sqlite3_vtab_config(void *db, int op, ...) {
    va_list ap;
    uint64_t args[1] = {0};

    va_start(ap, op);
    switch (op) {
    case SQLITE_VTAB_CONSTRAINT_SUPPORT:
        /* (int) */
        args[0] = (uint64_t)va_arg(ap, int);
        break;
    default:
        /* INNOCUOUS, DIRECTONLY, USES_ALL_SCHEMAS: no args */
        break;
    }
    va_end(ap);

    return sqlite3_vtab_config_args(db, op, args);
}
