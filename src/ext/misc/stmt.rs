
extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_sql(pStmt: *mut sqlite3_stmt) -> *const ::core::ffi::c_char;
    fn sqlite3_stmt_readonly(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_stmt_busy(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_next_stmt(pDb: *mut sqlite3, pStmt: *mut sqlite3_stmt) -> *mut sqlite3_stmt;
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
    fn sqlite3_stmt_status(
        _: *mut sqlite3_stmt,
        op: ::core::ffi::c_int,
        resetFlg: ::core::ffi::c_int,
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
        unsafe extern "C" fn(*mut sqlite3_vtab, *mut sqlite3_index_info) -> ::core::ffi::c_int,
    >,
    pub xDisconnect: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab,
            *mut *mut sqlite3_vtab_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
    pub xFilter: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_value,
        ) -> ::core::ffi::c_int,
    >,
    pub xNext: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
    pub xEof: Option<unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int>,
    pub xColumn: Option<
        unsafe extern "C" fn(
            *mut sqlite3_vtab_cursor,
            *mut sqlite3_context,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRowid: Option<
        unsafe extern "C" fn(*mut sqlite3_vtab_cursor, *mut sqlite3_int64) -> ::core::ffi::c_int,
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
        unsafe extern "C" fn(*mut sqlite3_vtab, *const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
    pub xSavepoint:
        Option<unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xRelease:
        Option<unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xRollbackTo:
        Option<unsafe extern "C" fn(*mut sqlite3_vtab, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub xShadowName: Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
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
pub struct sqlite3_vtab {
    pub pModule: *const sqlite3_module,
    pub nRef: ::core::ffi::c_int,
    pub zErrMsg: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_vtab_cursor {
    pub pVtab: *mut sqlite3_vtab,
}
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StmtRow {
    pub iRowid: sqlite3_int64,
    pub zSql: *mut ::core::ffi::c_char,
    pub aCol: [::core::ffi::c_int; 11],
    pub pNext: *mut StmtRow,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stmt_vtab {
    pub base: sqlite3_vtab,
    pub db: *mut sqlite3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stmt_cursor {
    pub base: sqlite3_vtab_cursor,
    pub db: *mut sqlite3,
    pub pRow: *mut StmtRow,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_FULLSCAN_STEP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_SORT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_AUTOINDEX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_VM_STEP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_REPREPARE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_RUN: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SQLITE_STMTSTATUS_MEMUSED: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
unsafe extern "C" fn stmtConnect(
    mut db: *mut sqlite3,
    mut _pAux: *mut ::core::ffi::c_void,
    mut _argc: ::core::ffi::c_int,
    mut _argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut _pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pNew: *mut stmt_vtab = ::core::ptr::null_mut::<stmt_vtab>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = sqlite3_declare_vtab(
        db,
        b"CREATE TABLE x(sql,ncol,ro,busy,nscan,nsort,naidx,nstep,reprep,run,mem)\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if rc == SQLITE_OK {
        pNew = sqlite3_malloc64(::core::mem::size_of::<stmt_vtab>() as sqlite3_uint64)
            as *mut stmt_vtab;
        *ppVtab = pNew as *mut sqlite3_vtab;
        if pNew.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<stmt_vtab>() as size_t,
        );
        (*pNew).db = db;
    }
    return rc;
}
pub const STMT_COLUMN_SQL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STMT_COLUMN_NCOL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STMT_COLUMN_RO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STMT_COLUMN_BUSY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const STMT_COLUMN_NSCAN: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const STMT_COLUMN_NSORT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const STMT_COLUMN_NAIDX: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const STMT_COLUMN_NSTEP: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const STMT_COLUMN_REPREP: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const STMT_COLUMN_RUN: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const STMT_COLUMN_MEM: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
unsafe extern "C" fn stmtDisconnect(mut pVtab: *mut sqlite3_vtab) -> ::core::ffi::c_int {
    sqlite3_free(pVtab as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
unsafe extern "C" fn stmtOpen(
    mut p: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCur: *mut stmt_cursor = ::core::ptr::null_mut::<stmt_cursor>();
    pCur = sqlite3_malloc64(::core::mem::size_of::<stmt_cursor>() as sqlite3_uint64)
        as *mut stmt_cursor;
    if pCur.is_null() {
        return SQLITE_NOMEM;
    }
    memset(
        pCur as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stmt_cursor>() as size_t,
    );
    (*pCur).db = (*(p as *mut stmt_vtab)).db;
    *ppCursor = &raw mut (*pCur).base;
    return SQLITE_OK;
}
unsafe extern "C" fn stmtCsrReset(mut pCur: *mut stmt_cursor) {
    let mut pRow: *mut StmtRow = ::core::ptr::null_mut::<StmtRow>();
    let mut pNext: *mut StmtRow = ::core::ptr::null_mut::<StmtRow>();
    pRow = (*pCur).pRow;
    while !pRow.is_null() {
        pNext = (*pRow).pNext;
        sqlite3_free(pRow as *mut ::core::ffi::c_void);
        pRow = pNext;
    }
    (*pCur).pRow = ::core::ptr::null_mut::<StmtRow>();
}
unsafe extern "C" fn stmtClose(mut cur: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    stmtCsrReset(cur as *mut stmt_cursor);
    sqlite3_free(cur as *mut ::core::ffi::c_void);
    return SQLITE_OK;
}
unsafe extern "C" fn stmtNext(mut cur: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut pCur: *mut stmt_cursor = cur as *mut stmt_cursor;
    let mut pNext: *mut StmtRow = (*(*pCur).pRow).pNext;
    sqlite3_free((*pCur).pRow as *mut ::core::ffi::c_void);
    (*pCur).pRow = pNext;
    return SQLITE_OK;
}
unsafe extern "C" fn stmtColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pCur: *mut stmt_cursor = cur as *mut stmt_cursor;
    let mut pRow: *mut StmtRow = (*pCur).pRow;
    if i == STMT_COLUMN_SQL {
        sqlite3_result_text(
            ctx,
            (*pRow).zSql,
            -(1 as ::core::ffi::c_int),
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
        );
    } else {
        sqlite3_result_int(ctx, (*pRow).aCol[i as usize]);
    }
    return SQLITE_OK;
}
unsafe extern "C" fn stmtRowid(
    mut cur: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    let mut pCur: *mut stmt_cursor = cur as *mut stmt_cursor;
    *pRowid = (*(*pCur).pRow).iRowid as sqlite_int64;
    return SQLITE_OK;
}
unsafe extern "C" fn stmtEof(mut cur: *mut sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let mut pCur: *mut stmt_cursor = cur as *mut stmt_cursor;
    return ((*pCur).pRow == ::core::ptr::null_mut::<StmtRow>()) as ::core::ffi::c_int;
}
unsafe extern "C" fn stmtFilter(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut _idxNum: ::core::ffi::c_int,
    mut _idxStr: *const ::core::ffi::c_char,
    mut _argc: ::core::ffi::c_int,
    mut _argv: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    let mut pCur: *mut stmt_cursor = pVtabCursor as *mut stmt_cursor;
    let mut p: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let mut iRowid: sqlite3_int64 = 1 as sqlite3_int64;
    let mut ppRow: *mut *mut StmtRow = ::core::ptr::null_mut::<*mut StmtRow>();
    stmtCsrReset(pCur);
    ppRow = &raw mut (*pCur).pRow;
    p = sqlite3_next_stmt((*pCur).db, ::core::ptr::null_mut::<sqlite3_stmt>());
    while !p.is_null() {
        let mut zSql: *const ::core::ffi::c_char = sqlite3_sql(p);
        let mut nSql: sqlite3_int64 = (if !zSql.is_null() {
            strlen(zSql).wrapping_add(1 as size_t)
        } else {
            0 as size_t
        }) as sqlite3_int64;
        let mut pNew: *mut StmtRow = sqlite3_malloc64(
            (::core::mem::size_of::<StmtRow>() as sqlite3_uint64)
                .wrapping_add(nSql as sqlite3_uint64),
        ) as *mut StmtRow;
        if pNew.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<StmtRow>() as size_t,
        );
        if !zSql.is_null() {
            (*pNew).zSql = pNew.offset(1 as ::core::ffi::c_int as isize) as *mut StmtRow
                as *mut ::core::ffi::c_char;
            memcpy(
                (*pNew).zSql as *mut ::core::ffi::c_void,
                zSql as *const ::core::ffi::c_void,
                nSql as size_t,
            );
        }
        (*pNew).aCol[STMT_COLUMN_NCOL as usize] = sqlite3_column_count(p);
        (*pNew).aCol[STMT_COLUMN_RO as usize] = sqlite3_stmt_readonly(p);
        (*pNew).aCol[STMT_COLUMN_BUSY as usize] = sqlite3_stmt_busy(p);
        (*pNew).aCol[STMT_COLUMN_NSCAN as usize] =
            sqlite3_stmt_status(p, SQLITE_STMTSTATUS_FULLSCAN_STEP, 0 as ::core::ffi::c_int);
        (*pNew).aCol[STMT_COLUMN_NSORT as usize] =
            sqlite3_stmt_status(p, SQLITE_STMTSTATUS_SORT, 0 as ::core::ffi::c_int);
        (*pNew).aCol[STMT_COLUMN_NAIDX as usize] =
            sqlite3_stmt_status(p, SQLITE_STMTSTATUS_AUTOINDEX, 0 as ::core::ffi::c_int);
        (*pNew).aCol[STMT_COLUMN_NSTEP as usize] =
            sqlite3_stmt_status(p, SQLITE_STMTSTATUS_VM_STEP, 0 as ::core::ffi::c_int);
        (*pNew).aCol[STMT_COLUMN_REPREP as usize] =
            sqlite3_stmt_status(p, SQLITE_STMTSTATUS_REPREPARE, 0 as ::core::ffi::c_int);
        (*pNew).aCol[STMT_COLUMN_RUN as usize] =
            sqlite3_stmt_status(p, SQLITE_STMTSTATUS_RUN, 0 as ::core::ffi::c_int);
        (*pNew).aCol[STMT_COLUMN_MEM as usize] =
            sqlite3_stmt_status(p, SQLITE_STMTSTATUS_MEMUSED, 0 as ::core::ffi::c_int);
        let fresh0 = iRowid;
        iRowid = iRowid + 1;
        (*pNew).iRowid = fresh0;
        *ppRow = pNew;
        ppRow = &raw mut (*pNew).pNext;
        p = sqlite3_next_stmt((*pCur).db, p);
    }
    return SQLITE_OK;
}
unsafe extern "C" fn stmtBestIndex(
    mut _tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    (*pIdxInfo).estimatedCost = 500 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*pIdxInfo).estimatedRows = 500 as sqlite3_int64;
    return SQLITE_OK;
}
static mut stmtModule: sqlite3_module = unsafe {
    sqlite3_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: None,
        xConnect: Some(
            stmtConnect
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
            stmtBestIndex
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            stmtDisconnect as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
        ),
        xDestroy: None,
        xOpen: Some(
            stmtOpen
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab,
                    *mut *mut sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            stmtClose as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            stmtFilter
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            stmtNext as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int,
        ),
        xEof: Some(stmtEof as unsafe extern "C" fn(*mut sqlite3_vtab_cursor) -> ::core::ffi::c_int),
        xColumn: Some(
            stmtColumn
                as unsafe extern "C" fn(
                    *mut sqlite3_vtab_cursor,
                    *mut sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            stmtRowid
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
#[no_mangle]
pub unsafe extern "C" fn sqlite3StmtVtabInit(mut db: *mut sqlite3) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    rc = sqlite3_create_module(
        db,
        b"sqlite_stmt\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut stmtModule,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    return rc;
}
