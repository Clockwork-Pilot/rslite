unsafe extern "C" {
    pub type Tcl_Command_;
    fn Tcl_Alloc(size: ::core::ffi::c_uint) -> *mut ::core::ffi::c_char;
    fn Tcl_Free(ptr: *mut ::core::ffi::c_char);
    fn TclFreeObj(objPtr: *mut Tcl_Obj);
    fn Tcl_GetIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetStringFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn Tcl_ListObjAppendElement(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objPtr: *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_ListObjGetElements(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objcPtr: *mut ::core::ffi::c_int,
        objvPtr: *mut *mut *mut Tcl_Obj,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewByteArrayObj(
        bytes: *const ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
    fn Tcl_NewObj() -> *mut Tcl_Obj;
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_ObjSetVar2(
        interp: *mut Tcl_Interp,
        part1Ptr: *mut Tcl_Obj,
        part2Ptr: *mut Tcl_Obj,
        newValuePtr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_ResetResult(interp: *mut Tcl_Interp);
    fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_GetIndexFromObjStruct(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        tablePtr: *const ::core::ffi::c_void,
        offset: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        indexPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetWideIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        widePtr: *mut Tcl_WideInt,
    ) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_malloc64(_: sqlite3_uint64) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3Fts3PutVarint(
        _: *mut ::core::ffi::c_char,
        _: sqlite3_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3GetVarint(
        _: *const ::core::ffi::c_char,
        _: *mut sqlite_int64,
    ) -> ::core::ffi::c_int;
    fn sqlite3Fts3GetVarint32(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type ClientData = *mut ::core::ffi::c_void;
pub type Tcl_WideInt = ::core::ffi::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Interp {
    pub resultDontUse: *mut ::core::ffi::c_char,
    pub freeProcDontUse: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ()>,
    pub errorLineDontUse: ::core::ffi::c_int,
}
pub type Tcl_Command = *mut Tcl_Command_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Obj {
    pub refCount: ::core::ffi::c_int,
    pub bytes: *mut ::core::ffi::c_char,
    pub length: ::core::ffi::c_int,
    pub typePtr: *const Tcl_ObjType,
    pub internalRep: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub longValue: ::core::ffi::c_long,
    pub doubleValue: ::core::ffi::c_double,
    pub otherValuePtr: *mut ::core::ffi::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: C2Rust_Unnamed_1,
    pub ptrAndLongRep: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub ptr: *mut ::core::ffi::c_void,
    pub value: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub ptr1: *mut ::core::ffi::c_void,
    pub ptr2: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_ObjType {
    pub name: *const ::core::ffi::c_char,
    pub freeIntRepProc: Option<Tcl_FreeInternalRepProc>,
    pub dupIntRepProc: Option<Tcl_DupInternalRepProc>,
    pub updateStringProc: Option<Tcl_UpdateStringProc>,
    pub setFromAnyProc: Option<Tcl_SetFromAnyProc>,
}
pub type Tcl_SetFromAnyProc = unsafe extern "C" fn(
    *mut Tcl_Interp,
    *mut Tcl_Obj,
) -> ::core::ffi::c_int;
pub type Tcl_UpdateStringProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
pub type Tcl_DupInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj, *mut Tcl_Obj) -> ();
pub type Tcl_FreeInternalRepProc = unsafe extern "C" fn(*mut Tcl_Obj) -> ();
pub type Tcl_CmdDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite_uint64 = ::core::ffi::c_ulonglong;
pub type sqlite3_int64 = sqlite_int64;
pub type sqlite3_uint64 = sqlite_uint64;
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
    pub xDestroy: Option<
        unsafe extern "C" fn(*mut sqlite3_tokenizer) -> ::core::ffi::c_int,
    >,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut sqlite3_tokenizer,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut sqlite3_tokenizer_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<
        unsafe extern "C" fn(*mut sqlite3_tokenizer_cursor) -> ::core::ffi::c_int,
    >,
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
pub type u8_0 = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NearPhrase {
    pub nNear: ::core::ffi::c_int,
    pub nToken: ::core::ffi::c_int,
    pub aToken: [NearToken; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NearToken {
    pub n: ::core::ffi::c_int,
    pub z: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NearDocument {
    pub nToken: ::core::ffi::c_int,
    pub aToken: *mut NearToken,
}
pub const NM_PHRASECOUNTS: NM_enum = 0;
pub type NM_enum = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestnmSubcmd {
    pub zName: *mut ::core::ffi::c_char,
    pub eOpt: NM_enum,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test_tokenizer {
    pub base: sqlite3_tokenizer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test_tokenizer_cursor {
    pub base: sqlite3_tokenizer_cursor,
    pub aInput: *const ::core::ffi::c_char,
    pub nInput: ::core::ffi::c_int,
    pub iInput: ::core::ffi::c_int,
    pub iToken: ::core::ffi::c_int,
    pub aBuffer: *mut ::core::ffi::c_char,
    pub nBuffer: ::core::ffi::c_int,
    pub iLangid: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const NM_MAX_TOKEN: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
unsafe extern "C" fn nm_phrase_match(
    mut p: *mut NearPhrase,
    mut aToken: *mut NearToken,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*p).nToken {
            let mut pToken: *mut NearToken = (&raw mut (*p).aToken as *mut NearToken)
                .offset(ii as isize) as *mut NearToken;
            if (*pToken).n > 0 as ::core::ffi::c_int
                && *(*pToken).z.offset(((*pToken).n - 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int == '*' as i32
            {
                if (*aToken.offset(ii as isize)).n
                    < (*pToken).n - 1 as ::core::ffi::c_int
                {
                    return 0 as ::core::ffi::c_int;
                }
                if memcmp(
                    (*aToken.offset(ii as isize)).z as *const ::core::ffi::c_void,
                    (*pToken).z as *const ::core::ffi::c_void,
                    ((*pToken).n - 1 as ::core::ffi::c_int) as size_t,
                ) != 0
                {
                    return 0 as ::core::ffi::c_int;
                }
            } else {
                if (*aToken.offset(ii as isize)).n != (*pToken).n {
                    return 0 as ::core::ffi::c_int;
                }
                if memcmp(
                    (*aToken.offset(ii as isize)).z as *const ::core::ffi::c_void,
                    (*pToken).z as *const ::core::ffi::c_void,
                    (*pToken).n as size_t,
                ) != 0
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
            ii += 1;
        }
        return 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn nm_near_chain(
    mut iDir: ::core::ffi::c_int,
    mut pDoc: *mut NearDocument,
    mut iPos: ::core::ffi::c_int,
    mut nPhrase: ::core::ffi::c_int,
    mut aPhrase: *mut NearPhrase,
    mut iPhrase: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iStart: ::core::ffi::c_int = 0;
        let mut iStop: ::core::ffi::c_int = 0;
        let mut ii: ::core::ffi::c_int = 0;
        let mut nNear: ::core::ffi::c_int = 0;
        let mut iPhrase2: ::core::ffi::c_int = 0;
        let mut p: *mut NearPhrase = ::core::ptr::null_mut::<NearPhrase>();
        let mut pPrev: *mut NearPhrase = ::core::ptr::null_mut::<NearPhrase>();
        if iDir == 1 as ::core::ffi::c_int {
            if iPhrase + 1 as ::core::ffi::c_int == nPhrase {
                return 1 as ::core::ffi::c_int;
            }
            nNear = (*aPhrase.offset((iPhrase + 1 as ::core::ffi::c_int) as isize))
                .nNear;
        } else {
            if iPhrase == 0 as ::core::ffi::c_int {
                return 1 as ::core::ffi::c_int;
            }
            nNear = (*aPhrase.offset(iPhrase as isize)).nNear;
        }
        pPrev = aPhrase.offset(iPhrase as isize) as *mut NearPhrase;
        iPhrase2 = iPhrase + iDir;
        p = aPhrase.offset(iPhrase2 as isize) as *mut NearPhrase;
        iStart = iPos - nNear - (*p).nToken;
        iStop = iPos + nNear + (*pPrev).nToken;
        if iStart < 0 as ::core::ffi::c_int {
            iStart = 0 as ::core::ffi::c_int;
        }
        if iStop > (*pDoc).nToken - (*p).nToken {
            iStop = (*pDoc).nToken - (*p).nToken;
        }
        ii = iStart;
        while ii <= iStop {
            if nm_phrase_match(p, (*pDoc).aToken.offset(ii as isize) as *mut NearToken)
                != 0
            {
                if nm_near_chain(iDir, pDoc, ii, nPhrase, aPhrase, iPhrase2) != 0 {
                    return 1 as ::core::ffi::c_int;
                }
            }
            ii += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn nm_match_count(
    mut pDoc: *mut NearDocument,
    mut nPhrase: ::core::ffi::c_int,
    mut aPhrase: *mut NearPhrase,
    mut iPhrase: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nOcc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ii: ::core::ffi::c_int = 0;
        let mut p: *mut NearPhrase = aPhrase.offset(iPhrase as isize) as *mut NearPhrase;
        ii = 0 as ::core::ffi::c_int;
        while ii < (*pDoc).nToken + 1 as ::core::ffi::c_int - (*p).nToken {
            if nm_phrase_match(p, (*pDoc).aToken.offset(ii as isize) as *mut NearToken)
                != 0
            {
                if !(0 as ::core::ffi::c_int
                    == nm_near_chain(
                        1 as ::core::ffi::c_int,
                        pDoc,
                        ii,
                        nPhrase,
                        aPhrase,
                        iPhrase,
                    ))
                {
                    if !(0 as ::core::ffi::c_int
                        == nm_near_chain(
                            -1 as ::core::ffi::c_int,
                            pDoc,
                            ii,
                            nPhrase,
                            aPhrase,
                            iPhrase,
                        ))
                    {
                        nOcc += 1;
                    }
                }
            }
            ii += 1;
        }
        return nOcc;
    }
}
unsafe extern "C" fn fts3_near_match_cmd(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut nTotal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = 0;
        let mut ii: ::core::ffi::c_int = 0;
        let mut nPhrase: ::core::ffi::c_int = 0;
        let mut aPhrase: *mut NearPhrase = ::core::ptr::null_mut::<NearPhrase>();
        let mut doc: NearDocument = NearDocument {
            nToken: 0 as ::core::ffi::c_int,
            aToken: ::core::ptr::null_mut::<NearToken>(),
        };
        let mut apDocToken: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<*mut Tcl_Obj>();
        let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pPhrasecount: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut apExprToken: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<*mut Tcl_Obj>();
        let mut nExprToken: ::core::ffi::c_int = 0;
        let mut nn: ::core::ffi::c_int = 0;
        if objc < 3 as ::core::ffi::c_int
            || objc % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DOCUMENT EXPR ?OPTION VALUE?...\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            rc = TCL_ERROR;
        } else {
            ii = 3 as ::core::ffi::c_int;
            while ii < objc {
                let mut aOpt: [TestnmSubcmd; 2] = [
                    TestnmSubcmd {
                        zName: b"-phrasecountvar\0".as_ptr()
                            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                        eOpt: NM_PHRASECOUNTS,
                    },
                    TestnmSubcmd {
                        zName: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        eOpt: NM_PHRASECOUNTS,
                    },
                ];
                let mut iOpt: ::core::ffi::c_int = 0;
                if Tcl_GetIndexFromObjStruct(
                    interp,
                    *objv.offset(ii as isize),
                    &raw mut aOpt as *mut TestnmSubcmd as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<TestnmSubcmd>() as ::core::ffi::c_int,
                    b"option\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                    &raw mut iOpt,
                ) != 0
                {
                    return TCL_ERROR;
                }
                match aOpt[iOpt as usize].eOpt as ::core::ffi::c_uint {
                    0 => {
                        pPhrasecount = *objv
                            .offset((ii + 1 as ::core::ffi::c_int) as isize);
                    }
                    _ => {}
                }
                ii += 2 as ::core::ffi::c_int;
            }
            rc = Tcl_ListObjGetElements(
                interp,
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut nn,
                &raw mut apDocToken,
            );
            doc.nToken = nn;
            if !(rc != TCL_OK) {
                doc.aToken = Tcl_Alloc(
                    (doc.nToken as usize)
                        .wrapping_mul(::core::mem::size_of::<NearToken>() as usize)
                        as ::core::ffi::c_uint,
                ) as *mut ::core::ffi::c_void as *mut NearToken;
                ii = 0 as ::core::ffi::c_int;
                while ii < doc.nToken {
                    let ref mut c2rust_fresh0 = (*doc.aToken.offset(ii as isize)).z;
                    *c2rust_fresh0 = Tcl_GetStringFromObj(
                        *apDocToken.offset(ii as isize),
                        &raw mut nn,
                    );
                    (*doc.aToken.offset(ii as isize)).n = nn;
                    ii += 1;
                }
                rc = Tcl_ListObjGetElements(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut nExprToken,
                    &raw mut apExprToken,
                );
                if !(rc != TCL_OK) {
                    nPhrase = (nExprToken + 1 as ::core::ffi::c_int)
                        / 2 as ::core::ffi::c_int;
                    aPhrase = Tcl_Alloc(
                        (nPhrase as usize)
                            .wrapping_mul(::core::mem::size_of::<NearPhrase>() as usize)
                            as ::core::ffi::c_uint,
                    ) as *mut ::core::ffi::c_void as *mut NearPhrase;
                    memset(
                        aPhrase as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (nPhrase as size_t)
                            .wrapping_mul(::core::mem::size_of::<NearPhrase>() as size_t),
                    );
                    ii = 0 as ::core::ffi::c_int;
                    loop {
                        if !(ii < nPhrase) {
                            c2rust_current_block = 7427571413727699167;
                            break;
                        }
                        let mut pPhrase: *mut Tcl_Obj = *apExprToken
                            .offset((ii * 2 as ::core::ffi::c_int) as isize);
                        let mut apToken: *mut *mut Tcl_Obj = ::core::ptr::null_mut::<
                            *mut Tcl_Obj,
                        >();
                        let mut nToken: ::core::ffi::c_int = 0;
                        let mut jj: ::core::ffi::c_int = 0;
                        rc = Tcl_ListObjGetElements(
                            interp,
                            pPhrase,
                            &raw mut nToken,
                            &raw mut apToken,
                        );
                        if rc != TCL_OK {
                            c2rust_current_block = 2512886781906808454;
                            break;
                        }
                        if nToken > NM_MAX_TOKEN {
                            Tcl_AppendResult(
                                interp,
                                b"Too many tokens in phrase\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                NULL,
                            );
                            rc = TCL_ERROR;
                            c2rust_current_block = 2512886781906808454;
                            break;
                        } else {
                            jj = 0 as ::core::ffi::c_int;
                            while jj < nToken {
                                let mut pT: *mut NearToken = (&raw mut (*aPhrase
                                    .offset(ii as isize))
                                    .aToken as *mut NearToken)
                                    .offset(jj as isize) as *mut NearToken;
                                (*pT).z = Tcl_GetStringFromObj(
                                    *apToken.offset(jj as isize),
                                    &raw mut nn,
                                );
                                (*pT).n = nn;
                                jj += 1;
                            }
                            (*aPhrase.offset(ii as isize)).nToken = nToken;
                            ii += 1;
                        }
                    }
                    match c2rust_current_block {
                        2512886781906808454 => {}
                        _ => {
                            ii = 1 as ::core::ffi::c_int;
                            loop {
                                if !(ii < nPhrase) {
                                    c2rust_current_block = 2122094917359643297;
                                    break;
                                }
                                let mut pNear: *mut Tcl_Obj = *apExprToken
                                    .offset(
                                        (2 as ::core::ffi::c_int * ii - 1 as ::core::ffi::c_int)
                                            as isize,
                                    );
                                let mut nNear: ::core::ffi::c_int = 0;
                                rc = Tcl_GetIntFromObj(interp, pNear, &raw mut nNear);
                                if rc != TCL_OK {
                                    c2rust_current_block = 2512886781906808454;
                                    break;
                                }
                                (*aPhrase.offset(ii as isize)).nNear = nNear;
                                ii += 1;
                            }
                            match c2rust_current_block {
                                2512886781906808454 => {}
                                _ => {
                                    pRet = Tcl_NewObj();
                                    (*pRet).refCount += 1;
                                    ii = 0 as ::core::ffi::c_int;
                                    while ii < nPhrase {
                                        let mut nOcc: ::core::ffi::c_int = nm_match_count(
                                            &raw mut doc,
                                            nPhrase,
                                            aPhrase,
                                            ii,
                                        );
                                        Tcl_ListObjAppendElement(interp, pRet, Tcl_NewIntObj(nOcc));
                                        nTotal += nOcc;
                                        ii += 1;
                                    }
                                    if !pPhrasecount.is_null() {
                                        Tcl_ObjSetVar2(
                                            interp,
                                            pPhrasecount,
                                            ::core::ptr::null_mut::<Tcl_Obj>(),
                                            pRet,
                                            0 as ::core::ffi::c_int,
                                        );
                                    }
                                    let mut _objPtr: *mut Tcl_Obj = pRet;
                                    let c2rust_fresh1 = (*_objPtr).refCount;
                                    (*_objPtr).refCount = (*_objPtr).refCount - 1;
                                    if c2rust_fresh1 <= 1 as ::core::ffi::c_int {
                                        TclFreeObj(_objPtr);
                                    }
                                    Tcl_SetObjResult(
                                        interp,
                                        Tcl_NewIntObj(
                                            ((nTotal > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                                                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                        ),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        Tcl_Free(aPhrase as *mut ::core::ffi::c_char);
        Tcl_Free(doc.aToken as *mut ::core::ffi::c_char);
        return rc;
    }
}
unsafe extern "C" fn fts3_configure_incr_load_cmd(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            static mut test_fts3_node_chunksize: ::core::ffi::c_int;
        }
        unsafe extern "C" {
            static mut test_fts3_node_chunk_threshold: ::core::ffi::c_int;
        }
        let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        if objc != 1 as ::core::ffi::c_int && objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?CHUNKSIZE THRESHOLD?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        pRet = Tcl_NewObj();
        (*pRet).refCount += 1;
        Tcl_ListObjAppendElement(interp, pRet, Tcl_NewIntObj(test_fts3_node_chunksize));
        Tcl_ListObjAppendElement(
            interp,
            pRet,
            Tcl_NewIntObj(test_fts3_node_chunk_threshold),
        );
        if objc == 3 as ::core::ffi::c_int {
            let mut iArg1: ::core::ffi::c_int = 0;
            let mut iArg2: ::core::ffi::c_int = 0;
            if Tcl_GetIntFromObj(
                interp,
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut iArg1,
            ) != 0
                || Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iArg2,
                ) != 0
            {
                let mut _objPtr: *mut Tcl_Obj = pRet;
                let c2rust_fresh2 = (*_objPtr).refCount;
                (*_objPtr).refCount = (*_objPtr).refCount - 1;
                if c2rust_fresh2 <= 1 as ::core::ffi::c_int {
                    TclFreeObj(_objPtr);
                }
                return TCL_ERROR;
            }
            test_fts3_node_chunksize = iArg1;
            test_fts3_node_chunk_threshold = iArg2;
        }
        Tcl_SetObjResult(interp, pRet);
        let mut _objPtr_0: *mut Tcl_Obj = pRet;
        let c2rust_fresh3 = (*_objPtr_0).refCount;
        (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
        if c2rust_fresh3 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr_0);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn testTokenizerCreate(
    mut argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
    mut ppTokenizer: *mut *mut sqlite3_tokenizer,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pNew: *mut test_tokenizer = ::core::ptr::null_mut::<test_tokenizer>();
        pNew = sqlite3_malloc(
            ::core::mem::size_of::<test_tokenizer>() as ::core::ffi::c_int,
        ) as *mut test_tokenizer;
        if pNew.is_null() {
            return SQLITE_NOMEM;
        }
        memset(
            pNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<test_tokenizer>() as size_t,
        );
        *ppTokenizer = pNew as *mut sqlite3_tokenizer;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn testTokenizerDestroy(
    mut pTokenizer: *mut sqlite3_tokenizer,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut test_tokenizer = pTokenizer as *mut test_tokenizer;
        sqlite3_free(p as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn testTokenizerOpen(
    mut pTokenizer: *mut sqlite3_tokenizer,
    mut pInput: *const ::core::ffi::c_char,
    mut nBytes: ::core::ffi::c_int,
    mut ppCursor: *mut *mut sqlite3_tokenizer_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pCsr: *mut test_tokenizer_cursor = ::core::ptr::null_mut::<
            test_tokenizer_cursor,
        >();
        pCsr = sqlite3_malloc(
            ::core::mem::size_of::<test_tokenizer_cursor>() as ::core::ffi::c_int,
        ) as *mut test_tokenizer_cursor;
        if pCsr.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            memset(
                pCsr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<test_tokenizer_cursor>() as size_t,
            );
            (*pCsr).aInput = pInput;
            if nBytes < 0 as ::core::ffi::c_int {
                (*pCsr).nInput = strlen(pInput) as ::core::ffi::c_int;
            } else {
                (*pCsr).nInput = nBytes;
            }
        }
        *ppCursor = pCsr as *mut sqlite3_tokenizer_cursor;
        return rc;
    }
}
unsafe extern "C" fn testTokenizerClose(
    mut pCursor: *mut sqlite3_tokenizer_cursor,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut test_tokenizer_cursor = pCursor as *mut test_tokenizer_cursor;
        sqlite3_free((*pCsr).aBuffer as *mut ::core::ffi::c_void);
        sqlite3_free(pCsr as *mut ::core::ffi::c_void);
        return SQLITE_OK;
    }
}
unsafe extern "C" fn testIsTokenChar(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    unsafe {
        return (c as ::core::ffi::c_int >= 'a' as i32
            && c as ::core::ffi::c_int <= 'z' as i32
            || c as ::core::ffi::c_int >= 'A' as i32
                && c as ::core::ffi::c_int <= 'Z' as i32) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn testTolower(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_char = c;
        if ret as ::core::ffi::c_int >= 'A' as i32
            && ret as ::core::ffi::c_int <= 'Z' as i32
        {
            ret = (ret as ::core::ffi::c_int - ('A' as i32 - 'a' as i32))
                as ::core::ffi::c_char;
        }
        return ret as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn testTokenizerNext(
    mut pCursor: *mut sqlite3_tokenizer_cursor,
    mut ppToken: *mut *const ::core::ffi::c_char,
    mut pnBytes: *mut ::core::ffi::c_int,
    mut piStartOffset: *mut ::core::ffi::c_int,
    mut piEndOffset: *mut ::core::ffi::c_int,
    mut piPosition: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCsr: *mut test_tokenizer_cursor = pCursor as *mut test_tokenizer_cursor;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pEnd: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        p = (*pCsr).aInput.offset((*pCsr).iInput as isize) as *const ::core::ffi::c_char;
        pEnd = (*pCsr).aInput.offset((*pCsr).nInput as isize)
            as *const ::core::ffi::c_char;
        while p < pEnd && testIsTokenChar(*p) == 0 as ::core::ffi::c_int {
            p = p.offset(1);
        }
        if p == pEnd {
            rc = SQLITE_DONE;
        } else {
            let mut pToken: *const ::core::ffi::c_char = p;
            let mut nToken: sqlite3_int64 = 0;
            while p < pEnd && testIsTokenChar(*p) != 0 {
                p = p.offset(1);
            }
            nToken = p.offset_from(pToken) as ::core::ffi::c_long as sqlite3_int64;
            if nToken > (*pCsr).nBuffer as ::core::ffi::c_longlong {
                sqlite3_free((*pCsr).aBuffer as *mut ::core::ffi::c_void);
                (*pCsr).aBuffer = sqlite3_malloc64(nToken as sqlite3_uint64)
                    as *mut ::core::ffi::c_char;
            }
            if (*pCsr).aBuffer.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                let mut i: ::core::ffi::c_int = 0;
                if (*pCsr).iLangid & 0x1 as ::core::ffi::c_int != 0 {
                    i = 0 as ::core::ffi::c_int;
                    while (i as ::core::ffi::c_longlong) < nToken {
                        *(*pCsr).aBuffer.offset(i as isize) = *pToken.offset(i as isize);
                        i += 1;
                    }
                } else {
                    i = 0 as ::core::ffi::c_int;
                    while (i as ::core::ffi::c_longlong) < nToken {
                        *(*pCsr).aBuffer.offset(i as isize) = testTolower(
                            *pToken.offset(i as isize),
                        ) as ::core::ffi::c_char;
                        i += 1;
                    }
                }
                (*pCsr).iToken += 1;
                (*pCsr).iInput = p.offset_from((*pCsr).aInput) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
                *ppToken = (*pCsr).aBuffer;
                *pnBytes = nToken as ::core::ffi::c_int;
                *piStartOffset = pToken.offset_from((*pCsr).aInput)
                    as ::core::ffi::c_long as ::core::ffi::c_int;
                *piEndOffset = p.offset_from((*pCsr).aInput) as ::core::ffi::c_long
                    as ::core::ffi::c_int;
                *piPosition = (*pCsr).iToken;
            }
        }
        return rc;
    }
}
unsafe extern "C" fn testTokenizerLanguage(
    mut pCursor: *mut sqlite3_tokenizer_cursor,
    mut iLangid: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pCsr: *mut test_tokenizer_cursor = pCursor as *mut test_tokenizer_cursor;
        (*pCsr).iLangid = iLangid;
        if (*pCsr).iLangid >= 100 as ::core::ffi::c_int {
            rc = SQLITE_ERROR;
        }
        return rc;
    }
}
unsafe extern "C" fn fts3_test_tokenizer_cmd(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        static mut testTokenizerModule: sqlite3_tokenizer_module = unsafe {
            sqlite3_tokenizer_module {
                iVersion: 1 as ::core::ffi::c_int,
                xCreate: Some(
                    testTokenizerCreate
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *const *const ::core::ffi::c_char,
                            *mut *mut sqlite3_tokenizer,
                        ) -> ::core::ffi::c_int,
                ),
                xDestroy: Some(
                    testTokenizerDestroy
                        as unsafe extern "C" fn(
                            *mut sqlite3_tokenizer,
                        ) -> ::core::ffi::c_int,
                ),
                xOpen: Some(
                    testTokenizerOpen
                        as unsafe extern "C" fn(
                            *mut sqlite3_tokenizer,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_tokenizer_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xClose: Some(
                    testTokenizerClose
                        as unsafe extern "C" fn(
                            *mut sqlite3_tokenizer_cursor,
                        ) -> ::core::ffi::c_int,
                ),
                xNext: Some(
                    testTokenizerNext
                        as unsafe extern "C" fn(
                            *mut sqlite3_tokenizer_cursor,
                            *mut *const ::core::ffi::c_char,
                            *mut ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                xLanguageid: Some(
                    testTokenizerLanguage
                        as unsafe extern "C" fn(
                            *mut sqlite3_tokenizer_cursor,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
            }
        };
        let mut pPtr: *const sqlite3_tokenizer_module = &raw const testTokenizerModule;
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewByteArrayObj(
                &raw mut pPtr as *const ::core::ffi::c_uchar,
                ::core::mem::size_of::<*mut sqlite3_tokenizer_module>()
                    as ::core::ffi::c_int,
            ),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn fts3_test_varint_cmd(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aBuf: [::core::ffi::c_char; 24] = [0; 24];
        let mut rc: ::core::ffi::c_int = 0;
        let mut w: Tcl_WideInt = 0;
        let mut w2: sqlite3_int64 = 0;
        let mut nByte: ::core::ffi::c_int = 0;
        let mut nByte2: ::core::ffi::c_int = 0;
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"INTEGER\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        rc = Tcl_GetWideIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut w,
        );
        if rc != TCL_OK {
            return rc;
        }
        nByte = sqlite3Fts3PutVarint(
            &raw mut aBuf as *mut ::core::ffi::c_char,
            w as sqlite3_int64,
        );
        nByte2 = sqlite3Fts3GetVarint(
            &raw mut aBuf as *mut ::core::ffi::c_char,
            &raw mut w2,
        );
        if w != w2 || nByte != nByte2 {
            let mut zErr: *mut ::core::ffi::c_char = sqlite3_mprintf(
                b"error testing %lld\0".as_ptr() as *const ::core::ffi::c_char,
                w,
            );
            Tcl_ResetResult(interp);
            Tcl_AppendResult(interp, zErr, NULL);
            return TCL_ERROR;
        }
        if w <= 2147483647 as ::core::ffi::c_longlong
            && w >= 0 as ::core::ffi::c_longlong
        {
            let mut i: ::core::ffi::c_int = 0;
            nByte2 = if *(&raw mut aBuf as *mut ::core::ffi::c_char as *mut u8_0)
                as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
            {
                sqlite3Fts3GetVarint32(
                    &raw mut aBuf as *mut ::core::ffi::c_char,
                    &raw mut i,
                )
            } else {
                i = *(&raw mut aBuf as *mut ::core::ffi::c_char as *mut u8_0)
                    as ::core::ffi::c_int;
                1 as ::core::ffi::c_int
            };
            if w as ::core::ffi::c_int != i || nByte != nByte2 {
                let mut zErr_0: *mut ::core::ffi::c_char = sqlite3_mprintf(
                    b"error testing %lld (32-bit)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    w,
                );
                Tcl_ResetResult(interp);
                Tcl_AppendResult(interp, zErr_0, NULL);
                return TCL_ERROR;
            }
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn fts3_may_be_corrupt(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sqlitetestfts3_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        Tcl_CreateObjCommand(
            interp,
            b"fts3_near_match\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                fts3_near_match_cmd
                    as unsafe extern "C" fn(
                        ClientData,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"fts3_configure_incr_load\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                fts3_configure_incr_load_cmd
                    as unsafe extern "C" fn(
                        ClientData,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"fts3_test_tokenizer\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                fts3_test_tokenizer_cmd
                    as unsafe extern "C" fn(
                        ClientData,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"fts3_test_varint\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                fts3_test_varint_cmd
                    as unsafe extern "C" fn(
                        ClientData,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        Tcl_CreateObjCommand(
            interp,
            b"sqlite3_fts3_may_be_corrupt\0".as_ptr() as *const ::core::ffi::c_char,
            Some(
                fts3_may_be_corrupt
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            None,
        );
        return TCL_OK;
    }
}
