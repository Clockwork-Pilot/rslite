unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    fn sqlite3_exec(
        _: *mut sqlite3,
        sql: *const ::core::ffi::c_char,
        callback: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut *mut ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        _: *mut ::core::ffi::c_void,
        errmsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_get_autocommit(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_db_filename(
        db: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
    ) -> sqlite3_filename;
    fn sqlite3_file_control(
        _: *mut sqlite3,
        zDbName: *const ::core::ffi::c_char,
        op: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_log(
        iErrCode: ::core::ffi::c_int,
        zFormat: *const ::core::ffi::c_char,
        ...
    );
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_file {
    pub pMethods: *const sqlite3_io_methods,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_io_methods {
    pub iVersion: ::core::ffi::c_int,
    pub xClose: Option<unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int>,
    pub xRead: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xWrite: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            *const ::core::ffi::c_void,
            ::core::ffi::c_int,
            sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xTruncate: Option<
        unsafe extern "C" fn(*mut sqlite3_file, sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xSync: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xFileSize: Option<
        unsafe extern "C" fn(*mut sqlite3_file, *mut sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xLock: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xUnlock: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xCheckReservedLock: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xFileControl: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xSectorSize: Option<
        unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
    >,
    pub xDeviceCharacteristics: Option<
        unsafe extern "C" fn(*mut sqlite3_file) -> ::core::ffi::c_int,
    >,
    pub xShmMap: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xShmLock: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xShmBarrier: Option<unsafe extern "C" fn(*mut sqlite3_file) -> ()>,
    pub xShmUnmap: Option<
        unsafe extern "C" fn(*mut sqlite3_file, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xFetch: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            sqlite3_int64,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xUnfetch: Option<
        unsafe extern "C" fn(
            *mut sqlite3_file,
            sqlite3_int64,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
pub type sqlite3_filename = *const ::core::ffi::c_char;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_MISUSE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_FCNTL_FILE_POINTER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_mmap_warm(
    mut db: *mut sqlite3,
    mut zDb: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pgsz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nTotal: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        if 0 as ::core::ffi::c_int == sqlite3_get_autocommit(db) {
            return SQLITE_MISUSE;
        }
        zSql = sqlite3_mprintf(
            b"BEGIN; SELECT * FROM %s%q%ssqlite_schema\0".as_ptr()
                as *const ::core::ffi::c_char,
            if !zDb.is_null() {
                b"'\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            if !zDb.is_null() {
                zDb
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            if !zDb.is_null() {
                b"'.\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if zSql.is_null() {
            return SQLITE_NOMEM;
        }
        rc = sqlite3_exec(
            db,
            zSql,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        if rc == SQLITE_OK {
            zSql = sqlite3_mprintf(
                b"PRAGMA %s%q%spage_size\0".as_ptr() as *const ::core::ffi::c_char,
                if !zDb.is_null() {
                    b"'\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                if !zDb.is_null() {
                    zDb
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                if !zDb.is_null() {
                    b"'.\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            if zSql.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                let mut pPgsz: *mut sqlite3_stmt = ::core::ptr::null_mut::<
                    sqlite3_stmt,
                >();
                rc = sqlite3_prepare_v2(
                    db,
                    zSql,
                    -1 as ::core::ffi::c_int,
                    &raw mut pPgsz,
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                );
                sqlite3_free(zSql as *mut ::core::ffi::c_void);
                if rc == SQLITE_OK {
                    if sqlite3_step(pPgsz) == SQLITE_ROW {
                        pgsz = sqlite3_column_int(pPgsz, 0 as ::core::ffi::c_int);
                    }
                    rc = sqlite3_finalize(pPgsz);
                }
                if rc == SQLITE_OK && pgsz == 0 as ::core::ffi::c_int {
                    rc = SQLITE_ERROR;
                }
            }
        }
        if rc == SQLITE_OK {
            let mut rc2: ::core::ffi::c_int = 0;
            let mut pFd: *mut sqlite3_file = ::core::ptr::null_mut::<sqlite3_file>();
            rc = sqlite3_file_control(
                db,
                zDb,
                SQLITE_FCNTL_FILE_POINTER,
                &raw mut pFd as *mut ::core::ffi::c_void,
            );
            if rc == SQLITE_OK && (*(*pFd).pMethods).iVersion >= 3 as ::core::ffi::c_int
            {
                let mut iPg: sqlite3_int64 = 1 as sqlite3_int64;
                let mut p: *const sqlite3_io_methods = (*pFd).pMethods
                    as *const sqlite3_io_methods;
                loop {
                    let mut pMap: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
                        ::core::ffi::c_uchar,
                    >();
                    rc = (*p)
                        .xFetch
                        .expect(
                            "non-null function pointer",
                        )(
                        pFd,
                        pgsz as sqlite3_int64 * iPg,
                        pgsz,
                        &raw mut pMap as *mut *mut ::core::ffi::c_void,
                    );
                    if rc != SQLITE_OK || pMap.is_null() {
                        break;
                    }
                    nTotal = nTotal
                        .wrapping_add(
                            *pMap.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_uint,
                        );
                    nTotal = nTotal
                        .wrapping_add(
                            *pMap.offset((pgsz - 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_uint,
                        );
                    rc = (*p)
                        .xUnfetch
                        .expect(
                            "non-null function pointer",
                        )(
                        pFd,
                        pgsz as sqlite3_int64 * iPg,
                        pMap as *mut ::core::ffi::c_void,
                    );
                    if rc != SQLITE_OK {
                        break;
                    }
                    iPg += 1;
                }
                sqlite3_log(
                    SQLITE_OK,
                    b"sqlite3_mmap_warm_cache: Warmed up %d pages of %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    if iPg == 1 as ::core::ffi::c_longlong {
                        0 as ::core::ffi::c_longlong
                    } else {
                        iPg as ::core::ffi::c_longlong
                    },
                    sqlite3_db_filename(db, zDb),
                );
            }
            rc2 = sqlite3_exec(
                db,
                b"END\0".as_ptr() as *const ::core::ffi::c_char,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            if rc == SQLITE_OK {
                rc = rc2;
            }
        }
        return rc;
    }
}
