/*
** C variadic entry point for sqlite3_config.
** Extracts va_args per op code and packs into uint64_t[] for Rust.
*/

#include <stdarg.h>
#include <stdint.h>

/* Rust non-variadic implementation */
extern int sqlite3_config_args(int op, const uint64_t *args);

/* SqliteConfig enum values — must match sqlite3_h.rs */
#define SQLITE_CONFIG_SINGLETHREAD       1
#define SQLITE_CONFIG_MULTITHREAD        2
#define SQLITE_CONFIG_SERIALIZED         3
#define SQLITE_CONFIG_MALLOC             4
#define SQLITE_CONFIG_GETMALLOC          5
#define SQLITE_CONFIG_PAGECACHE          7
#define SQLITE_CONFIG_MEMSTATUS          9
#define SQLITE_CONFIG_MUTEX             10
#define SQLITE_CONFIG_GETMUTEX          11
#define SQLITE_CONFIG_LOOKASIDE         13
#define SQLITE_CONFIG_PCACHE            14
#define SQLITE_CONFIG_GETPCACHE         15
#define SQLITE_CONFIG_LOG               16
#define SQLITE_CONFIG_URI               17
#define SQLITE_CONFIG_PCACHE2           18
#define SQLITE_CONFIG_GETPCACHE2        19
#define SQLITE_CONFIG_COVERING_INDEX_SCAN 20
#define SQLITE_CONFIG_MMAP_SIZE         22
#define SQLITE_CONFIG_PCACHE_HDRSZ      24
#define SQLITE_CONFIG_PMASZ             25
#define SQLITE_CONFIG_STMTJRNL_SPILL   26
#define SQLITE_CONFIG_SMALL_MALLOC      27
#define SQLITE_CONFIG_MEMDB_MAXSIZE     29
#define SQLITE_CONFIG_ROWID_IN_VIEW     30

__attribute__((visibility("default")))
int sqlite3_config(int op, ...) {
    va_list ap;
    uint64_t args[4] = {0};

    va_start(ap, op);
    switch (op) {
    /* No args */
    case SQLITE_CONFIG_SINGLETHREAD:
    case SQLITE_CONFIG_MULTITHREAD:
    case SQLITE_CONFIG_SERIALIZED:
    case SQLITE_CONFIG_PCACHE:
    case SQLITE_CONFIG_GETPCACHE:
        break;

    /* (ptr) — struct pointer in or out */
    case SQLITE_CONFIG_MUTEX:
    case SQLITE_CONFIG_GETMUTEX:
    case SQLITE_CONFIG_MALLOC:
    case SQLITE_CONFIG_GETMALLOC:
    case SQLITE_CONFIG_PCACHE2:
    case SQLITE_CONFIG_GETPCACHE2:
    case SQLITE_CONFIG_PCACHE_HDRSZ:
    case SQLITE_CONFIG_ROWID_IN_VIEW:
        args[0] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        break;

    /* (int) */
    case SQLITE_CONFIG_MEMSTATUS:
    case SQLITE_CONFIG_SMALL_MALLOC:
    case SQLITE_CONFIG_URI:
    case SQLITE_CONFIG_COVERING_INDEX_SCAN:
    case SQLITE_CONFIG_STMTJRNL_SPILL:
        args[0] = (uint64_t)va_arg(ap, int);
        break;

    /* (unsigned int) */
    case SQLITE_CONFIG_PMASZ:
        args[0] = (uint64_t)va_arg(ap, unsigned int);
        break;

    /* (int, int) */
    case SQLITE_CONFIG_LOOKASIDE:
        args[0] = (uint64_t)va_arg(ap, int);
        args[1] = (uint64_t)va_arg(ap, int);
        break;

    /* (ptr, int, int) */
    case SQLITE_CONFIG_PAGECACHE:
        args[0] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        args[1] = (uint64_t)va_arg(ap, int);
        args[2] = (uint64_t)va_arg(ap, int);
        break;

    /* (ptr, ptr) — function pointer + context */
    case SQLITE_CONFIG_LOG:
        args[0] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        args[1] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        break;

    /* (i64) */
    case SQLITE_CONFIG_MEMDB_MAXSIZE:
        args[0] = (uint64_t)va_arg(ap, long long);
        break;

    /* (i64, i64) */
    case SQLITE_CONFIG_MMAP_SIZE:
        args[0] = (uint64_t)va_arg(ap, long long);
        args[1] = (uint64_t)va_arg(ap, long long);
        break;

    default:
        break;
    }
    va_end(ap);

    return sqlite3_config_args(op, args);
}
