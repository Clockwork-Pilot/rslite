
extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3_db_config(_: *mut sqlite3, op: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_realloc64(
        _: *mut ::core::ffi::c_void,
        _: sqlite3_uint64,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_blob(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_void,
        n: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
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
    fn sqlite3_column_bytes(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3_column_type(_: *mut sqlite3_stmt, iCol: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_create_function(
        db: *mut sqlite3,
        zFunctionName: *const ::core::ffi::c_char,
        nArg: ::core::ffi::c_int,
        eTextRep: ::core::ffi::c_int,
        pApp: *mut ::core::ffi::c_void,
        xFunc: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xStep: Option<
            unsafe extern "C" fn(
                *mut sqlite3_context,
                ::core::ffi::c_int,
                *mut *mut sqlite3_value,
            ) -> (),
        >,
        xFinal: Option<unsafe extern "C" fn(*mut sqlite3_context) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_frombind(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_user_data(_: *mut sqlite3_context) -> *mut ::core::ffi::c_void;
    fn sqlite3_context_db_handle(_: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_blob(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3Fts3HashInsert(
        _: *mut Fts3Hash,
        pKey: *const ::core::ffi::c_void,
        nKey: ::core::ffi::c_int,
        pData: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3Fts3HashFind(
        _: *const Fts3Hash,
        pKey: *const ::core::ffi::c_void,
        nKey: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3Fts3ErrMsg(_: *mut *mut ::core::ffi::c_char, _: *const ::core::ffi::c_char, ...);
    fn sqlite3Fts3Dequote(_: *mut ::core::ffi::c_char);
    fn sqlite3Fts3OpenTokenizer(
        _: *mut sqlite3_tokenizer,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut *mut sqlite3_tokenizer_cursor,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3SimpleTokenizerModule(ppModule: *mut *const sqlite3_tokenizer_module);
}

#[cfg(feature = "test")]
extern "C" {
    pub type Tcl_Interp;
    fn TclFreeObj(objPtr: *mut Tcl_Obj);
    fn Tcl_ListObjAppendElement(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objPtr: *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewObj() -> *mut Tcl_Obj;
    fn Tcl_NewStringObj(bytes: *const ::core::ffi::c_char, length: Tcl_Size) -> *mut Tcl_Obj;
    fn Tcl_NewWideIntObj(wideValue: Tcl_WideInt) -> *mut Tcl_Obj;
    fn Tcl_GetStringFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut Tcl_Size,
    ) -> *mut ::core::ffi::c_char;
}

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
pub type sqlite3_destructor_type = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer_module {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut sqlite3_tokenizer,
        ) -> ::core::ffi::c_int,
    >,
    pub xDestroy: Option<unsafe extern "C" fn(*mut sqlite3_tokenizer) -> ::core::ffi::c_int>,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_tokenizer_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<unsafe extern "C" fn(*mut sqlite3_tokenizer_cursor) -> ::core::ffi::c_int>,
    pub xNext: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer_cursor,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xLanguageid: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer_cursor,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer_cursor {
    pub pTokenizer: *mut sqlite3_tokenizer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer {
    pub pModule: *const sqlite3_tokenizer_module,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3Hash {
    pub keyClass: ::core::ffi::c_char,
    pub copyKey: ::core::ffi::c_char,
    pub count: ::core::ffi::c_int,
    pub first: *mut Fts3HashElem,
    pub htsize: ::core::ffi::c_int,
    pub ht: *mut _fts3ht,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fts3ht {
    pub count: ::core::ffi::c_int,
    pub chain: *mut Fts3HashElem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fts3HashElem {
    pub next: *mut Fts3HashElem,
    pub prev: *mut Fts3HashElem,
    pub data: *mut ::core::ffi::c_void,
    pub pKey: *mut ::core::ffi::c_void,
    pub nKey: ::core::ffi::c_int,
}

#[cfg(feature = "test")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Obj {
    pub refCount: Tcl_Size,
    pub bytes: *mut ::core::ffi::c_char,
    pub length: Tcl_Size,
    pub typePtr: *const Tcl_ObjType,
    pub internalRep: Tcl_ObjInternalRep,
}
#[cfg(feature = "test")]
#[derive(Copy, Clone)]
#[repr(C)]
pub union Tcl_ObjInternalRep {
    pub longValue: ::core::ffi::c_long,
    pub doubleValue: ::core::ffi::c_double,
    pub otherValuePtr: *mut ::core::ffi::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: C2RustUnnamed_0,
    pub ptrAndLongRep: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ptr: *mut ::core::ffi::c_void,
    pub value: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ptr1: *mut ::core::ffi::c_void,
    pub ptr2: *mut ::core::ffi::c_void,
}

#[cfg(feature = "test")]
pub type Tcl_WideInt = ::core::ffi::c_longlong;

#[cfg(feature = "test")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_ObjType {
    pub name: *const ::core::ffi::c_char,
    pub freeIntRepProc: Option<Tcl_FreeInternalRepProc>,
    pub dupIntRepProc: Option<Tcl_DupInternalRepProc>,
    pub updateStringProc: Option<Tcl_UpdateStringProc>,
    pub setFromAnyProc: Option<Tcl_SetFromAnyProc>,
    pub version: size_t,
    pub lengthProc: Option<Tcl_ObjTypeLengthProc>,
    pub indexProc: Option<Tcl_ObjTypeIndexProc>,
    pub sliceProc: Option<Tcl_ObjTypeSliceProc>,
    pub reverseProc: Option<Tcl_ObjTypeReverseProc>,
    pub getElementsProc: Option<Tcl_ObjTypeGetElements>,
    pub setElementProc: Option<Tcl_ObjTypeSetElement>,
    pub replaceProc: Option<Tcl_ObjTypeReplaceProc>,
    pub inOperProc: Option<Tcl_ObjTypeInOperatorProc>,
}
#[cfg(feature = "test")]
pub type Tcl_ObjTypeInOperatorProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
    *mut Tcl_Obj,
    *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int;
#[cfg(feature = "test")]
pub type Tcl_ObjTypeReplaceProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
    Tcl_Size,
    Tcl_Size,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[cfg(feature = "test")]
pub type Tcl_Size = ptrdiff_t;
#[cfg(feature = "test")]
pub type Tcl_ObjTypeSetElement = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
    Tcl_Size,
    *const *mut Tcl_Obj,
    *mut Tcl_Obj,
) -> *mut Tcl_Obj;
#[cfg(feature = "test")]
pub type Tcl_ObjTypeGetElements = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
    *mut Tcl_Size,
    *mut *mut *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[cfg(feature = "test")]
pub type Tcl_ObjTypeReverseProc =
    unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj, *mut *mut Tcl_Obj) -> ::core::ffi::c_int;
#[cfg(feature = "test")]
pub type Tcl_ObjTypeSliceProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
    Tcl_Size,
    Tcl_Size,
    *mut *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[cfg(feature = "test")]
pub type Tcl_ObjTypeIndexProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
    Tcl_Size,
    *mut *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[cfg(feature = "test")]
pub type Tcl_ObjTypeLengthProc = unsafe extern "C" fn(*mut Tcl_Obj) -> Tcl_Size;
#[cfg(feature = "test")]
pub type Tcl_SetFromAnyProc =
    unsafe extern "C" fn(*mut Tcl_Interp, *mut Tcl_Obj) -> ::core::ffi::c_int;
#[cfg(feature = "test")]
pub type Tcl_UpdateStringProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
#[cfg(feature = "test")]
pub type Tcl_DupInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_Obj) -> ();
#[cfg(feature = "test")]
pub type Tcl_FreeInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER: ::core::ffi::c_int = 1004 as ::core::ffi::c_int;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_DIRECTONLY: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const SQLITE_STATIC: sqlite3_destructor_type = None;
unsafe extern "C" fn fts3TokenizerEnabled(mut context: *mut sqlite3_context) -> ::core::ffi::c_int {
    let mut db: *mut sqlite3 = sqlite3_context_db_handle(context);
    let mut isEnabled: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    sqlite3_db_config(
        db,
        SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER,
        -(1 as ::core::ffi::c_int),
        &raw mut isEnabled,
    );
    return isEnabled;
}
unsafe extern "C" fn fts3TokenizerFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut pHash: *mut Fts3Hash = ::core::ptr::null_mut::<Fts3Hash>();
    let mut pPtr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut zName: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut nName: ::core::ffi::c_int = 0;
    pHash = sqlite3_user_data(context) as *mut Fts3Hash;
    zName = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize));
    nName = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize))
        + 1 as ::core::ffi::c_int;
    if argc == 2 as ::core::ffi::c_int {
        if fts3TokenizerEnabled(context) != 0
            || sqlite3_value_frombind(*argv.offset(1 as ::core::ffi::c_int as isize)) != 0
        {
            let mut pOld: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut n: ::core::ffi::c_int =
                sqlite3_value_bytes(*argv.offset(1 as ::core::ffi::c_int as isize));
            if zName.is_null()
                || n as usize != ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
            {
                sqlite3_result_error(
                    context,
                    b"argument type mismatch\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int),
                );
                return;
            }
            pPtr = *(sqlite3_value_blob(*argv.offset(1 as ::core::ffi::c_int as isize))
                as *mut *mut ::core::ffi::c_void);
            pOld = sqlite3Fts3HashInsert(pHash, zName as *mut ::core::ffi::c_void, nName, pPtr);
            if pOld == pPtr {
                sqlite3_result_error(
                    context,
                    b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int),
                );
            }
        } else {
            sqlite3_result_error(
                context,
                b"fts3tokenize disabled\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int),
            );
            return;
        }
    } else {
        if !zName.is_null() {
            pPtr = sqlite3Fts3HashFind(pHash, zName as *const ::core::ffi::c_void, nName);
        }
        if pPtr.is_null() {
            let mut zErr: *mut ::core::ffi::c_char = sqlite3_mprintf(
                b"unknown tokenizer: %s\0" as *const u8 as *const ::core::ffi::c_char,
                zName,
            );
            sqlite3_result_error(context, zErr, -(1 as ::core::ffi::c_int));
            sqlite3_free(zErr as *mut ::core::ffi::c_void);
            return;
        }
    }
    if fts3TokenizerEnabled(context) != 0
        || sqlite3_value_frombind(*argv.offset(0 as ::core::ffi::c_int as isize)) != 0
    {
        sqlite3_result_blob(
            context,
            &raw mut pPtr as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<*mut ::core::ffi::c_void>() as ::core::ffi::c_int,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3IsIdChar(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    static mut isFtsIdChar: [::core::ffi::c_char; 128] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
    ];
    return (c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
        || isFtsIdChar[c as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3NextToken(
    mut zStr: *const ::core::ffi::c_char,
    mut pn: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut z1: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut z2: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    z1 = zStr;
    while z2.is_null() {
        let mut c: ::core::ffi::c_char = *z1;
        match c as ::core::ffi::c_int {
            0 => return ::core::ptr::null::<::core::ffi::c_char>(),
            39 | 34 | 96 => {
                z2 = z1;
                loop {
                    z2 = z2.offset(1);
                    if !(*z2 as ::core::ffi::c_int != 0
                        && (*z2 as ::core::ffi::c_int != c as ::core::ffi::c_int || {
                            z2 = z2.offset(1);
                            *z2 as ::core::ffi::c_int == c as ::core::ffi::c_int
                        }))
                    {
                        break;
                    }
                }
            }
            91 => {
                z2 = z1.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char;
                while *z2 as ::core::ffi::c_int != 0
                    && *z2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        != ']' as i32
                {
                    z2 = z2.offset(1);
                }
                if *z2 != 0 {
                    z2 = z2.offset(1);
                }
            }
            _ => {
                if sqlite3Fts3IsIdChar(*z1) != 0 {
                    z2 = z1.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char;
                    while sqlite3Fts3IsIdChar(*z2) != 0 {
                        z2 = z2.offset(1);
                    }
                } else {
                    z1 = z1.offset(1);
                }
            }
        }
    }
    *pn = z2.offset_from(z1) as ::core::ffi::c_long as ::core::ffi::c_int;
    return z1;
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3InitTokenizer(
    mut pHash: *mut Fts3Hash,
    mut zArg: *const ::core::ffi::c_char,
    mut ppTok: *mut *mut sqlite3_tokenizer,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut z: *mut ::core::ffi::c_char = zArg as *mut ::core::ffi::c_char;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut zCopy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zEnd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut m: *mut sqlite3_tokenizer_module = ::core::ptr::null_mut::<sqlite3_tokenizer_module>();
    zCopy = sqlite3_mprintf(b"%s\0" as *const u8 as *const ::core::ffi::c_char, zArg);
    if zCopy.is_null() {
        return SQLITE_NOMEM;
    }
    zEnd = zCopy.offset(
        (strlen as unsafe extern "C" fn(*const ::core::ffi::c_char) -> size_t)(zCopy) as isize,
    ) as *mut ::core::ffi::c_char;
    z = sqlite3Fts3NextToken(zCopy, &raw mut n) as *mut ::core::ffi::c_char;
    if z.is_null() {
        z = zCopy;
    }
    *z.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    sqlite3Fts3Dequote(z);
    m = sqlite3Fts3HashFind(
        pHash,
        z as *const ::core::ffi::c_void,
        strlen(z) as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
    ) as *mut sqlite3_tokenizer_module;
    if m.is_null() {
        sqlite3Fts3ErrMsg(
            pzErr,
            b"unknown tokenizer: %s\0" as *const u8 as *const ::core::ffi::c_char,
            z,
        );
        rc = SQLITE_ERROR;
    } else {
        let mut aArg: *mut *const ::core::ffi::c_char =
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        let mut iArg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        z = z.offset((n + 1 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_char;
        while z < zEnd && {
            z = sqlite3Fts3NextToken(z, &raw mut n) as *mut ::core::ffi::c_char;
            !z.is_null()
        } {
            let mut nNew: sqlite3_int64 = (::core::mem::size_of::<*mut ::core::ffi::c_char>()
                as usize)
                .wrapping_mul((iArg + 1 as ::core::ffi::c_int) as usize)
                as sqlite3_int64;
            let mut aNew: *mut *const ::core::ffi::c_char =
                sqlite3_realloc64(aArg as *mut ::core::ffi::c_void, nNew as sqlite3_uint64)
                    as *mut *const ::core::ffi::c_char;
            if aNew.is_null() {
                sqlite3_free(zCopy as *mut ::core::ffi::c_void);
                sqlite3_free(aArg as *mut ::core::ffi::c_void);
                return SQLITE_NOMEM;
            }
            aArg = aNew;
            let fresh1 = iArg;
            iArg = iArg + 1;
            let ref mut fresh2 = *aArg.offset(fresh1 as isize);
            *fresh2 = z;
            *z.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
            sqlite3Fts3Dequote(z);
            z = z.offset((n + 1 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_char;
        }
        rc = (*m).xCreate.expect("non-null function pointer")(iArg, aArg, ppTok);
        if rc != SQLITE_OK {
            sqlite3Fts3ErrMsg(
                pzErr,
                b"unknown tokenizer\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            (**ppTok).pModule = m;
        }
        sqlite3_free(aArg as *mut ::core::ffi::c_void);
    }
    sqlite3_free(zCopy as *mut ::core::ffi::c_void);
    return rc;
}
#[cfg(feature = "test")]
unsafe extern "C" fn testFunc(
    mut context: *mut sqlite3_context,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut pHash: *mut Fts3Hash = ::core::ptr::null_mut::<Fts3Hash>();
    let mut p: *mut sqlite3_tokenizer_module = ::core::ptr::null_mut::<sqlite3_tokenizer_module>();
    let mut pTokenizer: *mut sqlite3_tokenizer = ::core::ptr::null_mut::<sqlite3_tokenizer>();
    let mut pCsr: *mut sqlite3_tokenizer_cursor =
        ::core::ptr::null_mut::<sqlite3_tokenizer_cursor>();
    let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut zName: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nName: ::core::ffi::c_int = 0;
    let mut zInput: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nInput: ::core::ffi::c_int = 0;
    let mut azArg: [*const ::core::ffi::c_char; 64] =
        [::core::ptr::null::<::core::ffi::c_char>(); 64];
    let mut zToken: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iStart: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iEnd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iPos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
    if argc < 2 as ::core::ffi::c_int {
        sqlite3_result_error(
            context,
            b"insufficient arguments\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
        );
        return;
    }
    nName = sqlite3_value_bytes(*argv.offset(0 as ::core::ffi::c_int as isize));
    zName = sqlite3_value_text(*argv.offset(0 as ::core::ffi::c_int as isize))
        as *const ::core::ffi::c_char;
    nInput = sqlite3_value_bytes(*argv.offset((argc - 1 as ::core::ffi::c_int) as isize));
    zInput = sqlite3_value_text(*argv.offset((argc - 1 as ::core::ffi::c_int) as isize))
        as *const ::core::ffi::c_char;
    pHash = sqlite3_user_data(context) as *mut Fts3Hash;
    p = sqlite3Fts3HashFind(
        pHash,
        zName as *const ::core::ffi::c_void,
        nName + 1 as ::core::ffi::c_int,
    ) as *mut sqlite3_tokenizer_module;
    if p.is_null() {
        let mut zErr2: *mut ::core::ffi::c_char = sqlite3_mprintf(
            b"unknown tokenizer: %s\0" as *const u8 as *const ::core::ffi::c_char,
            zName,
        );
        sqlite3_result_error(context, zErr2, -(1 as ::core::ffi::c_int));
        sqlite3_free(zErr2 as *mut ::core::ffi::c_void);
        return;
    }
    pRet = Tcl_NewObj();
    (*pRet).refCount += 1;
    i = 1 as ::core::ffi::c_int;
    while i < argc - 1 as ::core::ffi::c_int {
        azArg[(i - 1 as ::core::ffi::c_int) as usize] =
            sqlite3_value_text(*argv.offset(i as isize)) as *const ::core::ffi::c_char;
        i += 1;
    }
    if SQLITE_OK
        != (*p).xCreate.expect("non-null function pointer")(
            argc - 2 as ::core::ffi::c_int,
            &raw mut azArg as *mut *const ::core::ffi::c_char,
            &raw mut pTokenizer,
        )
    {
        zErr = b"error in xCreate()\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        (*pTokenizer).pModule = p;
        if sqlite3Fts3OpenTokenizer(
            pTokenizer,
            0 as ::core::ffi::c_int,
            zInput,
            nInput,
            &raw mut pCsr,
        ) != 0
        {
            zErr = b"error in xOpen()\0" as *const u8 as *const ::core::ffi::c_char;
        } else {
            while SQLITE_OK
                == (*p).xNext.expect("non-null function pointer")(
                    pCsr,
                    &raw mut zToken,
                    &raw mut nToken,
                    &raw mut iStart,
                    &raw mut iEnd,
                    &raw mut iPos,
                )
            {
                Tcl_ListObjAppendElement(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    pRet,
                    Tcl_NewWideIntObj(iPos as Tcl_WideInt),
                );
                Tcl_ListObjAppendElement(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    pRet,
                    Tcl_NewStringObj(zToken, nToken as Tcl_Size),
                );
                zToken = zInput.offset(iStart as isize) as *const ::core::ffi::c_char;
                nToken = iEnd - iStart;
                Tcl_ListObjAppendElement(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    pRet,
                    Tcl_NewStringObj(zToken, nToken as Tcl_Size),
                );
            }
            if SQLITE_OK != (*p).xClose.expect("non-null function pointer")(pCsr) {
                zErr = b"error in xClose()\0" as *const u8 as *const ::core::ffi::c_char;
            } else if SQLITE_OK != (*p).xDestroy.expect("non-null function pointer")(pTokenizer) {
                zErr = b"error in xDestroy()\0" as *const u8 as *const ::core::ffi::c_char;
            }
        }
    }
    if !zErr.is_null() {
        sqlite3_result_error(context, zErr, -(1 as ::core::ffi::c_int));
    } else {
        sqlite3_result_text(
            context,
            Tcl_GetStringFromObj(pRet, NULL as *mut Tcl_Size),
            -(1 as ::core::ffi::c_int),
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-(1 as ::core::ffi::c_int) as ::libc::intptr_t),
        );
    }
    let mut _objPtr: *mut Tcl_Obj = pRet;
    let fresh0 = (*_objPtr).refCount;
    (*_objPtr).refCount = (*_objPtr).refCount - 1;
    if fresh0 <= 1 as Tcl_Size {
        TclFreeObj(_objPtr);
    }
}
unsafe extern "C" fn registerTokenizer(
    mut db: *mut sqlite3,
    mut zName: *mut ::core::ffi::c_char,
    mut p: *const sqlite3_tokenizer_module,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let zSql: [::core::ffi::c_char; 28] = ::core::mem::transmute::<
        [u8; 28],
        [::core::ffi::c_char; 28],
    >(*b"SELECT fts3_tokenizer(?, ?)\0");
    rc = sqlite3_prepare_v2(
        db,
        &raw const zSql as *const ::core::ffi::c_char,
        -(1 as ::core::ffi::c_int),
        &raw mut pStmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return rc;
    }
    sqlite3_bind_text(
        pStmt,
        1 as ::core::ffi::c_int,
        zName,
        -(1 as ::core::ffi::c_int),
        SQLITE_STATIC,
    );
    sqlite3_bind_blob(
        pStmt,
        2 as ::core::ffi::c_int,
        &raw mut p as *const ::core::ffi::c_void,
        ::core::mem::size_of::<*const sqlite3_tokenizer_module>() as ::core::ffi::c_int,
        SQLITE_STATIC,
    );
    sqlite3_step(pStmt);
    return sqlite3_finalize(pStmt);
}
unsafe extern "C" fn queryTokenizer(
    mut db: *mut sqlite3,
    mut zName: *mut ::core::ffi::c_char,
    mut pp: *mut *const sqlite3_tokenizer_module,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
    let zSql: [::core::ffi::c_char; 25] = ::core::mem::transmute::<
        [u8; 25],
        [::core::ffi::c_char; 25],
    >(*b"SELECT fts3_tokenizer(?)\0");
    *pp = ::core::ptr::null::<sqlite3_tokenizer_module>();
    rc = sqlite3_prepare_v2(
        db,
        &raw const zSql as *const ::core::ffi::c_char,
        -(1 as ::core::ffi::c_int),
        &raw mut pStmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != SQLITE_OK {
        return rc;
    }
    sqlite3_bind_text(
        pStmt,
        1 as ::core::ffi::c_int,
        zName,
        -(1 as ::core::ffi::c_int),
        SQLITE_STATIC,
    );
    if SQLITE_ROW == sqlite3_step(pStmt) {
        if sqlite3_column_type(pStmt, 0 as ::core::ffi::c_int) == SQLITE_BLOB
            && sqlite3_column_bytes(pStmt, 0 as ::core::ffi::c_int) as usize
                == ::core::mem::size_of::<*const sqlite3_tokenizer_module>() as usize
        {
            memcpy(
                pp as *mut ::core::ffi::c_void,
                sqlite3_column_blob(pStmt, 0 as ::core::ffi::c_int),
                ::core::mem::size_of::<*const sqlite3_tokenizer_module>() as size_t,
            );
        }
    }
    return sqlite3_finalize(pStmt);
}
unsafe extern "C" fn intTestFunc(
    mut context: *mut sqlite3_context,
    mut _argc: ::core::ffi::c_int,
    mut _argv: *mut *mut sqlite3_value,
) {
    let mut rc: ::core::ffi::c_int = 0;
    let mut p1: *const sqlite3_tokenizer_module = ::core::ptr::null::<sqlite3_tokenizer_module>();
    let mut p2: *const sqlite3_tokenizer_module = ::core::ptr::null::<sqlite3_tokenizer_module>();
    let mut db: *mut sqlite3 = sqlite3_user_data(context) as *mut sqlite3;
    sqlite3Fts3SimpleTokenizerModule(&raw mut p1);
    rc = queryTokenizer(
        db,
        b"simple\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut p2,
    );
    rc = queryTokenizer(
        db,
        b"nosuchtokenizer\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        &raw mut p2,
    );
    if fts3TokenizerEnabled(context) != 0 {
        rc = registerTokenizer(
            db,
            b"nosuchtokenizer\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            p1,
        );
        rc = queryTokenizer(
            db,
            b"nosuchtokenizer\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut p2,
        );
    }
    sqlite3_result_text(
        context,
        b"ok\0" as *const u8 as *const ::core::ffi::c_char,
        -(1 as ::core::ffi::c_int),
        SQLITE_STATIC,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sqlite3Fts3InitHashTable(
    mut db: *mut sqlite3,
    mut pHash: *mut Fts3Hash,
    mut zName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = SQLITE_OK;
    let mut p: *mut ::core::ffi::c_void = pHash as *mut ::core::ffi::c_void;
    let any: ::core::ffi::c_int = SQLITE_UTF8 | SQLITE_DIRECTONLY;
    let mut zTest: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut zTest2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pdb: *mut ::core::ffi::c_void = db as *mut ::core::ffi::c_void;
    zTest = sqlite3_mprintf(
        b"%s_test\0" as *const u8 as *const ::core::ffi::c_char,
        zName,
    );
    zTest2 = sqlite3_mprintf(
        b"%s_internal_test\0" as *const u8 as *const ::core::ffi::c_char,
        zName,
    );
    if zTest.is_null() || zTest2.is_null() {
        rc = SQLITE_NOMEM;
    }
    if SQLITE_OK == rc {
        rc = sqlite3_create_function(
            db,
            zName,
            1 as ::core::ffi::c_int,
            any,
            p,
            Some(
                fts3TokenizerFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
    }
    if SQLITE_OK == rc {
        rc = sqlite3_create_function(
            db,
            zName,
            2 as ::core::ffi::c_int,
            any,
            p,
            Some(
                fts3TokenizerFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
    }
    if SQLITE_OK == rc {
        #[cfg(feature = "test")]
        {
            rc = sqlite3_create_function(
                db,
                zTest,
                -(1 as ::core::ffi::c_int),
                any,
                p,
                Some(
                    testFunc as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
                ),
                None,
                None,
            );
        }

        #[cfg(not(feature = "test"))]
        {
            rc = sqlite3_create_function(
                db,
                zTest,
                -(1 as ::core::ffi::c_int),
                any,
                p,
                None,
                None,
                None,
            );
        }
    }
    if SQLITE_OK == rc {
        rc = sqlite3_create_function(
            db,
            zTest2,
            0 as ::core::ffi::c_int,
            any,
            pdb,
            Some(
                intTestFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
    }
    sqlite3_free(zTest as *mut ::core::ffi::c_void);
    sqlite3_free(zTest2 as *mut ::core::ffi::c_void);
    return rc;
}
