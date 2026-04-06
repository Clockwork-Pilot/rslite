use ::libc;
unsafe extern "C" {
    pub type Tcl_Command_;
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    pub type Fts5Context;
    pub type Fts5Tokenizer;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn Tcl_Alloc(size: ::core::ffi::c_uint) -> *mut ::core::ffi::c_char;
    fn Tcl_Free(ptr: *mut ::core::ffi::c_char);
    fn Tcl_DuplicateObj(objPtr: *mut Tcl_Obj) -> *mut Tcl_Obj;
    fn TclFreeObj(objPtr: *mut Tcl_Obj);
    fn Tcl_GetBooleanFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetByteArrayFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_uchar;
    fn Tcl_GetDoubleFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        doublePtr: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
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
    fn Tcl_NewByteArrayObj(
        bytes: *const ::core::ffi::c_uchar,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_NewDoubleObj(doubleValue: ::core::ffi::c_double) -> *mut Tcl_Obj;
    fn Tcl_NewIntObj(intValue: ::core::ffi::c_int) -> *mut Tcl_Obj;
    fn Tcl_NewObj() -> *mut Tcl_Obj;
    fn Tcl_NewStringObj(
        bytes: *const ::core::ffi::c_char,
        length: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
    fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        proc: Option<Tcl_ObjCmdProc>,
        clientData: ClientData,
        deleteProc: Option<Tcl_CmdDeleteProc>,
    ) -> Tcl_Command;
    fn Tcl_DeleteCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetCommandInfo(
        interp: *mut Tcl_Interp,
        cmdName: *const ::core::ffi::c_char,
        infoPtr: *mut Tcl_CmdInfo,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetObjResult(interp: *mut Tcl_Interp) -> *mut Tcl_Obj;
    fn Tcl_GetStringResult(interp: *mut Tcl_Interp) -> *const ::core::ffi::c_char;
    fn Tcl_ResetResult(interp: *mut Tcl_Interp);
    fn Tcl_SetResult(
        interp: *mut Tcl_Interp,
        result: *mut ::core::ffi::c_char,
        freeProc: Option<Tcl_FreeProc>,
    );
    fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
    fn Tcl_SplitList(
        interp: *mut Tcl_Interp,
        listStr: *const ::core::ffi::c_char,
        argcPtr: *mut ::core::ffi::c_int,
        argvPtr: *mut *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::core::ffi::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::core::ffi::c_char,
    );
    fn Tcl_EvalObjEx(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_GetIndexFromObjStruct(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        tablePtr: *const ::core::ffi::c_void,
        offset: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        indexPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn Tcl_SetVar2Ex(
        interp: *mut Tcl_Interp,
        part1: *const ::core::ffi::c_char,
        part2: *const ::core::ffi::c_char,
        newValuePtr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
    fn Tcl_GetWideIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        widePtr: *mut Tcl_WideInt,
    ) -> ::core::ffi::c_int;
    fn Tcl_NewWideIntObj(wideValue: Tcl_WideInt) -> *mut Tcl_Obj;
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
    fn sqlite3_db_config(
        _: *mut sqlite3,
        op: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_pointer(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_char,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
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
    fn sqlite3_value_double(_: *mut sqlite3_value) -> ::core::ffi::c_double;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_result_blob(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_double(_: *mut sqlite3_context, _: ::core::ffi::c_double);
    fn sqlite3_result_error(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    fn sqlite3_result_error_nomem(_: *mut sqlite3_context);
    fn sqlite3_result_int(_: *mut sqlite3_context, _: ::core::ffi::c_int);
    fn sqlite3_result_int64(_: *mut sqlite3_context, _: sqlite3_int64);
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_result_text64(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: sqlite3_uint64,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        encoding: ::core::ffi::c_uchar,
    );
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn sqlite3Fts5TestRegisterMatchinfo(_: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3Fts5TestRegisterTok(
        _: *mut sqlite3,
        _: *mut fts5_api,
    ) -> ::core::ffi::c_int;
    fn sqlite3ErrName(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
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
pub type Tcl_CmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;
pub type Tcl_FreeProc = unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ();
pub type Tcl_NamespaceDeleteProc = unsafe extern "C" fn(ClientData) -> ();
pub type Tcl_ObjCmdProc = unsafe extern "C" fn(
    ClientData,
    *mut Tcl_Interp,
    ::core::ffi::c_int,
    *const *mut Tcl_Obj,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_Namespace {
    pub name: *mut ::core::ffi::c_char,
    pub fullName: *mut ::core::ffi::c_char,
    pub clientData: ClientData,
    pub deleteProc: Option<Tcl_NamespaceDeleteProc>,
    pub parentPtr: *mut Tcl_Namespace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tcl_CmdInfo {
    pub isNativeObjectProc: ::core::ffi::c_int,
    pub objProc: Option<Tcl_ObjCmdProc>,
    pub objClientData: ClientData,
    pub proc: Option<Tcl_CmdProc>,
    pub clientData: ClientData,
    pub deleteProc: Option<Tcl_CmdDeleteProc>,
    pub deleteData: ClientData,
    pub namespacePtr: *mut Tcl_Namespace,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SqliteDb {
    pub db: *mut sqlite3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ErrorCode {
    pub rc: ::core::ffi::c_int,
    pub zError: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F5tFunction {
    pub interp: *mut Tcl_Interp,
    pub pScript: *mut Tcl_Obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F5tApi {
    pub pApi: *const Fts5ExtensionApi,
    pub pFts: *mut Fts5Context,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F5tAuxData {
    pub pObj: *mut Tcl_Obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sub {
    pub zName: *const ::core::ffi::c_char,
    pub nArg: ::core::ffi::c_int,
    pub zMsg: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F5tTokenizeCtx {
    pub pRet: *mut Tcl_Obj,
    pub bSubst: ::core::ffi::c_int,
    pub zInput: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F5tTokenizerContext {
    pub pCtx: *mut ::core::ffi::c_void,
    pub xToken: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub pInst: *mut F5tTokenizerInstance,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F5tTokenizerInstance {
    pub interp: *mut Tcl_Interp,
    pub pScript: *mut Tcl_Obj,
    pub pModule: *mut F5tTokenizerModule,
    pub pParent: *mut Fts5Tokenizer,
    pub pContext: *mut F5tTokenizerContext,
    pub pLocale: *const ::core::ffi::c_char,
    pub nLocale: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F5tTokenizerModule {
    pub interp: *mut Tcl_Interp,
    pub pScript: *mut Tcl_Obj,
    pub pParentCtx: *mut ::core::ffi::c_void,
    pub parent_v2: fts5_tokenizer_v2,
    pub parent: fts5_tokenizer,
    pub pContext: *mut F5tTokenizerContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallbackCtx {
    pub p: *mut Fts5Tokenizer,
    pub pCtx: *mut ::core::ffi::c_void,
    pub flags: ::core::ffi::c_int,
    pub xToken: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OriginTextCtx {
    pub db: *mut sqlite3,
    pub pApi: *mut fts5_api,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OriginTextTokenizer {
    pub pTok: *mut Fts5Tokenizer,
    pub tokapi: fts5_tokenizer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OriginTextCb {
    pub pCtx: *mut ::core::ffi::c_void,
    pub pText: *const ::core::ffi::c_char,
    pub nText: ::core::ffi::c_int,
    pub xToken: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub aBuf: *mut ::core::ffi::c_char,
    pub nBuf: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cmd {
    pub zName: *mut ::core::ffi::c_char,
    pub xProc: Option<Tcl_ObjCmdProc>,
    pub bTokenizeCtx: ::core::ffi::c_int,
}
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_BREAK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const TCL_CONTINUE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TCL_GLOBAL_ONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_DBCONFIG_DEFENSIVE: ::core::ffi::c_int = 1010 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FTS5_TOKENIZE_QUERY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const FTS5_TOKENIZE_PREFIX: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const FTS5_TOKENIZE_DOCUMENT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const FTS5_TOKENIZE_AUX: ::core::ffi::c_int = 8;
pub const FTS5_TOKEN_COLOCATED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
unsafe extern "C" fn f5tDbPointer(
    mut interp: *mut Tcl_Interp,
    mut pObj: *mut Tcl_Obj,
    mut ppDb: *mut *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut SqliteDb = ::core::ptr::null_mut::<SqliteDb>();
        let mut cmdInfo: Tcl_CmdInfo = Tcl_CmdInfo {
            isNativeObjectProc: 0,
            objProc: None,
            objClientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            proc: None,
            clientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            deleteProc: None,
            deleteData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            namespacePtr: ::core::ptr::null_mut::<Tcl_Namespace>(),
        };
        let mut z: *mut ::core::ffi::c_char = Tcl_GetString(pObj);
        if Tcl_GetCommandInfo(interp, z, &raw mut cmdInfo) != 0 {
            p = cmdInfo.objClientData as *mut SqliteDb;
            *ppDb = (*p).db;
            return TCL_OK;
        }
        return TCL_ERROR;
    }
}
unsafe extern "C" fn f5tResultToErrorCode(
    mut zRes: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aErr: [ErrorCode; 4] = [
            ErrorCode {
                rc: SQLITE_DONE,
                zError: b"SQLITE_DONE\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ErrorCode {
                rc: SQLITE_ERROR,
                zError: b"SQLITE_ERROR\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ErrorCode {
                rc: SQLITE_OK,
                zError: b"SQLITE_OK\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ErrorCode {
                rc: SQLITE_OK,
                zError: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[ErrorCode; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<ErrorCode>() as usize)
        {
            if 0 as ::core::ffi::c_int == sqlite3_stricmp(zRes, aErr[i as usize].zError)
            {
                return aErr[i as usize].rc;
            }
            i += 1;
        }
        return SQLITE_ERROR;
    }
}
unsafe extern "C" fn f5tDbAndApi(
    mut interp: *mut Tcl_Interp,
    mut pObj: *mut Tcl_Obj,
    mut ppDb: *mut *mut sqlite3,
    mut ppApi: *mut *mut fts5_api,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut rc: ::core::ffi::c_int = f5tDbPointer(interp, pObj, &raw mut db);
        if rc != TCL_OK {
            return TCL_ERROR
        } else {
            let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
            let mut pApi: *mut fts5_api = ::core::ptr::null_mut::<fts5_api>();
            rc = sqlite3_prepare_v2(
                db,
                b"SELECT fts5(?1)\0".as_ptr() as *const ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
                &raw mut pStmt,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            if rc != SQLITE_OK {
                Tcl_AppendResult(
                    interp,
                    b"error: \0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_errmsg(db),
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                return TCL_ERROR;
            }
            sqlite3_bind_pointer(
                pStmt,
                1 as ::core::ffi::c_int,
                &raw mut pApi as *mut ::core::ffi::c_void,
                b"fts5_api_ptr\0".as_ptr() as *const ::core::ffi::c_char,
                None,
            );
            sqlite3_step(pStmt);
            if sqlite3_finalize(pStmt) != SQLITE_OK {
                Tcl_AppendResult(
                    interp,
                    b"error: \0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_errmsg(db),
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                return TCL_ERROR;
            }
            *ppDb = db;
            *ppApi = pApi;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn xTokenizeCb(
    mut pCtx: *mut ::core::ffi::c_void,
    mut tflags: ::core::ffi::c_int,
    mut zToken: *const ::core::ffi::c_char,
    mut nToken: ::core::ffi::c_int,
    mut iStart: ::core::ffi::c_int,
    mut iEnd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut F5tFunction = pCtx as *mut F5tFunction;
        let mut pEval: *mut Tcl_Obj = Tcl_DuplicateObj((*p).pScript);
        let mut rc: ::core::ffi::c_int = 0;
        (*pEval).refCount += 1;
        Tcl_ListObjAppendElement((*p).interp, pEval, Tcl_NewStringObj(zToken, nToken));
        Tcl_ListObjAppendElement((*p).interp, pEval, Tcl_NewIntObj(iStart));
        Tcl_ListObjAppendElement((*p).interp, pEval, Tcl_NewIntObj(iEnd));
        rc = Tcl_EvalObjEx((*p).interp, pEval, 0 as ::core::ffi::c_int);
        let mut _objPtr: *mut Tcl_Obj = pEval;
        let c2rust_fresh0 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh0 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        if rc == TCL_OK {
            rc = f5tResultToErrorCode(Tcl_GetStringResult((*p).interp));
        }
        return rc;
    }
}
unsafe extern "C" fn xQueryPhraseCb(
    mut pApi: *const Fts5ExtensionApi,
    mut pFts: *mut Fts5Context,
    mut pCtx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut F5tFunction = pCtx as *mut F5tFunction;
        static mut iCmd: sqlite3_int64 = 0 as sqlite3_int64;
        let mut pEval: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut zCmd: [::core::ffi::c_char; 64] = [0; 64];
        let mut sApi: F5tApi = F5tApi {
            pApi: ::core::ptr::null::<Fts5ExtensionApi>(),
            pFts: ::core::ptr::null_mut::<Fts5Context>(),
        };
        sApi.pApi = pApi;
        sApi.pFts = pFts;
        let c2rust_fresh2 = iCmd;
        iCmd = iCmd + 1;
        sprintf(
            &raw mut zCmd as *mut ::core::ffi::c_char,
            b"f5t_2_%lld\0".as_ptr() as *const ::core::ffi::c_char,
            c2rust_fresh2,
        );
        Tcl_CreateObjCommand(
            (*p).interp,
            &raw mut zCmd as *mut ::core::ffi::c_char,
            Some(
                xF5tApi
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            &raw mut sApi as ClientData,
            None,
        );
        pEval = Tcl_DuplicateObj((*p).pScript);
        (*pEval).refCount += 1;
        Tcl_ListObjAppendElement(
            (*p).interp,
            pEval,
            Tcl_NewStringObj(
                &raw mut zCmd as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        rc = Tcl_EvalObjEx((*p).interp, pEval, 0 as ::core::ffi::c_int);
        let mut _objPtr: *mut Tcl_Obj = pEval;
        let c2rust_fresh3 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh3 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        Tcl_DeleteCommand((*p).interp, &raw mut zCmd as *mut ::core::ffi::c_char);
        if rc == TCL_OK {
            rc = f5tResultToErrorCode(Tcl_GetStringResult((*p).interp));
        }
        return rc;
    }
}
unsafe extern "C" fn xSetAuxdataDestructor(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        let mut pData: *mut F5tAuxData = p as *mut F5tAuxData;
        let mut _objPtr: *mut Tcl_Obj = (*pData).pObj;
        let c2rust_fresh1 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh1 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        sqlite3_free(pData as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn xF5tApi(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aSub: [Sub; 22] = [
            Sub {
                zName: b"xColumnCount\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xRowCount\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xColumnTotalSize\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"COL\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xTokenize\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 2 as ::core::ffi::c_int,
                zMsg: b"TEXT SCRIPT\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xPhraseCount\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xPhraseSize\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"PHRASE\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xInstCount\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xInst\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"IDX\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xRowid\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xColumnText\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"COL\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xColumnSize\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"COL\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xQueryPhrase\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 2 as ::core::ffi::c_int,
                zMsg: b"PHRASE SCRIPT\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xSetAuxdata\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"VALUE\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xGetAuxdata\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"CLEAR\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xSetAuxdataInt\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"INTEGER\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xGetAuxdataInt\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"CLEAR\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xPhraseForeach\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 4 as ::core::ffi::c_int,
                zMsg: b"IPHRASE COLVAR OFFVAR SCRIPT\0".as_ptr()
                    as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xPhraseColumnForeach\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 3 as ::core::ffi::c_int,
                zMsg: b"IPHRASE COLVAR SCRIPT\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xQueryToken\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 2 as ::core::ffi::c_int,
                zMsg: b"IPHRASE ITERM\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xInstToken\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 2 as ::core::ffi::c_int,
                zMsg: b"IDX ITERM\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: b"xColumnLocale\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"COL\0".as_ptr() as *const ::core::ffi::c_char,
            },
            Sub {
                zName: ::core::ptr::null::<::core::ffi::c_char>(),
                nArg: 0 as ::core::ffi::c_int,
                zMsg: ::core::ptr::null::<::core::ffi::c_char>(),
            },
        ];
        let mut rc: ::core::ffi::c_int = 0;
        let mut iSub: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p: *mut F5tApi = clientData as *mut F5tApi;
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SUB-COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        rc = Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut aSub as *mut Sub as *const ::core::ffi::c_void,
            ::core::mem::size_of::<Sub>() as ::core::ffi::c_int,
            b"SUB-COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iSub,
        );
        if rc != TCL_OK {
            return rc;
        }
        if aSub[iSub as usize].nArg != objc - 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                aSub[iSub as usize].zMsg,
            );
            return TCL_ERROR;
        }
        match iSub {
            0 => {
                let mut nCol: ::core::ffi::c_int = 0;
                nCol = (*(*p).pApi)
                    .xColumnCount
                    .expect("non-null function pointer")((*p).pFts);
                if rc == SQLITE_OK {
                    Tcl_SetObjResult(interp, Tcl_NewIntObj(nCol));
                }
            }
            1 => {
                let mut nRow: sqlite3_int64 = 0;
                rc = (*(*p).pApi)
                    .xRowCount
                    .expect("non-null function pointer")((*p).pFts, &raw mut nRow);
                if rc == SQLITE_OK {
                    Tcl_SetObjResult(interp, Tcl_NewWideIntObj(nRow as Tcl_WideInt));
                }
            }
            2 => {
                let mut iCol: ::core::ffi::c_int = 0;
                let mut nSize: sqlite3_int64 = 0;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iCol,
                ) != 0
                {
                    return TCL_ERROR;
                }
                rc = (*(*p).pApi)
                    .xColumnTotalSize
                    .expect(
                        "non-null function pointer",
                    )((*p).pFts, iCol, &raw mut nSize);
                if rc == SQLITE_OK {
                    Tcl_SetObjResult(interp, Tcl_NewWideIntObj(nSize as Tcl_WideInt));
                }
            }
            3 => {
                let mut nText: ::core::ffi::c_int = 0;
                let mut zText: *mut ::core::ffi::c_char = Tcl_GetStringFromObj(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut nText,
                );
                let mut ctx: F5tFunction = F5tFunction {
                    interp: ::core::ptr::null_mut::<Tcl_Interp>(),
                    pScript: ::core::ptr::null_mut::<Tcl_Obj>(),
                };
                ctx.interp = interp;
                ctx.pScript = *objv.offset(3 as ::core::ffi::c_int as isize);
                rc = (*(*p).pApi)
                    .xTokenize
                    .expect(
                        "non-null function pointer",
                    )(
                    (*p).pFts,
                    zText,
                    nText,
                    &raw mut ctx as *mut ::core::ffi::c_void,
                    Some(
                        xTokenizeCb
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
                if rc == SQLITE_OK {
                    Tcl_ResetResult(interp);
                }
                return rc;
            }
            4 => {
                let mut nPhrase: ::core::ffi::c_int = 0;
                nPhrase = (*(*p).pApi)
                    .xPhraseCount
                    .expect("non-null function pointer")((*p).pFts);
                if rc == SQLITE_OK {
                    Tcl_SetObjResult(interp, Tcl_NewIntObj(nPhrase));
                }
            }
            5 => {
                let mut iPhrase: ::core::ffi::c_int = 0;
                let mut sz: ::core::ffi::c_int = 0;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iPhrase,
                ) != 0
                {
                    return TCL_ERROR;
                }
                sz = (*(*p).pApi)
                    .xPhraseSize
                    .expect("non-null function pointer")((*p).pFts, iPhrase);
                if rc == SQLITE_OK {
                    Tcl_SetObjResult(interp, Tcl_NewIntObj(sz));
                }
            }
            6 => {
                let mut nInst: ::core::ffi::c_int = 0;
                rc = (*(*p).pApi)
                    .xInstCount
                    .expect("non-null function pointer")((*p).pFts, &raw mut nInst);
                if rc == SQLITE_OK {
                    Tcl_SetObjResult(interp, Tcl_NewIntObj(nInst));
                }
            }
            7 => {
                let mut iIdx: ::core::ffi::c_int = 0;
                let mut ip: ::core::ffi::c_int = 0;
                let mut ic: ::core::ffi::c_int = 0;
                let mut io: ::core::ffi::c_int = 0;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iIdx,
                ) != 0
                {
                    return TCL_ERROR;
                }
                rc = (*(*p).pApi)
                    .xInst
                    .expect(
                        "non-null function pointer",
                    )((*p).pFts, iIdx, &raw mut ip, &raw mut ic, &raw mut io);
                if rc == SQLITE_OK {
                    let mut pList: *mut Tcl_Obj = Tcl_NewObj();
                    Tcl_ListObjAppendElement(interp, pList, Tcl_NewIntObj(ip));
                    Tcl_ListObjAppendElement(interp, pList, Tcl_NewIntObj(ic));
                    Tcl_ListObjAppendElement(interp, pList, Tcl_NewIntObj(io));
                    Tcl_SetObjResult(interp, pList);
                }
            }
            8 => {
                let mut iRowid: sqlite3_int64 = (*(*p).pApi)
                    .xRowid
                    .expect("non-null function pointer")((*p).pFts);
                Tcl_SetObjResult(interp, Tcl_NewWideIntObj(iRowid as Tcl_WideInt));
            }
            9 => {
                let mut z: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut iCol_0: ::core::ffi::c_int = 0;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iCol_0,
                ) != 0
                {
                    return TCL_ERROR;
                }
                rc = (*(*p).pApi)
                    .xColumnText
                    .expect(
                        "non-null function pointer",
                    )((*p).pFts, iCol_0, &raw mut z, &raw mut n);
                if rc == SQLITE_OK {
                    Tcl_SetObjResult(interp, Tcl_NewStringObj(z, n));
                }
            }
            10 => {
                let mut n_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut iCol_1: ::core::ffi::c_int = 0;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iCol_1,
                ) != 0
                {
                    return TCL_ERROR;
                }
                rc = (*(*p).pApi)
                    .xColumnSize
                    .expect(
                        "non-null function pointer",
                    )((*p).pFts, iCol_1, &raw mut n_0);
                if rc == SQLITE_OK {
                    Tcl_SetObjResult(interp, Tcl_NewIntObj(n_0));
                }
            }
            11 => {
                let mut iPhrase_0: ::core::ffi::c_int = 0;
                let mut ctx_0: F5tFunction = F5tFunction {
                    interp: ::core::ptr::null_mut::<Tcl_Interp>(),
                    pScript: ::core::ptr::null_mut::<Tcl_Obj>(),
                };
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iPhrase_0,
                ) != 0
                {
                    return TCL_ERROR;
                }
                ctx_0.interp = interp;
                ctx_0.pScript = *objv.offset(3 as ::core::ffi::c_int as isize);
                rc = (*(*p).pApi)
                    .xQueryPhrase
                    .expect(
                        "non-null function pointer",
                    )(
                    (*p).pFts,
                    iPhrase_0,
                    &raw mut ctx_0 as *mut ::core::ffi::c_void,
                    Some(
                        xQueryPhraseCb
                            as unsafe extern "C" fn(
                                *const Fts5ExtensionApi,
                                *mut Fts5Context,
                                *mut ::core::ffi::c_void,
                            ) -> ::core::ffi::c_int,
                    ),
                );
                if rc == SQLITE_OK {
                    Tcl_ResetResult(interp);
                }
            }
            12 => {
                let mut pData: *mut F5tAuxData = sqlite3_malloc(
                    ::core::mem::size_of::<F5tAuxData>() as ::core::ffi::c_int,
                ) as *mut F5tAuxData;
                if pData.is_null() {
                    Tcl_AppendResult(
                        interp,
                        b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
                (*pData).pObj = *objv.offset(2 as ::core::ffi::c_int as isize);
                (*(*pData).pObj).refCount += 1;
                rc = (*(*p).pApi)
                    .xSetAuxdata
                    .expect(
                        "non-null function pointer",
                    )(
                    (*p).pFts,
                    pData as *mut ::core::ffi::c_void,
                    Some(
                        xSetAuxdataDestructor
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                );
            }
            13 => {
                let mut pData_0: *mut F5tAuxData = ::core::ptr::null_mut::<F5tAuxData>();
                let mut bClear: ::core::ffi::c_int = 0;
                if Tcl_GetBooleanFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut bClear,
                ) != 0
                {
                    return TCL_ERROR;
                }
                pData_0 = (*(*p).pApi)
                    .xGetAuxdata
                    .expect("non-null function pointer")((*p).pFts, bClear)
                    as *mut F5tAuxData;
                if pData_0.is_null() {
                    Tcl_ResetResult(interp);
                } else {
                    Tcl_SetObjResult(interp, (*pData_0).pObj);
                    if bClear != 0 {
                        xSetAuxdataDestructor(pData_0 as *mut ::core::ffi::c_void);
                    }
                }
            }
            14 => {
                let mut iVal: ::core::ffi::c_int = 0;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iVal,
                ) != 0
                {
                    return TCL_ERROR;
                }
                rc = (*(*p).pApi)
                    .xSetAuxdata
                    .expect(
                        "non-null function pointer",
                    )(
                    (*p).pFts,
                    ::core::ptr::null_mut::<::core::ffi::c_char>().offset(iVal as isize)
                        as *mut ::core::ffi::c_void,
                    None,
                );
            }
            15 => {
                let mut iVal_0: ::core::ffi::c_int = 0;
                let mut bClear_0: ::core::ffi::c_int = 0;
                if Tcl_GetBooleanFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut bClear_0,
                ) != 0
                {
                    return TCL_ERROR;
                }
                iVal_0 = ((*(*p).pApi)
                    .xGetAuxdata
                    .expect("non-null function pointer")((*p).pFts, bClear_0)
                    as *mut ::core::ffi::c_char)
                    .offset_from(::core::ptr::null_mut::<::core::ffi::c_char>())
                    as ::core::ffi::c_long as ::core::ffi::c_int;
                Tcl_SetObjResult(interp, Tcl_NewIntObj(iVal_0));
            }
            16 => {
                let mut iPhrase_1: ::core::ffi::c_int = 0;
                let mut iCol_2: ::core::ffi::c_int = 0;
                let mut iOff: ::core::ffi::c_int = 0;
                let mut zColvar: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut zOffvar: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut pScript: *mut Tcl_Obj = *objv
                    .offset(5 as ::core::ffi::c_int as isize);
                let mut iter: Fts5PhraseIter = Fts5PhraseIter {
                    a: ::core::ptr::null::<::core::ffi::c_uchar>(),
                    b: ::core::ptr::null::<::core::ffi::c_uchar>(),
                };
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iPhrase_1,
                ) != 0
                {
                    return TCL_ERROR;
                }
                zColvar = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
                zOffvar = Tcl_GetString(*objv.offset(4 as ::core::ffi::c_int as isize));
                rc = (*(*p).pApi)
                    .xPhraseFirst
                    .expect(
                        "non-null function pointer",
                    )(
                    (*p).pFts,
                    iPhrase_1,
                    &raw mut iter,
                    &raw mut iCol_2,
                    &raw mut iOff,
                );
                if rc != SQLITE_OK {
                    Tcl_AppendResult(
                        interp,
                        sqlite3ErrName(rc),
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
                while iCol_2 >= 0 as ::core::ffi::c_int {
                    Tcl_SetVar2Ex(
                        interp,
                        zColvar,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        Tcl_NewIntObj(iCol_2),
                        0 as ::core::ffi::c_int,
                    );
                    Tcl_SetVar2Ex(
                        interp,
                        zOffvar,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        Tcl_NewIntObj(iOff),
                        0 as ::core::ffi::c_int,
                    );
                    rc = Tcl_EvalObjEx(interp, pScript, 0 as ::core::ffi::c_int);
                    if rc == TCL_CONTINUE {
                        rc = TCL_OK;
                    }
                    if rc != TCL_OK {
                        if rc == TCL_BREAK {
                            rc = TCL_OK;
                        }
                        break;
                    } else {
                        (*(*p).pApi)
                            .xPhraseNext
                            .expect(
                                "non-null function pointer",
                            )((*p).pFts, &raw mut iter, &raw mut iCol_2, &raw mut iOff);
                    }
                }
            }
            17 => {
                let mut iPhrase_2: ::core::ffi::c_int = 0;
                let mut iCol_3: ::core::ffi::c_int = 0;
                let mut zColvar_0: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut pScript_0: *mut Tcl_Obj = *objv
                    .offset(4 as ::core::ffi::c_int as isize);
                let mut iter_0: Fts5PhraseIter = Fts5PhraseIter {
                    a: ::core::ptr::null::<::core::ffi::c_uchar>(),
                    b: ::core::ptr::null::<::core::ffi::c_uchar>(),
                };
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iPhrase_2,
                ) != 0
                {
                    return TCL_ERROR;
                }
                zColvar_0 = Tcl_GetString(
                    *objv.offset(3 as ::core::ffi::c_int as isize),
                );
                rc = (*(*p).pApi)
                    .xPhraseFirstColumn
                    .expect(
                        "non-null function pointer",
                    )((*p).pFts, iPhrase_2, &raw mut iter_0, &raw mut iCol_3);
                if rc != SQLITE_OK {
                    Tcl_SetResult(
                        interp,
                        sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                        ::core::mem::transmute::<
                            ::libc::intptr_t,
                            Option<Tcl_FreeProc>,
                        >(1 as ::core::ffi::c_int as ::libc::intptr_t),
                    );
                    return TCL_ERROR;
                }
                while iCol_3 >= 0 as ::core::ffi::c_int {
                    Tcl_SetVar2Ex(
                        interp,
                        zColvar_0,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        Tcl_NewIntObj(iCol_3),
                        0 as ::core::ffi::c_int,
                    );
                    rc = Tcl_EvalObjEx(interp, pScript_0, 0 as ::core::ffi::c_int);
                    if rc == TCL_CONTINUE {
                        rc = TCL_OK;
                    }
                    if rc != TCL_OK {
                        if rc == TCL_BREAK {
                            rc = TCL_OK;
                        }
                        break;
                    } else {
                        (*(*p).pApi)
                            .xPhraseNextColumn
                            .expect(
                                "non-null function pointer",
                            )((*p).pFts, &raw mut iter_0, &raw mut iCol_3);
                    }
                }
            }
            18 => {
                let mut pTerm: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut nTerm: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut iPhrase_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut iTerm: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iPhrase_3,
                ) != 0
                {
                    return TCL_ERROR;
                }
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(3 as ::core::ffi::c_int as isize),
                    &raw mut iTerm,
                ) != 0
                {
                    return TCL_ERROR;
                }
                rc = (*(*p).pApi)
                    .xQueryToken
                    .expect(
                        "non-null function pointer",
                    )((*p).pFts, iPhrase_3, iTerm, &raw mut pTerm, &raw mut nTerm);
                if rc == SQLITE_OK {
                    Tcl_SetObjResult(interp, Tcl_NewStringObj(pTerm, nTerm));
                }
            }
            19 => {
                let mut pTerm_0: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut nTerm_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut iIdx_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut iTerm_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iIdx_0,
                ) != 0
                {
                    return TCL_ERROR;
                }
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(3 as ::core::ffi::c_int as isize),
                    &raw mut iTerm_0,
                ) != 0
                {
                    return TCL_ERROR;
                }
                rc = (*(*p).pApi)
                    .xInstToken
                    .expect(
                        "non-null function pointer",
                    )((*p).pFts, iIdx_0, iTerm_0, &raw mut pTerm_0, &raw mut nTerm_0);
                if rc == SQLITE_OK {
                    Tcl_SetObjResult(interp, Tcl_NewStringObj(pTerm_0, nTerm_0));
                }
            }
            20 => {
                let mut z_0: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                let mut n_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut iCol_4: ::core::ffi::c_int = 0;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut iCol_4,
                ) != 0
                {
                    return TCL_ERROR;
                }
                rc = (*(*p).pApi)
                    .xColumnLocale
                    .expect(
                        "non-null function pointer",
                    )((*p).pFts, iCol_4, &raw mut z_0, &raw mut n_1);
                if rc == SQLITE_OK && !z_0.is_null() {
                    Tcl_SetObjResult(interp, Tcl_NewStringObj(z_0, n_1));
                }
            }
            _ => {}
        }
        if rc != SQLITE_OK {
            Tcl_SetResult(
                interp,
                sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<Tcl_FreeProc>,
                >(1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn xF5tFunction(
    mut pApi: *const Fts5ExtensionApi,
    mut pFts: *mut Fts5Context,
    mut pCtx: *mut sqlite3_context,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut p: *mut F5tFunction = (*pApi)
            .xUserData
            .expect("non-null function pointer")(pFts) as *mut F5tFunction;
        let mut pEval: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut i: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        static mut iCmd: sqlite3_int64 = 0 as sqlite3_int64;
        let mut zCmd: [::core::ffi::c_char; 64] = [0; 64];
        let mut sApi: F5tApi = F5tApi {
            pApi: ::core::ptr::null::<Fts5ExtensionApi>(),
            pFts: ::core::ptr::null_mut::<Fts5Context>(),
        };
        sApi.pApi = pApi;
        sApi.pFts = pFts;
        let c2rust_fresh4 = iCmd;
        iCmd = iCmd + 1;
        sprintf(
            &raw mut zCmd as *mut ::core::ffi::c_char,
            b"f5t_%lld\0".as_ptr() as *const ::core::ffi::c_char,
            c2rust_fresh4,
        );
        Tcl_CreateObjCommand(
            (*p).interp,
            &raw mut zCmd as *mut ::core::ffi::c_char,
            Some(
                xF5tApi
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            &raw mut sApi as ClientData,
            None,
        );
        pEval = Tcl_DuplicateObj((*p).pScript);
        (*pEval).refCount += 1;
        Tcl_ListObjAppendElement(
            (*p).interp,
            pEval,
            Tcl_NewStringObj(
                &raw mut zCmd as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        i = 0 as ::core::ffi::c_int;
        while i < nVal {
            let mut pObj: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
            match sqlite3_value_type(*apVal.offset(i as isize)) {
                SQLITE_TEXT => {
                    pObj = Tcl_NewStringObj(
                        sqlite3_value_text(*apVal.offset(i as isize))
                            as *const ::core::ffi::c_char,
                        -1 as ::core::ffi::c_int,
                    );
                }
                SQLITE_BLOB => {
                    pObj = Tcl_NewByteArrayObj(
                        sqlite3_value_blob(*apVal.offset(i as isize))
                            as *const ::core::ffi::c_uchar,
                        sqlite3_value_bytes(*apVal.offset(i as isize)),
                    );
                }
                SQLITE_INTEGER => {
                    pObj = Tcl_NewWideIntObj(
                        sqlite3_value_int64(*apVal.offset(i as isize)) as Tcl_WideInt,
                    );
                }
                SQLITE_FLOAT => {
                    pObj = Tcl_NewDoubleObj(
                        sqlite3_value_double(*apVal.offset(i as isize)),
                    );
                }
                _ => {
                    pObj = Tcl_NewObj();
                }
            }
            Tcl_ListObjAppendElement((*p).interp, pEval, pObj);
            i += 1;
        }
        rc = Tcl_EvalObjEx((*p).interp, pEval, TCL_GLOBAL_ONLY);
        let mut _objPtr: *mut Tcl_Obj = pEval;
        let c2rust_fresh5 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh5 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        Tcl_DeleteCommand((*p).interp, &raw mut zCmd as *mut ::core::ffi::c_char);
        if rc != TCL_OK {
            sqlite3_result_error(
                pCtx,
                Tcl_GetStringResult((*p).interp),
                -1 as ::core::ffi::c_int,
            );
        } else {
            let mut pVar: *mut Tcl_Obj = Tcl_GetObjResult((*p).interp);
            let mut zType: *const ::core::ffi::c_char = if !(*pVar).typePtr.is_null() {
                (*(*pVar).typePtr).name
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            };
            let mut c: ::core::ffi::c_char = *zType
                .offset(0 as ::core::ffi::c_int as isize);
            if c as ::core::ffi::c_int == 'b' as i32
                && strcmp(zType, b"bytearray\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int && (*pVar).bytes.is_null()
            {
                let mut nn: ::core::ffi::c_int = 0;
                let mut data: *mut ::core::ffi::c_uchar = Tcl_GetByteArrayFromObj(
                    pVar,
                    &raw mut nn,
                );
                sqlite3_result_blob(
                    pCtx,
                    data as *const ::core::ffi::c_void,
                    nn,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            } else if c as ::core::ffi::c_int == 'b' as i32
                && strcmp(zType, b"boolean\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                let mut n: ::core::ffi::c_int = 0;
                Tcl_GetIntFromObj(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    pVar,
                    &raw mut n,
                );
                sqlite3_result_int(pCtx, n);
            } else if c as ::core::ffi::c_int == 'd' as i32
                && strcmp(zType, b"double\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                let mut r: ::core::ffi::c_double = 0.;
                Tcl_GetDoubleFromObj(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    pVar,
                    &raw mut r,
                );
                sqlite3_result_double(pCtx, r);
            } else if c as ::core::ffi::c_int == 'w' as i32
                && strcmp(zType, b"wideInt\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || c as ::core::ffi::c_int == 'i' as i32
                    && strcmp(zType, b"int\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
            {
                let mut v: Tcl_WideInt = 0;
                Tcl_GetWideIntFromObj(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    pVar,
                    &raw mut v,
                );
                sqlite3_result_int64(pCtx, v as sqlite3_int64);
            } else {
                let mut nn_0: ::core::ffi::c_int = 0;
                let mut data_0: *mut ::core::ffi::c_uchar = Tcl_GetStringFromObj(
                    pVar,
                    &raw mut nn_0,
                ) as *mut ::core::ffi::c_uchar;
                sqlite3_result_text(
                    pCtx,
                    data_0 as *mut ::core::ffi::c_char,
                    nn_0,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                    >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
            }
        };
    }
}
unsafe extern "C" fn xF5tDestroy(mut pCtx: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut F5tFunction = pCtx as *mut F5tFunction;
        let mut _objPtr: *mut Tcl_Obj = (*p).pScript;
        let c2rust_fresh6 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh6 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        Tcl_Free(p as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn f5tCreateFunction(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pScript: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pApi: *mut fts5_api = ::core::ptr::null_mut::<fts5_api>();
        let mut pCtx: *mut F5tFunction = ::core::ptr::null_mut::<F5tFunction>();
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB NAME SCRIPT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if f5tDbAndApi(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
            &raw mut pApi,
        ) != 0
        {
            return TCL_ERROR;
        }
        zName = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        pScript = *objv.offset(3 as ::core::ffi::c_int as isize);
        pCtx = Tcl_Alloc(::core::mem::size_of::<F5tFunction>() as ::core::ffi::c_uint)
            as *mut ::core::ffi::c_void as *mut F5tFunction;
        (*pCtx).interp = interp;
        (*pCtx).pScript = pScript;
        (*pScript).refCount += 1;
        rc = (*pApi)
            .xCreateFunction
            .expect(
                "non-null function pointer",
            )(
            pApi,
            zName,
            pCtx as *mut ::core::ffi::c_void,
            Some(
                xF5tFunction
                    as unsafe extern "C" fn(
                        *const Fts5ExtensionApi,
                        *mut Fts5Context,
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            Some(xF5tDestroy as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        if rc != SQLITE_OK {
            Tcl_AppendResult(
                interp,
                b"error: \0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn xTokenizeCb2(
    mut pCtx: *mut ::core::ffi::c_void,
    mut tflags: ::core::ffi::c_int,
    mut zToken: *const ::core::ffi::c_char,
    mut nToken: ::core::ffi::c_int,
    mut iStart: ::core::ffi::c_int,
    mut iEnd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut F5tTokenizeCtx = pCtx as *mut F5tTokenizeCtx;
        if (*p).bSubst != 0 {
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                (*p).pRet,
                Tcl_NewStringObj(zToken, nToken),
            );
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                (*p).pRet,
                Tcl_NewStringObj(
                    (*p).zInput.offset(iStart as isize) as *const ::core::ffi::c_char,
                    iEnd - iStart,
                ),
            );
        } else {
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                (*p).pRet,
                Tcl_NewStringObj(zToken, nToken),
            );
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                (*p).pRet,
                Tcl_NewIntObj(iStart),
            );
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                (*p).pRet,
                Tcl_NewIntObj(iEnd),
            );
        }
        return SQLITE_OK;
    }
}
unsafe extern "C" fn f5tTokenize(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pCopy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut zText: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut nText: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pApi: *mut fts5_api = ::core::ptr::null_mut::<fts5_api>();
        let mut pTok: *mut Fts5Tokenizer = ::core::ptr::null_mut::<Fts5Tokenizer>();
        let mut tokenizer: fts5_tokenizer = fts5_tokenizer {
            xCreate: None,
            xDelete: None,
            xTokenize: None,
        };
        let mut pRet: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pUserdata: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        let mut nArg: ::core::ffi::c_int = 0;
        let mut azArg: *mut *const ::core::ffi::c_char = ::core::ptr::null_mut::<
            *const ::core::ffi::c_char,
        >();
        let mut ctx: F5tTokenizeCtx = F5tTokenizeCtx {
            pRet: ::core::ptr::null_mut::<Tcl_Obj>(),
            bSubst: 0,
            zInput: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        if objc != 4 as ::core::ffi::c_int && objc != 5 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?-subst? DB NAME TEXT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if objc == 5 as ::core::ffi::c_int {
            let mut zOpt: *mut ::core::ffi::c_char = Tcl_GetString(
                *objv.offset(1 as ::core::ffi::c_int as isize),
            );
            if strcmp(b"-subst\0".as_ptr() as *const ::core::ffi::c_char, zOpt) != 0 {
                Tcl_AppendResult(
                    interp,
                    b"unrecognized option: \0".as_ptr() as *const ::core::ffi::c_char,
                    zOpt,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                return TCL_ERROR;
            }
        }
        if f5tDbAndApi(
            interp,
            *objv.offset((objc - 3 as ::core::ffi::c_int) as isize),
            &raw mut db,
            &raw mut pApi,
        ) != 0
        {
            return TCL_ERROR;
        }
        if Tcl_SplitList(
            interp,
            Tcl_GetString(*objv.offset((objc - 2 as ::core::ffi::c_int) as isize)),
            &raw mut nArg,
            &raw mut azArg,
        ) != 0
        {
            return TCL_ERROR;
        }
        if nArg == 0 as ::core::ffi::c_int {
            Tcl_AppendResult(
                interp,
                b"no such tokenizer: \0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            Tcl_Free(azArg as *mut ::core::ffi::c_void as *mut ::core::ffi::c_char);
            return TCL_ERROR;
        }
        zText = Tcl_GetStringFromObj(
            *objv.offset((objc - 1 as ::core::ffi::c_int) as isize),
            &raw mut nText,
        );
        rc = (*pApi)
            .xFindTokenizer
            .expect(
                "non-null function pointer",
            )(
            pApi,
            *azArg.offset(0 as ::core::ffi::c_int as isize),
            &raw mut pUserdata,
            &raw mut tokenizer,
        );
        if rc != SQLITE_OK {
            Tcl_AppendResult(
                interp,
                b"no such tokenizer: \0".as_ptr() as *const ::core::ffi::c_char,
                *azArg.offset(0 as ::core::ffi::c_int as isize),
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        rc = tokenizer
            .xCreate
            .expect(
                "non-null function pointer",
            )(
            pUserdata,
            azArg.offset(1 as ::core::ffi::c_int as isize)
                as *mut *const ::core::ffi::c_char,
            nArg - 1 as ::core::ffi::c_int,
            &raw mut pTok,
        );
        if rc != SQLITE_OK {
            Tcl_AppendResult(
                interp,
                b"error in tokenizer.xCreate()\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        if nText > 0 as ::core::ffi::c_int {
            pCopy = sqlite3_malloc(nText) as *mut ::core::ffi::c_char;
            if pCopy.is_null() {
                tokenizer.xDelete.expect("non-null function pointer")(pTok);
                Tcl_AppendResult(
                    interp,
                    b"error in sqlite3_malloc()\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
                return TCL_ERROR;
            } else {
                memcpy(
                    pCopy as *mut ::core::ffi::c_void,
                    zText as *const ::core::ffi::c_void,
                    nText as size_t,
                );
            }
        }
        pRet = Tcl_NewObj();
        (*pRet).refCount += 1;
        ctx.bSubst = (objc == 5 as ::core::ffi::c_int) as ::core::ffi::c_int;
        ctx.pRet = pRet;
        ctx.zInput = pCopy;
        rc = tokenizer
            .xTokenize
            .expect(
                "non-null function pointer",
            )(
            pTok,
            &raw mut ctx as *mut ::core::ffi::c_void,
            FTS5_TOKENIZE_DOCUMENT,
            pCopy,
            nText,
            Some(
                xTokenizeCb2
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
        tokenizer.xDelete.expect("non-null function pointer")(pTok);
        sqlite3_free(pCopy as *mut ::core::ffi::c_void);
        if rc != SQLITE_OK {
            Tcl_AppendResult(
                interp,
                b"error in tokenizer.xTokenize()\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            let mut _objPtr: *mut Tcl_Obj = pRet;
            let c2rust_fresh7 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh7 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
            return TCL_ERROR;
        }
        Tcl_Free(azArg as *mut ::core::ffi::c_void as *mut ::core::ffi::c_char);
        Tcl_SetObjResult(interp, pRet);
        let mut _objPtr_0: *mut Tcl_Obj = pRet;
        let c2rust_fresh8 = (*_objPtr_0).refCount;
        (*_objPtr_0).refCount = (*_objPtr_0).refCount - 1;
        if c2rust_fresh8 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr_0);
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn f5tTokenizerCreate(
    mut pCtx: *mut ::core::ffi::c_void,
    mut azArg: *mut *const ::core::ffi::c_char,
    mut nArg: ::core::ffi::c_int,
    mut ppOut: *mut *mut Fts5Tokenizer,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pParent: *mut Fts5Tokenizer = ::core::ptr::null_mut::<Fts5Tokenizer>();
        let mut pMod: *mut F5tTokenizerModule = pCtx as *mut F5tTokenizerModule;
        let mut pEval: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut rc: ::core::ffi::c_int = TCL_OK;
        let mut i: ::core::ffi::c_int = 0;
        if (*pMod).parent_v2.xCreate.is_some() {
            rc = (*pMod)
                .parent_v2
                .xCreate
                .expect(
                    "non-null function pointer",
                )(
                (*pMod).pParentCtx,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
                &raw mut pParent,
            );
        }
        if (*pMod).parent.xCreate.is_some() {
            rc = (*pMod)
                .parent
                .xCreate
                .expect(
                    "non-null function pointer",
                )(
                (*pMod).pParentCtx,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
                &raw mut pParent,
            );
        }
        pEval = Tcl_DuplicateObj((*pMod).pScript);
        (*pEval).refCount += 1;
        i = 0 as ::core::ffi::c_int;
        while rc == TCL_OK && i < nArg {
            let mut pObj: *mut Tcl_Obj = Tcl_NewStringObj(
                *azArg.offset(i as isize),
                -1 as ::core::ffi::c_int,
            );
            rc = Tcl_ListObjAppendElement((*pMod).interp, pEval, pObj);
            i += 1;
        }
        if rc == TCL_OK {
            rc = Tcl_EvalObjEx((*pMod).interp, pEval, TCL_GLOBAL_ONLY);
        }
        let mut _objPtr: *mut Tcl_Obj = pEval;
        let c2rust_fresh9 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh9 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        if rc == TCL_OK {
            let mut pInst: *mut F5tTokenizerInstance = ::core::ptr::null_mut::<
                F5tTokenizerInstance,
            >();
            pInst = Tcl_Alloc(
                ::core::mem::size_of::<F5tTokenizerInstance>() as ::core::ffi::c_uint,
            ) as *mut ::core::ffi::c_void as *mut F5tTokenizerInstance;
            memset(
                pInst as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<F5tTokenizerInstance>() as size_t,
            );
            (*pInst).interp = (*pMod).interp;
            (*pInst).pScript = Tcl_GetObjResult((*pMod).interp);
            (*pInst).pContext = (*pMod).pContext;
            (*pInst).pParent = pParent;
            (*pInst).pModule = pMod;
            (*(*pInst).pScript).refCount += 1;
            *ppOut = pInst as *mut Fts5Tokenizer;
        }
        return rc;
    }
}
unsafe extern "C" fn f5tTokenizerDelete(mut p: *mut Fts5Tokenizer) {
    unsafe {
        let mut pInst: *mut F5tTokenizerInstance = p as *mut F5tTokenizerInstance;
        if !pInst.is_null() {
            if !(*pInst).pParent.is_null() {
                if (*(*pInst).pModule).parent_v2.xDelete.is_some() {
                    (*(*pInst).pModule)
                        .parent_v2
                        .xDelete
                        .expect("non-null function pointer")((*pInst).pParent);
                } else {
                    (*(*pInst).pModule)
                        .parent
                        .xDelete
                        .expect("non-null function pointer")((*pInst).pParent);
                }
            }
            let mut _objPtr: *mut Tcl_Obj = (*pInst).pScript;
            let c2rust_fresh10 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh10 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
            Tcl_Free(pInst as *mut ::core::ffi::c_char);
        }
    }
}
unsafe extern "C" fn f5tTokenizerReallyTokenize(
    mut p: *mut Fts5Tokenizer,
    mut pCtx: *mut ::core::ffi::c_void,
    mut flags: ::core::ffi::c_int,
    mut pText: *const ::core::ffi::c_char,
    mut nText: ::core::ffi::c_int,
    mut xToken: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pInst: *mut F5tTokenizerInstance = p as *mut F5tTokenizerInstance;
        let mut pOldInst: *mut F5tTokenizerInstance = ::core::ptr::null_mut::<
            F5tTokenizerInstance,
        >();
        let mut pOldCtx: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut xOldToken: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        > = None;
        let mut pEval: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut zFlags: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        pOldCtx = (*(*pInst).pContext).pCtx;
        xOldToken = (*(*pInst).pContext).xToken;
        pOldInst = (*(*pInst).pContext).pInst;
        (*(*pInst).pContext).pCtx = pCtx;
        (*(*pInst).pContext).xToken = xToken;
        (*(*pInst).pContext).pInst = pInst;
        pEval = Tcl_DuplicateObj((*pInst).pScript);
        (*pEval).refCount += 1;
        match flags {
            FTS5_TOKENIZE_DOCUMENT => {
                zFlags = b"document\0".as_ptr() as *const ::core::ffi::c_char;
            }
            FTS5_TOKENIZE_AUX => {
                zFlags = b"aux\0".as_ptr() as *const ::core::ffi::c_char;
            }
            FTS5_TOKENIZE_QUERY => {
                zFlags = b"query\0".as_ptr() as *const ::core::ffi::c_char;
            }
            3 => {
                zFlags = b"prefixquery\0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => {
                zFlags = b"invalid\0".as_ptr() as *const ::core::ffi::c_char;
            }
        }
        Tcl_ListObjAppendElement(
            (*pInst).interp,
            pEval,
            Tcl_NewStringObj(zFlags, -1 as ::core::ffi::c_int),
        );
        Tcl_ListObjAppendElement((*pInst).interp, pEval, Tcl_NewStringObj(pText, nText));
        rc = Tcl_EvalObjEx((*pInst).interp, pEval, TCL_GLOBAL_ONLY);
        let mut _objPtr: *mut Tcl_Obj = pEval;
        let c2rust_fresh11 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh11 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        (*(*pInst).pContext).pCtx = pOldCtx;
        (*(*pInst).pContext).xToken = xOldToken;
        (*(*pInst).pContext).pInst = pOldInst;
        return rc;
    }
}
unsafe extern "C" fn f5tTokenizeCallback(
    mut pCtx: *mut ::core::ffi::c_void,
    mut tflags: ::core::ffi::c_int,
    mut z: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
    mut iStart: ::core::ffi::c_int,
    mut iEnd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut CallbackCtx = pCtx as *mut CallbackCtx;
        return f5tTokenizerReallyTokenize(
            (*p).p,
            (*p).pCtx,
            (*p).flags,
            z,
            n,
            (*p).xToken,
        );
    }
}
unsafe extern "C" fn f5tTokenizerTokenize_v2(
    mut p: *mut Fts5Tokenizer,
    mut pCtx: *mut ::core::ffi::c_void,
    mut flags: ::core::ffi::c_int,
    mut pText: *const ::core::ffi::c_char,
    mut nText: ::core::ffi::c_int,
    mut pLoc: *const ::core::ffi::c_char,
    mut nLoc: ::core::ffi::c_int,
    mut xToken: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut pInst: *mut F5tTokenizerInstance = p as *mut F5tTokenizerInstance;
        (*pInst).pLocale = pLoc;
        (*pInst).nLocale = nLoc;
        if !(*pInst).pParent.is_null() {
            let mut ctx: CallbackCtx = CallbackCtx {
                p: ::core::ptr::null_mut::<Fts5Tokenizer>(),
                pCtx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                flags: 0,
                xToken: None,
            };
            ctx.p = p;
            ctx.pCtx = pCtx;
            ctx.flags = flags;
            ctx.xToken = xToken;
            if (*(*pInst).pModule).parent_v2.xTokenize.is_some() {
                rc = (*(*pInst).pModule)
                    .parent_v2
                    .xTokenize
                    .expect(
                        "non-null function pointer",
                    )(
                    (*pInst).pParent,
                    &raw mut ctx as *mut ::core::ffi::c_void,
                    flags,
                    pText,
                    nText,
                    pLoc,
                    nLoc,
                    Some(
                        f5tTokenizeCallback
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
            } else {
                rc = (*(*pInst).pModule)
                    .parent
                    .xTokenize
                    .expect(
                        "non-null function pointer",
                    )(
                    (*pInst).pParent,
                    &raw mut ctx as *mut ::core::ffi::c_void,
                    flags,
                    pText,
                    nText,
                    Some(
                        f5tTokenizeCallback
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
        } else {
            rc = f5tTokenizerReallyTokenize(p, pCtx, flags, pText, nText, xToken);
        }
        (*pInst).pLocale = ::core::ptr::null::<::core::ffi::c_char>();
        (*pInst).nLocale = 0 as ::core::ffi::c_int;
        return rc;
    }
}
unsafe extern "C" fn f5tTokenizerTokenize(
    mut p: *mut Fts5Tokenizer,
    mut pCtx: *mut ::core::ffi::c_void,
    mut flags: ::core::ffi::c_int,
    mut pText: *const ::core::ffi::c_char,
    mut nText: ::core::ffi::c_int,
    mut xToken: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
) -> ::core::ffi::c_int {
    unsafe {
        return f5tTokenizerTokenize_v2(
            p,
            pCtx,
            flags,
            pText,
            nText,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
            xToken,
        );
    }
}
unsafe extern "C" fn f5tTokenizerLocale(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut F5tTokenizerContext = clientData as *mut F5tTokenizerContext;
        if objc != 1 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if (*p).xToken.is_none() {
            Tcl_AppendResult(
                interp,
                b"sqlite3_fts5_locale may only be used by tokenizer callback\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewStringObj((*(*p).pInst).pLocale, (*(*p).pInst).nLocale),
        );
        return TCL_OK;
    }
}
unsafe extern "C" fn f5tTokenizerReturn(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut p: *mut F5tTokenizerContext = clientData as *mut F5tTokenizerContext;
        let mut iStart: ::core::ffi::c_int = 0;
        let mut iEnd: ::core::ffi::c_int = 0;
        let mut nToken: ::core::ffi::c_int = 0;
        let mut tflags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zToken: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        if objc == 5 as ::core::ffi::c_int {
            let mut nArg: ::core::ffi::c_int = 0;
            let mut zArg: *mut ::core::ffi::c_char = Tcl_GetStringFromObj(
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut nArg,
            );
            if nArg <= 10 as ::core::ffi::c_int && nArg >= 2 as ::core::ffi::c_int
                && memcmp(
                    b"-colocated\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    zArg as *const ::core::ffi::c_void,
                    nArg as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                tflags |= FTS5_TOKEN_COLOCATED;
                c2rust_current_block = 13183875560443969876;
            } else {
                c2rust_current_block = 5073879406252420704;
            }
        } else if objc != 4 as ::core::ffi::c_int {
            c2rust_current_block = 5073879406252420704;
        } else {
            c2rust_current_block = 13183875560443969876;
        }
        match c2rust_current_block {
            5073879406252420704 => {
                Tcl_WrongNumArgs(
                    interp,
                    1 as ::core::ffi::c_int,
                    objv,
                    b"?-colocated? TEXT START END\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return TCL_ERROR;
            }
            _ => {
                zToken = Tcl_GetStringFromObj(
                    *objv.offset((objc - 3 as ::core::ffi::c_int) as isize),
                    &raw mut nToken,
                );
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset((objc - 2 as ::core::ffi::c_int) as isize),
                    &raw mut iStart,
                ) != 0
                    || Tcl_GetIntFromObj(
                        interp,
                        *objv.offset((objc - 1 as ::core::ffi::c_int) as isize),
                        &raw mut iEnd,
                    ) != 0
                {
                    return TCL_ERROR;
                }
                if (*p).xToken.is_none() {
                    Tcl_AppendResult(
                        interp,
                        b"sqlite3_fts5_token may only be used by tokenizer callback\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                    return TCL_ERROR;
                }
                rc = (*p)
                    .xToken
                    .expect(
                        "non-null function pointer",
                    )((*p).pCtx, tflags, zToken, nToken, iStart, iEnd);
                Tcl_SetResult(
                    interp,
                    sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                    ::core::mem::transmute::<
                        ::libc::intptr_t,
                        Option<Tcl_FreeProc>,
                    >(1 as ::core::ffi::c_int as ::libc::intptr_t),
                );
                return if rc == SQLITE_OK { TCL_OK } else { TCL_ERROR };
            }
        };
    }
}
unsafe extern "C" fn f5tDelTokenizer(mut pCtx: *mut ::core::ffi::c_void) {
    unsafe {
        let mut pMod: *mut F5tTokenizerModule = pCtx as *mut F5tTokenizerModule;
        let mut _objPtr: *mut Tcl_Obj = (*pMod).pScript;
        let c2rust_fresh12 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh12 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        Tcl_Free(pMod as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn f5tCreateTokenizer(
    mut clientData: ClientData,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pContext: *mut F5tTokenizerContext = clientData
            as *mut F5tTokenizerContext;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pApi: *mut fts5_api = ::core::ptr::null_mut::<fts5_api>();
        let mut zName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pScript: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pMod: *mut F5tTokenizerModule = ::core::ptr::null_mut::<
            F5tTokenizerModule,
        >();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut bV2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iVersion: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
        let mut zParent: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut ii: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if objc < 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?OPTIONS? DB NAME SCRIPT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        ii = 1 as ::core::ffi::c_int;
        while ii < objc - 3 as ::core::ffi::c_int {
            let mut iOpt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut azOpt: [*const ::core::ffi::c_char; 4] = [
                b"-v2\0".as_ptr() as *const ::core::ffi::c_char,
                b"-parent\0".as_ptr() as *const ::core::ffi::c_char,
                b"-version\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            ];
            if Tcl_GetIndexFromObjStruct(
                interp,
                *objv.offset(ii as isize),
                &raw mut azOpt as *mut *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<*mut ::core::ffi::c_char>() as ::core::ffi::c_int,
                b"OPTION\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                &raw mut iOpt,
            ) != 0
            {
                return TCL_ERROR;
            }
            match iOpt {
                0 => {
                    bV2 = 1 as ::core::ffi::c_int;
                }
                1 => {
                    ii += 1;
                    if ii == objc - 3 as ::core::ffi::c_int {
                        Tcl_AppendResult(
                            interp,
                            b"option requires an argument: -parent\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                        return TCL_ERROR;
                    }
                    zParent = Tcl_GetString(*objv.offset(ii as isize));
                }
                2 => {
                    ii += 1;
                    if ii == objc - 3 as ::core::ffi::c_int {
                        Tcl_AppendResult(
                            interp,
                            b"option requires an argument: -version\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        );
                        return TCL_ERROR;
                    }
                    if Tcl_GetIntFromObj(
                        interp,
                        *objv.offset(ii as isize),
                        &raw mut iVersion,
                    ) != 0
                    {
                        return TCL_ERROR;
                    }
                }
                _ => {}
            }
            ii += 1;
        }
        if f5tDbAndApi(
            interp,
            *objv.offset((objc - 3 as ::core::ffi::c_int) as isize),
            &raw mut db,
            &raw mut pApi,
        ) != 0
        {
            return TCL_ERROR;
        }
        zName = Tcl_GetString(*objv.offset((objc - 2 as ::core::ffi::c_int) as isize));
        pScript = *objv.offset((objc - 1 as ::core::ffi::c_int) as isize);
        pMod = Tcl_Alloc(
            ::core::mem::size_of::<F5tTokenizerModule>() as ::core::ffi::c_uint,
        ) as *mut ::core::ffi::c_void as *mut F5tTokenizerModule;
        memset(
            pMod as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<F5tTokenizerModule>() as size_t,
        );
        (*pMod).interp = interp;
        (*pMod).pScript = pScript;
        (*pScript).refCount += 1;
        (*pMod).pContext = pContext;
        if !zParent.is_null() {
            if bV2 != 0 {
                let mut pParent: *mut fts5_tokenizer_v2 = ::core::ptr::null_mut::<
                    fts5_tokenizer_v2,
                >();
                rc = (*pApi)
                    .xFindTokenizer_v2
                    .expect(
                        "non-null function pointer",
                    )(pApi, zParent, &raw mut (*pMod).pParentCtx, &raw mut pParent);
                if rc == SQLITE_OK {
                    memcpy(
                        &raw mut (*pMod).parent_v2 as *mut ::core::ffi::c_void,
                        pParent as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<fts5_tokenizer_v2>() as size_t,
                    );
                    (*pMod)
                        .parent_v2
                        .xDelete
                        .expect(
                            "non-null function pointer",
                        )(::core::ptr::null_mut::<Fts5Tokenizer>());
                }
            } else {
                rc = (*pApi)
                    .xFindTokenizer
                    .expect(
                        "non-null function pointer",
                    )(
                    pApi,
                    zParent,
                    &raw mut (*pMod).pParentCtx,
                    &raw mut (*pMod).parent,
                );
                if rc == SQLITE_OK {
                    (*pMod)
                        .parent
                        .xDelete
                        .expect(
                            "non-null function pointer",
                        )(::core::ptr::null_mut::<Fts5Tokenizer>());
                }
            }
        }
        if rc == SQLITE_OK {
            let mut pModCtx: *mut ::core::ffi::c_void = pMod as *mut ::core::ffi::c_void;
            if bV2 == 0 as ::core::ffi::c_int {
                let mut t: fts5_tokenizer = fts5_tokenizer {
                    xCreate: None,
                    xDelete: None,
                    xTokenize: None,
                };
                t.xCreate = Some(
                    f5tTokenizerCreate
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut *mut Fts5Tokenizer,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut *mut Fts5Tokenizer,
                        ) -> ::core::ffi::c_int,
                    >;
                t.xTokenize = Some(
                    f5tTokenizerTokenize
                        as unsafe extern "C" fn(
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
                )
                    as Option<
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
                    >;
                t.xDelete = Some(
                    f5tTokenizerDelete as unsafe extern "C" fn(*mut Fts5Tokenizer) -> (),
                ) as Option<unsafe extern "C" fn(*mut Fts5Tokenizer) -> ()>;
                rc = (*pApi)
                    .xCreateTokenizer
                    .expect(
                        "non-null function pointer",
                    )(
                    pApi,
                    zName,
                    pModCtx,
                    &raw mut t,
                    Some(
                        f5tDelTokenizer
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                );
            } else {
                let mut t2: fts5_tokenizer_v2 = fts5_tokenizer_v2 {
                    iVersion: 0,
                    xCreate: None,
                    xDelete: None,
                    xTokenize: None,
                };
                memset(
                    &raw mut t2 as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<fts5_tokenizer_v2>() as size_t,
                );
                t2.iVersion = iVersion;
                t2.xCreate = Some(
                    f5tTokenizerCreate
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut *mut Fts5Tokenizer,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut *mut Fts5Tokenizer,
                        ) -> ::core::ffi::c_int,
                    >;
                t2.xTokenize = Some(
                    f5tTokenizerTokenize_v2
                        as unsafe extern "C" fn(
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
                )
                    as Option<
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
                    >;
                t2.xDelete = Some(
                    f5tTokenizerDelete as unsafe extern "C" fn(*mut Fts5Tokenizer) -> (),
                ) as Option<unsafe extern "C" fn(*mut Fts5Tokenizer) -> ()>;
                rc = (*pApi)
                    .xCreateTokenizer_v2
                    .expect(
                        "non-null function pointer",
                    )(
                    pApi,
                    zName,
                    pModCtx,
                    &raw mut t2,
                    Some(
                        f5tDelTokenizer
                            as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                );
            }
        }
        if rc != SQLITE_OK {
            Tcl_AppendResult(
                interp,
                if bV2 != 0 {
                    b"error in fts5_api.xCreateTokenizer_v2()\0".as_ptr()
                        as *const ::core::ffi::c_char
                } else {
                    b"error in fts5_api.xCreateTokenizer()\0".as_ptr()
                        as *const ::core::ffi::c_char
                },
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn xF5tFree(mut clientData: ClientData) {
    unsafe {
        Tcl_Free(clientData as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn f5tMayBeCorrupt(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        return TCL_OK;
    }
}
unsafe extern "C" fn f5t_fts5HashKey(
    mut nSlot: ::core::ffi::c_int,
    mut p: *const ::core::ffi::c_char,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut h: ::core::ffi::c_uint = 13 as ::core::ffi::c_uint;
        i = n - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            h = h << 3 as ::core::ffi::c_int ^ h
                ^ *p.offset(i as isize) as ::core::ffi::c_uint;
            i -= 1;
        }
        return h.wrapping_rem(nSlot as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn f5tTokenHash(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut n: ::core::ffi::c_int = 0;
        let mut iVal: ::core::ffi::c_uint = 0;
        let mut nSlot: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"NSLOT TOKEN\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut nSlot,
        ) != 0
        {
            return TCL_ERROR;
        }
        z = Tcl_GetStringFromObj(
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut n,
        );
        iVal = f5t_fts5HashKey(nSlot, z, n);
        Tcl_SetObjResult(interp, Tcl_NewIntObj(iVal as ::core::ffi::c_int));
        return TCL_OK;
    }
}
unsafe extern "C" fn f5tRegisterMatchinfo(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if f5tDbPointer(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3Fts5TestRegisterMatchinfo(db);
        if rc != SQLITE_OK {
            Tcl_SetResult(
                interp,
                sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<Tcl_FreeProc>,
                >(1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn f5tRegisterTok(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pApi: *mut fts5_api = ::core::ptr::null_mut::<fts5_api>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if f5tDbAndApi(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
            &raw mut pApi,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3Fts5TestRegisterTok(db, pApi);
        if rc != SQLITE_OK {
            Tcl_SetResult(
                interp,
                sqlite3ErrName(rc) as *mut ::core::ffi::c_char,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<Tcl_FreeProc>,
                >(1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn f5tOrigintextTokenizerDelete(mut pCtx: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut OriginTextCtx = pCtx as *mut OriginTextCtx;
        Tcl_Free(p as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn f5tOrigintextCreate(
    mut pCtx: *mut ::core::ffi::c_void,
    mut azArg: *mut *const ::core::ffi::c_char,
    mut nArg: ::core::ffi::c_int,
    mut ppOut: *mut *mut Fts5Tokenizer,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut OriginTextCtx = pCtx as *mut OriginTextCtx;
        let mut pTok: *mut OriginTextTokenizer = ::core::ptr::null_mut::<
            OriginTextTokenizer,
        >();
        let mut pTokCtx: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        pTok = sqlite3_malloc(
            ::core::mem::size_of::<OriginTextTokenizer>() as ::core::ffi::c_int,
        ) as *mut OriginTextTokenizer;
        if pTok.is_null() {
            rc = SQLITE_NOMEM;
        } else if nArg < 1 as ::core::ffi::c_int {
            rc = SQLITE_ERROR;
        } else {
            rc = (*(*p).pApi)
                .xFindTokenizer
                .expect(
                    "non-null function pointer",
                )(
                (*p).pApi,
                *azArg.offset(0 as ::core::ffi::c_int as isize),
                &raw mut pTokCtx,
                &raw mut (*pTok).tokapi,
            );
        }
        if rc == SQLITE_OK {
            rc = (*pTok)
                .tokapi
                .xCreate
                .expect(
                    "non-null function pointer",
                )(
                pTokCtx,
                azArg.offset(1 as ::core::ffi::c_int as isize)
                    as *mut *const ::core::ffi::c_char,
                nArg - 1 as ::core::ffi::c_int,
                &raw mut (*pTok).pTok,
            );
        }
        if rc != SQLITE_OK {
            sqlite3_free(pTok as *mut ::core::ffi::c_void);
            pTok = ::core::ptr::null_mut::<OriginTextTokenizer>();
        }
        *ppOut = pTok as *mut Fts5Tokenizer;
        return rc;
    }
}
unsafe extern "C" fn f5tOrigintextDelete(mut pTokenizer: *mut Fts5Tokenizer) {
    unsafe {
        let mut p: *mut OriginTextTokenizer = pTokenizer as *mut OriginTextTokenizer;
        if !(*p).pTok.is_null() {
            (*p).tokapi.xDelete.expect("non-null function pointer")((*p).pTok);
        }
        sqlite3_free(p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn xOriginToken(
    mut pCtx: *mut ::core::ffi::c_void,
    mut tflags: ::core::ffi::c_int,
    mut pToken: *const ::core::ffi::c_char,
    mut nToken: ::core::ffi::c_int,
    mut iStart: ::core::ffi::c_int,
    mut iEnd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut OriginTextCb = pCtx as *mut OriginTextCb;
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if nToken == iEnd - iStart
            && 0 as ::core::ffi::c_int
                == memcmp(
                    pToken as *const ::core::ffi::c_void,
                    (*p).pText.offset(iStart as isize) as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    nToken as size_t,
                )
        {
            ret = (*p)
                .xToken
                .expect(
                    "non-null function pointer",
                )((*p).pCtx, tflags, pToken, nToken, iStart, iEnd);
        } else {
            let mut nReq: ::core::ffi::c_int = nToken + 1 as ::core::ffi::c_int
                + (iEnd - iStart);
            if nReq > (*p).nBuf {
                sqlite3_free((*p).aBuf as *mut ::core::ffi::c_void);
                (*p).aBuf = sqlite3_malloc(nReq * 2 as ::core::ffi::c_int)
                    as *mut ::core::ffi::c_char;
                if (*p).aBuf.is_null() {
                    return SQLITE_NOMEM;
                }
                (*p).nBuf = nReq * 2 as ::core::ffi::c_int;
            }
            memcpy(
                (*p).aBuf as *mut ::core::ffi::c_void,
                pToken as *const ::core::ffi::c_void,
                nToken as size_t,
            );
            *(*p).aBuf.offset(nToken as isize) = '\0' as i32 as ::core::ffi::c_char;
            memcpy(
                (*p).aBuf.offset((nToken + 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                (*p).pText.offset(iStart as isize) as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                (iEnd - iStart) as size_t,
            );
            ret = (*p)
                .xToken
                .expect(
                    "non-null function pointer",
                )((*p).pCtx, tflags, (*p).aBuf, nReq, iStart, iEnd);
        }
        return ret;
    }
}
unsafe extern "C" fn f5tOrigintextTokenize(
    mut pTokenizer: *mut Fts5Tokenizer,
    mut pCtx: *mut ::core::ffi::c_void,
    mut flags: ::core::ffi::c_int,
    mut pText: *const ::core::ffi::c_char,
    mut nText: ::core::ffi::c_int,
    mut xToken: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut OriginTextTokenizer = pTokenizer as *mut OriginTextTokenizer;
        let mut cb: OriginTextCb = OriginTextCb {
            pCtx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pText: ::core::ptr::null::<::core::ffi::c_char>(),
            nText: 0,
            xToken: None,
            aBuf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            nBuf: 0,
        };
        let mut ret: ::core::ffi::c_int = 0;
        memset(
            &raw mut cb as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<OriginTextCb>() as size_t,
        );
        cb.pCtx = pCtx;
        cb.pText = pText;
        cb.nText = nText;
        cb.xToken = xToken;
        ret = (*p)
            .tokapi
            .xTokenize
            .expect(
                "non-null function pointer",
            )(
            (*p).pTok,
            &raw mut cb as *mut ::core::ffi::c_void,
            flags,
            pText,
            nText,
            Some(
                xOriginToken
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
        sqlite3_free(cb.aBuf as *mut ::core::ffi::c_void);
        return ret;
    }
}
unsafe extern "C" fn f5tRegisterOriginText(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pApi: *mut fts5_api = ::core::ptr::null_mut::<fts5_api>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut tok: fts5_tokenizer = fts5_tokenizer {
            xCreate: None,
            xDelete: None,
            xTokenize: None,
        };
        let mut pCtx: *mut OriginTextCtx = ::core::ptr::null_mut::<OriginTextCtx>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if f5tDbAndApi(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
            &raw mut pApi,
        ) != 0
        {
            return TCL_ERROR;
        }
        pCtx = Tcl_Alloc(::core::mem::size_of::<OriginTextCtx>() as ::core::ffi::c_uint)
            as *mut ::core::ffi::c_void as *mut OriginTextCtx;
        (*pCtx).db = db;
        (*pCtx).pApi = pApi;
        tok.xCreate = Some(
            f5tOrigintextCreate
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut Fts5Tokenizer,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut *mut Fts5Tokenizer,
                ) -> ::core::ffi::c_int,
            >;
        tok.xDelete = Some(
            f5tOrigintextDelete as unsafe extern "C" fn(*mut Fts5Tokenizer) -> (),
        ) as Option<unsafe extern "C" fn(*mut Fts5Tokenizer) -> ()>;
        tok.xTokenize = Some(
            f5tOrigintextTokenize
                as unsafe extern "C" fn(
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
        )
            as Option<
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
            >;
        rc = (*pApi)
            .xCreateTokenizer
            .expect(
                "non-null function pointer",
            )(
            pApi,
            b"origintext\0".as_ptr() as *const ::core::ffi::c_char,
            pCtx as *mut ::core::ffi::c_void,
            &raw mut tok,
            Some(
                f5tOrigintextTokenizerDelete
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
        );
        Tcl_ResetResult(interp);
        if rc != SQLITE_OK {
            Tcl_AppendResult(
                interp,
                b"error: \0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite3_fts5_drop_corrupt_table(
    mut db: *mut sqlite3,
    mut zDb: *const ::core::ffi::c_char,
    mut zTab: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut bDef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        rc = sqlite3_db_config(
            db,
            SQLITE_DBCONFIG_DEFENSIVE,
            -1 as ::core::ffi::c_int,
            &raw mut bDef,
        );
        if rc == SQLITE_OK {
            let mut zScript: *mut ::core::ffi::c_char = sqlite3_mprintf(
                b"DELETE FROM %Q.'%q_data';DELETE FROM %Q.'%q_config';INSERT INTO %Q.'%q_data' VALUES(10, X'0000000000');INSERT INTO %Q.'%q_config' VALUES('version', 4);DROP TABLE %Q.'%q';\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                zDb,
                zTab,
                zDb,
                zTab,
                zDb,
                zTab,
                zDb,
                zTab,
                zDb,
                zTab,
            );
            if zScript.is_null() {
                rc = SQLITE_NOMEM;
            } else {
                if bDef != 0 {
                    sqlite3_db_config(
                        db,
                        SQLITE_DBCONFIG_DEFENSIVE,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                }
                rc = sqlite3_exec(
                    db,
                    zScript,
                    None,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                if bDef != 0 {
                    sqlite3_db_config(
                        db,
                        SQLITE_DBCONFIG_DEFENSIVE,
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                }
                sqlite3_free(zScript as *mut ::core::ffi::c_void);
            }
        }
        return rc;
    }
}
unsafe extern "C" fn f5tDropCorruptTable(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut zDb: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut zTab: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB DATABASE TABLE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if f5tDbPointer(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zDb = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize));
        zTab = Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize));
        rc = sqlite3_fts5_drop_corrupt_table(db, zDb, zTab);
        if rc != SQLITE_OK {
            Tcl_AppendResult(
                interp,
                b"error: \0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_errmsg(db),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            );
            return TCL_ERROR;
        }
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn f5tFree(mut p: *mut ::core::ffi::c_void) {
    unsafe {
        let mut x: *mut ::core::ffi::c_char = p as *mut ::core::ffi::c_char;
        Tcl_Free(
            x.offset(-8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn f5tStrFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: ::core::ffi::c_int,
    mut apArg: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zText: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        zText = sqlite3_value_text(*apArg.offset(0 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        if !zText.is_null() {
            let mut nText: sqlite3_int64 = strlen(zText) as sqlite3_int64;
            let mut zCopy: *mut ::core::ffi::c_char = Tcl_Alloc(
                (nText as ::core::ffi::c_longlong + 8 as ::core::ffi::c_longlong)
                    as ::core::ffi::c_uint,
            ) as *mut ::core::ffi::c_void as *mut ::core::ffi::c_char;
            if zCopy.is_null() {
                sqlite3_result_error_nomem(pCtx);
            } else {
                zCopy = zCopy.offset(8 as ::core::ffi::c_int as isize);
                memcpy(
                    zCopy as *mut ::core::ffi::c_void,
                    zText as *const ::core::ffi::c_void,
                    nText as size_t,
                );
                sqlite3_result_text64(
                    pCtx,
                    zCopy,
                    nText as sqlite3_uint64,
                    Some(
                        f5tFree as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                    ),
                    SQLITE_UTF8 as ::core::ffi::c_uchar,
                );
            }
        }
    }
}
unsafe extern "C" fn f5tRegisterStr(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if f5tDbPointer(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        sqlite3_create_function(
            db,
            b"str\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            SQLITE_UTF8,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            Some(
                f5tStrFunc
                    as unsafe extern "C" fn(
                        *mut sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Fts5tcl_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aCmd: [Cmd; 12] = unsafe {
            [
                Cmd {
                    zName: b"sqlite3_fts5_create_tokenizer\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        f5tCreateTokenizer
                            as unsafe extern "C" fn(
                                ClientData,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    bTokenizeCtx: 1 as ::core::ffi::c_int,
                },
                Cmd {
                    zName: b"sqlite3_fts5_token\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                    xProc: Some(
                        f5tTokenizerReturn
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    bTokenizeCtx: 1 as ::core::ffi::c_int,
                },
                Cmd {
                    zName: b"sqlite3_fts5_locale\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        f5tTokenizerLocale
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    bTokenizeCtx: 1 as ::core::ffi::c_int,
                },
                Cmd {
                    zName: b"sqlite3_fts5_tokenize\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        f5tTokenize
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    bTokenizeCtx: 0 as ::core::ffi::c_int,
                },
                Cmd {
                    zName: b"sqlite3_fts5_create_function\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        f5tCreateFunction
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    bTokenizeCtx: 0 as ::core::ffi::c_int,
                },
                Cmd {
                    zName: b"sqlite3_fts5_may_be_corrupt\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        f5tMayBeCorrupt
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    bTokenizeCtx: 0 as ::core::ffi::c_int,
                },
                Cmd {
                    zName: b"sqlite3_fts5_token_hash\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        f5tTokenHash
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    bTokenizeCtx: 0 as ::core::ffi::c_int,
                },
                Cmd {
                    zName: b"sqlite3_fts5_register_matchinfo\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        f5tRegisterMatchinfo
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    bTokenizeCtx: 0 as ::core::ffi::c_int,
                },
                Cmd {
                    zName: b"sqlite3_fts5_register_fts5tokenize\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        f5tRegisterTok
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    bTokenizeCtx: 0 as ::core::ffi::c_int,
                },
                Cmd {
                    zName: b"sqlite3_fts5_register_origintext\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        f5tRegisterOriginText
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    bTokenizeCtx: 0 as ::core::ffi::c_int,
                },
                Cmd {
                    zName: b"sqlite3_fts5_drop_corrupt_table\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        f5tDropCorruptTable
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    bTokenizeCtx: 0 as ::core::ffi::c_int,
                },
                Cmd {
                    zName: b"sqlite3_fts5_register_str\0".as_ptr()
                        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                    xProc: Some(
                        f5tRegisterStr
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut Tcl_Interp,
                                ::core::ffi::c_int,
                                *const *mut Tcl_Obj,
                            ) -> ::core::ffi::c_int,
                    ),
                    bTokenizeCtx: 0 as ::core::ffi::c_int,
                },
            ]
        };
        let mut i: ::core::ffi::c_int = 0;
        let mut pContext: *mut F5tTokenizerContext = ::core::ptr::null_mut::<
            F5tTokenizerContext,
        >();
        pContext = Tcl_Alloc(
            ::core::mem::size_of::<F5tTokenizerContext>() as ::core::ffi::c_uint,
        ) as *mut ::core::ffi::c_void as *mut F5tTokenizerContext;
        memset(
            pContext as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<F5tTokenizerContext>() as size_t,
        );
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[Cmd; 12]>() as usize)
                .wrapping_div(::core::mem::size_of::<Cmd>() as usize)
        {
            let mut p: *mut Cmd = (&raw mut aCmd as *mut Cmd).offset(i as isize)
                as *mut Cmd;
            let mut pCtx: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            if (*p).bTokenizeCtx != 0 {
                pCtx = pContext as *mut ::core::ffi::c_void;
            }
            Tcl_CreateObjCommand(
                interp,
                (*p).zName,
                (*p).xProc,
                pCtx as ClientData,
                if i != 0 {
                    None
                } else {
                    Some(xF5tFree as unsafe extern "C" fn(ClientData) -> ())
                },
            );
            i += 1;
        }
        return TCL_OK;
    }
}
