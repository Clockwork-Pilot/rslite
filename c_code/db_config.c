/*
** C variadic entry point for sqlite3_db_config.
** Extracts va_args per op code and packs into uint64_t[] for Rust.
*/

#include <stdarg.h>
#include <stdint.h>

/* Rust non-variadic implementation */
extern int sqlite3_db_config_args(void *db, int op, const uint64_t *args);

/* SQLITE_DBCONFIG constants — must match sqlite3.h */
#define SQLITE_DBCONFIG_MAINDBNAME            1000
#define SQLITE_DBCONFIG_LOOKASIDE             1001
/* 1002..1022 are all flag ops: (int, int*) */

__attribute__((visibility("default")))
int sqlite3_db_config(void *db, int op, ...) {
    va_list ap;
    uint64_t args[3] = {0};

    va_start(ap, op);
    switch (op) {
    case SQLITE_DBCONFIG_MAINDBNAME:
        /* (const char *) */
        args[0] = (uint64_t)(uintptr_t)va_arg(ap, const char*);
        break;

    case SQLITE_DBCONFIG_LOOKASIDE:
        /* (void*, int, int) */
        args[0] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        args[1] = (uint64_t)va_arg(ap, int);
        args[2] = (uint64_t)va_arg(ap, int);
        break;

    default:
        /* All flag ops 1002..1022: (int, int*) */
        args[0] = (uint64_t)va_arg(ap, int);
        args[1] = (uint64_t)(uintptr_t)va_arg(ap, int*);
        break;
    }
    va_end(ap);

    return sqlite3_db_config_args(db, op, args);
}
