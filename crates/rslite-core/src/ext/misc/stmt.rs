pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_AUTOINDEX;
pub use crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_FULLSCAN_STEP;
pub use crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_MEMUSED;
pub use crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_REPREPARE;
pub use crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_RUN;
pub use crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_SORT;
pub use crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_VM_STEP;
pub use crate::src::headers::sqlite3_h::sqlite_int64;
pub use crate::src::headers::sqlite3_h::sqlite_uint64;
pub use crate::src::headers::sqlite3_h::sqlite3_destructor_type;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;
pub use crate::src::headers::sqlite3_h::sqlite3_index_info;
pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;
pub use crate::src::headers::sqlite3_h::sqlite3_int64;
pub use crate::src::headers::sqlite3_h::sqlite3_module;
pub use crate::src::headers::sqlite3_h::sqlite3_stmt;
pub use crate::src::headers::sqlite3_h::sqlite3_uint64;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;
pub use crate::src::headers::sqliteInt_h::sqlite3;
pub use crate::src::headers::vdbeInt_h::sqlite3_context;
pub use crate::src::headers::vdbeInt_h::sqlite3_value;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3_malloc64;
pub use crate::src::src::vdbeapi::sqlite3_column_count;
pub use crate::src::src::vdbeapi::sqlite3_next_stmt;
pub use crate::src::src::vdbeapi::sqlite3_result_int;
pub use crate::src::src::vdbeapi::sqlite3_result_text;
pub use crate::src::src::vdbeapi::sqlite3_sql;
pub use crate::src::src::vdbeapi::sqlite3_stmt_busy;
pub use crate::src::src::vdbeapi::sqlite3_stmt_readonly;
pub use crate::src::src::vdbeapi::sqlite3_stmt_status;
pub use crate::src::src::vtab::sqlite3_create_module;
pub use crate::src::src::vtab::sqlite3_declare_vtab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StmtRow {
    pub iRowid: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub zSql: *mut ::core::ffi::c_char,
    pub aCol: [::core::ffi::c_int; 11],
    pub pNext: *mut StmtRow,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stmt_vtab {
    pub base: crate::src::headers::sqlite3_h::sqlite3_vtab,
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stmt_cursor {
    pub base: crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    pub db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pub pRow: *mut StmtRow,
}

unsafe extern "C" fn stmtConnect(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut _pAux: *mut ::core::ffi::c_void,
    mut _argc: ::core::ffi::c_int,
    mut _argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    mut _pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pNew: *mut stmt_vtab = ::core::ptr::null_mut::<stmt_vtab>();
    let mut rc: ::core::ffi::c_int = 0;
    rc = crate::src::src::vtab::sqlite3_declare_vtab(
        db,
        b"CREATE TABLE x(sql,ncol,ro,busy,nscan,nsort,naidx,nstep,reprep,run,mem)\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        pNew =
            crate::src::src::malloc::sqlite3_malloc64(::core::mem::size_of::<stmt_vtab>()
                as crate::src::headers::sqlite3_h::sqlite3_uint64) as *mut stmt_vtab;
        *ppVtab = pNew as *mut crate::src::headers::sqlite3_h::sqlite3_vtab;
        if pNew.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        ::libc::memset(
            pNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<stmt_vtab>() as crate::__stddef_size_t_h::size_t,
        );
        (*pNew).db = db;
    }
    rc
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

unsafe extern "C" fn stmtDisconnect(
    mut pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
) -> ::core::ffi::c_int {
    crate::src::src::malloc::sqlite3_free(pVtab as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn stmtOpen(
    mut p: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    mut ppCursor: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCur: *mut stmt_cursor = ::core::ptr::null_mut::<stmt_cursor>();
    pCur = crate::src::src::malloc::sqlite3_malloc64(
        ::core::mem::size_of::<stmt_cursor>() as crate::src::headers::sqlite3_h::sqlite3_uint64
    ) as *mut stmt_cursor;
    if pCur.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pCur as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<stmt_cursor>() as crate::__stddef_size_t_h::size_t,
    );
    (*pCur).db = (*(p as *mut stmt_vtab)).db;
    *ppCursor = &raw mut (*pCur).base;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn stmtCsrReset(mut pCur: *mut stmt_cursor) {
    let mut pRow: *mut StmtRow = ::core::ptr::null_mut::<StmtRow>();
    let mut pNext: *mut StmtRow = ::core::ptr::null_mut::<StmtRow>();
    pRow = (*pCur).pRow;
    while !pRow.is_null() {
        pNext = (*pRow).pNext;
        crate::src::src::malloc::sqlite3_free(pRow as *mut ::core::ffi::c_void);
        pRow = pNext;
    }
    (*pCur).pRow = ::core::ptr::null_mut::<StmtRow>();
}

unsafe extern "C" fn stmtClose(
    mut cur: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    stmtCsrReset(cur as *mut stmt_cursor);
    crate::src::src::malloc::sqlite3_free(cur as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn stmtNext(
    mut cur: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCur: *mut stmt_cursor = cur as *mut stmt_cursor;
    let __pCur_ref = { &mut *pCur };
    let mut pNext: *mut StmtRow = (*__pCur_ref.pRow).pNext;
    crate::src::src::malloc::sqlite3_free(__pCur_ref.pRow as *mut ::core::ffi::c_void);
    __pCur_ref.pRow = pNext;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn stmtColumn(
    mut cur: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    mut ctx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut i: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let pCur = &*(cur as *mut stmt_cursor);
    let mut pRow: *mut StmtRow = pCur.pRow;
    if i == STMT_COLUMN_SQL {
        crate::src::src::vdbeapi::sqlite3_result_text(
            ctx,
            (*pRow).zSql,
            -(1 as ::core::ffi::c_int),
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
        );
    } else {
        crate::src::src::vdbeapi::sqlite3_result_int(ctx, (*pRow).aCol[i as usize]);
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn stmtRowid(
    mut cur: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    mut pRowid: *mut crate::src::headers::sqlite3_h::sqlite_int64,
) -> ::core::ffi::c_int {
    let pCur = &*(cur as *mut stmt_cursor);
    *pRowid = (*pCur.pRow).iRowid as crate::src::headers::sqlite3_h::sqlite_int64;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn stmtEof(
    mut cur: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let pCur = &*(cur as *mut stmt_cursor);
    (pCur.pRow == ::core::ptr::null_mut::<StmtRow>()) as ::core::ffi::c_int
}

unsafe extern "C" fn stmtFilter(
    mut pVtabCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    mut _idxNum: ::core::ffi::c_int,
    mut _idxStr: *const ::core::ffi::c_char,
    mut _argc: ::core::ffi::c_int,
    mut _argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let mut pCur: *mut stmt_cursor = pVtabCursor as *mut stmt_cursor;
    let mut p: *mut crate::src::headers::sqlite3_h::sqlite3_stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>();
    let mut iRowid: crate::src::headers::sqlite3_h::sqlite3_int64 =
        1 as crate::src::headers::sqlite3_h::sqlite3_int64;
    let mut ppRow: *mut *mut StmtRow = ::core::ptr::null_mut::<*mut StmtRow>();
    stmtCsrReset(pCur);
    ppRow = &raw mut (*pCur).pRow;
    p = crate::src::src::vdbeapi::sqlite3_next_stmt(
        (*pCur).db,
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_stmt>(),
    );
    while !p.is_null() {
        let mut zSql: *const ::core::ffi::c_char = crate::src::src::vdbeapi::sqlite3_sql(p);
        let mut nSql: crate::src::headers::sqlite3_h::sqlite3_int64 = (if !zSql.is_null() {
            ::libc::strlen(zSql).wrapping_add(1 as crate::__stddef_size_t_h::size_t)
        } else {
            0 as crate::__stddef_size_t_h::size_t
        })
            as crate::src::headers::sqlite3_h::sqlite3_int64;
        let mut pNew: *mut StmtRow = crate::src::src::malloc::sqlite3_malloc64(
            (::core::mem::size_of::<StmtRow>() as crate::src::headers::sqlite3_h::sqlite3_uint64)
                .wrapping_add(nSql as crate::src::headers::sqlite3_h::sqlite3_uint64),
        ) as *mut StmtRow;
        if pNew.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        ::libc::memset(
            pNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<StmtRow>() as crate::__stddef_size_t_h::size_t,
        );
        let __pNew_ref = { &mut *pNew };
        if !zSql.is_null() {
            __pNew_ref.zSql = pNew.offset(1 as isize) as *mut StmtRow as *mut ::core::ffi::c_char;
            ::core::ptr::copy_nonoverlapping(
                zSql as *const u8,
                __pNew_ref.zSql as *mut u8,
                nSql as usize,
            );
        }
        __pNew_ref.aCol[STMT_COLUMN_NCOL as usize] =
            crate::src::src::vdbeapi::sqlite3_column_count(p);
        __pNew_ref.aCol[STMT_COLUMN_RO as usize] =
            crate::src::src::vdbeapi::sqlite3_stmt_readonly(p);
        __pNew_ref.aCol[STMT_COLUMN_BUSY as usize] = crate::src::src::vdbeapi::sqlite3_stmt_busy(p);
        __pNew_ref.aCol[STMT_COLUMN_NSCAN as usize] = crate::src::src::vdbeapi::sqlite3_stmt_status(
            p,
            crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_FULLSCAN_STEP,
            0 as ::core::ffi::c_int,
        );
        __pNew_ref.aCol[STMT_COLUMN_NSORT as usize] = crate::src::src::vdbeapi::sqlite3_stmt_status(
            p,
            crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_SORT,
            0 as ::core::ffi::c_int,
        );
        __pNew_ref.aCol[STMT_COLUMN_NAIDX as usize] = crate::src::src::vdbeapi::sqlite3_stmt_status(
            p,
            crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_AUTOINDEX,
            0 as ::core::ffi::c_int,
        );
        __pNew_ref.aCol[STMT_COLUMN_NSTEP as usize] = crate::src::src::vdbeapi::sqlite3_stmt_status(
            p,
            crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_VM_STEP,
            0 as ::core::ffi::c_int,
        );
        __pNew_ref.aCol[STMT_COLUMN_REPREP as usize] =
            crate::src::src::vdbeapi::sqlite3_stmt_status(
                p,
                crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_REPREPARE,
                0 as ::core::ffi::c_int,
            );
        __pNew_ref.aCol[STMT_COLUMN_RUN as usize] = crate::src::src::vdbeapi::sqlite3_stmt_status(
            p,
            crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_RUN,
            0 as ::core::ffi::c_int,
        );
        __pNew_ref.aCol[STMT_COLUMN_MEM as usize] = crate::src::src::vdbeapi::sqlite3_stmt_status(
            p,
            crate::src::headers::sqlite3_h::SQLITE_STMTSTATUS_MEMUSED,
            0 as ::core::ffi::c_int,
        );
        let fresh0 = iRowid;
        iRowid += 1;
        __pNew_ref.iRowid = fresh0;
        *ppRow = pNew;
        ppRow = &raw mut __pNew_ref.pNext;
        p = crate::src::src::vdbeapi::sqlite3_next_stmt((*pCur).db, p);
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn stmtBestIndex(
    mut _tab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    mut pIdxInfo: *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
) -> ::core::ffi::c_int {
    (*pIdxInfo).estimatedCost = 500 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*pIdxInfo).estimatedRows = 500 as crate::src::headers::sqlite3_h::sqlite3_int64;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

static mut stmtModule: crate::src::headers::sqlite3_h::sqlite3_module = {
    crate::src::headers::sqlite3_h::sqlite3_module {
        iVersion: 0 as ::core::ffi::c_int,
        xCreate: None,
        xConnect: Some(
            stmtConnect
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqliteInt_h::sqlite3,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const *const ::core::ffi::c_char,
                    *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        xBestIndex: Some(
            stmtBestIndex
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
                ) -> ::core::ffi::c_int,
        ),
        xDisconnect: Some(
            stmtDisconnect
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                ) -> ::core::ffi::c_int,
        ),
        xDestroy: None,
        xOpen: Some(
            stmtOpen
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                    *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xClose: Some(
            stmtClose
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xFilter: Some(
            stmtFilter
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                ) -> ::core::ffi::c_int,
        ),
        xNext: Some(
            stmtNext
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xEof: Some(
            stmtEof
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                ) -> ::core::ffi::c_int,
        ),
        xColumn: Some(
            stmtColumn
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
        xRowid: Some(
            stmtRowid
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    *mut crate::src::headers::sqlite3_h::sqlite_int64,
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
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3StmtVtabInit(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    rc = crate::src::src::vtab::sqlite3_create_module(
        db,
        b"sqlite_stmt\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut stmtModule as *mut _ as *const crate::src::headers::sqlite3_h::sqlite3_module,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    rc
}
