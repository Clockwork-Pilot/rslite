/*
** C variadic entry point for sqlite3_test_control.
** Extracts va_args per op code and packs into uint64_t[] for Rust.
*/

#include <stdarg.h>
#include <stdint.h>
#include <string.h>

/* Rust non-variadic implementation */
extern int sqlite3_test_control_args(int op, const uint64_t *args);

/* SQLITE_TESTCTRL constants — must match sqlite3.h */
#define SQLITE_TESTCTRL_PRNG_SAVE             5
#define SQLITE_TESTCTRL_PRNG_RESTORE          6
#define SQLITE_TESTCTRL_FK_NO_ACTION          7
#define SQLITE_TESTCTRL_BITVEC_TEST           8
#define SQLITE_TESTCTRL_FAULT_INSTALL         9
#define SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS  10
#define SQLITE_TESTCTRL_PENDING_BYTE         11
#define SQLITE_TESTCTRL_ASSERT               12
#define SQLITE_TESTCTRL_ALWAYS               13
#define SQLITE_TESTCTRL_JSON_SELFCHECK       14
#define SQLITE_TESTCTRL_OPTIMIZATIONS        15
#define SQLITE_TESTCTRL_GETOPT               16
#define SQLITE_TESTCTRL_INTERNAL_FUNCTIONS   17
#define SQLITE_TESTCTRL_LOCALTIME_FAULT      18
#define SQLITE_TESTCTRL_ONCE_RESET_THRESHOLD 19
#define SQLITE_TESTCTRL_NEVER_CORRUPT        20
#define SQLITE_TESTCTRL_VDBE_COVERAGE        21
#define SQLITE_TESTCTRL_BYTEORDER            22
#define SQLITE_TESTCTRL_ISINIT               23
#define SQLITE_TESTCTRL_SORTER_MMAP          24
#define SQLITE_TESTCTRL_IMPOSTER             25
#define SQLITE_TESTCTRL_RESULT_INTREAL       27
#define SQLITE_TESTCTRL_PRNG_SEED            28
#define SQLITE_TESTCTRL_EXTRA_SCHEMA_CHECKS  29
#define SQLITE_TESTCTRL_SEEK_COUNT           30
#define SQLITE_TESTCTRL_TRACEFLAGS           31
#define SQLITE_TESTCTRL_LOGEST               33

__attribute__((visibility("default")))
int sqlite3_test_control(int op, ...) {
    va_list ap;
    uint64_t args[4] = {0};

    va_start(ap, op);
    switch (op) {
    /* No args */
    case SQLITE_TESTCTRL_PRNG_SAVE:
    case SQLITE_TESTCTRL_PRNG_RESTORE:
    case SQLITE_TESTCTRL_ASSERT:
    case SQLITE_TESTCTRL_BYTEORDER:
    case SQLITE_TESTCTRL_ISINIT:
    case SQLITE_TESTCTRL_VDBE_COVERAGE:
    case SQLITE_TESTCTRL_JSON_SELFCHECK:
        break;

    /* (int) */
    case SQLITE_TESTCTRL_ALWAYS:
    case SQLITE_TESTCTRL_NEVER_CORRUPT:
    case SQLITE_TESTCTRL_EXTRA_SCHEMA_CHECKS:
    case SQLITE_TESTCTRL_ONCE_RESET_THRESHOLD:
        args[0] = (uint64_t)va_arg(ap, int);
        break;

    /* (unsigned int) */
    case SQLITE_TESTCTRL_PENDING_BYTE:
        args[0] = (uint64_t)va_arg(ap, unsigned int);
        break;

    /* (ptr) */
    case SQLITE_TESTCTRL_INTERNAL_FUNCTIONS:
    case SQLITE_TESTCTRL_FAULT_INSTALL:
    case SQLITE_TESTCTRL_RESULT_INTREAL:
        args[0] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        break;

    /* (int, ptr) */
    case SQLITE_TESTCTRL_PRNG_SEED:
    case SQLITE_TESTCTRL_BITVEC_TEST:
    case SQLITE_TESTCTRL_TRACEFLAGS:
        args[0] = (uint64_t)va_arg(ap, int);
        args[1] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        break;

    /* (ptr, int) */
    case SQLITE_TESTCTRL_FK_NO_ACTION:
    case SQLITE_TESTCTRL_SORTER_MMAP:
        args[0] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        args[1] = (uint64_t)va_arg(ap, int);
        break;

    /* (ptr, u32) */
    case SQLITE_TESTCTRL_OPTIMIZATIONS:
        args[0] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        args[1] = (uint64_t)va_arg(ap, unsigned int);
        break;

    /* (ptr, ptr) */
    case SQLITE_TESTCTRL_GETOPT:
    case SQLITE_TESTCTRL_SEEK_COUNT:
    case SQLITE_TESTCTRL_BENIGN_MALLOC_HOOKS:
        args[0] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        args[1] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        break;

    /* (int, maybe ptr) */
    case SQLITE_TESTCTRL_LOCALTIME_FAULT: {
        int fault = va_arg(ap, int);
        args[0] = (uint64_t)fault;
        if (fault == 2) {
            args[1] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        }
        break;
    }

    /* (ptr, str, int, int) */
    case SQLITE_TESTCTRL_IMPOSTER:
        args[0] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        args[1] = (uint64_t)(uintptr_t)va_arg(ap, const char*);
        args[2] = (uint64_t)va_arg(ap, int);
        args[3] = (uint64_t)va_arg(ap, int);
        break;

    /* (double, ptr, ptr, ptr) */
    case SQLITE_TESTCTRL_LOGEST: {
        double rIn = va_arg(ap, double);
        uint64_t bits;
        memcpy(&bits, &rIn, sizeof(bits));
        args[0] = bits;
        args[1] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        args[2] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        args[3] = (uint64_t)(uintptr_t)va_arg(ap, void*);
        break;
    }

    default:
        break;
    }
    va_end(ap);

    return sqlite3_test_control_args(op, args);
}
