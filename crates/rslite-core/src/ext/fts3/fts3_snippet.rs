pub use crate::__stddef_size_t_h::SizeT;

pub use crate::fts3Int_h::FTS_CORRUPT_VTAB;
pub use crate::fts3Int_h::FTSQUERY_NOT_1;
pub use crate::fts3Int_h::FTSQUERY_PHRASE;
pub use crate::fts3Int_h::Fts3Cursor;
pub use crate::fts3Int_h::Fts3DeferredToken;
pub use crate::fts3Int_h::Fts3Doclist;
pub use crate::fts3Int_h::Fts3Expr;
pub use crate::fts3Int_h::Fts3Index;
pub use crate::fts3Int_h::Fts3MultiSegReader;
pub use crate::fts3Int_h::Fts3Phrase;
pub use crate::fts3Int_h::Fts3PhraseToken;
pub use crate::fts3Int_h::Fts3SegFilter;
pub use crate::fts3Int_h::Fts3SegReader;
pub use crate::fts3Int_h::Fts3Table;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3ErrMsg;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3EvalPhrasePoslist;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3EvalPhraseStats;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3EvalTestDeferred;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3GetVarintBounded;
pub use crate::src::ext::fts3::fts3::sqlite3Fts3MsrCancel;
pub use crate::src::ext::fts3::fts3_expr::sqlite3Fts3MallocZero;
pub use crate::src::ext::fts3::fts3_expr::sqlite3Fts3OpenTokenizer;
pub use crate::src::ext::fts3::fts3_hash::_fts3ht;
pub use crate::src::ext::fts3::fts3_hash::Fts3Hash;
pub use crate::src::ext::fts3::fts3_hash::Fts3HashElem;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor;
pub use crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SegmentsClose;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SelectDocsize;
pub use crate::src::ext::fts3::fts3_write::sqlite3Fts3SelectDoctotal;
pub use crate::src::ext::rtree::rtree::I64_0;
pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::ext::rtree::rtree::U64_0;
pub use crate::src::fts5::I16_0;
pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT;
pub use crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
pub use crate::src::headers::sqlite3_h::SQLITE_DONE;
pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;
pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_NULL;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SQLITE_STATIC;
pub use crate::src::headers::sqlite3_h::SqliteInt64;
pub use crate::src::headers::sqlite3_h::SqliteUint64;
pub use crate::src::headers::sqlite3_h::Sqlite3Blob;
pub use crate::src::headers::sqlite3_h::Sqlite3DestructorType;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint;
pub use crate::src::headers::sqlite3_h::sqlite3_index_constraint_usage;
pub use crate::src::headers::sqlite3_h::sqlite3_index_info;
pub use crate::src::headers::sqlite3_h::sqlite3_index_orderby;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::sqlite3_module;
pub use crate::src::headers::sqlite3_h::Sqlite3Stmt;
pub use crate::src::headers::sqlite3_h::Sqlite3Uint64;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab;
pub use crate::src::headers::sqlite3_h::sqlite3_vtab_cursor;
pub use crate::src::headers::sqliteInt_h::sqlite3;
pub use crate::src::headers::vdbeInt_h::sqlite3_context;
pub use crate::src::headers::vdbeInt_h::sqlite3_value;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3_malloc64;
pub use crate::src::src::malloc::sqlite3_realloc64;
pub use crate::src::src::vdbeapi::sqlite3_column_blob;
pub use crate::src::src::vdbeapi::sqlite3_column_bytes;
pub use crate::src::src::vdbeapi::sqlite3_column_text;
pub use crate::src::src::vdbeapi::sqlite3_column_type;
pub use crate::src::src::vdbeapi::sqlite3_reset;
pub use crate::src::src::vdbeapi::sqlite3_result_blob;
pub use crate::src::src::vdbeapi::sqlite3_result_error;
pub use crate::src::src::vdbeapi::sqlite3_result_error_code;
pub use crate::src::src::vdbeapi::sqlite3_result_text;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchinfoBuffer {
    pub aRef: [crate::src::ext::rtree::rtree::U8_0; 3],
    pub nElem: ::core::ffi::c_int,
    pub bGlobal: ::core::ffi::c_int,
    pub zMatchinfo: *mut ::core::ffi::c_char,
    pub aMI: [crate::src::ext::rtree::rtree::U32_0; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StrBuffer {
    pub z: *mut ::core::ffi::c_char,
    pub n: ::core::ffi::c_int,
    pub nAlloc: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TermOffset {
    pub pList: *mut ::core::ffi::c_char,
    pub iPos: crate::src::ext::rtree::rtree::I64_0,
    pub iOff: crate::src::ext::rtree::rtree::I64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TermOffsetCtx {
    pub pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pub iCol: ::core::ffi::c_int,
    pub iTerm: ::core::ffi::c_int,
    pub iDocid: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub aTerm: *mut TermOffset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadDoclistCtx {
    pub pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pub nPhrase: ::core::ffi::c_int,
    pub nToken: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SnippetFragment {
    pub iCol: ::core::ffi::c_int,
    pub iPos: ::core::ffi::c_int,
    pub covered: crate::src::ext::rtree::rtree::U64_0,
    pub hlmask: crate::src::ext::rtree::rtree::U64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SnippetPhrase {
    pub nToken: ::core::ffi::c_int,
    pub pList: *mut ::core::ffi::c_char,
    pub iHead: crate::src::ext::rtree::rtree::I64_0,
    pub pHead: *mut ::core::ffi::c_char,
    pub iTail: crate::src::ext::rtree::rtree::I64_0,
    pub pTail: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SnippetIter {
    pub pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pub iCol: ::core::ffi::c_int,
    pub nSnippet: ::core::ffi::c_int,
    pub nPhrase: ::core::ffi::c_int,
    pub aPhrase: *mut SnippetPhrase,
    pub iCurrent: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchInfo {
    pub pCursor: *mut crate::fts3Int_h::Fts3Cursor,
    pub nCol: ::core::ffi::c_int,
    pub nPhrase: ::core::ffi::c_int,
    pub nDoc: crate::src::headers::sqlite3_h::Sqlite3Int64,
    pub flag: ::core::ffi::c_char,
    pub aMatchinfo: *mut crate::src::ext::rtree::rtree::U32_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LcsIterator {
    pub pExpr: *mut crate::fts3Int_h::Fts3Expr,
    pub iPosOffset: ::core::ffi::c_int,
    pub pRead: *mut ::core::ffi::c_char,
    pub iPos: ::core::ffi::c_int,
}

pub const FTS3_MATCHINFO_NPHRASE: ::core::ffi::c_int = 112;

pub const FTS3_MATCHINFO_NCOL: ::core::ffi::c_int = 99;

pub const FTS3_MATCHINFO_NDOC: ::core::ffi::c_int = 110;

pub const FTS3_MATCHINFO_AVGLENGTH: ::core::ffi::c_int = 97;

pub const FTS3_MATCHINFO_LENGTH: ::core::ffi::c_int = 108;

pub const FTS3_MATCHINFO_LCS: ::core::ffi::c_int = 115;

pub const FTS3_MATCHINFO_HITS: ::core::ffi::c_int = 'x' as i32;

pub const FTS3_MATCHINFO_LHITS: ::core::ffi::c_int = 121;

pub const FTS3_MATCHINFO_LHITS_BM: ::core::ffi::c_int = 98;

pub const FTS3_MATCHINFO_DEFAULT: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"pcx\0") };

unsafe extern "C" fn fts3MIBufferNew(
    nElem: crate::__stddef_size_t_h::SizeT,
    zMatchinfo: *const ::core::ffi::c_char,
) -> *mut MatchinfoBuffer {
    
    let nByte: crate::src::headers::sqlite3_h::Sqlite3Int64 = (::core::mem::size_of::<
        crate::src::ext::rtree::rtree::U32_0,
    >()
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(
            (2 as crate::src::headers::sqlite3_h::Sqlite3Int64
                * nElem as crate::src::headers::sqlite3_h::Sqlite3Int64
                + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64)
                as ::core::ffi::c_ulonglong,
        )
        .wrapping_add(
            24_usize.wrapping_add(
                (((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int)
                    as usize)
                    .wrapping_mul(
                        ::core::mem::size_of::<crate::src::ext::rtree::rtree::U64_0>() as usize,
                    ),
            ) as ::core::ffi::c_ulonglong,
        )
        as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let nStr: crate::src::headers::sqlite3_h::Sqlite3Int64 =
        ::libc::strlen(zMatchinfo) as crate::src::headers::sqlite3_h::Sqlite3Int64;
    let pRet: *mut MatchinfoBuffer = crate::src::ext::fts3::fts3_expr::sqlite3Fts3MallocZero(
        nByte as crate::src::ext::rtree::rtree::I64_0
            + nStr as crate::src::ext::rtree::rtree::I64_0
            + 1 as crate::src::ext::rtree::rtree::I64_0,
    ) as *mut MatchinfoBuffer;
    if !pRet.is_null() {
        let __pRet_ref = unsafe { &mut *pRet };
        *(&raw mut __pRet_ref.aMI as *mut crate::src::ext::rtree::rtree::U32_0)
            .offset(0_isize) =
            ((&raw mut __pRet_ref.aMI as *mut crate::src::ext::rtree::rtree::U32_0)
                .offset(1_isize) as *mut crate::src::ext::rtree::rtree::U32_0
                as *mut crate::src::ext::rtree::rtree::U8_0)
                .offset_from(pRet as *mut crate::src::ext::rtree::rtree::U8_0)
                as ::core::ffi::c_long as crate::src::ext::rtree::rtree::U32_0;
        *(&raw mut __pRet_ref.aMI as *mut crate::src::ext::rtree::rtree::U32_0)
            .add((1 as crate::__stddef_size_t_h::SizeT).wrapping_add(nElem)) =
            (*(&raw mut __pRet_ref.aMI as *mut crate::src::ext::rtree::rtree::U32_0)
                .offset(0_isize) as usize)
                .wrapping_add(
                    (::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>() as usize)
                        .wrapping_mul(
                            (nElem as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize,
                        ),
                ) as crate::src::ext::rtree::rtree::U32_0;
        __pRet_ref.nElem = nElem as ::core::ffi::c_int;
        __pRet_ref.zMatchinfo = (pRet as *mut ::core::ffi::c_char).offset(nByte as isize);
        ::core::ptr::copy_nonoverlapping(
            zMatchinfo as *const u8,
            __pRet_ref.zMatchinfo as *mut u8,
            (nStr + 1 as crate::src::headers::sqlite3_h::Sqlite3Int64) as usize,
        );
        __pRet_ref.aRef[0 as ::core::ffi::c_int as usize] =
            1 as crate::src::ext::rtree::rtree::U8_0;
    }
    pRet
}

unsafe extern "C" fn fts3MIBufferFree(p: *mut ::core::ffi::c_void) {
    let pBuf: *mut MatchinfoBuffer = (p as *mut crate::src::ext::rtree::rtree::U8_0).offset(
        -(*(p as *mut crate::src::ext::rtree::rtree::U32_0)
            .offset(-(1 as ::core::ffi::c_int) as isize) as isize),
    ) as *mut MatchinfoBuffer;
    let __pBuf_ref = unsafe { &mut *pBuf };
    if std::ptr::eq(p as *mut crate::src::ext::rtree::rtree::U32_0, (&raw mut __pBuf_ref.aMI as *mut crate::src::ext::rtree::rtree::U32_0).offset(1_isize))
    {
        __pBuf_ref.aRef[1 as ::core::ffi::c_int as usize] =
            0 as crate::src::ext::rtree::rtree::U8_0;
    } else {
        __pBuf_ref.aRef[2 as ::core::ffi::c_int as usize] =
            0 as crate::src::ext::rtree::rtree::U8_0;
    }
    if __pBuf_ref.aRef[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        && __pBuf_ref.aRef[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        && __pBuf_ref.aRef[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        crate::src::src::malloc::sqlite3_free(pBuf as *mut ::core::ffi::c_void);
    }
}

unsafe extern "C" fn fts3MIBufferAlloc(
    p: *mut MatchinfoBuffer,
    paOut: *mut *mut crate::src::ext::rtree::rtree::U32_0,
) -> Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> {
    let mut xRet: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> = None;
    let aOut: *mut crate::src::ext::rtree::rtree::U32_0;
    if (*p).aRef[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        (*p).aRef[1 as ::core::ffi::c_int as usize] = 1 as crate::src::ext::rtree::rtree::U8_0;
        aOut = (&raw mut (*p).aMI as *mut crate::src::ext::rtree::rtree::U32_0).offset(1_isize)
            as *mut crate::src::ext::rtree::rtree::U32_0;
        xRet = Some(fts3MIBufferFree as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    } else if (*p).aRef[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        let __p_ref = unsafe { &mut *p };
        __p_ref.aRef[2 as ::core::ffi::c_int as usize] = 1 as crate::src::ext::rtree::rtree::U8_0;
        aOut = (&raw mut __p_ref.aMI as *mut crate::src::ext::rtree::rtree::U32_0)
            .offset((__p_ref.nElem + 2 as ::core::ffi::c_int) as isize)
            as *mut crate::src::ext::rtree::rtree::U32_0;
        xRet = Some(fts3MIBufferFree as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    } else {
        aOut = crate::src::src::malloc::sqlite3_malloc64(
            ((*p).nElem as usize).wrapping_mul(::core::mem::size_of::<
                crate::src::ext::rtree::rtree::U32_0,
            >() as usize) as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut crate::src::ext::rtree::rtree::U32_0;
        if !aOut.is_null() {
            xRet = Some(
                crate::src::src::malloc::sqlite3_free
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
            if (*p).bGlobal != 0 {
                ::core::ptr::copy_nonoverlapping(
                    (&raw mut (*p).aMI as *mut crate::src::ext::rtree::rtree::U32_0)
                        .offset(1_isize)
                        as *mut crate::src::ext::rtree::rtree::U32_0
                        as *const u8,
                    aOut as *mut u8,
                    (((*p).nElem as crate::__stddef_size_t_h::SizeT).wrapping_mul(
                        ::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>()
                            as crate::__stddef_size_t_h::SizeT,
                    )) as usize,
                );
            }
        }
    }
    *paOut = aOut;
    xRet
}

unsafe extern "C" fn fts3MIBufferSetGlobal(p: *mut MatchinfoBuffer) {
    let __p_ref = unsafe { &mut *p };
    __p_ref.bGlobal = 1 as ::core::ffi::c_int;
    ::core::ptr::copy_nonoverlapping(
        (&raw mut __p_ref.aMI as *mut crate::src::ext::rtree::rtree::U32_0).offset(1_isize)
            as *mut crate::src::ext::rtree::rtree::U32_0 as *const u8,
        (&raw mut __p_ref.aMI as *mut crate::src::ext::rtree::rtree::U32_0)
            .offset((2 as ::core::ffi::c_int + __p_ref.nElem) as isize)
            as *mut crate::src::ext::rtree::rtree::U32_0 as *mut u8,
        ((__p_ref.nElem as crate::__stddef_size_t_h::SizeT).wrapping_mul(::core::mem::size_of::<
            crate::src::ext::rtree::rtree::U32_0,
        >()
            as crate::__stddef_size_t_h::SizeT)) as usize,
    );
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3Fts3MIBufferFree(p: *mut MatchinfoBuffer) {
    if !p.is_null() {
        let __p_ref = unsafe { &mut *p };
        __p_ref.aRef[0 as ::core::ffi::c_int as usize] = 0 as crate::src::ext::rtree::rtree::U8_0;
        if __p_ref.aRef[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && __p_ref.aRef[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            && __p_ref.aRef[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }
}

unsafe extern "C" fn fts3GetDeltaPosition(
    pp: *mut *mut ::core::ffi::c_char,
    piPos: *mut crate::src::ext::rtree::rtree::I64_0,
) {
    let mut iVal: ::core::ffi::c_int = 0;
    *pp = (*pp).offset(
        (if *(*pp as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            != 0
        {
            crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(*pp, &raw mut iVal)
        } else {
            iVal = *(*pp as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
            1 as ::core::ffi::c_int
        }) as isize,
    );
    *piPos += (iVal - 2 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::I64_0;
}

unsafe extern "C" fn fts3ExprIterate2(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    piPhrase: *mut ::core::ffi::c_int,
    x: Option<
        unsafe extern "C" fn(
            *mut crate::fts3Int_h::Fts3Expr,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let eType: ::core::ffi::c_int = (*pExpr).eType;
    if eType != crate::fts3Int_h::FTSQUERY_PHRASE {
        rc = fts3ExprIterate2((*pExpr).pLeft, piPhrase, x, pCtx);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK
            && eType != crate::fts3Int_h::FTSQUERY_NOT_1
        {
            rc = fts3ExprIterate2((*pExpr).pRight, piPhrase, x, pCtx);
        }
    } else {
        rc = x.expect("non-null function pointer")(pExpr, *piPhrase, pCtx);
        *piPhrase += 1;
    }
    rc
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3Fts3ExprIterate(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    x: Option<
        unsafe extern "C" fn(
            *mut crate::fts3Int_h::Fts3Expr,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut iPhrase: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    fts3ExprIterate2(pExpr, &raw mut iPhrase, x, pCtx)
}

unsafe extern "C" fn fts3ExprLoadDoclistsCb(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    mut _iPhrase: ::core::ffi::c_int,
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let pPhrase: *mut crate::fts3Int_h::Fts3Phrase = (*pExpr).pPhrase;
    let p: *mut LoadDoclistCtx = ctx as *mut LoadDoclistCtx;
    (*p).nPhrase += 1;
    (*p).nToken += (*pPhrase).nToken;
    rc
}

unsafe extern "C" fn fts3ExprLoadDoclists(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pnPhrase: *mut ::core::ffi::c_int,
    pnToken: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    
    let mut sCtx: LoadDoclistCtx = unsafe { ::core::mem::zeroed() };
    sCtx.pCsr = pCsr;
    let rc: ::core::ffi::c_int = sqlite3Fts3ExprIterate(
        (*pCsr).pExpr,
        Some(
            fts3ExprLoadDoclistsCb
                as unsafe extern "C" fn(
                    *mut crate::fts3Int_h::Fts3Expr,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut sCtx as *mut ::core::ffi::c_void,
    );
    if !pnPhrase.is_null() {
        *pnPhrase = sCtx.nPhrase;
    }
    if !pnToken.is_null() {
        *pnToken = sCtx.nToken;
    }
    rc
}

unsafe extern "C" fn fts3ExprPhraseCountCb(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    iPhrase: ::core::ffi::c_int,
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let fresh3 = &mut *(ctx as *mut ::core::ffi::c_int);
    *fresh3 += 1;
    (*pExpr).iPhrase = iPhrase;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3ExprPhraseCount(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
) -> ::core::ffi::c_int {
    let mut nPhrase: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    sqlite3Fts3ExprIterate(
        pExpr,
        Some(
            fts3ExprPhraseCountCb
                as unsafe extern "C" fn(
                    *mut crate::fts3Int_h::Fts3Expr,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut nPhrase as *mut ::core::ffi::c_void,
    );
    nPhrase
}

unsafe extern "C" fn fts3SnippetAdvance(
    ppIter: *mut *mut ::core::ffi::c_char,
    piIter: *mut crate::src::ext::rtree::rtree::I64_0,
    iNext: ::core::ffi::c_int,
) {
    let mut pIter: *mut ::core::ffi::c_char = *ppIter;
    if !pIter.is_null() {
        let mut iIter: crate::src::ext::rtree::rtree::I64_0 = *piIter;
        while iIter < iNext as crate::src::ext::rtree::rtree::I64_0 {
            if 0 as ::core::ffi::c_int == *pIter as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int
            {
                iIter = -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::I64_0;
                pIter = ::core::ptr::null_mut::<::core::ffi::c_char>();
                break;
            } else {
                fts3GetDeltaPosition(&raw mut pIter, &raw mut iIter);
            }
        }
        *piIter = iIter;
        *ppIter = pIter;
    }
}

unsafe extern "C" fn fts3SnippetNextCandidate(pIter: *mut SnippetIter) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int;
    if (*pIter).iCurrent < 0 as ::core::ffi::c_int {
        (*pIter).iCurrent = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*pIter).nPhrase {
            let pPhrase: *mut SnippetPhrase =
                (*pIter).aPhrase.offset(i as isize) as *mut SnippetPhrase;
            fts3SnippetAdvance(
                &raw mut (*pPhrase).pHead,
                &raw mut (*pPhrase).iHead,
                (*pIter).nSnippet,
            );
            i += 1;
        }
    } else {
        
        let mut iEnd: ::core::ffi::c_int = 0x7fffffff as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        let __pIter_ref = unsafe { &mut *pIter };
        while i < __pIter_ref.nPhrase {
            let pPhrase_0 = &*(__pIter_ref.aPhrase.offset(i as isize) as *mut SnippetPhrase);

            if !pPhrase_0.pHead.is_null()
                && pPhrase_0.iHead < iEnd as crate::src::ext::rtree::rtree::I64_0
            {
                iEnd = pPhrase_0.iHead as ::core::ffi::c_int;
            }
            i += 1;
        }
        if iEnd == 0x7fffffff as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        let iStart: ::core::ffi::c_int = iEnd - __pIter_ref.nSnippet + 1 as ::core::ffi::c_int;
        __pIter_ref.iCurrent = iStart;
        i = 0 as ::core::ffi::c_int;
        while i < __pIter_ref.nPhrase {
            let pPhrase_1: *mut SnippetPhrase =
                __pIter_ref.aPhrase.offset(i as isize) as *mut SnippetPhrase;
            let __pPhrase_1_ref = unsafe { &mut *pPhrase_1 };
            fts3SnippetAdvance(
                &raw mut __pPhrase_1_ref.pHead,
                &raw mut __pPhrase_1_ref.iHead,
                iEnd + 1 as ::core::ffi::c_int,
            );
            fts3SnippetAdvance(
                &raw mut __pPhrase_1_ref.pTail,
                &raw mut __pPhrase_1_ref.iTail,
                iStart,
            );
            i += 1;
        }
    }
    0 as ::core::ffi::c_int
}

unsafe extern "C" fn fts3SnippetDetails(
    pIter: *mut SnippetIter,
    mCovered: crate::src::ext::rtree::rtree::U64_0,
    piToken: *mut ::core::ffi::c_int,
    piScore: *mut ::core::ffi::c_int,
    pmCover: *mut crate::src::ext::rtree::rtree::U64_0,
    pmHighlight: *mut crate::src::ext::rtree::rtree::U64_0,
) {
    let iStart: ::core::ffi::c_int = (*pIter).iCurrent;
    let mut iScore: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    let mut mCover: crate::src::ext::rtree::rtree::U64_0 =
        0 as crate::src::ext::rtree::rtree::U64_0;
    let mut mHighlight: crate::src::ext::rtree::rtree::U64_0 =
        0 as crate::src::ext::rtree::rtree::U64_0;
    i = 0 as ::core::ffi::c_int;
    while i < (*pIter).nPhrase {
        let pPhrase: *mut SnippetPhrase =
            (*pIter).aPhrase.offset(i as isize) as *mut SnippetPhrase;
        if !(*pPhrase).pTail.is_null() {
            let mut pCsr: *mut ::core::ffi::c_char = (*pPhrase).pTail;
            let mut iCsr: crate::src::ext::rtree::rtree::I64_0 = (*pPhrase).iTail;
            while iCsr < (iStart + (*pIter).nSnippet) as crate::src::ext::rtree::rtree::I64_0
                && iCsr >= iStart as crate::src::ext::rtree::rtree::I64_0
            {
                let mut j: ::core::ffi::c_int;
                let mPhrase: crate::src::ext::rtree::rtree::U64_0 = (1 as ::core::ffi::c_int
                    as crate::src::ext::rtree::rtree::U64_0) << (i % 64 as ::core::ffi::c_int);
                let mPos: crate::src::ext::rtree::rtree::U64_0 = (1 as ::core::ffi::c_int
                    as crate::src::ext::rtree::rtree::U64_0) << (iCsr - iStart as crate::src::ext::rtree::rtree::I64_0);
                if (mCover | mCovered) & mPhrase != 0 {
                    iScore += 1;
                } else {
                    iScore += 1000 as ::core::ffi::c_int;
                }
                mCover |= mPhrase;
                j = 0 as ::core::ffi::c_int;
                while j < (*pPhrase).nToken && j < (*pIter).nSnippet {
                    mHighlight |= mPos >> j;
                    j += 1;
                }
                if 0 as ::core::ffi::c_int
                    == *pCsr as ::core::ffi::c_int & 0xfe as ::core::ffi::c_int
                {
                    break;
                }
                fts3GetDeltaPosition(&raw mut pCsr, &raw mut iCsr);
            }
        }
        i += 1;
    }
    *piToken = iStart;
    *piScore = iScore;
    *pmCover = mCover;
    *pmHighlight = mHighlight;
}

unsafe extern "C" fn fts3SnippetFindPositions(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    iPhrase: ::core::ffi::c_int,
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let p: *mut SnippetIter = ctx as *mut SnippetIter;
    let __p_ref = unsafe { &mut *p };
    let pPhrase: *mut SnippetPhrase =
        __p_ref.aPhrase.offset(iPhrase as isize) as *mut SnippetPhrase;
    let mut pCsr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rc: ::core::ffi::c_int;
    (*pPhrase).nToken = (*(*pExpr).pPhrase).nToken;
    rc = crate::src::ext::fts3::fts3::sqlite3Fts3EvalPhrasePoslist(
        __p_ref.pCsr as *mut crate::fts3Int_h::Fts3Cursor,
        pExpr as *mut crate::fts3Int_h::Fts3Expr,
        __p_ref.iCol,
        &raw mut pCsr,
    );
    if !pCsr.is_null() {
        let mut iFirst: crate::src::ext::rtree::rtree::I64_0 =
            0 as crate::src::ext::rtree::rtree::I64_0;
        (*pPhrase).pList = pCsr;
        fts3GetDeltaPosition(&raw mut pCsr, &raw mut iFirst);
        if iFirst < 0 as crate::src::ext::rtree::rtree::I64_0 {
            rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
        } else {
            let __pPhrase_ref = unsafe { &mut *pPhrase };
            __pPhrase_ref.pHead = pCsr;
            __pPhrase_ref.pTail = pCsr;
            __pPhrase_ref.iHead = iFirst;
            __pPhrase_ref.iTail = iFirst;
        }
    }
    rc
}

unsafe extern "C" fn fts3BestSnippet(
    nSnippet: ::core::ffi::c_int,
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    iCol: ::core::ffi::c_int,
    mCovered: crate::src::ext::rtree::rtree::U64_0,
    pmSeen: *mut crate::src::ext::rtree::rtree::U64_0,
    pFragment: *mut SnippetFragment,
    piScore: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int;
    let mut nList: ::core::ffi::c_int = 0;
    let mut sIter: SnippetIter = unsafe { ::core::mem::zeroed() };
    
    let mut iBestScore: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int;
    rc = fts3ExprLoadDoclists(
        pCsr,
        &raw mut nList,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    let nByte: crate::src::headers::sqlite3_h::Sqlite3Int64 = (::core::mem::size_of::<SnippetPhrase>() as usize).wrapping_mul(nList as usize)
        as crate::src::headers::sqlite3_h::Sqlite3Int64;
    sIter.aPhrase = crate::src::ext::fts3::fts3_expr::sqlite3Fts3MallocZero(
        nByte as crate::src::ext::rtree::rtree::I64_0,
    ) as *mut SnippetPhrase;
    if sIter.aPhrase.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    sIter.pCsr = pCsr;
    sIter.iCol = iCol;
    sIter.nSnippet = nSnippet;
    sIter.nPhrase = nList;
    sIter.iCurrent = -(1 as ::core::ffi::c_int);
    rc = sqlite3Fts3ExprIterate(
        (*pCsr).pExpr,
        Some(
            fts3SnippetFindPositions
                as unsafe extern "C" fn(
                    *mut crate::fts3Int_h::Fts3Expr,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut sIter as *mut ::core::ffi::c_void,
    );
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        i = 0 as ::core::ffi::c_int;
        while i < nList {
            if !(*sIter.aPhrase.offset(i as isize)).pHead.is_null() {
                *pmSeen |= (1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U64_0) << (i % 64 as ::core::ffi::c_int);
            }
            i += 1;
        }
        (*pFragment).iCol = iCol;
        while fts3SnippetNextCandidate(&raw mut sIter) == 0 {
            let mut iPos: ::core::ffi::c_int = 0;
            let mut iScore: ::core::ffi::c_int = 0;
            let mut mCover: crate::src::ext::rtree::rtree::U64_0 = 0;
            let mut mHighlite: crate::src::ext::rtree::rtree::U64_0 = 0;
            fts3SnippetDetails(
                &raw mut sIter,
                mCovered,
                &raw mut iPos,
                &raw mut iScore,
                &raw mut mCover,
                &raw mut mHighlite,
            );
            if iScore > iBestScore {
                let __pFragment_ref = unsafe { &mut *pFragment };
                __pFragment_ref.iPos = iPos;
                __pFragment_ref.hlmask = mHighlite;
                __pFragment_ref.covered = mCover;
                iBestScore = iScore;
            }
        }
        *piScore = iBestScore;
    }
    crate::src::src::malloc::sqlite3_free(sIter.aPhrase as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn fts3StringAppend(
    pStr: *mut StrBuffer,
    zAppend: *const ::core::ffi::c_char,
    mut nAppend: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if nAppend < 0 as ::core::ffi::c_int {
        nAppend = ::libc::strlen(zAppend) as ::core::ffi::c_int;
    }
    let __pStr_ref = unsafe { &mut *pStr };
    if __pStr_ref.n + nAppend + 1 as ::core::ffi::c_int >= __pStr_ref.nAlloc {
        let nAlloc: crate::src::headers::sqlite3_h::Sqlite3Int64 = __pStr_ref.nAlloc
            as crate::src::headers::sqlite3_h::Sqlite3Int64
            + nAppend as crate::src::headers::sqlite3_h::Sqlite3Int64
            + 100 as crate::src::headers::sqlite3_h::Sqlite3Int64;
        let zNew: *mut ::core::ffi::c_char = crate::src::src::malloc::sqlite3_realloc64(
            __pStr_ref.z as *mut ::core::ffi::c_void,
            nAlloc as crate::src::headers::sqlite3_h::Sqlite3Uint64,
        ) as *mut ::core::ffi::c_char;
        if zNew.is_null() {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        __pStr_ref.z = zNew;
        __pStr_ref.nAlloc = nAlloc as ::core::ffi::c_int;
    }
    ::core::ptr::copy_nonoverlapping(
        zAppend as *const u8,
        __pStr_ref.z.offset(__pStr_ref.n as isize) as *mut ::core::ffi::c_char as *mut u8,
        nAppend as usize,
    );
    __pStr_ref.n += nAppend;
    *__pStr_ref.z.offset(__pStr_ref.n as isize) = '\0' as i32 as ::core::ffi::c_char;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3SnippetShift(
    pTab: *mut crate::fts3Int_h::Fts3Table,
    iLangid: ::core::ffi::c_int,
    nSnippet: ::core::ffi::c_int,
    zDoc: *const ::core::ffi::c_char,
    nDoc: ::core::ffi::c_int,
    piPos: *mut ::core::ffi::c_int,
    pHlmask: *mut crate::src::ext::rtree::rtree::U64_0,
) -> ::core::ffi::c_int {
    let hlmask: crate::src::ext::rtree::rtree::U64_0 = *pHlmask;
    if hlmask != 0 {
        let mut nLeft: ::core::ffi::c_int;
        let mut nRight: ::core::ffi::c_int;
        
        nLeft = 0 as ::core::ffi::c_int;
        while hlmask & (1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U64_0) << nLeft
            == 0
        {
            nLeft += 1;
        }
        nRight = 0 as ::core::ffi::c_int;
        while hlmask
            & (1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U64_0) << (nSnippet - 1 as ::core::ffi::c_int - nRight)
            == 0
        {
            nRight += 1;
        }
        let nDesired: ::core::ffi::c_int = (nLeft - nRight) / 2 as ::core::ffi::c_int;
        if nDesired > 0 as ::core::ffi::c_int {
            
            let mut iCurrent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut rc: ::core::ffi::c_int;
            
            let mut pC: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor =
                ::core::ptr::null_mut::<
                    crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
                >();
            let pMod: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = (*(*pTab).pTokenizer).pModule
                as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;
            rc = crate::src::ext::fts3::fts3_expr::sqlite3Fts3OpenTokenizer(
                (*pTab).pTokenizer as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                iLangid,
                zDoc,
                nDoc,
                &raw mut pC as *mut _
                    as *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
            );
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
            while rc == crate::src::headers::sqlite3_h::SQLITE_OK && iCurrent < nSnippet + nDesired
            {
                let mut ZDUMMY: *const ::core::ffi::c_char =
                    ::core::ptr::null::<::core::ffi::c_char>();
                let mut DUMMY1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut DUMMY2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut DUMMY3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                rc = (*pMod).xNext.expect("non-null function pointer")(
                    pC,
                    &raw mut ZDUMMY,
                    &raw mut DUMMY1,
                    &raw mut DUMMY2,
                    &raw mut DUMMY3,
                    &raw mut iCurrent,
                );
            }
            (*pMod).xClose.expect("non-null function pointer")(pC);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK
                && rc != crate::src::headers::sqlite3_h::SQLITE_DONE
            {
                return rc;
            }
            let nShift: ::core::ffi::c_int = (rc == crate::src::headers::sqlite3_h::SQLITE_DONE) as ::core::ffi::c_int
                + iCurrent
                - nSnippet;
            if nShift > 0 as ::core::ffi::c_int {
                *piPos += nShift;
                *pHlmask = hlmask >> nShift;
            }
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3SnippetText(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pFragment: *mut SnippetFragment,
    iFragment: ::core::ffi::c_int,
    isLast: ::core::ffi::c_int,
    nSnippet: ::core::ffi::c_int,
    zOpen: *const ::core::ffi::c_char,
    zClose: *const ::core::ffi::c_char,
    zEllipsis: *const ::core::ffi::c_char,
    pOut: *mut StrBuffer,
) -> ::core::ffi::c_int {
    let __pCsr_ref = unsafe { &*pCsr };
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        __pCsr_ref.base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut rc: ::core::ffi::c_int;
    
    
    let mut iCurrent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iEnd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut isShiftDone: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let __pFragment_ref = unsafe { &*pFragment };
    let mut iPos: ::core::ffi::c_int = __pFragment_ref.iPos;
    let mut hlmask: crate::src::ext::rtree::rtree::U64_0 = __pFragment_ref.hlmask;
    let iCol: ::core::ffi::c_int = __pFragment_ref.iCol + 1 as ::core::ffi::c_int;
    
    let mut pC: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor =
        ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor>();
    let zDoc: *const ::core::ffi::c_char = crate::src::src::vdbeapi::sqlite3_column_text(__pCsr_ref.pStmt, iCol)
        as *const ::core::ffi::c_char;
    if zDoc.is_null() {
        if crate::src::src::vdbeapi::sqlite3_column_type(__pCsr_ref.pStmt, iCol)
            != crate::src::headers::sqlite3_h::SQLITE_NULL
        {
            return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    let nDoc: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_column_bytes(__pCsr_ref.pStmt, iCol);
    let pMod: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = (*(*pTab).pTokenizer).pModule
        as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;
    rc = crate::src::ext::fts3::fts3_expr::sqlite3Fts3OpenTokenizer(
        (*pTab).pTokenizer as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
        __pCsr_ref.iLangid,
        zDoc,
        nDoc,
        &raw mut pC as *mut _
            as *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut ZDUMMY: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut DUMMY1: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut iBegin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iFin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let isHighlight: ::core::ffi::c_int;
        rc = (*pMod).xNext.expect("non-null function pointer")(
            pC,
            &raw mut ZDUMMY,
            &raw mut DUMMY1,
            &raw mut iBegin,
            &raw mut iFin,
            &raw mut iCurrent,
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
                rc = fts3StringAppend(
                    pOut,
                    zDoc.offset(iEnd as isize) as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int),
                );
            }
            break;
        } else {
            if iCurrent < iPos {
                continue;
            }
            if isShiftDone == 0 {
                let n: ::core::ffi::c_int = nDoc - iBegin;
                rc = fts3SnippetShift(
                    pTab,
                    __pCsr_ref.iLangid,
                    nSnippet,
                    zDoc.offset(iBegin as isize) as *const ::core::ffi::c_char,
                    n,
                    &raw mut iPos,
                    &raw mut hlmask,
                );
                isShiftDone = 1 as ::core::ffi::c_int;
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    if iPos > 0 as ::core::ffi::c_int || iFragment > 0 as ::core::ffi::c_int {
                        rc = fts3StringAppend(pOut, zEllipsis, -(1 as ::core::ffi::c_int));
                    } else if iBegin != 0 {
                        rc = fts3StringAppend(pOut, zDoc, iBegin);
                    }
                }
                if rc != crate::src::headers::sqlite3_h::SQLITE_OK || iCurrent < iPos {
                    continue;
                }
            }
            if iCurrent >= iPos + nSnippet {
                if isLast != 0 {
                    rc = fts3StringAppend(pOut, zEllipsis, -(1 as ::core::ffi::c_int));
                }
                break;
            } else {
                isHighlight = (hlmask
                    & (1 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U64_0) << (iCurrent - iPos)
                    != 0 as crate::src::ext::rtree::rtree::U64_0)
                    as ::core::ffi::c_int;
                if iCurrent > iPos {
                    rc = fts3StringAppend(
                        pOut,
                        zDoc.offset(iEnd as isize) as *const ::core::ffi::c_char,
                        iBegin - iEnd,
                    );
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK && isHighlight != 0 {
                    rc = fts3StringAppend(pOut, zOpen, -(1 as ::core::ffi::c_int));
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    rc = fts3StringAppend(
                        pOut,
                        zDoc.offset(iBegin as isize) as *const ::core::ffi::c_char,
                        iFin - iBegin,
                    );
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK && isHighlight != 0 {
                    rc = fts3StringAppend(pOut, zClose, -(1 as ::core::ffi::c_int));
                }
                iEnd = iFin;
            }
        }
    }
    (*pMod).xClose.expect("non-null function pointer")(pC);
    rc
}

unsafe extern "C" fn fts3ColumnlistCount(
    ppCollist: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pEnd: *mut ::core::ffi::c_char = *ppCollist;
    let mut c: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    let mut nEntry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while 0xfe as ::core::ffi::c_int & (*pEnd as ::core::ffi::c_int | c as ::core::ffi::c_int) != 0
    {
        let fresh1 = pEnd;
        pEnd = pEnd.offset(1);
        c = (*fresh1 as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        if c == 0 {
            nEntry += 1;
        }
    }
    *ppCollist = pEnd;
    nEntry
}

unsafe extern "C" fn fts3ExprLHits(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    p: *mut MatchInfo,
) -> ::core::ffi::c_int {
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        (*(*p).pCursor).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let iStart: ::core::ffi::c_int;
    let pPhrase: *mut crate::fts3Int_h::Fts3Phrase = (*pExpr).pPhrase;
    let mut pIter: *mut ::core::ffi::c_char = (*pPhrase).doclist.pList;
    let mut iCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*p).flag as ::core::ffi::c_int == FTS3_MATCHINFO_LHITS {
        iStart = (*pExpr).iPhrase * (*p).nCol;
    } else {
        iStart =
            (*pExpr).iPhrase * (((*p).nCol + 31 as ::core::ffi::c_int) / 32 as ::core::ffi::c_int);
    }
    if !pIter.is_null() {
        loop {
            let nHit: ::core::ffi::c_int = fts3ColumnlistCount(&raw mut pIter);
            if (*pPhrase).iColumn >= (*pTab).nColumn || (*pPhrase).iColumn == iCol {
                if (*p).flag as ::core::ffi::c_int == FTS3_MATCHINFO_LHITS {
                    *(*p).aMatchinfo.offset((iStart + iCol) as isize) =
                        nHit as crate::src::ext::rtree::rtree::U32_0;
                } else if nHit != 0 {
                    *(*p).aMatchinfo.offset(
                        (iStart + (iCol + 1 as ::core::ffi::c_int) / 32 as ::core::ffi::c_int)
                            as isize,
                    ) |= ((1 as ::core::ffi::c_int) << (iCol & 0x1f as ::core::ffi::c_int))
                        as crate::src::ext::rtree::rtree::U32_0;
                }
            }
            if *pIter as ::core::ffi::c_int != 0x1 as ::core::ffi::c_int {
                break;
            }
            pIter = pIter.offset(1);
            pIter = pIter.offset(
                (if *(pIter as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    != 0
                {
                    crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint32(pIter, &raw mut iCol)
                } else {
                    iCol =
                        *(pIter as *mut crate::src::ext::rtree::rtree::U8_0) as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int
                }) as isize,
            );
            if iCol >= (*p).nCol {
                return crate::fts3Int_h::FTS_CORRUPT_VTAB;
            }
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3ExprLHitGather(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    p: *mut MatchInfo,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if (*pExpr).bEof as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*pExpr).iDocid == (*(*p).pCursor).iPrevId
    {
        if !(*pExpr).pLeft.is_null() {
            rc = fts3ExprLHitGather((*pExpr).pLeft, p);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = fts3ExprLHitGather((*pExpr).pRight, p);
            }
        } else {
            rc = fts3ExprLHits(pExpr, p);
        }
    }
    rc
}

unsafe extern "C" fn fts3ExprGlobalHitsCb(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    iPhrase: ::core::ffi::c_int,
    pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let p: *mut MatchInfo = pCtx as *mut MatchInfo;
    let __p_ref = unsafe { &mut *p };
    crate::src::ext::fts3::fts3::sqlite3Fts3EvalPhraseStats(
        __p_ref.pCursor as *mut crate::fts3Int_h::Fts3Cursor,
        pExpr as *mut crate::fts3Int_h::Fts3Expr,
        __p_ref
            .aMatchinfo
            .offset((3 as ::core::ffi::c_int * iPhrase * __p_ref.nCol) as isize)
            as *mut crate::src::ext::rtree::rtree::U32_0,
    )
}

unsafe extern "C" fn fts3ExprLocalHitsCb(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    iPhrase: ::core::ffi::c_int,
    pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let p: *mut MatchInfo = pCtx as *mut MatchInfo;
    let iStart: ::core::ffi::c_int = iPhrase * (*p).nCol * 3 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*p).nCol && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut pCsr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        rc = crate::src::ext::fts3::fts3::sqlite3Fts3EvalPhrasePoslist(
            (*p).pCursor as *mut crate::fts3Int_h::Fts3Cursor,
            pExpr as *mut crate::fts3Int_h::Fts3Expr,
            i,
            &raw mut pCsr,
        );
        if !pCsr.is_null() {
            *(*p)
                .aMatchinfo
                .offset((iStart + i * 3 as ::core::ffi::c_int) as isize) =
                fts3ColumnlistCount(&raw mut pCsr) as crate::src::ext::rtree::rtree::U32_0;
        } else {
            *(*p)
                .aMatchinfo
                .offset((iStart + i * 3 as ::core::ffi::c_int) as isize) =
                0 as crate::src::ext::rtree::rtree::U32_0;
        }
        i += 1;
    }
    rc
}

unsafe extern "C" fn fts3MatchinfoCheck(
    pTab: *mut crate::fts3Int_h::Fts3Table,
    cArg: ::core::ffi::c_char,
    pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let __pTab_ref = unsafe { &*pTab };
    if cArg as ::core::ffi::c_int == FTS3_MATCHINFO_NPHRASE
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_NCOL
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_NDOC
            && __pTab_ref.bFts4 as ::core::ffi::c_int != 0
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_AVGLENGTH
            && __pTab_ref.bFts4 as ::core::ffi::c_int != 0
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_LENGTH
            && __pTab_ref.bHasDocsize as ::core::ffi::c_int != 0
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_LCS
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_HITS
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_LHITS
        || cArg as ::core::ffi::c_int == FTS3_MATCHINFO_LHITS_BM
    {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    crate::src::ext::fts3::fts3::sqlite3Fts3ErrMsg(
        pzErr,
        b"unrecognized matchinfo request: %c\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::src::src::printf::PrintfArg::Char(cArg as u32)],
    );
    crate::src::headers::sqlite3_h::SQLITE_ERROR
}

unsafe extern "C" fn fts3MatchinfoSize(
    pInfo: *mut MatchInfo,
    cArg: ::core::ffi::c_char,
) -> crate::__stddef_size_t_h::SizeT {
    let nVal: crate::__stddef_size_t_h::SizeT;
    match cArg as ::core::ffi::c_int {
        FTS3_MATCHINFO_NDOC | FTS3_MATCHINFO_NPHRASE | FTS3_MATCHINFO_NCOL => {
            nVal = 1 as crate::__stddef_size_t_h::SizeT;
        }
        FTS3_MATCHINFO_AVGLENGTH | FTS3_MATCHINFO_LENGTH | FTS3_MATCHINFO_LCS => {
            nVal = (*pInfo).nCol as crate::__stddef_size_t_h::SizeT;
        }
        FTS3_MATCHINFO_LHITS => {
            nVal = ((*pInfo).nCol as crate::__stddef_size_t_h::SizeT)
                .wrapping_mul((*pInfo).nPhrase as crate::__stddef_size_t_h::SizeT);
        }
        FTS3_MATCHINFO_LHITS_BM => {
            nVal = ((*pInfo).nPhrase as crate::__stddef_size_t_h::SizeT).wrapping_mul(
                (((*pInfo).nCol + 31 as ::core::ffi::c_int) / 32 as ::core::ffi::c_int)
                    as crate::__stddef_size_t_h::SizeT,
            );
        }
        _ => {
            nVal = ((*pInfo).nCol as crate::__stddef_size_t_h::SizeT)
                .wrapping_mul((*pInfo).nPhrase as crate::__stddef_size_t_h::SizeT)
                .wrapping_mul(3 as crate::__stddef_size_t_h::SizeT);
        }
    }
    nVal
}

unsafe extern "C" fn fts3MatchinfoSelectDoctotal(
    pTab: *mut crate::fts3Int_h::Fts3Table,
    ppStmt: *mut *mut crate::src::headers::sqlite3_h::Sqlite3Stmt,
    pnDoc: *mut crate::src::headers::sqlite3_h::Sqlite3Int64,
    paLen: *mut *const ::core::ffi::c_char,
    ppEnd: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    
    let mut a: *const ::core::ffi::c_char;
    
    let mut nDoc: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
    
    if (*ppStmt).is_null() {
        let rc: ::core::ffi::c_int =
            crate::src::ext::fts3::fts3_write::sqlite3Fts3SelectDoctotal(
                pTab as *mut crate::fts3Int_h::Fts3Table,
                ppStmt,
            );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
    }
    let pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt = *ppStmt;
    let n: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, 0 as ::core::ffi::c_int);
    a = crate::src::src::vdbeapi::sqlite3_column_blob(pStmt, 0 as ::core::ffi::c_int)
        as *const ::core::ffi::c_char;
    if a.is_null() {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    let pEnd: *const ::core::ffi::c_char = a.offset(n as isize);
    a = a.offset(
        crate::src::ext::fts3::fts3::sqlite3Fts3GetVarintBounded(a, pEnd, &raw mut nDoc) as isize,
    );
    if nDoc <= 0 as crate::src::headers::sqlite3_h::Sqlite3Int64 || a > pEnd {
        return crate::fts3Int_h::FTS_CORRUPT_VTAB;
    }
    *pnDoc = nDoc;
    if !paLen.is_null() {
        *paLen = a;
    }
    if !ppEnd.is_null() {
        *ppEnd = pEnd;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3MatchinfoLcsCb(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    iPhrase: ::core::ffi::c_int,
    pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let aIter: *mut LcsIterator = pCtx as *mut LcsIterator;
    let fresh2 = &mut (*aIter.offset(iPhrase as isize)).pExpr;
    *fresh2 = pExpr;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fts3LcsIteratorAdvance(pIter: *mut LcsIterator) -> ::core::ffi::c_int {
    let mut pRead: *mut ::core::ffi::c_char;
    let mut iRead: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
    let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if pIter.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    pRead = (*pIter).pRead;
    pRead = pRead
        .offset(crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(pRead, &raw mut iRead) as isize);
    if iRead == 0 as crate::src::headers::sqlite3_h::Sqlite3Int64
        || iRead == 1 as crate::src::headers::sqlite3_h::Sqlite3Int64
    {
        pRead = ::core::ptr::null_mut::<::core::ffi::c_char>();
        rc = 1 as ::core::ffi::c_int;
    } else {
        (*pIter).iPos +=
            (iRead - 2 as crate::src::headers::sqlite3_h::Sqlite3Int64) as ::core::ffi::c_int;
    }
    (*pIter).pRead = pRead;
    rc
}

unsafe extern "C" fn fts3MatchinfoLcs(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    pInfo: *mut MatchInfo,
) -> ::core::ffi::c_int {
    
    let mut i: ::core::ffi::c_int;
    let mut iCol: ::core::ffi::c_int;
    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let aIter: *mut LcsIterator = crate::src::ext::fts3::fts3_expr::sqlite3Fts3MallocZero(
        (::core::mem::size_of::<LcsIterator>() as usize).wrapping_mul((*pCsr).nPhrase as usize)
            as crate::src::ext::rtree::rtree::I64_0,
    ) as *mut LcsIterator;
    if aIter.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    sqlite3Fts3ExprIterate(
        (*pCsr).pExpr,
        Some(
            fts3MatchinfoLcsCb
                as unsafe extern "C" fn(
                    *mut crate::fts3Int_h::Fts3Expr,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        aIter as *mut ::core::ffi::c_void,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*pInfo).nPhrase {
        let pIter = &mut *(aIter.offset(i as isize) as *mut LcsIterator);
        nToken -= (*(*pIter.pExpr).pPhrase).nToken;
        pIter.iPosOffset = nToken;
        i += 1;
    }
    iCol = 0 as ::core::ffi::c_int;
    's_48: while iCol < (*pInfo).nCol {
        let mut nLcs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nLive: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < (*pInfo).nPhrase {
            let pIt: *mut LcsIterator = aIter.offset(i as isize) as *mut LcsIterator;
            let __pIt_ref = unsafe { &mut *pIt };
            rc = crate::src::ext::fts3::fts3::sqlite3Fts3EvalPhrasePoslist(
                pCsr as *mut crate::fts3Int_h::Fts3Cursor,
                __pIt_ref.pExpr as *mut crate::fts3Int_h::Fts3Expr,
                iCol,
                &raw mut __pIt_ref.pRead,
            );
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                break 's_48;
            }
            if !__pIt_ref.pRead.is_null() {
                __pIt_ref.iPos = __pIt_ref.iPosOffset;
                fts3LcsIteratorAdvance(pIt);
                if __pIt_ref.pRead.is_null() {
                    rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
                    break 's_48;
                } else {
                    nLive += 1;
                }
            }
            i += 1;
        }
        while nLive > 0 as ::core::ffi::c_int {
            let mut pAdv: *mut LcsIterator = ::core::ptr::null_mut::<LcsIterator>();
            let mut nThisLcs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < (*pInfo).nPhrase {
                let pIter_0: *mut LcsIterator = aIter.offset(i as isize) as *mut LcsIterator;
                if (*pIter_0).pRead.is_null() {
                    nThisLcs = 0 as ::core::ffi::c_int;
                } else {
                    if pAdv.is_null() || (*pIter_0).iPos < (*pAdv).iPos {
                        pAdv = pIter_0;
                    }
                    if nThisLcs == 0 as ::core::ffi::c_int
                        || (*pIter_0).iPos
                            == (*pIter_0.offset(-(1 as ::core::ffi::c_int) as isize)).iPos
                    {
                        nThisLcs += 1;
                    } else {
                        nThisLcs = 1 as ::core::ffi::c_int;
                    }
                    if nThisLcs > nLcs {
                        nLcs = nThisLcs;
                    }
                }
                i += 1;
            }
            if fts3LcsIteratorAdvance(pAdv) != 0 {
                nLive -= 1;
            }
        }
        *(*pInfo).aMatchinfo.offset(iCol as isize) = nLcs as crate::src::ext::rtree::rtree::U32_0;
        iCol += 1;
    }
    crate::src::src::malloc::sqlite3_free(aIter as *mut ::core::ffi::c_void);
    rc
}

unsafe extern "C" fn fts3MatchinfoValues(
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    bGlobal: ::core::ffi::c_int,
    pInfo: *mut MatchInfo,
    zArg: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut i: ::core::ffi::c_int;
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut pSelect: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    i = 0 as ::core::ffi::c_int;
    while rc == crate::src::headers::sqlite3_h::SQLITE_OK
        && *zArg.offset(i as isize) as ::core::ffi::c_int != 0
    {
        let __pInfo_ref = unsafe { &mut *pInfo };
        __pInfo_ref.flag = *zArg.offset(i as isize);
        let mut current_block_42: u64;
        match *zArg.offset(i as isize) as ::core::ffi::c_int {
            FTS3_MATCHINFO_NPHRASE => {
                if bGlobal != 0 {
                    *__pInfo_ref.aMatchinfo.offset(0_isize) =
                        __pInfo_ref.nPhrase as crate::src::ext::rtree::rtree::U32_0;
                }
            }
            FTS3_MATCHINFO_NCOL => {
                if bGlobal != 0 {
                    *__pInfo_ref.aMatchinfo.offset(0_isize) =
                        __pInfo_ref.nCol as crate::src::ext::rtree::rtree::U32_0;
                }
            }
            FTS3_MATCHINFO_NDOC => {
                if bGlobal != 0 {
                    let mut nDoc: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                        0 as crate::src::headers::sqlite3_h::Sqlite3Int64;
                    rc = fts3MatchinfoSelectDoctotal(
                        pTab,
                        &raw mut pSelect,
                        &raw mut nDoc,
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                    );
                    *__pInfo_ref.aMatchinfo.offset(0_isize) =
                        nDoc as crate::src::ext::rtree::rtree::U32_0;
                }
            }
            FTS3_MATCHINFO_AVGLENGTH => {
                if bGlobal != 0 {
                    let mut nDoc_0: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
                    let mut a: *const ::core::ffi::c_char =
                        ::core::ptr::null::<::core::ffi::c_char>();
                    let mut pEnd: *const ::core::ffi::c_char =
                        ::core::ptr::null::<::core::ffi::c_char>();
                    rc = fts3MatchinfoSelectDoctotal(
                        pTab,
                        &raw mut pSelect,
                        &raw mut nDoc_0,
                        &raw mut a,
                        &raw mut pEnd,
                    );
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        let mut iCol: ::core::ffi::c_int;
                        iCol = 0 as ::core::ffi::c_int;
                        while iCol < __pInfo_ref.nCol {
                            let iVal: crate::src::ext::rtree::rtree::U32_0;
                            let mut nToken: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
                            a = a.offset(crate::src::ext::fts3::fts3::sqlite3Fts3GetVarint(
                                a,
                                &raw mut nToken,
                            ) as isize);
                            if a > pEnd {
                                rc = crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
                                break;
                            } else {
                                iVal = (((nToken
                                    & 0xffffffff as crate::src::headers::sqlite3_h::Sqlite3Int64)
                                    as crate::src::ext::rtree::rtree::U32_0
                                    as crate::src::headers::sqlite3_h::Sqlite3Int64
                                    + nDoc_0 / 2 as crate::src::headers::sqlite3_h::Sqlite3Int64)
                                    / nDoc_0)
                                    as crate::src::ext::rtree::rtree::U32_0;
                                *__pInfo_ref.aMatchinfo.offset(iCol as isize) = iVal;
                                iCol += 1;
                            }
                        }
                    }
                }
            }
            FTS3_MATCHINFO_LENGTH => {
                let mut pSelectDocsize: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
                    ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
                rc = crate::src::ext::fts3::fts3_write::sqlite3Fts3SelectDocsize(
                    pTab as *mut crate::fts3Int_h::Fts3Table,
                    (*pCsr).iPrevId,
                    &raw mut pSelectDocsize,
                );
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    let mut iCol_0: ::core::ffi::c_int;
                    let mut a_0: *const ::core::ffi::c_char =
                        crate::src::src::vdbeapi::sqlite3_column_blob(
                            pSelectDocsize,
                            0 as ::core::ffi::c_int,
                        ) as *const ::core::ffi::c_char;
                    let pEnd_0: *const ::core::ffi::c_char =
                        a_0.offset(crate::src::src::vdbeapi::sqlite3_column_bytes(
                            pSelectDocsize,
                            0 as ::core::ffi::c_int,
                        ) as isize);
                    iCol_0 = 0 as ::core::ffi::c_int;
                    while iCol_0 < __pInfo_ref.nCol {
                        let mut nToken_0: crate::src::headers::sqlite3_h::Sqlite3Int64 = 0;
                        a_0 = a_0.offset(crate::src::ext::fts3::fts3::sqlite3Fts3GetVarintBounded(
                            a_0,
                            pEnd_0,
                            &raw mut nToken_0,
                        ) as isize);
                        if a_0 > pEnd_0 {
                            rc = crate::src::headers::sqlite3_h::SQLITE_CORRUPT_VTAB;
                            break;
                        } else {
                            *__pInfo_ref.aMatchinfo.offset(iCol_0 as isize) =
                                nToken_0 as crate::src::ext::rtree::rtree::U32_0;
                            iCol_0 += 1;
                        }
                    }
                }
                crate::src::src::vdbeapi::sqlite3_reset(pSelectDocsize);
            }
            FTS3_MATCHINFO_LCS => {
                rc = fts3ExprLoadDoclists(
                    pCsr,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    rc = fts3MatchinfoLcs(pCsr, pInfo);
                }
            }
            FTS3_MATCHINFO_LHITS_BM | FTS3_MATCHINFO_LHITS => {
                let nZero: crate::__stddef_size_t_h::SizeT =
                    fts3MatchinfoSize(pInfo, *zArg.offset(i as isize)).wrapping_mul(
                        ::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>()
                            as crate::__stddef_size_t_h::SizeT,
                    );
                ::libc::memset(
                    __pInfo_ref.aMatchinfo as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    nZero,
                );
                rc = fts3ExprLHitGather((*pCsr).pExpr, pInfo);
            }
            _ => {
                
                let pExpr: *mut crate::fts3Int_h::Fts3Expr = (*pCsr).pExpr;
                rc = fts3ExprLoadDoclists(
                    pCsr,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
                    if bGlobal != 0 {
                        if !(*pCsr).pDeferred.is_null() {
                            rc = fts3MatchinfoSelectDoctotal(
                                pTab,
                                &raw mut pSelect,
                                &raw mut __pInfo_ref.nDoc,
                                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                            );
                            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                current_block_42 = 2520131295878969859;
                            } else {
                                current_block_42 = 13826291924415791078;
                            }
                        } else {
                            current_block_42 = 13826291924415791078;
                        }
                        match current_block_42 {
                            2520131295878969859 => {}
                            _ => {
                                rc = sqlite3Fts3ExprIterate(
                                    pExpr,
                                    Some(
                                        fts3ExprGlobalHitsCb
                                            as unsafe extern "C" fn(
                                                *mut crate::fts3Int_h::Fts3Expr,
                                                ::core::ffi::c_int,
                                                *mut ::core::ffi::c_void,
                                            )
                                                -> ::core::ffi::c_int,
                                    ),
                                    pInfo as *mut ::core::ffi::c_void,
                                );
                                crate::src::ext::fts3::fts3::sqlite3Fts3EvalTestDeferred(
                                    pCsr as *mut crate::fts3Int_h::Fts3Cursor,
                                    &raw mut rc,
                                );
                                if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                                    current_block_42 = 2520131295878969859;
                                } else {
                                    current_block_42 = 7343950298149844727;
                                }
                            }
                        }
                    } else {
                        current_block_42 = 7343950298149844727;
                    }
                    match current_block_42 {
                        2520131295878969859 => {}
                        _ => {
                            sqlite3Fts3ExprIterate(
                                pExpr,
                                Some(
                                    fts3ExprLocalHitsCb
                                        as unsafe extern "C" fn(
                                            *mut crate::fts3Int_h::Fts3Expr,
                                            ::core::ffi::c_int,
                                            *mut ::core::ffi::c_void,
                                        )
                                            -> ::core::ffi::c_int,
                                ),
                                pInfo as *mut ::core::ffi::c_void,
                            );
                        }
                    }
                }
            }
        }
        __pInfo_ref.aMatchinfo = (*pInfo)
            .aMatchinfo
            .add(fts3MatchinfoSize(pInfo, *zArg.offset(i as isize)));
        i += 1;
    }
    crate::src::src::vdbeapi::sqlite3_reset(pSelect);
    rc
}

unsafe extern "C" fn fts3GetMatchinfo(
    pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    zArg: *const ::core::ffi::c_char,
) {
    let mut sInfo: MatchInfo = unsafe { ::core::mem::zeroed() };
    let __pCsr_ref = unsafe { &mut *pCsr };
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        __pCsr_ref.base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut bGlobal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut aOut: *mut crate::src::ext::rtree::rtree::U32_0 =
        ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::U32_0>();
    let mut xDestroyOut: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> = None;
    sInfo.pCursor = pCsr;
    sInfo.nCol = (*pTab).nColumn;
    if !__pCsr_ref.pMIBuffer.is_null()
        && ::libc::strcmp((*__pCsr_ref.pMIBuffer).zMatchinfo, zArg) != 0
    {
        sqlite3Fts3MIBufferFree(__pCsr_ref.pMIBuffer);
        __pCsr_ref.pMIBuffer = ::core::ptr::null_mut::<MatchinfoBuffer>();
    }
    if __pCsr_ref.pMIBuffer.is_null() {
        let mut nMatchinfo: crate::__stddef_size_t_h::SizeT =
            0 as crate::__stddef_size_t_h::SizeT;
        let mut i: ::core::ffi::c_int;
        __pCsr_ref.nPhrase = fts3ExprPhraseCount(__pCsr_ref.pExpr);
        sInfo.nPhrase = __pCsr_ref.nPhrase;
        i = 0 as ::core::ffi::c_int;
        while *zArg.offset(i as isize) != 0 {
            let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if fts3MatchinfoCheck(pTab, *zArg.offset(i as isize), &raw mut zErr) != 0 {
                crate::src::src::vdbeapi::sqlite3_result_error(
                    pCtx,
                    zErr,
                    -(1 as ::core::ffi::c_int),
                );
                crate::src::src::malloc::sqlite3_free(zErr as *mut ::core::ffi::c_void);
                return;
            }
            nMatchinfo = nMatchinfo
                .wrapping_add(fts3MatchinfoSize(&raw mut sInfo, *zArg.offset(i as isize)));
            i += 1;
        }
        __pCsr_ref.pMIBuffer = fts3MIBufferNew(nMatchinfo, zArg);
        if __pCsr_ref.pMIBuffer.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
        __pCsr_ref.isMatchinfoNeeded = 1 as ::core::ffi::c_int;
        bGlobal = 1 as ::core::ffi::c_int;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        xDestroyOut = fts3MIBufferAlloc(__pCsr_ref.pMIBuffer, &raw mut aOut);
        if xDestroyOut.is_none() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        }
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        sInfo.aMatchinfo = aOut;
        sInfo.nPhrase = __pCsr_ref.nPhrase;
        rc = fts3MatchinfoValues(pCsr, bGlobal, &raw mut sInfo, zArg);
        if bGlobal != 0 {
            fts3MIBufferSetGlobal(__pCsr_ref.pMIBuffer);
        }
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_result_error_code(pCtx, rc);
        if xDestroyOut.is_some() {
            xDestroyOut.expect("non-null function pointer")(aOut as *mut ::core::ffi::c_void);
        }
    } else {
        let n: ::core::ffi::c_int = ((*__pCsr_ref.pMIBuffer).nElem as usize)
            .wrapping_mul(::core::mem::size_of::<crate::src::ext::rtree::rtree::U32_0>() as usize)
            as ::core::ffi::c_int;
        crate::src::src::vdbeapi::sqlite3_result_blob(
            pCtx,
            aOut as *const ::core::ffi::c_void,
            n,
            xDestroyOut,
        );
    };
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3Fts3Snippet(
    pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    zStart: *const ::core::ffi::c_char,
    zEnd: *const ::core::ffi::c_char,
    zEllipsis: *const ::core::ffi::c_char,
    iCol: ::core::ffi::c_int,
    mut nToken: ::core::ffi::c_int,
) {
    let current_block: u64;
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut i: ::core::ffi::c_int;
    let mut res: StrBuffer = unsafe { ::core::mem::zeroed() };
    let mut nSnippet: ::core::ffi::c_int;
    let mut aSnippet: [SnippetFragment; 4] = [SnippetFragment {
        iCol: 0,
        iPos: 0,
        covered: 0,
        hlmask: 0,
    }; 4];
    let mut nFToken: ::core::ffi::c_int;
    if (*pCsr).pExpr.is_null() {
        crate::src::src::vdbeapi::sqlite3_result_text(
            pCtx,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            crate::src::headers::sqlite3_h::SQLITE_STATIC,
        );
        return;
    }
    if nToken < -(64 as ::core::ffi::c_int) {
        nToken = -(64 as ::core::ffi::c_int);
    }
    if nToken > 64 as ::core::ffi::c_int {
        nToken = 64 as ::core::ffi::c_int;
    }
    nSnippet = 1 as ::core::ffi::c_int;
    's_52: loop {
        let mut iSnip: ::core::ffi::c_int;
        let mut mCovered: crate::src::ext::rtree::rtree::U64_0 =
            0 as crate::src::ext::rtree::rtree::U64_0;
        let mut mSeen: crate::src::ext::rtree::rtree::U64_0 =
            0 as crate::src::ext::rtree::rtree::U64_0;
        if nToken >= 0 as ::core::ffi::c_int {
            nFToken = (nToken + nSnippet - 1 as ::core::ffi::c_int) / nSnippet;
        } else {
            nFToken = -(1 as ::core::ffi::c_int) * nToken;
        }
        iSnip = 0 as ::core::ffi::c_int;
        while iSnip < nSnippet {
            let mut iBestScore: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            let mut iRead: ::core::ffi::c_int;
            let pFragment: *mut SnippetFragment = (&raw mut aSnippet as *mut SnippetFragment)
                .offset(iSnip as isize)
                as *mut SnippetFragment;
            ::libc::memset(
                pFragment as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<SnippetFragment>() as crate::__stddef_size_t_h::SizeT,
            );
            iRead = 0 as ::core::ffi::c_int;
            while iRead < (*pTab).nColumn {
                let mut sF: SnippetFragment = unsafe { ::core::mem::zeroed() };
                let mut iS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if !(iCol >= 0 as ::core::ffi::c_int && iRead != iCol) {
                    rc = fts3BestSnippet(
                        nFToken,
                        pCsr,
                        iRead,
                        mCovered,
                        &raw mut mSeen,
                        &raw mut sF,
                        &raw mut iS,
                    );
                    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                        current_block = 4442782934716781880;
                        break 's_52;
                    }
                    if iS > iBestScore {
                        *pFragment = sF;
                        iBestScore = iS;
                    }
                }
                iRead += 1;
            }
            mCovered |= (*pFragment).covered;
            iSnip += 1;
        }
        if mSeen == mCovered
            || nSnippet
                == (::core::mem::size_of::<[SnippetFragment; 4]>() as usize)
                    .wrapping_div(::core::mem::size_of::<SnippetFragment>() as usize)
                    as ::core::ffi::c_int
        {
            current_block = 1538046216550696469;
            break;
        }
        nSnippet += 1;
    }
    if current_block == 1538046216550696469 {
        i = 0 as ::core::ffi::c_int;
        while i < nSnippet && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            rc = fts3SnippetText(
                pCsr,
                (&raw mut aSnippet as *mut SnippetFragment).offset(i as isize)
                    as *mut SnippetFragment,
                i,
                (i == nSnippet - 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                nFToken,
                zStart,
                zEnd,
                zEllipsis,
                &raw mut res,
            );
            i += 1;
        }
    }
    crate::src::ext::fts3::fts3_write::sqlite3Fts3SegmentsClose(
        pTab as *mut crate::fts3Int_h::Fts3Table,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_result_error_code(pCtx, rc);
        crate::src::src::malloc::sqlite3_free(res.z as *mut ::core::ffi::c_void);
    } else {
        crate::src::src::vdbeapi::sqlite3_result_text(
            pCtx,
            res.z,
            -(1 as ::core::ffi::c_int),
            Some(
                crate::src::src::malloc::sqlite3_free
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
        );
    };
}

unsafe extern "C" fn fts3ExprTermOffsetInit(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    mut _iPhrase: ::core::ffi::c_int,
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let p: *mut TermOffsetCtx = ctx as *mut TermOffsetCtx;
    
    let mut iTerm: ::core::ffi::c_int;
    let mut pList: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iPos: crate::src::ext::rtree::rtree::I64_0 = 0 as crate::src::ext::rtree::rtree::I64_0;
    
    let rc: ::core::ffi::c_int = crate::src::ext::fts3::fts3::sqlite3Fts3EvalPhrasePoslist(
        (*p).pCsr as *mut crate::fts3Int_h::Fts3Cursor,
        pExpr as *mut crate::fts3Int_h::Fts3Expr,
        (*p).iCol,
        &raw mut pList,
    );
    let nTerm: ::core::ffi::c_int = (*(*pExpr).pPhrase).nToken;
    if !pList.is_null() {
        fts3GetDeltaPosition(&raw mut pList, &raw mut iPos);
    }
    iTerm = 0 as ::core::ffi::c_int;
    while iTerm < nTerm {
        let __p_ref = unsafe { &mut *p };
        let fresh0 = __p_ref.iTerm;
        __p_ref.iTerm += 1;
        let pT: *mut TermOffset = __p_ref.aTerm.offset(fresh0 as isize) as *mut TermOffset;
        let __pT_ref = unsafe { &mut *pT };
        __pT_ref.iOff =
            (nTerm - iTerm - 1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::I64_0;
        __pT_ref.pList = pList;
        __pT_ref.iPos = iPos;
        iTerm += 1;
    }
    rc
}

unsafe extern "C" fn fts3ExprRestartIfCb(
    pExpr: *mut crate::fts3Int_h::Fts3Expr,
    mut _iPhrase: ::core::ffi::c_int,
    ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let p: *mut TermOffsetCtx = ctx as *mut TermOffsetCtx;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if !(*pExpr).pPhrase.is_null() && (*(*pExpr).pPhrase).bIncr != 0 {
        rc = crate::src::ext::fts3::fts3::sqlite3Fts3MsrCancel(
            (*p).pCsr as *mut crate::fts3Int_h::Fts3Cursor,
            pExpr as *mut crate::fts3Int_h::Fts3Expr,
        );
        (*(*pExpr).pPhrase).bIncr = 0 as ::core::ffi::c_int;
    }
    rc
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3Fts3Offsets(
    pCtx: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
) {
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let pMod: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module =
        (*(*pTab).pTokenizer).pModule;
    let mut rc: ::core::ffi::c_int;
    let mut nToken: ::core::ffi::c_int = 0;
    let mut iCol: ::core::ffi::c_int;
    let mut res: StrBuffer = unsafe { ::core::mem::zeroed() };
    let mut sCtx: TermOffsetCtx = unsafe { ::core::mem::zeroed() };
    if (*pCsr).pExpr.is_null() {
        crate::src::src::vdbeapi::sqlite3_result_text(
            pCtx,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            crate::src::headers::sqlite3_h::SQLITE_STATIC,
        );
        return;
    }
    rc = fts3ExprLoadDoclists(
        pCsr,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut nToken,
    );
    if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
        sCtx.aTerm = crate::src::ext::fts3::fts3_expr::sqlite3Fts3MallocZero(
            (::core::mem::size_of::<TermOffset>() as usize).wrapping_mul(nToken as usize)
                as crate::src::ext::rtree::rtree::I64_0,
        ) as *mut TermOffset;
        if sCtx.aTerm.is_null() {
            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
        } else {
            sCtx.iDocid = (*pCsr).iPrevId;
            sCtx.pCsr = pCsr;
            rc = sqlite3Fts3ExprIterate(
                (*pCsr).pExpr,
                Some(
                    fts3ExprRestartIfCb
                        as unsafe extern "C" fn(
                            *mut crate::fts3Int_h::Fts3Expr,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                &raw mut sCtx as *mut ::core::ffi::c_void,
            );
            if (rc == crate::src::headers::sqlite3_h::SQLITE_OK) {
                iCol = 0 as ::core::ffi::c_int;
                while iCol < (*pTab).nColumn {
                    let mut pC: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor =
                        ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor>();
                    let mut ZDUMMY: *const ::core::ffi::c_char =
                        ::core::ptr::null::<::core::ffi::c_char>();
                    let mut NDUMMY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iStart: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iEnd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut iCurrent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    
                    
                    sCtx.iCol = iCol;
                    sCtx.iTerm = 0 as ::core::ffi::c_int;
                    let __pCsr_ref = unsafe { &*pCsr };
                    rc = sqlite3Fts3ExprIterate(
                        __pCsr_ref.pExpr,
                        Some(
                            fts3ExprTermOffsetInit
                                as unsafe extern "C" fn(
                                    *mut crate::fts3Int_h::Fts3Expr,
                                    ::core::ffi::c_int,
                                    *mut ::core::ffi::c_void,
                                )
                                    -> ::core::ffi::c_int,
                        ),
                        &raw mut sCtx as *mut ::core::ffi::c_void,
                    );
                    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                        break;
                    }
                    let zDoc: *const ::core::ffi::c_char = crate::src::src::vdbeapi::sqlite3_column_text(
                        __pCsr_ref.pStmt,
                        iCol + 1 as ::core::ffi::c_int,
                    ) as *const ::core::ffi::c_char;
                    let nDoc: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_column_bytes(
                        __pCsr_ref.pStmt,
                        iCol + 1 as ::core::ffi::c_int,
                    );
                    if zDoc.is_null() {
                        if (crate::src::src::vdbeapi::sqlite3_column_type(
                            __pCsr_ref.pStmt,
                            iCol + 1 as ::core::ffi::c_int,
                        ) != crate::src::headers::sqlite3_h::SQLITE_NULL)
                        {
                            rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
                            break;
                        }
                    } else {
                        rc = crate::src::ext::fts3::fts3_expr::sqlite3Fts3OpenTokenizer(
                            
                            (*pTab).pTokenizer as
    *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
                            __pCsr_ref.iLangid,
                            zDoc,
                            nDoc,
                            
                            &raw mut pC as *mut _ as
    *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
                        );
                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                            break;
                        }
                        rc = (*pMod).xNext.expect("non-null function pointer")(
                            pC,
                            &raw mut ZDUMMY,
                            &raw mut NDUMMY,
                            &raw mut iStart,
                            &raw mut iEnd,
                            &raw mut iCurrent,
                        );
                        while rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            let mut i: ::core::ffi::c_int;
                            let mut iMinPos: ::core::ffi::c_int = 0x7fffffff as ::core::ffi::c_int;
                            let mut pTerm: *mut TermOffset = ::core::ptr::null_mut::<TermOffset>();
                            i = 0 as ::core::ffi::c_int;
                            while i < nToken {
                                let pT: *mut TermOffset =
                                    sCtx.aTerm.offset(i as isize) as *mut TermOffset;
                                let __pT_ref = unsafe { &mut *pT };
                                if !__pT_ref.pList.is_null()
                                    && __pT_ref.iPos - __pT_ref.iOff
                                        < iMinPos as crate::src::ext::rtree::rtree::I64_0
                                {
                                    iMinPos = (__pT_ref.iPos - __pT_ref.iOff) as ::core::ffi::c_int;
                                    pTerm = pT;
                                }
                                i += 1;
                            }
                            if pTerm.is_null() {
                                rc = crate::src::headers::sqlite3_h::SQLITE_DONE;
                            } else {
                                if 0 as ::core::ffi::c_int
                                    == 0xfe as ::core::ffi::c_int
                                        & *(*pTerm).pList as ::core::ffi::c_int
                                {
                                    (*pTerm).pList = ::core::ptr::null_mut::<::core::ffi::c_char>();
                                } else {
                                    fts3GetDeltaPosition(
                                        &raw mut (*pTerm).pList,
                                        &raw mut (*pTerm).iPos,
                                    );
                                }
                                while rc == crate::src::headers::sqlite3_h::SQLITE_OK
                                    && iCurrent < iMinPos
                                {
                                    rc = (*pMod).xNext.expect("non-null function pointer")(
                                        pC,
                                        &raw mut ZDUMMY,
                                        &raw mut NDUMMY,
                                        &raw mut iStart,
                                        &raw mut iEnd,
                                        &raw mut iCurrent,
                                    );
                                }
                                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                                    let mut aBuffer: [::core::ffi::c_char; 64] = [0; 64];
                                    crate::sqlite_snprintf!(
                                        &raw mut aBuffer as *mut ::core::ffi::c_char,
                                        ::core::mem::size_of::<[::core::ffi::c_char; 64]>()
                                            as ::core::ffi::c_int,
                                        "%d %d %d %d ",
                                        iCol,
                                        pTerm.offset_from(sCtx.aTerm) as ::core::ffi::c_long,
                                        iStart,
                                        iEnd - iStart,
                                    );
                                    rc = fts3StringAppend(
                                        &raw mut res,
                                        &raw mut aBuffer as *mut ::core::ffi::c_char,
                                        -(1 as ::core::ffi::c_int),
                                    );
                                } else if rc == crate::src::headers::sqlite3_h::SQLITE_DONE
                                    && (*pTab).zContentTbl.is_null()
                                {
                                    rc = crate::fts3Int_h::FTS_CORRUPT_VTAB;
                                }
                            }
                        }
                        if rc == crate::src::headers::sqlite3_h::SQLITE_DONE {
                            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                        }
                        (*pMod).xClose.expect("non-null function pointer")(pC);
                        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                            break;
                        }
                    }
                    iCol += 1;
                }
            }
        }
    }
    crate::src::src::malloc::sqlite3_free(sCtx.aTerm as *mut ::core::ffi::c_void);
    crate::src::ext::fts3::fts3_write::sqlite3Fts3SegmentsClose(
        pTab as *mut crate::fts3Int_h::Fts3Table,
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::vdbeapi::sqlite3_result_error_code(pCtx, rc);
        crate::src::src::malloc::sqlite3_free(res.z as *mut ::core::ffi::c_void);
    } else {
        crate::src::src::vdbeapi::sqlite3_result_text(
            pCtx,
            res.z,
            res.n - 1 as ::core::ffi::c_int,
            Some(
                crate::src::src::malloc::sqlite3_free
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
        );
    };
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3Fts3Matchinfo(
    pContext: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    pCsr: *mut crate::fts3Int_h::Fts3Cursor,
    zArg: *const ::core::ffi::c_char,
) {
    let pTab: *mut crate::fts3Int_h::Fts3Table =
        (*pCsr).base.pVtab as *mut crate::fts3Int_h::Fts3Table;
    let zFormat: *const ::core::ffi::c_char;
    if !zArg.is_null() {
        zFormat = zArg;
    } else {
        zFormat = FTS3_MATCHINFO_DEFAULT.as_ptr();
    }
    if (*pCsr).pExpr.is_null() {
        crate::src::src::vdbeapi::sqlite3_result_blob(
            pContext,
            b"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            crate::src::headers::sqlite3_h::SQLITE_STATIC,
        );
    } else {
        fts3GetMatchinfo(pContext, pCsr, zFormat);
        crate::src::ext::fts3::fts3_write::sqlite3Fts3SegmentsClose(
            pTab as *mut crate::fts3Int_h::Fts3Table,
        );
    }
}
