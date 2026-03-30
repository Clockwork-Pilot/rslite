






pub use crate::__stddef_size_t_h::size_t;




pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::src::ext::fts3::fts3::sqlite3Fts3Dequote;pub use crate::src::ext::fts3::fts3::sqlite3Fts3ErrMsg;pub use crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint;pub use crate::src::ext::fts3::fts3::sqlite3Fts3SegReaderCursor;pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderFinish;pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderStart;pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderStep;pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SegmentsClose;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::fts3Int_h::Fts3Index;pub use crate::fts3Int_h::Fts3MultiSegReader;pub use crate::fts3Int_h::Fts3SegFilter;pub use crate::fts3Int_h::Fts3SegReader;pub use crate::fts3Int_h::Fts3Table;pub use crate::fts3Int_h::FTS3_SEGCURSOR_ALL;pub use crate::fts3Int_h::FTS3_SEGMENT_IGNORE_EMPTY;pub use crate::fts3Int_h::FTS3_SEGMENT_REQUIRE_POS;pub use crate::fts3Int_h::FTS3_SEGMENT_SCAN;pub use crate::src::ext::fts3::fts3_hash::Fts3Hash;pub use crate::src::ext::fts3::fts3_hash::Fts3HashElem;pub use crate::src::ext::fts3::fts3_hash::_fts3ht;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor;pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;pub use crate::src::headers::sqliteInt_h::sqlite3;pub use crate::src::headers::sqlite3_h::sqlite3_blob;pub use crate::src::headers::vdbeInt_h::sqlite3_context;pub use crate::src::src::vtab::sqlite3_create_module;pub use crate::src::src::vtab::sqlite3_declare_vtab;pub use crate::src::headers::sqlite3_h::sqlite3_destructor_type;pub use crate::src::src::vdbeapi::sqlite3_finalize;pub use crate::src::src::malloc::sqlite3_free;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;pub use crate::src::headers::sqlite3_h::sqlite3_index_info;pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;pub use crate::src::headers::sqlite3_h::sqlite3_int64;pub use crate::src::src::malloc::sqlite3_malloc;pub use crate::src::src::malloc::sqlite3_malloc64;pub use crate::src::headers::sqlite3_h::sqlite3_module;pub use crate::src::src::printf::sqlite3_mprintf;pub use crate::src::src::malloc::sqlite3_realloc64;pub use crate::src::src::vdbeapi::sqlite3_result_int;pub use crate::src::src::vdbeapi::sqlite3_result_int64;pub use crate::src::src::vdbeapi::sqlite3_result_text;pub use crate::src::headers::sqlite3_h::sqlite3_stmt;pub use crate::src::src::util::sqlite3_strnicmp;pub use crate::src::headers::sqlite3_h::sqlite3_uint64;pub use crate::src::headers::vdbeInt_h::sqlite3_value;pub use crate::src::src::vdbeapi::sqlite3_value_int;pub use crate::src::src::vdbeapi::sqlite3_value_text;pub use crate::src::headers::sqlite3_h::sqlite3_vtab;pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;pub use crate::src::headers::sqlite3_h::sqlite_int64;pub use crate::src::headers::sqlite3_h::sqlite_uint64;pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT;pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ;pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_GE_1;pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_GT_1;pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_LE_1;pub use crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_LT_1;pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;pub use crate::src::headers::sqlite3_h::SQLITE_OK;pub use crate::src::headers::sqlite3_h::SQLITE_ROW;pub use crate::src::headers::sqlite3_h::SQLITE_STATIC;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct Fts3auxCursor {
    pub base: crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    pub csr: crate::fts3Int_h::Fts3MultiSegReader,
    pub filter: crate::fts3Int_h::Fts3SegFilter,
    pub zStop: *mut ::core::ffi::c_char,
    pub nStop: ::core::ffi::c_int,
    pub iLangid: ::core::ffi::c_int,
    pub isEof: ::core::ffi::c_int,
    pub iRowid: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub iCol: ::core::ffi::c_int,
    pub nStat: ::core::ffi::c_int,
    pub aStat: *mut Fts3auxColstats,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct Fts3auxColstats {
    pub nDoc: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub nOcc: crate::src::headers::sqlite3_h::sqlite3_int64,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct Fts3auxTable {
    pub base: crate::src::headers::sqlite3_h::sqlite3_vtab,
    pub pFts3Tab: *mut crate::fts3Int_h::Fts3Table,
}

pub const FTS3_AUX_SCHEMA: [::core::ffi::c_char; 69] = unsafe {
    ::core::mem::transmute::<[u8; 69], [::core::ffi::c_char; 69]>(
        *b"CREATE TABLE x(term, col, documents, occurrences, languageid HIDDEN)\0",
    )
};

unsafe extern "C" fn fts3auxConnectMethod(
    mut db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    mut _pUnused: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut zFts3: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nDb: ::core::ffi::c_int = 0;
    let mut nFts3: ::core::ffi::c_int = 0;
    let mut nByte: crate::src::headers::sqlite3_h::sqlite3_int64 = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let mut p: *mut Fts3auxTable = ::core::ptr::null_mut::<Fts3auxTable>();
    if !(argc != 4 as ::core::ffi::c_int && argc != 5 as ::core::ffi::c_int) {
        zDb = *argv.offset(1 as isize);
        nDb = ::libc::strlen(zDb) as ::core::ffi::c_int;
        if argc == 5 as ::core::ffi::c_int {
            if nDb == 4 as ::core::ffi::c_int
                && 0 as ::core::ffi::c_int
                    == crate::src::src::util::sqlite3_strnicmp(
                        b"temp\0" as *const u8 as *const ::core::ffi::c_char,
                        zDb,
                        4 as ::core::ffi::c_int,
                    )
            {
                zDb = *argv.offset(3 as isize);
                nDb = ::libc::strlen(zDb) as ::core::ffi::c_int;
                zFts3 = *argv.offset(4 as isize);
                current_block = 1856101646708284338;
            } else {
                current_block = 10728627893248296414;
            }
        } else {
            zFts3 = *argv.offset(3 as isize);
            current_block = 1856101646708284338;
        }
        match current_block {
            10728627893248296414 => {}
            _ => {
                nFts3 = ::libc::strlen(zFts3) as ::core::ffi::c_int;
                rc = crate::src::src::vtab::sqlite3_declare_vtab(db, FTS3_AUX_SCHEMA.as_ptr());
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                    return rc;
                }
                nByte = (::core::mem::size_of::<Fts3auxTable>() as usize)
                    .wrapping_add(::core::mem::size_of::<crate::fts3Int_h::Fts3Table>() as usize)
                    .wrapping_add(nDb as usize)
                    .wrapping_add(nFts3 as usize)
                    .wrapping_add(2 as usize) as crate::src::headers::sqlite3_h::sqlite3_int64;
                p = crate::src::src::malloc::sqlite3_malloc64(nByte as crate::src::headers::sqlite3_h::sqlite3_uint64) as *mut Fts3auxTable;
                if p.is_null() {
                    return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                }
                ::libc::memset(
                    p as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    nByte as crate::__stddef_size_t_h::size_t,
                );
                (*p).pFts3Tab = p.offset(1 as isize) as *mut Fts3auxTable
                    as *mut crate::fts3Int_h::Fts3Table;
                (*(*p).pFts3Tab).zDb = (*p).pFts3Tab.offset(1 as isize)
                    as *mut crate::fts3Int_h::Fts3Table
                    as *mut ::core::ffi::c_char;
                (*(*p).pFts3Tab).zName = (*(*p).pFts3Tab)
                    .zDb
                    .offset((nDb + 1 as ::core::ffi::c_int) as isize)
                    as *const ::core::ffi::c_char;
                (*(*p).pFts3Tab).db = db;
                (*(*p).pFts3Tab).nIndex = 1 as ::core::ffi::c_int;
                ::core::ptr::copy_nonoverlapping(
                    zDb as *const u8,
                    (*(*p).pFts3Tab).zDb as *mut ::core::ffi::c_char as *mut u8,
                    nDb as usize,
                );
                ::core::ptr::copy_nonoverlapping(
                    zFts3 as *const u8,
                    (*(*p).pFts3Tab).zName as *mut ::core::ffi::c_char as *mut u8,
                    nFts3 as usize,
                );
                crate::src::ext::fts3::fts3::sqlite3Fts3Dequote((*(*p).pFts3Tab).zName as *mut ::core::ffi::c_char);
                *ppVtab = p as *mut crate::src::headers::sqlite3_h::sqlite3_vtab;
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            }
        }
    }
    crate::src::ext::fts3::fts3::sqlite3Fts3ErrMsg(
        pzErr,
        b"invalid arguments to fts4aux constructor\0" as *const u8 as *const ::core::ffi::c_char,
    );
    crate::src::headers::sqlite3_h::SQLITE_ERROR
}

unsafe extern "C" fn fts3auxDisconnectMethod(mut pVtab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int {
    let mut p: *mut Fts3auxTable = pVtab as *mut Fts3auxTable;
    let mut pFts3: *mut crate::fts3Int_h::Fts3Table = (*p).pFts3Tab;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[*mut crate::src::headers::sqlite3_h::sqlite3_stmt; 40]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut crate::src::headers::sqlite3_h::sqlite3_stmt>() as usize)
            as ::core::ffi::c_int
    {
        crate::src::src::vdbeapi::sqlite3_finalize((*pFts3).aStmt[i as usize]);
        i += 1;
    }
    crate::src::src::malloc::sqlite3_free((*pFts3).zSegmentsTbl as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

pub const FTS4AUX_EQ_CONSTRAINT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const FTS4AUX_GE_CONSTRAINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const FTS4AUX_LE_CONSTRAINT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

unsafe extern "C" fn fts3auxBestIndexMethod(
    mut _pVTab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    mut pInfo: *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut iEq: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iGe: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iLe: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iLangid: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iNext: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let __pInfo_ref = unsafe { &mut *pInfo };
    if __pInfo_ref.nOrderBy == 1 as ::core::ffi::c_int
        && (*__pInfo_ref.aOrderBy.offset(0 as isize)).iColumn
            == 0 as ::core::ffi::c_int
        && (*__pInfo_ref.aOrderBy.offset(0 as isize)).desc as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        __pInfo_ref.orderByConsumed = 1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < __pInfo_ref.nConstraint {
        if (*__pInfo_ref.aConstraint.offset(i as isize)).usable != 0 {
            let mut op: ::core::ffi::c_int =
                (*__pInfo_ref.aConstraint.offset(i as isize)).op as ::core::ffi::c_int;
            let mut iCol: ::core::ffi::c_int = (*__pInfo_ref.aConstraint.offset(i as isize)).iColumn;
            if iCol == 0 as ::core::ffi::c_int {
                if op == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ {
                    iEq = i;
                }
                if op == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_LT_1 {
                    iLe = i;
                }
                if op == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_LE_1 {
                    iLe = i;
                }
                if op == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_GT_1 {
                    iGe = i;
                }
                if op == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_GE_1 {
                    iGe = i;
                }
            }
            if iCol == 4 as ::core::ffi::c_int {
                if op == crate::src::headers::sqlite3_h::SQLITE_INDEX_CONSTRAINT_EQ {
                    iLangid = i;
                }
            }
        }
        i += 1;
    }
    if iEq >= 0 as ::core::ffi::c_int {
        __pInfo_ref.idxNum = FTS4AUX_EQ_CONSTRAINT;
        let fresh9 = iNext;
        iNext += 1;
        (*__pInfo_ref.aConstraintUsage.offset(iEq as isize)).argvIndex = fresh9;
        __pInfo_ref.estimatedCost = 5 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else {
        __pInfo_ref.idxNum = 0 as ::core::ffi::c_int;
        __pInfo_ref.estimatedCost = 20000 as ::core::ffi::c_int as ::core::ffi::c_double;
        if iGe >= 0 as ::core::ffi::c_int {
            __pInfo_ref.idxNum += FTS4AUX_GE_CONSTRAINT;
            let fresh10 = iNext;
            iNext += 1;
            (*__pInfo_ref.aConstraintUsage.offset(iGe as isize)).argvIndex = fresh10;
            __pInfo_ref.estimatedCost /= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        if iLe >= 0 as ::core::ffi::c_int {
            __pInfo_ref.idxNum += FTS4AUX_LE_CONSTRAINT;
            let fresh11 = iNext;
            iNext += 1;
            (*__pInfo_ref.aConstraintUsage.offset(iLe as isize)).argvIndex = fresh11;
            __pInfo_ref.estimatedCost /= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
    }
    if iLangid >= 0 as ::core::ffi::c_int {
        let fresh12 = iNext;
        iNext += 1;
        (*__pInfo_ref.aConstraintUsage.offset(iLangid as isize)).argvIndex = fresh12;
        __pInfo_ref.estimatedCost -= 1.;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3auxOpenMethod(
    mut _pVTab: *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
    mut ppCsr: *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3auxCursor = ::core::ptr::null_mut::<Fts3auxCursor>();
    pCsr = crate::src::src::malloc::sqlite3_malloc(::core::mem::size_of::<Fts3auxCursor>() as ::core::ffi::c_int)
        as *mut Fts3auxCursor;
    if pCsr.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    ::libc::memset(
        pCsr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Fts3auxCursor>() as crate::__stddef_size_t_h::size_t,
    );
    *ppCsr = pCsr as *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3auxCloseMethod(
    mut pCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pFts3: *mut crate::fts3Int_h::Fts3Table = (*((*pCursor).pVtab as *mut Fts3auxTable)).pFts3Tab;
    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    crate::src::ext::fts3::fts3_write::sqlite3Fts3SegmentsClose(pFts3 as *mut crate::fts3Int_h::Fts3Table);
    let __pCsr_ref = unsafe { &mut *pCsr };
    crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderFinish(&raw mut __pCsr_ref.csr as *mut _ as *mut crate::fts3Int_h::Fts3MultiSegReader);
    crate::src::src::malloc::sqlite3_free(__pCsr_ref.filter.zTerm as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(__pCsr_ref.zStop as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(__pCsr_ref.aStat as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(pCsr as *mut ::core::ffi::c_void);
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3auxGrowStatArray(
    mut pCsr: *mut Fts3auxCursor,
    mut nSize: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if nSize > (*pCsr).nStat {
        let mut aNew: *mut Fts3auxColstats = ::core::ptr::null_mut::<Fts3auxColstats>();
        let __pCsr_ref = unsafe { &mut *pCsr };
        aNew = crate::src::src::malloc::sqlite3_realloc64(
            __pCsr_ref.aStat as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<Fts3auxColstats>() as usize).wrapping_mul(nSize as usize)
                as crate::src::headers::sqlite3_h::sqlite3_uint64,
        ) as *mut Fts3auxColstats;
        if aNew.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        ::libc::memset(
            aNew.offset(__pCsr_ref.nStat as isize) as *mut Fts3auxColstats as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<Fts3auxColstats>() as crate::__stddef_size_t_h::size_t)
                .wrapping_mul((nSize - __pCsr_ref.nStat) as crate::__stddef_size_t_h::size_t),
        );
        __pCsr_ref.aStat = aNew as *mut Fts3auxColstats;
        __pCsr_ref.nStat = nSize;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3auxNextMethod(
    mut pCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    let mut pFts3: *mut crate::fts3Int_h::Fts3Table = (*((*pCursor).pVtab as *mut Fts3auxTable)).pFts3Tab;
    let mut rc: ::core::ffi::c_int = 0;
    let __pCsr_ref = unsafe { &mut *pCsr };
    __pCsr_ref.iRowid += 1;
    __pCsr_ref.iCol += 1;
    while __pCsr_ref.iCol < __pCsr_ref.nStat {
        if (*__pCsr_ref.aStat.offset(__pCsr_ref.iCol as isize)).nDoc > 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        __pCsr_ref.iCol += 1;
    }
    rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderStep(pFts3 as *mut crate::fts3Int_h::Fts3Table,  &raw mut __pCsr_ref.csr as *mut _ as *mut crate::fts3Int_h::Fts3MultiSegReader);
    if rc == crate::src::headers::sqlite3_h::SQLITE_ROW {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nDoclist: ::core::ffi::c_int = __pCsr_ref.csr.nDoclist;
        let mut aDoclist: *mut ::core::ffi::c_char = __pCsr_ref.csr.aDoclist;
        let mut iCol: ::core::ffi::c_int = 0;
        let mut eState: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !__pCsr_ref.zStop.is_null() {
            let mut n: ::core::ffi::c_int = if __pCsr_ref.nStop < __pCsr_ref.csr.nTerm {
                __pCsr_ref.nStop
            } else {
                __pCsr_ref.csr.nTerm
            };
            let mut mc: ::core::ffi::c_int = ::libc::memcmp(
                __pCsr_ref.zStop as *const ::core::ffi::c_void,
                __pCsr_ref.csr.zTerm as *const ::core::ffi::c_void,
                n as crate::__stddef_size_t_h::size_t,
            );
            if mc < 0 as ::core::ffi::c_int
                || mc == 0 as ::core::ffi::c_int && __pCsr_ref.csr.nTerm > __pCsr_ref.nStop
            {
                __pCsr_ref.isEof = 1 as ::core::ffi::c_int;
                return crate::src::headers::sqlite3_h::SQLITE_OK;
            }
        }
        if fts3auxGrowStatArray(pCsr, 2 as ::core::ffi::c_int) != 0 {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        ::libc::memset(
            __pCsr_ref.aStat as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<Fts3auxColstats>() as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(__pCsr_ref.nStat as crate::__stddef_size_t_h::size_t),
        );
        iCol = 0 as ::core::ffi::c_int;
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        while i < nDoclist {
            let mut v: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
            i += crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(
                aDoclist.offset(i as isize) as *mut ::core::ffi::c_char,
                &raw mut v,
            );
            let mut current_block_42: u64;
            match eState {
                0 => {
                    let ref mut fresh0 =
                        (*__pCsr_ref.aStat.offset(0 as isize)).nDoc;
                    *fresh0 += 1;
                    eState = 1 as ::core::ffi::c_int;
                    iCol = 0 as ::core::ffi::c_int;
                    current_block_42 = 18435049525520518667;
                }
                1 => {
                    if v > 1 as crate::src::headers::sqlite3_h::sqlite3_int64 {
                        let ref mut fresh1 =
                            (*__pCsr_ref.aStat.offset(1 as isize)).nDoc;
                        *fresh1 += 1;
                    }
                    eState = 2 as ::core::ffi::c_int;
                    current_block_42 = 1445032439528402095;
                }
                2 => {
                    current_block_42 = 1445032439528402095;
                }
                _ => {
                    iCol = v as ::core::ffi::c_int;
                    if iCol < 1 as ::core::ffi::c_int {
                        rc = crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
                    } else {
                        if fts3auxGrowStatArray(pCsr, iCol + 2 as ::core::ffi::c_int) != 0 {
                            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                        }
                        let ref mut fresh4 = (*(*pCsr)
                            .aStat
                            .offset((iCol + 1 as ::core::ffi::c_int) as isize))
                        .nDoc;
                        *fresh4 += 1;
                        eState = 2 as ::core::ffi::c_int;
                    }
                    current_block_42 = 18435049525520518667;
                }
            }
            match current_block_42 {
                1445032439528402095 => {
                    if v == 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
                        eState = 0 as ::core::ffi::c_int;
                    } else if v == 1 as crate::src::headers::sqlite3_h::sqlite3_int64 {
                        eState = 3 as ::core::ffi::c_int;
                    } else {
                        let ref mut fresh2 = (*(*pCsr)
                            .aStat
                            .offset((iCol + 1 as ::core::ffi::c_int) as isize))
                        .nOcc;
                        *fresh2 += 1;
                        let ref mut fresh3 =
                            (*__pCsr_ref.aStat.offset(0 as isize)).nOcc;
                        *fresh3 += 1;
                    }
                }
                _ => {}
            }
        }
        __pCsr_ref.iCol = 0 as ::core::ffi::c_int;
    } else {
        __pCsr_ref.isEof = 1 as ::core::ffi::c_int;
    }
    rc
}

unsafe extern "C" fn fts3auxFilterMethod(
    mut pCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut _idxStr: *const ::core::ffi::c_char,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) -> ::core::ffi::c_int {
    let mut pCsr: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    let mut pFts3: *mut crate::fts3Int_h::Fts3Table = (*((*pCursor).pVtab as *mut Fts3auxTable)).pFts3Tab;
    let mut rc: ::core::ffi::c_int = 0;
    let mut isScan: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iLangVal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iEq: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iGe: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iLe: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iLangid: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut iNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if idxNum == FTS4AUX_EQ_CONSTRAINT {
        let fresh5 = iNext;
        iNext += 1;
        iEq = fresh5;
    } else {
        isScan = 1 as ::core::ffi::c_int;
        if idxNum & FTS4AUX_GE_CONSTRAINT != 0 {
            let fresh6 = iNext;
            iNext += 1;
            iGe = fresh6;
        }
        if idxNum & FTS4AUX_LE_CONSTRAINT != 0 {
            let fresh7 = iNext;
            iNext += 1;
            iLe = fresh7;
        }
    }
    if iNext < nVal {
        let fresh8 = iNext;
        iNext += 1;
        iLangid = fresh8;
    }
    let __pCsr_ref = unsafe { &mut *pCsr };
    crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderFinish(&raw mut __pCsr_ref.csr as *mut _ as *mut crate::fts3Int_h::Fts3MultiSegReader);
    crate::src::src::malloc::sqlite3_free(__pCsr_ref.filter.zTerm as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(__pCsr_ref.aStat as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(__pCsr_ref.zStop as *mut ::core::ffi::c_void);
    ::libc::memset(
        &raw mut __pCsr_ref.csr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (pCsr.offset(1 as isize) as *mut Fts3auxCursor as *mut crate::src::ext::rtree::rtree::u8_0)
            .offset_from(&raw mut __pCsr_ref.csr as *mut crate::src::ext::rtree::rtree::u8_0) as ::core::ffi::c_long
            as crate::__stddef_size_t_h::size_t,
    );
    __pCsr_ref.filter.flags = crate::fts3Int_h::FTS3_SEGMENT_REQUIRE_POS | crate::fts3Int_h::FTS3_SEGMENT_IGNORE_EMPTY;
    if isScan != 0 {
        __pCsr_ref.filter.flags |= crate::fts3Int_h::FTS3_SEGMENT_SCAN;
    }
    if iEq >= 0 as ::core::ffi::c_int || iGe >= 0 as ::core::ffi::c_int {
        let mut zStr: *const ::core::ffi::c_uchar =
            crate::src::src::vdbeapi::sqlite3_value_text(*apVal.offset(0 as isize));
        if !zStr.is_null() {
            __pCsr_ref.filter.zTerm =
                crate::src::src::printf::sqlite3_mprintf(b"%s\0" as *const u8 as *const ::core::ffi::c_char, zStr);
            if __pCsr_ref.filter.zTerm.is_null() {
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            __pCsr_ref.filter.nTerm = ::libc::strlen(__pCsr_ref.filter.zTerm) as ::core::ffi::c_int;
        }
    }
    if iLe >= 0 as ::core::ffi::c_int {
        __pCsr_ref.zStop = crate::src::src::printf::sqlite3_mprintf(
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            crate::src::src::vdbeapi::sqlite3_value_text(*apVal.offset(iLe as isize)),
        );
        if __pCsr_ref.zStop.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        __pCsr_ref.nStop = ::libc::strlen(__pCsr_ref.zStop) as ::core::ffi::c_int;
    }
    if iLangid >= 0 as ::core::ffi::c_int {
        iLangVal = crate::src::src::vdbeapi::sqlite3_value_int(*apVal.offset(iLangid as isize));
        if iLangVal < 0 as ::core::ffi::c_int {
            iLangVal = 0 as ::core::ffi::c_int;
        }
    }
    __pCsr_ref.iLangid = iLangVal;
    rc = crate::src::ext::fts3::fts3::sqlite3Fts3SegReaderCursor(
        
        pFts3 as *mut crate::fts3Int_h::Fts3Table,
        iLangVal,
        0 as ::core::ffi::c_int,
        crate::fts3Int_h::FTS3_SEGCURSOR_ALL,
        __pCsr_ref.filter.zTerm,
        __pCsr_ref.filter.nTerm,
        0 as ::core::ffi::c_int,
        isScan,
        
        &raw mut __pCsr_ref.csr as *mut _ as *mut crate::fts3Int_h::Fts3MultiSegReader,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3SegReaderStart(pFts3 as *mut crate::fts3Int_h::Fts3Table,  &raw mut __pCsr_ref.csr as *mut _ as *mut crate::fts3Int_h::Fts3MultiSegReader,  &raw mut __pCsr_ref.filter as *mut _ as *mut crate::fts3Int_h::Fts3SegFilter);
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = fts3auxNextMethod(pCursor);
    }
    rc
}

unsafe extern "C" fn fts3auxEofMethod(mut pCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int {
    let pCsr = &*(pCursor as *mut Fts3auxCursor);
    pCsr.isEof
}

unsafe extern "C" fn fts3auxColumnMethod(
    mut pCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    mut pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut Fts3auxCursor = pCursor as *mut Fts3auxCursor;
    match iCol {
        0 => {
            crate::src::src::vdbeapi::sqlite3_result_text(
                pCtx,
                (*p).csr.zTerm,
                (*p).csr.nTerm,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
            );
        }
        1 => {
            if (*p).iCol != 0 {
                crate::src::src::vdbeapi::sqlite3_result_int(pCtx, (*p).iCol - 1 as ::core::ffi::c_int);
            } else {
                crate::src::src::vdbeapi::sqlite3_result_text(
                    pCtx,
                    b"*\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int),
                    crate::src::headers::sqlite3_h::SQLITE_STATIC,
                );
            }
        }
        2 => {
            crate::src::src::vdbeapi::sqlite3_result_int64(pCtx, (*(*p).aStat.offset((*p).iCol as isize)).nDoc);
        }
        3 => {
            crate::src::src::vdbeapi::sqlite3_result_int64(pCtx, (*(*p).aStat.offset((*p).iCol as isize)).nOcc);
        }
        _ => {
            crate::src::src::vdbeapi::sqlite3_result_int(pCtx, (*p).iLangid);
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3auxRowidMethod(
    mut pCursor: *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
    mut pRowid: *mut crate::src::headers::sqlite3_h::sqlite_int64,
) -> ::core::ffi::c_int {
    let pCsr = &*(pCursor as *mut Fts3auxCursor);
    *pRowid = pCsr.iRowid as crate::src::headers::sqlite3_h::sqlite_int64;
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[no_mangle]

pub unsafe extern "C" fn sqlite3Fts3InitAux(mut db: *mut crate::src::headers::sqliteInt_h::sqlite3) -> ::core::ffi::c_int {
    static mut fts3aux_module: crate::src::headers::sqlite3_h::sqlite3_module = unsafe {
        crate::src::headers::sqlite3_h::sqlite3_module {
    iVersion:  0 as ::core::ffi::c_int,
    xCreate:  Some(
                fts3auxConnectMethod
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqliteInt_h::sqlite3,
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const *const ::core::ffi::c_char,
                        *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        *mut *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
    xConnect:  Some(
                fts3auxConnectMethod
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqliteInt_h::sqlite3,
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *const *const ::core::ffi::c_char,
                        *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        *mut *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
    xBestIndex:  Some(
                fts3auxBestIndexMethod
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        *mut crate::src::headers::sqlite3_h::sqlite3_index_info,
                    ) -> ::core::ffi::c_int,
            ),
    xDisconnect:  Some(
                fts3auxDisconnectMethod
                    as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int,
            ),
    xDestroy:  Some(
                fts3auxDisconnectMethod
                    as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab) -> ::core::ffi::c_int,
            ),
    xOpen:  Some(
                fts3auxOpenMethod
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab,
                        *mut *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                    ) -> ::core::ffi::c_int,
            ),
    xClose:  Some(
                fts3auxCloseMethod
                    as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
    xFilter:  Some(
                fts3auxFilterMethod
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> ::core::ffi::c_int,
            ),
    xNext:  Some(
                fts3auxNextMethod
                    as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
    xEof:  Some(
                fts3auxEofMethod
                    as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor) -> ::core::ffi::c_int,
            ),
    xColumn:  Some(
                fts3auxColumnMethod
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
    xRowid:  Some(
                fts3auxRowidMethod
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::sqlite3_h::sqlite3_vtab_cursor,
                        *mut crate::src::headers::sqlite3_h::sqlite_int64,
                    ) -> ::core::ffi::c_int,
            ),
    xUpdate:  None,
    xBegin:  None,
    xSync:  None,
    xCommit:  None,
    xRollback:  None,
    xFindFunction:  None,
    xRename:  None,
    xSavepoint:  None,
    xRelease:  None,
    xRollbackTo:  None,
    xShadowName:  None,
    xIntegrity:  None,
}
    };
    let mut rc: ::core::ffi::c_int = 0;
    rc = crate::src::src::vtab::sqlite3_create_module(
        db,
        b"fts4aux\0" as *const u8 as *const ::core::ffi::c_char,
        
        &raw const fts3aux_module as *const _ as
    *const crate::src::headers::sqlite3_h::sqlite3_module,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    rc
}
