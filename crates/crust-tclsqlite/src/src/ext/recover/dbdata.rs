use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_api_routines;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_int64(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_blob(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_void;
    fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_bytes(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_result_blob(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_double(_: *mut sqlite3_context, _: ::core::ffi::c_double);
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
    fn sqlite3_result_null(_: *mut sqlite3_context);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_text16le(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_text16be(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_create_module(
        db: *mut sqlite3,
        zName: *const ::core::ffi::c_char,
        p: *const sqlite3_module,
        pClientData: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3_declare_vtab(
        _: *mut sqlite3,
        zSQL: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_vtab_config(
        _: *mut sqlite3,
        op: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab {
    pub pModule: *const sqlite3_module,
    pub nRef: ::core::ffi::c_int,
    pub zErrMsg: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_module {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xConnect: Option<
        unsafe extern "C" fn(
            *mut sqlite3,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_vtab,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xBestIndex: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut sqlite3_index_info,
        ) -> ::core::ffi::c_int,
    >,
    pub xDisconnect: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
    >,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut *mut sqlite3_vtab_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xFilter: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub xNext: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xEof: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
    >,
    pub xColumn: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_context,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRowid: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xUpdate: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
            *mut sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xBegin: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xSync: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xCommit: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xRollback: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xFindFunction: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *mut Option<
                unsafe extern "C" fn(
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> (),
            >,
            *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub xRename: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub xSavepoint: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xRelease: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xRollbackTo: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xShadowName: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
    pub xIntegrity: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab_cursor {
    pub pVtab: *mut sqlite3_vtab,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_info {
    pub nConstraint: ::core::ffi::c_int,
    pub aConstraint: *mut sqlite3_index_constraint,
    pub nOrderBy: ::core::ffi::c_int,
    pub aOrderBy: *mut sqlite3_index_orderby,
    pub aConstraintUsage: *mut sqlite3_index_constraint_usage,
    pub idxNum: ::core::ffi::c_int,
    pub idxStr: *mut ::core::ffi::c_char,
    pub needToFreeIdxStr: ::core::ffi::c_int,
    pub orderByConsumed: ::core::ffi::c_int,
    pub estimatedCost: ::core::ffi::c_double,
    pub estimatedRows: sqlite3_int64,
    pub idxFlags: ::core::ffi::c_int,
    pub colUsed: sqlite3_uint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint_usage {
    pub argvIndex: ::core::ffi::c_int,
    pub omit: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_orderby {
    pub iColumn: ::core::ffi::c_int,
    pub desc: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_index_constraint {
    pub iColumn: ::core::ffi::c_int,
    pub op: ::core::ffi::c_uchar,
    pub usable: ::core::ffi::c_uchar,
    pub iTermOffset: ::core::ffi::c_int,
}
pub type u8_0 = ::core::ffi::c_uchar;
pub type u32_0 = ::core::ffi::c_uint;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbdataTable {
    pub base: sqlite3_vtab,
    pub db: *mut sqlite3,
    pub pStmt: *mut sqlite3_stmt,
    pub bPtr: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbdataCursor {
    pub base: sqlite3_vtab_cursor,
    pub pStmt: *mut sqlite3_stmt,
    pub iPgno: ::core::ffi::c_int,
    pub aPage: *mut u8_0,
    pub nPage: ::core::ffi::c_int,
    pub nCell: ::core::ffi::c_int,
    pub iCell: ::core::ffi::c_int,
    pub bOnePage: ::core::ffi::c_int,
    pub szDb: ::core::ffi::c_int,
    pub iRowid: sqlite3_int64,
    pub rec: DbdataBuffer,
    pub nRec: sqlite3_int64,
    pub nHdr: sqlite3_int64,
    pub iField: ::core::ffi::c_int,
    pub pHdrPtr: *mut u8_0,
    pub pPtr: *mut u8_0,
    pub enc: u32_0,
    pub iIntkey: sqlite3_int64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DbdataBuffer {
    pub aBuf: *mut u8_0,
    pub nBuf: sqlite3_int64,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CONSTRAINT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_UTF16LE: u32_0 = 2 as u32_0;
pub const SQLITE_UTF16BE: u32_0 = 3 as u32_0;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_VTAB_USES_ALL_SCHEMAS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DBDATA_PADDING_BYTES: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const DBDATA_COLUMN_PGNO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DBDATA_COLUMN_CELL: ::core::ffi::c_int = 1;
pub const DBDATA_COLUMN_FIELD: ::core::ffi::c_int = 2;
pub const DBDATA_COLUMN_VALUE: ::core::ffi::c_int = 3;
pub const DBDATA_COLUMN_SCHEMA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DBDATA_SCHEMA: [::core::ffi::c_char; 95] = unsafe {
    ::core::mem::transmute::<
        [u8; 95],
        [::core::ffi::c_char; 95],
    >(
        *b"CREATE TABLE x(  pgno INTEGER,  cell INTEGER,  field INTEGER,  value ANY,  schema TEXT HIDDEN)\0",
    )
};
pub const DBPTR_COLUMN_PGNO: ::core::ffi::c_int = 0;
pub const DBPTR_COLUMN_CHILD: ::core::ffi::c_int = 1;
pub const DBPTR_COLUMN_SCHEMA: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DBPTR_SCHEMA: [::core::ffi::c_char; 68] = unsafe {
    ::core::mem::transmute::<
        [u8; 68],
        [::core::ffi::c_char; 68],
    >(*b"CREATE TABLE x(  pgno INTEGER,  child INTEGER,  schema TEXT HIDDEN)\0")
};
unsafe extern "C" fn dbdataBufferSize(
    mut pBuf: *mut DbdataBuffer,
    mut nMin: sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        if nMin > (*pBuf).nBuf {
            let mut nNew: sqlite3_int64 = nMin + 16384 as sqlite3_int64;
            let mut aNew: *mut u8_0 = sqlite3_realloc64(
                (*pBuf).aBuf as *mut ::core::ffi::c_void,
                nNew as sqlite3_uint64,
            ) as *mut u8_0;
            if aNew.is_null() {
                return SQLITE_NOMEM;
            }
            (*pBuf).aBuf = aNew;
            (*pBuf).nBuf = nNew;
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn dbdataBufferFree(mut pBuf: *mut DbdataBuffer) {
    unsafe {
        sqlite3_free((*pBuf).aBuf as *mut ::core::ffi::c_void);
        memset(
            pBuf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<DbdataBuffer>() as size_t,
        );
    }
}
unsafe extern "C" fn dbdataConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut DbdataTable = ::core::ptr::null_mut::<DbdataTable>();
        let mut rc: ::core::ffi::c_int = sqlite3_declare_vtab(
            db,
            if !pAux.is_null() { DBPTR_SCHEMA.as_ptr() } else { DBDATA_SCHEMA.as_ptr() },
        );
        sqlite3_vtab_config(db, SQLITE_VTAB_USES_ALL_SCHEMAS);
        if rc == SQLITE_OK {
            pTab = sqlite3_malloc64(
                ::core::mem::size_of::<DbdataTable>() as sqlite3_uint64,
            ) as *mut DbdataTable;
            if pTab.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                memset(
                    pTab as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<DbdataTable>() as size_t,
                );
                (*pTab).db = db;
                (*pTab).bPtr = !pAux.is_null() as ::core::ffi::c_int;
            }
        }
        *ppVtab = pTab as *mut sqlite3_vtab;
        return rc;
    }
}
unsafe extern "C" fn dbdataDisconnect(
    mut pVtab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut DbdataTable = pVtab as *mut DbdataTable;
        if !pTab.is_null() {
            sqlite3_finalize((*pTab).pStmt);
            sqlite3_free(pVtab as *mut ::core::ffi::c_void);
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn dbdataBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdx: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut DbdataTable = tab as *mut DbdataTable;
        let mut i: ::core::ffi::c_int = 0;
        let mut iSchema: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut iPgno: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut colSchema: ::core::ffi::c_int = if (*pTab).bPtr != 0 {
            DBPTR_COLUMN_SCHEMA
        } else {
            DBDATA_COLUMN_SCHEMA
        };
        i = 0 as ::core::ffi::c_int;
        while i < (*pIdx).nConstraint {
            let mut p: *mut sqlite3_index_constraint = (*pIdx)
                .aConstraint
                .offset(i as isize) as *mut sqlite3_index_constraint;
            if (*p).op as ::core::ffi::c_int == SQLITE_INDEX_CONSTRAINT_EQ {
                if (*p).iColumn == colSchema {
                    if (*p).usable as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        return SQLITE_CONSTRAINT;
                    }
                    iSchema = i;
                }
                if (*p).iColumn == DBDATA_COLUMN_PGNO
                    && (*p).usable as ::core::ffi::c_int != 0
                {
                    iPgno = i;
                }
            }
            i += 1;
        }
        if iSchema >= 0 as ::core::ffi::c_int {
            (*(*pIdx).aConstraintUsage.offset(iSchema as isize)).argvIndex = 1
                as ::core::ffi::c_int;
            (*(*pIdx).aConstraintUsage.offset(iSchema as isize)).omit = 1
                as ::core::ffi::c_uchar;
        }
        if iPgno >= 0 as ::core::ffi::c_int {
            (*(*pIdx).aConstraintUsage.offset(iPgno as isize)).argvIndex = 1
                as ::core::ffi::c_int
                + (iSchema >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            (*(*pIdx).aConstraintUsage.offset(iPgno as isize)).omit = 1
                as ::core::ffi::c_uchar;
            (*pIdx).estimatedCost = 100 as ::core::ffi::c_int as ::core::ffi::c_double;
            (*pIdx).estimatedRows = 50 as sqlite3_int64;
            if (*pTab).bPtr == 0 as ::core::ffi::c_int && (*pIdx).nOrderBy != 0
                && (*(*pIdx).aOrderBy.offset(0 as ::core::ffi::c_int as isize)).desc
                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                let mut iCol: ::core::ffi::c_int = (*(*pIdx)
                    .aOrderBy
                    .offset(0 as ::core::ffi::c_int as isize))
                    .iColumn;
                if (*pIdx).nOrderBy == 1 as ::core::ffi::c_int {
                    (*pIdx).orderByConsumed = (iCol == 0 as ::core::ffi::c_int
                        || iCol == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
                } else if (*pIdx).nOrderBy == 2 as ::core::ffi::c_int
                    && (*(*pIdx).aOrderBy.offset(1 as ::core::ffi::c_int as isize)).desc
                        as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && iCol == 0 as ::core::ffi::c_int
                {
                    (*pIdx).orderByConsumed = ((*(*pIdx)
                        .aOrderBy
                        .offset(1 as ::core::ffi::c_int as isize))
                        .iColumn == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
                }
            }
        } else {
            (*pIdx).estimatedCost = 100000000 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            (*pIdx).estimatedRows = 1000000000 as sqlite3_int64;
        }
        (*pIdx).idxNum = (if iSchema >= 0 as ::core::ffi::c_int {
            0x1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })
            | (if iPgno >= 0 as ::core::ffi::c_int {
                0x2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            });
        return SQLITE_OK;
    }
}
unsafe extern "C" fn dbdataOpen(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut DbdataCursor = ::core::ptr::null_mut::<DbdataCursor>();
        pCsr = sqlite3_malloc64(::core::mem::size_of::<DbdataCursor>() as sqlite3_uint64)
            as *mut DbdataCursor;
        if pCsr.is_null() {
            return SQLITE_NOMEM
        } else {
            memset(
                pCsr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<DbdataCursor>() as size_t,
            );
            (*pCsr).base.pVtab = pVTab;
        }
        *ppCursor = pCsr as *mut sqlite3_vtab_cursor;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn dbdataResetCursor(mut pCsr: *mut DbdataCursor) {
    unsafe {
        let mut pTab: *mut DbdataTable = (*pCsr).base.pVtab as *mut DbdataTable;
        if (*pTab).pStmt.is_null() {
            (*pTab).pStmt = (*pCsr).pStmt;
        } else {
            sqlite3_finalize((*pCsr).pStmt);
        }
        (*pCsr).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        (*pCsr).iPgno = 1 as ::core::ffi::c_int;
        (*pCsr).iCell = 0 as ::core::ffi::c_int;
        (*pCsr).iField = 0 as ::core::ffi::c_int;
        (*pCsr).bOnePage = 0 as ::core::ffi::c_int;
        sqlite3_free((*pCsr).aPage as *mut ::core::ffi::c_void);
        dbdataBufferFree(&raw mut (*pCsr).rec);
        (*pCsr).aPage = ::core::ptr::null_mut::<u8_0>();
        (*pCsr).nRec = 0 as sqlite3_int64;
    }
}
unsafe extern "C" fn dbdataClose(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut DbdataCursor = pCursor as *mut DbdataCursor;
        dbdataResetCursor(pCsr);
        sqlite3_free(pCsr as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn get_uint16(mut a: *mut ::core::ffi::c_uchar) -> u32_0 {
    unsafe {
        return ((*a.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *a.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as u32_0;
    }
}
unsafe extern "C" fn get_uint32(mut a: *mut ::core::ffi::c_uchar) -> u32_0 {
    unsafe {
        return (*a.offset(0 as ::core::ffi::c_int as isize) as u32_0)
            << 24 as ::core::ffi::c_int
            | (*a.offset(1 as ::core::ffi::c_int as isize) as u32_0)
                << 16 as ::core::ffi::c_int
            | (*a.offset(2 as ::core::ffi::c_int as isize) as u32_0)
                << 8 as ::core::ffi::c_int
            | *a.offset(3 as ::core::ffi::c_int as isize) as u32_0;
    }
}
unsafe extern "C" fn dbdataLoadPage(
    mut pCsr: *mut DbdataCursor,
    mut pgno: u32_0,
    mut ppPage: *mut *mut u8_0,
    mut pnPage: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc2: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pStmt: *mut sqlite3_stmt = (*pCsr).pStmt;
        *ppPage = ::core::ptr::null_mut::<u8_0>();
        *pnPage = 0 as ::core::ffi::c_int;
        if pgno > 0 as ::core::ffi::c_uint {
            sqlite3_bind_int64(pStmt, 2 as ::core::ffi::c_int, pgno as sqlite3_int64);
            if SQLITE_ROW == sqlite3_step(pStmt) {
                let mut nCopy: ::core::ffi::c_int = sqlite3_column_bytes(
                    pStmt,
                    0 as ::core::ffi::c_int,
                );
                if nCopy > 0 as ::core::ffi::c_int {
                    let mut pPage: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                    pPage = sqlite3_malloc64(
                        (nCopy + DBDATA_PADDING_BYTES) as sqlite3_uint64,
                    ) as *mut u8_0;
                    if pPage.is_null() {
                        rc = SQLITE_NOMEM;
                    } else {
                        let mut pCopy: *const u8_0 = sqlite3_column_blob(
                            pStmt,
                            0 as ::core::ffi::c_int,
                        ) as *const u8_0;
                        memcpy(
                            pPage as *mut ::core::ffi::c_void,
                            pCopy as *const ::core::ffi::c_void,
                            nCopy as size_t,
                        );
                        memset(
                            pPage.offset(nCopy as isize) as *mut u8_0
                                as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            DBDATA_PADDING_BYTES as size_t,
                        );
                    }
                    *ppPage = pPage;
                    *pnPage = nCopy;
                }
            }
            rc2 = sqlite3_reset(pStmt);
            if rc == SQLITE_OK {
                rc = rc2;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn dbdataGetVarint(
    mut z: *const u8_0,
    mut pVal: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut u: sqlite3_uint64 = 0 as sqlite3_uint64;
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            u = ((u as ::core::ffi::c_ulonglong) << 7 as ::core::ffi::c_int)
                .wrapping_add(
                    (*z.offset(i as isize) as ::core::ffi::c_int
                        & 0x7f as ::core::ffi::c_int) as ::core::ffi::c_ulonglong,
                ) as sqlite3_uint64;
            if *z.offset(i as isize) as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                *pVal = u as sqlite3_int64;
                return i + 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        u = ((u as ::core::ffi::c_ulonglong) << 8 as ::core::ffi::c_int)
            .wrapping_add(
                (*z.offset(i as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int) as ::core::ffi::c_ulonglong,
            ) as sqlite3_uint64;
        *pVal = u as sqlite3_int64;
        return 9 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn dbdataGetVarintU32(
    mut z: *const u8_0,
    mut pVal: *mut sqlite3_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut val: sqlite3_int64 = 0;
        let mut nRet: ::core::ffi::c_int = dbdataGetVarint(z, &raw mut val);
        if val < 0 as ::core::ffi::c_longlong
            || val > 0xffffffff as ::core::ffi::c_longlong
        {
            val = 0 as sqlite3_int64;
        }
        *pVal = val;
        return nRet;
    }
}
unsafe extern "C" fn dbdataValueBytes(
    mut eType: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        match eType {
            0 | 8 | 9 | 10 | 11 => return 0 as ::core::ffi::c_int,
            1 => return 1 as ::core::ffi::c_int,
            2 => return 2 as ::core::ffi::c_int,
            3 => return 3 as ::core::ffi::c_int,
            4 => return 4 as ::core::ffi::c_int,
            5 => return 6 as ::core::ffi::c_int,
            6 | 7 => return 8 as ::core::ffi::c_int,
            _ => {
                if eType > 0 as ::core::ffi::c_int {
                    return (eType - 12 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
                }
                return 0 as ::core::ffi::c_int;
            }
        };
    }
}
unsafe extern "C" fn dbdataValue(
    mut pCtx: *mut sqlite3_context,
    mut enc: u32_0,
    mut eType: ::core::ffi::c_int,
    mut pData: *mut u8_0,
    mut nData: sqlite3_int64,
) {
    unsafe {
        if eType >= 0 as ::core::ffi::c_int {
            if dbdataValueBytes(eType) as ::core::ffi::c_longlong <= nData {
                match eType {
                    0 | 10 | 11 => {
                        sqlite3_result_null(pCtx);
                    }
                    8 => {
                        sqlite3_result_int(pCtx, 0 as ::core::ffi::c_int);
                    }
                    9 => {
                        sqlite3_result_int(pCtx, 1 as ::core::ffi::c_int);
                    }
                    1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                        let mut v: sqlite3_uint64 = *pData
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_schar as sqlite3_uint64;
                        pData = pData.offset(1);
                        let mut c2rust_current_block_13: u64;
                        match eType {
                            7 | 6 => {
                                v = ((v as ::core::ffi::c_ulonglong)
                                    << 16 as ::core::ffi::c_int)
                                    .wrapping_add(
                                        ((*pData.offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                            as ::core::ffi::c_ulonglong,
                                    )
                                    .wrapping_add(
                                        *pData.offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_ulonglong,
                                    ) as sqlite3_uint64;
                                pData = pData.offset(2 as ::core::ffi::c_int as isize);
                                c2rust_current_block_13 = 11316695763593360198;
                            }
                            5 => {
                                c2rust_current_block_13 = 11316695763593360198;
                            }
                            4 => {
                                c2rust_current_block_13 = 3511887571667711723;
                            }
                            3 => {
                                c2rust_current_block_13 = 13501662918186941651;
                            }
                            2 => {
                                c2rust_current_block_13 = 3944213605077522399;
                            }
                            _ => {
                                c2rust_current_block_13 = 17833034027772472439;
                            }
                        }
                        match c2rust_current_block_13 {
                            11316695763593360198 => {
                                v = ((v as ::core::ffi::c_ulonglong)
                                    << 16 as ::core::ffi::c_int)
                                    .wrapping_add(
                                        ((*pData.offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                                            as ::core::ffi::c_ulonglong,
                                    )
                                    .wrapping_add(
                                        *pData.offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_ulonglong,
                                    ) as sqlite3_uint64;
                                pData = pData.offset(2 as ::core::ffi::c_int as isize);
                                c2rust_current_block_13 = 3511887571667711723;
                            }
                            _ => {}
                        }
                        match c2rust_current_block_13 {
                            3511887571667711723 => {
                                v = ((v as ::core::ffi::c_ulonglong)
                                    << 8 as ::core::ffi::c_int)
                                    .wrapping_add(
                                        *pData.offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_ulonglong,
                                    ) as sqlite3_uint64;
                                pData = pData.offset(1);
                                c2rust_current_block_13 = 13501662918186941651;
                            }
                            _ => {}
                        }
                        match c2rust_current_block_13 {
                            13501662918186941651 => {
                                v = ((v as ::core::ffi::c_ulonglong)
                                    << 8 as ::core::ffi::c_int)
                                    .wrapping_add(
                                        *pData.offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_ulonglong,
                                    ) as sqlite3_uint64;
                                pData = pData.offset(1);
                                c2rust_current_block_13 = 3944213605077522399;
                            }
                            _ => {}
                        }
                        match c2rust_current_block_13 {
                            3944213605077522399 => {
                                v = ((v as ::core::ffi::c_ulonglong)
                                    << 8 as ::core::ffi::c_int)
                                    .wrapping_add(
                                        *pData.offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_ulonglong,
                                    ) as sqlite3_uint64;
                                pData = pData.offset(1);
                            }
                            _ => {}
                        }
                        if eType == 7 as ::core::ffi::c_int {
                            let mut r: ::core::ffi::c_double = 0.;
                            memcpy(
                                &raw mut r as *mut ::core::ffi::c_void,
                                &raw mut v as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                            );
                            sqlite3_result_double(pCtx, r);
                        } else {
                            sqlite3_result_int64(pCtx, v as sqlite3_int64);
                        }
                    }
                    _ => {
                        let mut n: ::core::ffi::c_int = (eType
                            - 12 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
                        if eType % 2 as ::core::ffi::c_int != 0 {
                            match enc {
                                3 => {
                                    sqlite3_result_text16be(
                                        pCtx,
                                        pData as *mut ::core::ffi::c_void,
                                        n,
                                        ::core::mem::transmute::<
                                            ::libc::intptr_t,
                                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                                    );
                                }
                                2 => {
                                    sqlite3_result_text16le(
                                        pCtx,
                                        pData as *mut ::core::ffi::c_void,
                                        n,
                                        ::core::mem::transmute::<
                                            ::libc::intptr_t,
                                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                                    );
                                }
                                _ => {
                                    sqlite3_result_text(
                                        pCtx,
                                        pData as *mut ::core::ffi::c_char,
                                        n,
                                        ::core::mem::transmute::<
                                            ::libc::intptr_t,
                                            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                                        >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                                    );
                                }
                            }
                        } else {
                            sqlite3_result_blob(
                                pCtx,
                                pData as *const ::core::ffi::c_void,
                                n,
                                ::core::mem::transmute::<
                                    ::libc::intptr_t,
                                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                            );
                        }
                    }
                }
            } else if eType == 7 as ::core::ffi::c_int {
                sqlite3_result_double(pCtx, 0.0f64);
            } else if eType < 7 as ::core::ffi::c_int {
                sqlite3_result_int(pCtx, 0 as ::core::ffi::c_int);
            } else if eType % 2 as ::core::ffi::c_int != 0 {
                sqlite3_result_text(
                    pCtx,
                    b"\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            } else {
                sqlite3_result_blob(
                    pCtx,
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    SQLITE_STATIC,
                );
            }
        }
    }
}
pub const DBDATA_MX_FIELD: ::core::ffi::c_int = 32676 as ::core::ffi::c_int;
unsafe extern "C" fn dbdataNext(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut DbdataCursor = pCursor as *mut DbdataCursor;
        let mut pTab: *mut DbdataTable = (*pCursor).pVtab as *mut DbdataTable;
        (*pCsr).iRowid += 1;
        loop {
            let mut rc: ::core::ffi::c_int = 0;
            let mut iOff: ::core::ffi::c_int = if (*pCsr).iPgno
                == 1 as ::core::ffi::c_int
            {
                100 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            let mut bNextPage: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if (*pCsr).aPage.is_null() {
                loop {
                    if (*pCsr).bOnePage == 0 as ::core::ffi::c_int
                        && (*pCsr).iPgno > (*pCsr).szDb
                    {
                        return SQLITE_OK;
                    }
                    rc = dbdataLoadPage(
                        pCsr,
                        (*pCsr).iPgno as u32_0,
                        &raw mut (*pCsr).aPage,
                        &raw mut (*pCsr).nPage,
                    );
                    if rc != SQLITE_OK {
                        return rc;
                    }
                    if !(*pCsr).aPage.is_null()
                        && (*pCsr).nPage >= 256 as ::core::ffi::c_int
                    {
                        break;
                    }
                    sqlite3_free((*pCsr).aPage as *mut ::core::ffi::c_void);
                    (*pCsr).aPage = ::core::ptr::null_mut::<u8_0>();
                    if (*pCsr).bOnePage != 0 {
                        return SQLITE_OK;
                    }
                    (*pCsr).iPgno += 1;
                }
                (*pCsr).iCell = if (*pTab).bPtr != 0 {
                    -2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                };
                (*pCsr).nCell = get_uint16(
                    (*pCsr).aPage.offset((iOff + 3 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_uchar,
                ) as ::core::ffi::c_int;
                if (*pCsr).nCell
                    > ((*pCsr).nPage - 8 as ::core::ffi::c_int) / 6 as ::core::ffi::c_int
                {
                    (*pCsr).nCell = ((*pCsr).nPage - 8 as ::core::ffi::c_int)
                        / 6 as ::core::ffi::c_int;
                }
            }
            if (*pTab).bPtr != 0 {
                if *(*pCsr).aPage.offset(iOff as isize) as ::core::ffi::c_int
                    != 0x2 as ::core::ffi::c_int
                    && *(*pCsr).aPage.offset(iOff as isize) as ::core::ffi::c_int
                        != 0x5 as ::core::ffi::c_int
                {
                    (*pCsr).iCell = (*pCsr).nCell;
                }
                (*pCsr).iCell += 1;
                if (*pCsr).iCell >= (*pCsr).nCell {
                    sqlite3_free((*pCsr).aPage as *mut ::core::ffi::c_void);
                    (*pCsr).aPage = ::core::ptr::null_mut::<u8_0>();
                    if (*pCsr).bOnePage != 0 {
                        return SQLITE_OK;
                    }
                    (*pCsr).iPgno += 1;
                } else {
                    return SQLITE_OK
                }
            } else {
                if (*pCsr).nRec == 0 as ::core::ffi::c_longlong {
                    let mut bHasRowid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut nPointer: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut nPayload: sqlite3_int64 = 0 as sqlite3_int64;
                    let mut nHdr: sqlite3_int64 = 0 as sqlite3_int64;
                    let mut iHdr: ::core::ffi::c_int = 0;
                    let mut U: ::core::ffi::c_int = 0;
                    let mut X: ::core::ffi::c_int = 0;
                    let mut nLocal: ::core::ffi::c_int = 0;
                    match *(*pCsr).aPage.offset(iOff as isize) as ::core::ffi::c_int {
                        2 => {
                            nPointer = 4 as ::core::ffi::c_int;
                        }
                        10 => {}
                        13 => {
                            bHasRowid = 1 as ::core::ffi::c_int;
                        }
                        _ => {
                            (*pCsr).iCell = (*pCsr).nCell;
                        }
                    }
                    if (*pCsr).iCell >= (*pCsr).nCell {
                        bNextPage = 1 as ::core::ffi::c_int;
                    } else {
                        let mut iCellPtr: ::core::ffi::c_int = iOff
                            + 8 as ::core::ffi::c_int + nPointer
                            + (*pCsr).iCell * 2 as ::core::ffi::c_int;
                        if iCellPtr > (*pCsr).nPage {
                            bNextPage = 1 as ::core::ffi::c_int;
                        } else {
                            iOff = get_uint16(
                                (*pCsr).aPage.offset(iCellPtr as isize)
                                    as *mut ::core::ffi::c_uchar,
                            ) as ::core::ffi::c_int;
                        }
                        iOff += nPointer;
                        if bNextPage != 0 || iOff > (*pCsr).nPage || iOff <= iCellPtr {
                            bNextPage = 1 as ::core::ffi::c_int;
                        } else {
                            iOff
                                += dbdataGetVarintU32(
                                    (*pCsr).aPage.offset(iOff as isize) as *mut u8_0,
                                    &raw mut nPayload,
                                );
                            if nPayload > 0x7fffff00 as ::core::ffi::c_longlong {
                                nPayload &= 0x3fff as ::core::ffi::c_longlong;
                            }
                            if nPayload == 0 as ::core::ffi::c_longlong {
                                nPayload = 1 as sqlite3_int64;
                            }
                        }
                        if bHasRowid != 0 && bNextPage == 0 && iOff < (*pCsr).nPage {
                            iOff
                                += dbdataGetVarint(
                                    (*pCsr).aPage.offset(iOff as isize) as *mut u8_0,
                                    &raw mut (*pCsr).iIntkey,
                                );
                        }
                        U = (*pCsr).nPage;
                        if bHasRowid != 0 {
                            X = U - 35 as ::core::ffi::c_int;
                        } else {
                            X = (U - 12 as ::core::ffi::c_int) * 64 as ::core::ffi::c_int
                                / 255 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
                        }
                        if nPayload <= X as ::core::ffi::c_longlong {
                            nLocal = nPayload as ::core::ffi::c_int;
                        } else {
                            let mut M: ::core::ffi::c_int = 0;
                            let mut K: ::core::ffi::c_int = 0;
                            M = (U - 12 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int
                                / 255 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
                            K = (M as ::core::ffi::c_longlong
                                + (nPayload as ::core::ffi::c_longlong
                                    - M as ::core::ffi::c_longlong)
                                    % (U - 4 as ::core::ffi::c_int) as ::core::ffi::c_longlong)
                                as ::core::ffi::c_int;
                            if K <= X {
                                nLocal = K;
                            } else {
                                nLocal = M;
                            }
                        }
                        if bNextPage != 0 || nLocal + iOff > (*pCsr).nPage {
                            bNextPage = 1 as ::core::ffi::c_int;
                        } else {
                            rc = dbdataBufferSize(
                                &raw mut (*pCsr).rec,
                                nPayload + DBDATA_PADDING_BYTES as sqlite3_int64,
                            );
                            if rc != SQLITE_OK {
                                return rc;
                            }
                            memcpy(
                                (*pCsr).rec.aBuf as *mut ::core::ffi::c_void,
                                (*pCsr).aPage.offset(iOff as isize) as *mut u8_0
                                    as *const ::core::ffi::c_void,
                                nLocal as size_t,
                            );
                            iOff += nLocal;
                            if nPayload > nLocal as ::core::ffi::c_longlong {
                                let mut nRem: sqlite3_int64 = nPayload
                                    - nLocal as sqlite3_int64;
                                let mut pgnoOvfl: u32_0 = get_uint32(
                                    (*pCsr).aPage.offset(iOff as isize)
                                        as *mut ::core::ffi::c_uchar,
                                );
                                while nRem > 0 as ::core::ffi::c_longlong {
                                    let mut aOvfl: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                                    let mut nOvfl: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    let mut nCopy: ::core::ffi::c_int = 0;
                                    rc = dbdataLoadPage(
                                        pCsr,
                                        pgnoOvfl,
                                        &raw mut aOvfl,
                                        &raw mut nOvfl,
                                    );
                                    if rc != SQLITE_OK {
                                        return rc;
                                    }
                                    if aOvfl.is_null() {
                                        break;
                                    }
                                    nCopy = U - 4 as ::core::ffi::c_int;
                                    if nCopy as ::core::ffi::c_longlong > nRem {
                                        nCopy = nRem as ::core::ffi::c_int;
                                    }
                                    memcpy(
                                        (*pCsr).rec.aBuf.offset((nPayload - nRem) as isize)
                                            as *mut u8_0 as *mut ::core::ffi::c_void,
                                        aOvfl.offset(4 as ::core::ffi::c_int as isize) as *mut u8_0
                                            as *const ::core::ffi::c_void,
                                        nCopy as size_t,
                                    );
                                    nRem -= nCopy as ::core::ffi::c_longlong;
                                    pgnoOvfl = get_uint32(aOvfl as *mut ::core::ffi::c_uchar);
                                    sqlite3_free(aOvfl as *mut ::core::ffi::c_void);
                                }
                                nPayload -= nRem as ::core::ffi::c_longlong;
                            }
                            memset(
                                (*pCsr).rec.aBuf.offset(nPayload as isize) as *mut u8_0
                                    as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                DBDATA_PADDING_BYTES as size_t,
                            );
                            (*pCsr).nRec = nPayload;
                            iHdr = dbdataGetVarintU32((*pCsr).rec.aBuf, &raw mut nHdr);
                            if nHdr > nPayload {
                                nHdr = 0 as sqlite3_int64;
                            }
                            (*pCsr).nHdr = nHdr;
                            (*pCsr).pHdrPtr = (*pCsr).rec.aBuf.offset(iHdr as isize)
                                as *mut u8_0;
                            (*pCsr).pPtr = (*pCsr).rec.aBuf.offset((*pCsr).nHdr as isize)
                                as *mut u8_0;
                            (*pCsr).iField = if bHasRowid != 0 {
                                -1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            };
                        }
                    }
                } else {
                    (*pCsr).iField += 1;
                    if (*pCsr).iField > 0 as ::core::ffi::c_int {
                        let mut iType: sqlite3_int64 = 0;
                        if (*pCsr).pHdrPtr
                            >= (*pCsr).rec.aBuf.offset((*pCsr).nRec as isize)
                                as *mut u8_0 || (*pCsr).iField >= DBDATA_MX_FIELD
                        {
                            bNextPage = 1 as ::core::ffi::c_int;
                        } else {
                            let mut szField: ::core::ffi::c_int = 0
                                as ::core::ffi::c_int;
                            (*pCsr).pHdrPtr = (*pCsr)
                                .pHdrPtr
                                .offset(
                                    dbdataGetVarintU32((*pCsr).pHdrPtr, &raw mut iType) as isize,
                                );
                            szField = dbdataValueBytes(iType as ::core::ffi::c_int);
                            if ((*pCsr).nRec as ::core::ffi::c_longlong
                                - (*pCsr).pPtr.offset_from((*pCsr).rec.aBuf)
                                    as ::core::ffi::c_long as ::core::ffi::c_longlong)
                                < szField as ::core::ffi::c_longlong
                            {
                                (*pCsr).pPtr = (*pCsr)
                                    .rec
                                    .aBuf
                                    .offset((*pCsr).nRec as isize) as *mut u8_0;
                            } else {
                                (*pCsr).pPtr = (*pCsr).pPtr.offset(szField as isize);
                            }
                        }
                    }
                }
                if bNextPage != 0 {
                    sqlite3_free((*pCsr).aPage as *mut ::core::ffi::c_void);
                    (*pCsr).aPage = ::core::ptr::null_mut::<u8_0>();
                    (*pCsr).nRec = 0 as sqlite3_int64;
                    if (*pCsr).bOnePage != 0 {
                        return SQLITE_OK;
                    }
                    (*pCsr).iPgno += 1;
                } else {
                    if (*pCsr).iField < 0 as ::core::ffi::c_int
                        || (*pCsr).pHdrPtr
                            < (*pCsr).rec.aBuf.offset((*pCsr).nHdr as isize) as *mut u8_0
                    {
                        return SQLITE_OK;
                    }
                    (*pCsr).nRec = 0 as sqlite3_int64;
                    (*pCsr).iCell += 1;
                }
            }
        };
    }
}
unsafe extern "C" fn dbdataEof(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut DbdataCursor = pCursor as *mut DbdataCursor;
        return (*pCsr).aPage.is_null() as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn dbdataIsFunction(
    mut zSchema: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: size_t = strlen(zSchema);
        if n > 2 as size_t
            && *zSchema.offset(n.wrapping_sub(2 as size_t) as isize)
                as ::core::ffi::c_int == '(' as i32
            && *zSchema.offset(n.wrapping_sub(1 as size_t) as isize)
                as ::core::ffi::c_int == ')' as i32
        {
            return n as ::core::ffi::c_int - 2 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn dbdataDbsize(
    mut pCsr: *mut DbdataCursor,
    mut zSchema: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut DbdataTable = (*pCsr).base.pVtab as *mut DbdataTable;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        let mut rc2: ::core::ffi::c_int = 0;
        let mut nFunc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        nFunc = dbdataIsFunction(zSchema);
        if nFunc > 0 as ::core::ffi::c_int {
            zSql = sqlite3_mprintf(
                b"SELECT %.*s(0)\0".as_ptr() as *const ::core::ffi::c_char,
                nFunc,
                zSchema,
            );
        } else {
            zSql = sqlite3_mprintf(
                b"PRAGMA %Q.page_count\0".as_ptr() as *const ::core::ffi::c_char,
                zSchema,
            );
        }
        if zSql.is_null() {
            return SQLITE_NOMEM;
        }
        rc = sqlite3_prepare_v2(
            (*pTab).db,
            zSql,
            -1 as ::core::ffi::c_int,
            &raw mut pStmt,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        );
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        if rc == SQLITE_OK && sqlite3_step(pStmt) == SQLITE_ROW {
            (*pCsr).szDb = sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
        }
        rc2 = sqlite3_finalize(pStmt);
        if rc == SQLITE_OK {
            rc = rc2;
        }
        return rc;
    }
}
unsafe extern "C" fn dbdataGetEncoding(
    mut pCsr: *mut DbdataCursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nPg1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut aPg1: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
        rc = dbdataLoadPage(pCsr, 1 as u32_0, &raw mut aPg1, &raw mut nPg1);
        if rc == SQLITE_OK && nPg1 >= 56 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
        {
            (*pCsr).enc = get_uint32(
                aPg1.offset(56 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_uchar,
            );
        }
        sqlite3_free(aPg1 as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn dbdataFilter(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut DbdataCursor = pCursor as *mut DbdataCursor;
        let mut pTab: *mut DbdataTable = (*pCursor).pVtab as *mut DbdataTable;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zSchema: *const ::core::ffi::c_char = b"main\0".as_ptr()
            as *const ::core::ffi::c_char;
        dbdataResetCursor(pCsr);
        if idxNum & 0x1 as ::core::ffi::c_int != 0 {
            zSchema = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
                as *const ::core::ffi::c_char;
            if zSchema.is_null() {
                zSchema = b"\0".as_ptr() as *const ::core::ffi::c_char;
            }
        }
        if idxNum & 0x2 as ::core::ffi::c_int != 0 {
            (*pCsr).iPgno = sqlite3_value_int(
                *argv.offset((idxNum & 0x1 as ::core::ffi::c_int) as isize),
            );
            (*pCsr).bOnePage = 1 as ::core::ffi::c_int;
        } else {
            rc = dbdataDbsize(pCsr, zSchema);
        }
        if rc == SQLITE_OK {
            let mut nFunc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if !(*pTab).pStmt.is_null() {
                (*pCsr).pStmt = (*pTab).pStmt;
                (*pTab).pStmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            } else {
                nFunc = dbdataIsFunction(zSchema);
                if nFunc > 0 as ::core::ffi::c_int {
                    let mut zSql: *mut ::core::ffi::c_char = sqlite3_mprintf(
                        b"SELECT %.*s(?2)\0".as_ptr() as *const ::core::ffi::c_char,
                        nFunc,
                        zSchema,
                    );
                    if zSql.is_null() {
                        rc = SQLITE_NOMEM;
                    } else {
                        rc = sqlite3_prepare_v2(
                            (*pTab).db,
                            zSql,
                            -1 as ::core::ffi::c_int,
                            &raw mut (*pCsr).pStmt,
                            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                        );
                        sqlite3_free(zSql as *mut ::core::ffi::c_void);
                    }
                } else {
                    rc = sqlite3_prepare_v2(
                        (*pTab).db,
                        b"SELECT data FROM sqlite_dbpage(?) WHERE pgno=?\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                        &raw mut (*pCsr).pStmt,
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                    );
                }
            }
        }
        if rc == SQLITE_OK {
            rc = sqlite3_bind_text(
                (*pCsr).pStmt,
                1 as ::core::ffi::c_int,
                zSchema,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
        }
        if rc == SQLITE_OK {
            rc = dbdataGetEncoding(pCsr);
        }
        if rc != SQLITE_OK {
            (*pTab).base.zErrMsg = sqlite3_mprintf(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg((*pTab).db),
            );
        }
        if rc == SQLITE_OK {
            rc = dbdataNext(pCursor);
        }
        return rc;
    }
}
unsafe extern "C" fn dbdataColumn(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut DbdataCursor = pCursor as *mut DbdataCursor;
        let mut pTab: *mut DbdataTable = (*pCursor).pVtab as *mut DbdataTable;
        if (*pTab).bPtr != 0 {
            match i {
                DBPTR_COLUMN_PGNO => {
                    sqlite3_result_int64(ctx, (*pCsr).iPgno as sqlite3_int64);
                }
                DBPTR_COLUMN_CHILD => {
                    let mut iOff: ::core::ffi::c_int = if (*pCsr).iPgno
                        == 1 as ::core::ffi::c_int
                    {
                        100 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    if (*pCsr).iCell < 0 as ::core::ffi::c_int {
                        iOff += 8 as ::core::ffi::c_int;
                    } else {
                        iOff
                            += 12 as ::core::ffi::c_int
                                + (*pCsr).iCell * 2 as ::core::ffi::c_int;
                        if iOff > (*pCsr).nPage {
                            return SQLITE_OK;
                        }
                        iOff = get_uint16(
                            (*pCsr).aPage.offset(iOff as isize)
                                as *mut ::core::ffi::c_uchar,
                        ) as ::core::ffi::c_int;
                    }
                    if iOff <= (*pCsr).nPage {
                        sqlite3_result_int64(
                            ctx,
                            get_uint32(
                                (*pCsr).aPage.offset(iOff as isize)
                                    as *mut ::core::ffi::c_uchar,
                            ) as sqlite3_int64,
                        );
                    }
                }
                _ => {}
            }
        } else {
            match i {
                DBDATA_COLUMN_PGNO => {
                    sqlite3_result_int64(ctx, (*pCsr).iPgno as sqlite3_int64);
                }
                DBDATA_COLUMN_CELL => {
                    sqlite3_result_int(ctx, (*pCsr).iCell);
                }
                DBDATA_COLUMN_FIELD => {
                    sqlite3_result_int(ctx, (*pCsr).iField);
                }
                DBDATA_COLUMN_VALUE => {
                    if (*pCsr).iField < 0 as ::core::ffi::c_int {
                        sqlite3_result_int64(ctx, (*pCsr).iIntkey);
                    } else if (*pCsr).rec.aBuf.offset((*pCsr).nRec as isize) as *mut u8_0
                        >= (*pCsr).pPtr
                    {
                        let mut iType: sqlite3_int64 = 0;
                        dbdataGetVarintU32((*pCsr).pHdrPtr, &raw mut iType);
                        dbdataValue(
                            ctx,
                            (*pCsr).enc,
                            iType as ::core::ffi::c_int,
                            (*pCsr).pPtr,
                            ((*pCsr).rec.aBuf.offset((*pCsr).nRec as isize) as *mut u8_0)
                                .offset_from((*pCsr).pPtr) as ::core::ffi::c_long
                                as sqlite3_int64,
                        );
                    }
                }
                _ => {}
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn dbdataRowid(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut DbdataCursor = pCursor as *mut DbdataCursor;
        *pRowid = (*pCsr).iRowid as sqlite_int64;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn sqlite3DbdataRegister(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    unsafe {
        static mut dbdata_module: sqlite3_module = unsafe {
            sqlite3_module {
                iVersion: 0 as ::core::ffi::c_int,
                xCreate: None,
                xConnect: Some(
                    dbdataConnect
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const *const ::core::ffi::c_char,
                            *mut *mut sqlite3_vtab,
                            *mut *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                xBestIndex: Some(
                    dbdataBestIndex
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut sqlite3_index_info,
                        ) -> ::core::ffi::c_int,
                ),
                xDisconnect: Some(
                    dbdataDisconnect
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xDestroy: None,
                xOpen: Some(
                    dbdataOpen
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xClose: Some(
                    dbdataClose
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xFilter: Some(
                    dbdataFilter
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> ::core::ffi::c_int,
                ),
                xNext: Some(
                    dbdataNext
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xEof: Some(
                    dbdataEof
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xColumn: Some(
                    dbdataColumn
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xRowid: Some(
                    dbdataRowid
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            *mut sqlite_int64,
                        ) -> ::core::ffi::c_int,
                ),
                xUpdate: None,
                xBegin: None,
                xSync: None,
                xCommit: None,
                xRollback: None,
                xFindFunction: None,
                xRename: None,
                xSavepoint: None,
                xRelease: None,
                xRollbackTo: None,
                xShadowName: None,
                xIntegrity: None,
            }
        };
        let mut rc: ::core::ffi::c_int = sqlite3_create_module(
            db,
            b"sqlite_dbdata\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut dbdata_module,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        if rc == SQLITE_OK {
            rc = sqlite3_create_module(
                db,
                b"sqlite_dbptr\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut dbdata_module,
                1 as ::core::ffi::c_int as *mut ::core::ffi::c_void,
            );
        }
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_dbdata_init(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut ::core::ffi::c_char,
    mut pApi: *const sqlite3_api_routines,
) -> ::core::ffi::c_int {
    unsafe {
        return sqlite3DbdataRegister(db);
    }
}
