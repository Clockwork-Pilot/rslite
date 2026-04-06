use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Fts5Context;
    pub type Fts5Tokenizer;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts5ExtensionApi {
    pub iVersion: ::core::ffi::c_int,
    pub xUserData: Option<
        unsafe extern "C" fn(*mut Fts5Context) -> *mut ::core::ffi::c_void,
    >,
    pub xColumnCount: Option<
        unsafe extern "C" fn(*mut Fts5Context) -> ::core::ffi::c_int,
    >,
    pub xRowCount: Option<
        unsafe extern "C" fn(*mut Fts5Context, *mut sqlite3_int64) -> ::core::ffi::c_int,
    >,
    pub xColumnTotalSize: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut sqlite3_int64,
        ) -> ::core::ffi::c_int,
    >,
    pub xTokenize: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        ) -> ::core::ffi::c_int,
    >,
    pub xPhraseCount: Option<
        unsafe extern "C" fn(*mut Fts5Context) -> ::core::ffi::c_int,
    >,
    pub xPhraseSize: Option<
        unsafe extern "C" fn(*mut Fts5Context, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub xInstCount: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xInst: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xRowid: Option<unsafe extern "C" fn(*mut Fts5Context) -> sqlite3_int64>,
    pub xColumnText: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xColumnSize: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xQueryPhrase: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *const Fts5ExtensionApi,
                    *mut Fts5Context,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >,
        ) -> ::core::ffi::c_int,
    >,
    pub xSetAuxdata: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            *mut ::core::ffi::c_void,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub xGetAuxdata: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub xPhraseFirst: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut Fts5PhraseIter,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xPhraseNext: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            *mut Fts5PhraseIter,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub xPhraseFirstColumn: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut Fts5PhraseIter,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xPhraseNextColumn: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            *mut Fts5PhraseIter,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub xQueryToken: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xInstToken: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xColumnLocale: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            ::core::ffi::c_int,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xTokenize_v2: Option<
        unsafe extern "C" fn(
            *mut Fts5Context,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts5PhraseIter {
    pub a: *const ::core::ffi::c_uchar,
    pub b: *const ::core::ffi::c_uchar,
}
pub type fts5_extension_function = Option<
    unsafe extern "C" fn(
        *const Fts5ExtensionApi,
        *mut Fts5Context,
        *mut sqlite3_context,
        ::core::ffi::c_int,
        *mut *mut sqlite3_value,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fts5_tokenizer_v2 {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut Fts5Tokenizer,
        ) -> ::core::ffi::c_int,
    >,
    pub xDelete: Option<unsafe extern "C" fn(*mut Fts5Tokenizer) -> ()>,
    pub xTokenize: Option<
        unsafe extern "C" fn(
            *mut Fts5Tokenizer,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fts5_tokenizer {
    pub xCreate: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut Fts5Tokenizer,
        ) -> ::core::ffi::c_int,
    >,
    pub xDelete: Option<unsafe extern "C" fn(*mut Fts5Tokenizer) -> ()>,
    pub xTokenize: Option<
        unsafe extern "C" fn(
            *mut Fts5Tokenizer,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fts5_api {
    pub iVersion: ::core::ffi::c_int,
    pub xCreateTokenizer: Option<
        unsafe extern "C" fn(
            *mut fts5_api,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
            *mut fts5_tokenizer,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub xFindTokenizer: Option<
        unsafe extern "C" fn(
            *mut fts5_api,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_void,
            *mut fts5_tokenizer,
        ) -> ::core::ffi::c_int,
    >,
    pub xCreateFunction: Option<
        unsafe extern "C" fn(
            *mut fts5_api,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
            fts5_extension_function,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub xCreateTokenizer_v2: Option<
        unsafe extern "C" fn(
            *mut fts5_api,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
            *mut fts5_tokenizer_v2,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        ) -> ::core::ffi::c_int,
    >,
    pub xFindTokenizer_v2: Option<
        unsafe extern "C" fn(
            *mut fts5_api,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_void,
            *mut *mut fts5_tokenizer_v2,
        ) -> ::core::ffi::c_int,
    >,
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts5tokTable {
    pub base: sqlite3_vtab,
    pub tok: fts5_tokenizer,
    pub pTok: *mut Fts5Tokenizer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts5tokCursor {
    pub base: sqlite3_vtab_cursor,
    pub iRowid: ::core::ffi::c_int,
    pub zInput: *mut ::core::ffi::c_char,
    pub nRow: ::core::ffi::c_int,
    pub aRow: *mut Fts5tokRow,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts5tokRow {
    pub zToken: *mut ::core::ffi::c_char,
    pub iStart: ::core::ffi::c_int,
    pub iEnd: ::core::ffi::c_int,
    pub iPos: ::core::ffi::c_int,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_INDEX_CONSTRAINT_EQ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FTS5_TOKEN_COLOCATED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
unsafe extern "C" fn fts5tokDequote(mut z: *mut ::core::ffi::c_char) {
    unsafe {
        let mut q: ::core::ffi::c_char = *z.offset(0 as ::core::ffi::c_int as isize);
        if q as ::core::ffi::c_int == '[' as i32
            || q as ::core::ffi::c_int == '\'' as i32
            || q as ::core::ffi::c_int == '"' as i32
            || q as ::core::ffi::c_int == '`' as i32
        {
            let mut iIn: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut iOut: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if q as ::core::ffi::c_int == '[' as i32 {
                q = ']' as i32 as ::core::ffi::c_char;
            }
            while *z.offset(iIn as isize) != 0 {
                if *z.offset(iIn as isize) as ::core::ffi::c_int
                    == q as ::core::ffi::c_int
                {
                    if *z.offset((iIn + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int != q as ::core::ffi::c_int
                    {
                        iIn += 1;
                        break;
                    } else {
                        iIn += 2 as ::core::ffi::c_int;
                        let c2rust_fresh0 = iOut;
                        iOut = iOut + 1;
                        *z.offset(c2rust_fresh0 as isize) = q;
                    }
                } else {
                    let c2rust_fresh1 = iIn;
                    iIn = iIn + 1;
                    let c2rust_fresh2 = iOut;
                    iOut = iOut + 1;
                    *z.offset(c2rust_fresh2 as isize) = *z
                        .offset(c2rust_fresh1 as isize);
                }
            }
            *z.offset(iOut as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
    }
}
unsafe extern "C" fn fts5tokDequoteArray(
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut pazDequote: *mut *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if argc == 0 as ::core::ffi::c_int {
            *pazDequote = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        } else {
            let mut i: ::core::ffi::c_int = 0;
            let mut nByte: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut azDequote: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                *mut ::core::ffi::c_char,
            >();
            i = 0 as ::core::ffi::c_int;
            while i < argc {
                nByte
                    += strlen(*argv.offset(i as isize)).wrapping_add(1 as size_t)
                        as ::core::ffi::c_int;
                i += 1;
            }
            azDequote = sqlite3_malloc64(
                (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                    .wrapping_mul(argc as usize)
                    .wrapping_add(nByte as usize) as sqlite3_uint64,
            ) as *mut *mut ::core::ffi::c_char;
            *pazDequote = azDequote;
            if azDequote.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                let mut pSpace: *mut ::core::ffi::c_char = azDequote
                    .offset(argc as isize) as *mut *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                i = 0 as ::core::ffi::c_int;
                while i < argc {
                    let mut n: ::core::ffi::c_int = strlen(*argv.offset(i as isize))
                        as ::core::ffi::c_int;
                    let ref mut c2rust_fresh3 = *azDequote.offset(i as isize);
                    *c2rust_fresh3 = pSpace;
                    memcpy(
                        pSpace as *mut ::core::ffi::c_void,
                        *argv.offset(i as isize) as *const ::core::ffi::c_void,
                        (n + 1 as ::core::ffi::c_int) as size_t,
                    );
                    fts5tokDequote(pSpace);
                    pSpace = pSpace.offset((n + 1 as ::core::ffi::c_int) as isize);
                    i += 1;
                }
            }
        }
        return rc;
    }
}
unsafe extern "C" fn fts5tokConnectMethod(
    mut db: *mut sqlite3,
    mut pCtx: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pApi: *mut fts5_api = pCtx as *mut fts5_api;
        let mut pTab: *mut Fts5tokTable = ::core::ptr::null_mut::<Fts5tokTable>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut azDequote: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            *mut ::core::ffi::c_char,
        >();
        let mut nDequote: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = sqlite3_declare_vtab(
            db,
            b"CREATE TABLE x(input HIDDEN, token, start, end, position)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        if rc == SQLITE_OK {
            nDequote = argc - 3 as ::core::ffi::c_int;
            rc = fts5tokDequoteArray(
                nDequote,
                argv.offset(3 as ::core::ffi::c_int as isize)
                    as *const *const ::core::ffi::c_char,
                &raw mut azDequote,
            );
        }
        if rc == SQLITE_OK {
            pTab = sqlite3_malloc(
                ::core::mem::size_of::<Fts5tokTable>() as ::core::ffi::c_int,
            ) as *mut Fts5tokTable;
            if pTab.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                memset(
                    pTab as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<Fts5tokTable>() as size_t,
                );
            }
        }
        if rc == SQLITE_OK {
            let mut pTokCtx: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            let mut zModule: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            if nDequote > 0 as ::core::ffi::c_int {
                zModule = *azDequote.offset(0 as ::core::ffi::c_int as isize);
            }
            rc = (*pApi)
                .xFindTokenizer
                .expect(
                    "non-null function pointer",
                )(pApi, zModule, &raw mut pTokCtx, &raw mut (*pTab).tok);
            if rc == SQLITE_OK {
                let mut azArg: *mut *const ::core::ffi::c_char = if nDequote
                    > 1 as ::core::ffi::c_int
                {
                    azDequote.offset(1 as ::core::ffi::c_int as isize)
                        as *mut *mut ::core::ffi::c_char
                        as *mut *const ::core::ffi::c_char
                } else {
                    ::core::ptr::null_mut::<*const ::core::ffi::c_char>()
                };
                let mut nArg: ::core::ffi::c_int = if nDequote > 0 as ::core::ffi::c_int
                {
                    nDequote - 1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                };
                rc = (*pTab)
                    .tok
                    .xCreate
                    .expect(
                        "non-null function pointer",
                    )(pTokCtx, azArg, nArg, &raw mut (*pTab).pTok);
            }
        }
        if rc != SQLITE_OK {
            sqlite3_free(pTab as *mut ::core::ffi::c_void);
            pTab = ::core::ptr::null_mut::<Fts5tokTable>();
        }
        *ppVtab = pTab as *mut sqlite3_vtab;
        sqlite3_free(azDequote as *mut ::core::ffi::c_void);
        return rc;
    }
}
unsafe extern "C" fn fts5tokDisconnectMethod(
    mut pVtab: *mut sqlite3_vtab,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pTab: *mut Fts5tokTable = pVtab as *mut Fts5tokTable;
        if !(*pTab).pTok.is_null() {
            (*pTab).tok.xDelete.expect("non-null function pointer")((*pTab).pTok);
        }
        sqlite3_free(pTab as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts5tokBestIndexMethod(
    mut pVTab: *mut sqlite3_vtab,
    mut pInfo: *mut sqlite3_index_info,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pInfo).nConstraint {
            if (*(*pInfo).aConstraint.offset(i as isize)).usable as ::core::ffi::c_int
                != 0
                && (*(*pInfo).aConstraint.offset(i as isize)).iColumn
                    == 0 as ::core::ffi::c_int
                && (*(*pInfo).aConstraint.offset(i as isize)).op as ::core::ffi::c_int
                    == SQLITE_INDEX_CONSTRAINT_EQ
            {
                (*pInfo).idxNum = 1 as ::core::ffi::c_int;
                (*(*pInfo).aConstraintUsage.offset(i as isize)).argvIndex = 1
                    as ::core::ffi::c_int;
                (*(*pInfo).aConstraintUsage.offset(i as isize)).omit = 1
                    as ::core::ffi::c_uchar;
                (*pInfo).estimatedCost = 1 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                return SQLITE_OK;
            }
            i += 1;
        }
        (*pInfo).idxNum = 0 as ::core::ffi::c_int;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts5tokOpenMethod(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCsr: *mut *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut Fts5tokCursor = ::core::ptr::null_mut::<Fts5tokCursor>();
        pCsr = sqlite3_malloc(
            ::core::mem::size_of::<Fts5tokCursor>() as ::core::ffi::c_int,
        ) as *mut Fts5tokCursor;
        if pCsr.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pCsr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<Fts5tokCursor>() as size_t,
        );
        *ppCsr = pCsr as *mut sqlite3_vtab_cursor;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts5tokResetCursor(mut pCsr: *mut Fts5tokCursor) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*pCsr).nRow {
            sqlite3_free(
                (*(*pCsr).aRow.offset(i as isize)).zToken as *mut ::core::ffi::c_void,
            );
            i += 1;
        }
        sqlite3_free((*pCsr).zInput as *mut ::core::ffi::c_void);
        sqlite3_free((*pCsr).aRow as *mut ::core::ffi::c_void);
        (*pCsr).zInput = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*pCsr).aRow = ::core::ptr::null_mut::<Fts5tokRow>();
        (*pCsr).nRow = 0 as ::core::ffi::c_int;
        (*pCsr).iRowid = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fts5tokCloseMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut Fts5tokCursor = pCursor as *mut Fts5tokCursor;
        fts5tokResetCursor(pCsr);
        sqlite3_free(pCsr as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts5tokNextMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut Fts5tokCursor = pCursor as *mut Fts5tokCursor;
        (*pCsr).iRowid += 1;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts5tokCb(
    mut pCtx: *mut ::core::ffi::c_void,
    mut tflags: ::core::ffi::c_int,
    mut pToken: *const ::core::ffi::c_char,
    mut nToken: ::core::ffi::c_int,
    mut iStart: ::core::ffi::c_int,
    mut iEnd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut Fts5tokCursor = pCtx as *mut Fts5tokCursor;
        let mut pRow: *mut Fts5tokRow = ::core::ptr::null_mut::<Fts5tokRow>();
        if (*pCsr).nRow & (*pCsr).nRow - 1 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            let mut nNew: ::core::ffi::c_int = if (*pCsr).nRow != 0 {
                (*pCsr).nRow * 2 as ::core::ffi::c_int
            } else {
                32 as ::core::ffi::c_int
            };
            let mut aNew: *mut Fts5tokRow = ::core::ptr::null_mut::<Fts5tokRow>();
            aNew = sqlite3_realloc64(
                (*pCsr).aRow as *mut ::core::ffi::c_void,
                (nNew as usize)
                    .wrapping_mul(::core::mem::size_of::<Fts5tokRow>() as usize)
                    as sqlite3_uint64,
            ) as *mut Fts5tokRow;
            if aNew.is_null() {
                return SQLITE_NOMEM;
            }
            memset(
                aNew.offset((*pCsr).nRow as isize) as *mut Fts5tokRow
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (::core::mem::size_of::<Fts5tokRow>() as size_t)
                    .wrapping_mul((nNew - (*pCsr).nRow) as size_t),
            );
            (*pCsr).aRow = aNew;
        }
        pRow = (*pCsr).aRow.offset((*pCsr).nRow as isize) as *mut Fts5tokRow;
        (*pRow).iStart = iStart;
        (*pRow).iEnd = iEnd;
        if (*pCsr).nRow != 0 {
            (*pRow).iPos = (*pRow.offset(-1 as ::core::ffi::c_int as isize)).iPos
                + (if tflags & FTS5_TOKEN_COLOCATED != 0 {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                });
        }
        (*pRow).zToken = sqlite3_malloc(nToken + 1 as ::core::ffi::c_int)
            as *mut ::core::ffi::c_char;
        if (*pRow).zToken.is_null() {
            return SQLITE_NOMEM;
        }
        memcpy(
            (*pRow).zToken as *mut ::core::ffi::c_void,
            pToken as *const ::core::ffi::c_void,
            nToken as size_t,
        );
        *(*pRow).zToken.offset(nToken as isize) = 0 as ::core::ffi::c_char;
        (*pCsr).nRow += 1;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts5tokFilterMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: ::core::ffi::c_int,
    mut idxStr: *const ::core::ffi::c_char,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_ERROR;
        let mut pCsr: *mut Fts5tokCursor = pCursor as *mut Fts5tokCursor;
        let mut pTab: *mut Fts5tokTable = (*pCursor).pVtab as *mut Fts5tokTable;
        fts5tokResetCursor(pCsr);
        if idxNum == 1 as ::core::ffi::c_int {
            let mut zByte: *const ::core::ffi::c_char = sqlite3_value_text(
                *apVal.offset(0 as ::core::ffi::c_int as isize),
            ) as *const ::core::ffi::c_char;
            let mut nByte: ::core::ffi::c_int = sqlite3_value_bytes(
                *apVal.offset(0 as ::core::ffi::c_int as isize),
            );
            (*pCsr).zInput = sqlite3_malloc(nByte + 1 as ::core::ffi::c_int)
                as *mut ::core::ffi::c_char;
            if (*pCsr).zInput.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                if nByte > 0 as ::core::ffi::c_int {
                    memcpy(
                        (*pCsr).zInput as *mut ::core::ffi::c_void,
                        zByte as *const ::core::ffi::c_void,
                        nByte as size_t,
                    );
                }
                *(*pCsr).zInput.offset(nByte as isize) = 0 as ::core::ffi::c_char;
                rc = (*pTab)
                    .tok
                    .xTokenize
                    .expect(
                        "non-null function pointer",
                    )(
                    (*pTab).pTok,
                    pCsr as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    zByte,
                    nByte,
                    Some(
                        fts5tokCb
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                ::core::ffi::c_int,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                                ::core::ffi::c_int,
                                ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                );
            }
        }
        if rc != SQLITE_OK {
            return rc;
        }
        return fts5tokNextMethod(pCursor);
    }
}
unsafe extern "C" fn fts5tokEofMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut Fts5tokCursor = pCursor as *mut Fts5tokCursor;
        return ((*pCsr).iRowid > (*pCsr).nRow) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fts5tokColumnMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pCtx: *mut sqlite3_context,
    mut iCol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut Fts5tokCursor = pCursor as *mut Fts5tokCursor;
        let mut pRow: *mut Fts5tokRow = (*pCsr)
            .aRow
            .offset(((*pCsr).iRowid - 1 as ::core::ffi::c_int) as isize)
            as *mut Fts5tokRow;
        match iCol {
            0 => {
                sqlite3_result_text(
                    pCtx,
                    (*pCsr).zInput,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            }
            1 => {
                sqlite3_result_text(
                    pCtx,
                    (*pRow).zToken,
                    -1 as ::core::ffi::c_int,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            }
            2 => {
                sqlite3_result_int(pCtx, (*pRow).iStart);
            }
            3 => {
                sqlite3_result_int(pCtx, (*pRow).iEnd);
            }
            _ => {
                sqlite3_result_int(pCtx, (*pRow).iPos);
            }
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn fts5tokRowidMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut sqlite_int64,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut Fts5tokCursor = pCursor as *mut Fts5tokCursor;
        *pRowid = (*pCsr).iRowid as sqlite3_int64 as sqlite_int64;
        return SQLITE_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3Fts5TestRegisterTok(
    mut db: *mut sqlite3,
    mut pApi: *mut fts5_api,
) -> ::core::ffi::c_int {
    unsafe {
        static mut fts5tok_module: sqlite3_module = unsafe {
            sqlite3_module {
                iVersion: 0 as ::core::ffi::c_int,
                xCreate: Some(
                    fts5tokConnectMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3,
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_int,
                            *const *const ::core::ffi::c_char,
                            *mut *mut sqlite3_vtab,
                            *mut *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                xConnect: Some(
                    fts5tokConnectMethod
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
                    fts5tokBestIndexMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut sqlite3_index_info,
                        ) -> ::core::ffi::c_int,
                ),
                xDisconnect: Some(
                    fts5tokDisconnectMethod
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xDestroy: Some(
                    fts5tokDisconnectMethod
                        as unsafe extern "C" fn(*mut sqlite3_vtab) -> ::core::ffi::c_int,
                ),
                xOpen: Some(
                    fts5tokOpenMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab,
                            *mut *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xClose: Some(
                    fts5tokCloseMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xFilter: Some(
                    fts5tokFilterMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> ::core::ffi::c_int,
                ),
                xNext: Some(
                    fts5tokNextMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xEof: Some(
                    fts5tokEofMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xColumn: Some(
                    fts5tokColumnMethod
                        as unsafe extern "C" fn(
                            *mut sqlite3_vtab_cursor,
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xRowid: Some(
                    fts5tokRowidMethod
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
        let mut rc: ::core::ffi::c_int = 0;
        rc = sqlite3_create_module(
            db,
            b"fts5tokenize\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const fts5tok_module,
            pApi as *mut ::core::ffi::c_void,
        );
        return rc;
    }
}
