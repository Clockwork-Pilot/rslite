unsafe extern "C" {
    fn sqlite3ErrName(rc: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_value;
    pub type sqlite3_session;
    pub type sqlite3_changeset_iter;
    pub type sqlite3_changegroup;
    pub type sqlite3_rebaser;
    pub type Tcl_Command_;
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
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_realloc(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_value_blob(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_double(_: *mut sqlite3_value) -> ::core::ffi::c_double;
    fn sqlite3_value_int64(_: *mut sqlite3_value) -> sqlite3_int64;
    fn sqlite3_value_text16(_: *mut sqlite3_value) -> *const ::core::ffi::c_void;
    fn sqlite3_value_bytes(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_type(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3session_create(
        db: *mut sqlite3,
        zDb: *const ::core::ffi::c_char,
        ppSession: *mut *mut sqlite3_session,
    ) -> ::core::ffi::c_int;
    fn sqlite3session_delete(pSession: *mut sqlite3_session);
    fn sqlite3session_object_config(
        _: *mut sqlite3_session,
        op: ::core::ffi::c_int,
        pArg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3session_enable(
        pSession: *mut sqlite3_session,
        bEnable: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3session_indirect(
        pSession: *mut sqlite3_session,
        bIndirect: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3session_attach(
        pSession: *mut sqlite3_session,
        zTab: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3session_table_filter(
        pSession: *mut sqlite3_session,
        xFilter: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pCtx: *mut ::core::ffi::c_void,
    );
    fn sqlite3session_changeset(
        pSession: *mut sqlite3_session,
        pnChangeset: *mut ::core::ffi::c_int,
        ppChangeset: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3session_changeset_size(pSession: *mut sqlite3_session) -> sqlite3_int64;
    fn sqlite3session_diff(
        pSession: *mut sqlite3_session,
        zFromDb: *const ::core::ffi::c_char,
        zTbl: *const ::core::ffi::c_char,
        pzErrMsg: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3session_patchset(
        pSession: *mut sqlite3_session,
        pnPatchset: *mut ::core::ffi::c_int,
        ppPatchset: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3session_isempty(pSession: *mut sqlite3_session) -> ::core::ffi::c_int;
    fn sqlite3session_memory_used(pSession: *mut sqlite3_session) -> sqlite3_int64;
    fn sqlite3changeset_start(
        pp: *mut *mut sqlite3_changeset_iter,
        nChangeset: ::core::ffi::c_int,
        pChangeset: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_start_v2(
        pp: *mut *mut sqlite3_changeset_iter,
        nChangeset: ::core::ffi::c_int,
        pChangeset: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_next(pIter: *mut sqlite3_changeset_iter) -> ::core::ffi::c_int;
    fn sqlite3changeset_op(
        pIter: *mut sqlite3_changeset_iter,
        pzTab: *mut *const ::core::ffi::c_char,
        pnCol: *mut ::core::ffi::c_int,
        pOp: *mut ::core::ffi::c_int,
        pbIndirect: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_pk(
        pIter: *mut sqlite3_changeset_iter,
        pabPK: *mut *mut ::core::ffi::c_uchar,
        pnCol: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_old(
        pIter: *mut sqlite3_changeset_iter,
        iVal: ::core::ffi::c_int,
        ppValue: *mut *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_new(
        pIter: *mut sqlite3_changeset_iter,
        iVal: ::core::ffi::c_int,
        ppValue: *mut *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_conflict(
        pIter: *mut sqlite3_changeset_iter,
        iVal: ::core::ffi::c_int,
        ppValue: *mut *mut sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_fk_conflicts(
        pIter: *mut sqlite3_changeset_iter,
        pnOut: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_finalize(
        pIter: *mut sqlite3_changeset_iter,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_invert(
        nIn: ::core::ffi::c_int,
        pIn: *const ::core::ffi::c_void,
        pnOut: *mut ::core::ffi::c_int,
        ppOut: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_concat(
        nA: ::core::ffi::c_int,
        pA: *mut ::core::ffi::c_void,
        nB: ::core::ffi::c_int,
        pB: *mut ::core::ffi::c_void,
        pnOut: *mut ::core::ffi::c_int,
        ppOut: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3changegroup_new(pp: *mut *mut sqlite3_changegroup) -> ::core::ffi::c_int;
    fn sqlite3changegroup_schema(
        _: *mut sqlite3_changegroup,
        _: *mut sqlite3,
        zDb: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3changegroup_add(
        _: *mut sqlite3_changegroup,
        nData: ::core::ffi::c_int,
        pData: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3changegroup_add_change(
        _: *mut sqlite3_changegroup,
        _: *mut sqlite3_changeset_iter,
    ) -> ::core::ffi::c_int;
    fn sqlite3changegroup_output(
        _: *mut sqlite3_changegroup,
        pnData: *mut ::core::ffi::c_int,
        ppData: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3changegroup_delete(_: *mut sqlite3_changegroup);
    fn sqlite3changeset_apply(
        db: *mut sqlite3,
        nChangeset: ::core::ffi::c_int,
        pChangeset: *mut ::core::ffi::c_void,
        xFilter: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        xConflict: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut sqlite3_changeset_iter,
            ) -> ::core::ffi::c_int,
        >,
        pCtx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_apply_v2(
        db: *mut sqlite3,
        nChangeset: ::core::ffi::c_int,
        pChangeset: *mut ::core::ffi::c_void,
        xFilter: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        xConflict: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut sqlite3_changeset_iter,
            ) -> ::core::ffi::c_int,
        >,
        pCtx: *mut ::core::ffi::c_void,
        ppRebase: *mut *mut ::core::ffi::c_void,
        pnRebase: *mut ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_apply_v3(
        db: *mut sqlite3,
        nChangeset: ::core::ffi::c_int,
        pChangeset: *mut ::core::ffi::c_void,
        xFilter: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut sqlite3_changeset_iter,
            ) -> ::core::ffi::c_int,
        >,
        xConflict: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut sqlite3_changeset_iter,
            ) -> ::core::ffi::c_int,
        >,
        pCtx: *mut ::core::ffi::c_void,
        ppRebase: *mut *mut ::core::ffi::c_void,
        pnRebase: *mut ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3rebaser_create(ppNew: *mut *mut sqlite3_rebaser) -> ::core::ffi::c_int;
    fn sqlite3rebaser_configure(
        _: *mut sqlite3_rebaser,
        nRebase: ::core::ffi::c_int,
        pRebase: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3rebaser_rebase(
        _: *mut sqlite3_rebaser,
        nIn: ::core::ffi::c_int,
        pIn: *const ::core::ffi::c_void,
        pnOut: *mut ::core::ffi::c_int,
        ppOut: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3rebaser_delete(p: *mut sqlite3_rebaser);
    fn sqlite3changeset_apply_strm(
        db: *mut sqlite3,
        xInput: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pIn: *mut ::core::ffi::c_void,
        xFilter: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        xConflict: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut sqlite3_changeset_iter,
            ) -> ::core::ffi::c_int,
        >,
        pCtx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_apply_v2_strm(
        db: *mut sqlite3,
        xInput: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pIn: *mut ::core::ffi::c_void,
        xFilter: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        xConflict: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut sqlite3_changeset_iter,
            ) -> ::core::ffi::c_int,
        >,
        pCtx: *mut ::core::ffi::c_void,
        ppRebase: *mut *mut ::core::ffi::c_void,
        pnRebase: *mut ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_apply_v3_strm(
        db: *mut sqlite3,
        xInput: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pIn: *mut ::core::ffi::c_void,
        xFilter: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut sqlite3_changeset_iter,
            ) -> ::core::ffi::c_int,
        >,
        xConflict: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *mut sqlite3_changeset_iter,
            ) -> ::core::ffi::c_int,
        >,
        pCtx: *mut ::core::ffi::c_void,
        ppRebase: *mut *mut ::core::ffi::c_void,
        pnRebase: *mut ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_concat_strm(
        xInputA: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pInA: *mut ::core::ffi::c_void,
        xInputB: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pInB: *mut ::core::ffi::c_void,
        xOutput: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pOut: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_invert_strm(
        xInput: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pIn: *mut ::core::ffi::c_void,
        xOutput: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pOut: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_start_strm(
        pp: *mut *mut sqlite3_changeset_iter,
        xInput: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pIn: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3changeset_start_v2_strm(
        pp: *mut *mut sqlite3_changeset_iter,
        xInput: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pIn: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3session_changeset_strm(
        pSession: *mut sqlite3_session,
        xOutput: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pOut: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3session_patchset_strm(
        pSession: *mut sqlite3_session,
        xOutput: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pOut: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3rebaser_rebase_strm(
        pRebaser: *mut sqlite3_rebaser,
        xInput: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pIn: *mut ::core::ffi::c_void,
        xOutput: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pOut: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn sqlite3session_config(
        op: ::core::ffi::c_int,
        pArg: *mut ::core::ffi::c_void,
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
    fn Tcl_BackgroundError(interp: *mut Tcl_Interp);
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
    fn Tcl_ObjGetVar2(
        interp: *mut Tcl_Interp,
        part1Ptr: *mut Tcl_Obj,
        part2Ptr: *mut Tcl_Obj,
        flags: ::core::ffi::c_int,
    ) -> *mut Tcl_Obj;
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
    fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::core::ffi::c_char;
    fn Tcl_NewWideIntObj(wideValue: Tcl_WideInt) -> *mut Tcl_Obj;
}
pub type sqlite_int64 = ::core::ffi::c_longlong;
pub type sqlite3_int64 = sqlite_int64;
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
pub type u8_0 = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestSession {
    pub pSession: *mut sqlite3_session,
    pub interp: *mut Tcl_Interp,
    pub pFilterScript: *mut Tcl_Obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestStreamInput {
    pub nStream: ::core::ffi::c_int,
    pub aData: *mut ::core::ffi::c_uchar,
    pub nData: ::core::ffi::c_int,
    pub iData: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestSessionsBlob {
    pub p: *mut ::core::ffi::c_void,
    pub n: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjConfOpt {
    pub zName: *const ::core::ffi::c_char,
    pub opt: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SessionSubcmd {
    pub zSub: *const ::core::ffi::c_char,
    pub nArg: ::core::ffi::c_int,
    pub zMsg: *const ::core::ffi::c_char,
    pub iSub: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestConflictHandler {
    pub interp: *mut Tcl_Interp,
    pub pConflictScript: *mut Tcl_Obj,
    pub pFilterScript: *mut Tcl_Obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RebaseSubcmd {
    pub zSub: *const ::core::ffi::c_char,
    pub nArg: ::core::ffi::c_int,
    pub zMsg: *const ::core::ffi::c_char,
    pub iSub: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConfigOpt {
    pub zSub: *const ::core::ffi::c_char,
    pub op: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestChangegroup {
    pub pGrp: *mut sqlite3_changegroup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestChangeIter {
    pub pIter: *mut sqlite3_changeset_iter,
    pub in_0: TestStreamInput,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChangegroupCmd {
    pub zSub: *const ::core::ffi::c_char,
    pub nArg: ::core::ffi::c_int,
    pub zMsg: *const ::core::ffi::c_char,
    pub iSub: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cmd {
    pub zCmd: *const ::core::ffi::c_char,
    pub xProc: Option<Tcl_ObjCmdProc>,
}
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DELETE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SQLITE_INSERT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SQLITE_UPDATE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const SQLITE_INTEGER: ::core::ffi::c_int = 1;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2;
pub const SQLITE_NULL: ::core::ffi::c_int = 5;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3;
pub const SQLITE_SESSION_OBJCONFIG_SIZE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_SESSION_OBJCONFIG_ROWID: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_CHANGESETSTART_INVERT: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_CHANGESETAPPLY_NOSAVEPOINT: ::core::ffi::c_int = 0x1
    as ::core::ffi::c_int;
pub const SQLITE_CHANGESETAPPLY_INVERT: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SQLITE_CHANGESETAPPLY_IGNORENOOP: ::core::ffi::c_int = 0x4
    as ::core::ffi::c_int;
pub const SQLITE_CHANGESETAPPLY_FKNOACTION: ::core::ffi::c_int = 0x8
    as ::core::ffi::c_int;
pub const SQLITE_CHANGESET_DATA: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_CHANGESET_NOTFOUND: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_CHANGESET_CONFLICT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SQLITE_CHANGESET_CONSTRAINT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SQLITE_CHANGESET_FOREIGN_KEY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_CHANGESET_OMIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_CHANGESET_REPLACE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SQLITE_CHANGESET_ABORT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SQLITE_SESSION_CONFIG_STRMSIZE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>()
    as *mut ::core::ffi::c_void;
pub const TCL_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCL_ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCL_BREAK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const TCL_CONTINUE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TCL_EVAL_GLOBAL: ::core::ffi::c_int = 0x20000 as ::core::ffi::c_int;
pub const TCL_GLOBAL_ONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn dbHandleFromObj(
    mut interp: *mut Tcl_Interp,
    mut pObj: *mut Tcl_Obj,
    mut pDb: *mut *mut sqlite3,
) -> ::core::ffi::c_int {
    unsafe {
        let mut info: Tcl_CmdInfo = Tcl_CmdInfo {
            isNativeObjectProc: 0,
            objProc: None,
            objClientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            proc: None,
            clientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            deleteProc: None,
            deleteData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            namespacePtr: ::core::ptr::null_mut::<Tcl_Namespace>(),
        };
        if 0 as ::core::ffi::c_int
            == Tcl_GetCommandInfo(interp, Tcl_GetString(pObj), &raw mut info)
        {
            Tcl_AppendResult(
                interp,
                b"no such handle: \0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(pObj),
                NULL,
            );
            return TCL_ERROR;
        }
        *pDb = *(info.objClientData as *mut *mut sqlite3);
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sql_exec_changeset(
    mut db: *mut sqlite3,
    mut zSql: *const ::core::ffi::c_char,
    mut pnChangeset: *mut ::core::ffi::c_int,
    mut ppChangeset: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pSession: *mut sqlite3_session = ::core::ptr::null_mut::<
            sqlite3_session,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        let mut val: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        rc = sqlite3session_create(
            db,
            b"main\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut pSession,
        );
        sqlite3session_object_config(
            pSession,
            SQLITE_SESSION_OBJCONFIG_ROWID,
            &raw mut val as *mut ::core::ffi::c_void,
        );
        if rc == SQLITE_OK {
            rc = sqlite3session_attach(
                pSession,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        if rc == SQLITE_OK {
            rc = sqlite3_exec(
                db,
                zSql,
                None,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
        }
        if rc == SQLITE_OK {
            rc = sqlite3session_changeset(pSession, pnChangeset, ppChangeset);
        }
        sqlite3session_delete(pSession);
        return rc;
    }
}
unsafe extern "C" fn test_sql_exec_changeset(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut zSql: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut pChangeset: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut nChangeset: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB SQL\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if dbHandleFromObj(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut db,
        ) != 0
        {
            return TCL_ERROR;
        }
        zSql = Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize))
            as *const ::core::ffi::c_char;
        rc = sql_exec_changeset(db, zSql, &raw mut nChangeset, &raw mut pChangeset);
        if rc != SQLITE_OK {
            Tcl_ResetResult(interp);
            Tcl_AppendResult(
                interp,
                b"error in sql_exec_changeset()\0".as_ptr()
                    as *const ::core::ffi::c_char,
                NULL,
            );
            return TCL_ERROR;
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewByteArrayObj(pChangeset as *const ::core::ffi::c_uchar, nChangeset),
        );
        sqlite3_free(pChangeset);
        return TCL_OK;
    }
}
pub const SESSION_STREAM_TCL_VAR: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<
        [u8; 23],
        [::core::ffi::c_char; 23],
    >(*b"sqlite3session_streams\0")
};
unsafe extern "C" fn test_tcl_integer(
    mut interp: *mut Tcl_Interp,
    mut zVar: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pObj: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut iVal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pName: *mut Tcl_Obj = Tcl_NewStringObj(zVar, -1 as ::core::ffi::c_int);
        (*pName).refCount += 1;
        pObj = Tcl_ObjGetVar2(
            interp,
            pName,
            ::core::ptr::null_mut::<Tcl_Obj>(),
            TCL_GLOBAL_ONLY,
        );
        let mut _objPtr: *mut Tcl_Obj = pName;
        let c2rust_fresh0 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh0 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        if !pObj.is_null() {
            Tcl_GetIntFromObj(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pObj,
                &raw mut iVal,
            );
        }
        return iVal;
    }
}
unsafe extern "C" fn test_session_error(
    mut interp: *mut Tcl_Interp,
    mut rc: ::core::ffi::c_int,
    mut zErr: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        unsafe extern "C" {
            #[link_name = "sqlite3ErrName"]
            fn sqlite3ErrName_0(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
        }
        Tcl_SetObjResult(
            interp,
            Tcl_NewStringObj(sqlite3ErrName_0(rc), -1 as ::core::ffi::c_int),
        );
        if !zErr.is_null() {
            Tcl_AppendResult(
                interp,
                b" - \0".as_ptr() as *const ::core::ffi::c_char,
                zErr,
                NULL,
            );
            sqlite3_free(zErr as *mut ::core::ffi::c_void);
        }
        return TCL_ERROR;
    }
}
unsafe extern "C" fn test_table_filter(
    mut pCtx: *mut ::core::ffi::c_void,
    mut zTbl: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut TestSession = pCtx as *mut TestSession;
        let mut pEval: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut rc: ::core::ffi::c_int = 0;
        let mut bRes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        pEval = Tcl_DuplicateObj((*p).pFilterScript);
        (*pEval).refCount += 1;
        rc = Tcl_ListObjAppendElement(
            (*p).interp,
            pEval,
            Tcl_NewStringObj(zTbl, -1 as ::core::ffi::c_int),
        );
        if rc == TCL_OK {
            rc = Tcl_EvalObjEx((*p).interp, pEval, TCL_EVAL_GLOBAL);
        }
        if rc == TCL_OK {
            rc = Tcl_GetBooleanFromObj(
                (*p).interp,
                Tcl_GetObjResult((*p).interp),
                &raw mut bRes,
            );
        }
        if rc != TCL_OK {
            Tcl_BackgroundError((*p).interp);
        }
        let mut _objPtr: *mut Tcl_Obj = pEval;
        let c2rust_fresh1 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh1 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        return bRes;
    }
}
unsafe extern "C" fn testStreamOutput(
    mut pCtx: *mut ::core::ffi::c_void,
    mut pData: *const ::core::ffi::c_void,
    mut nData: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pBlob: *mut TestSessionsBlob = pCtx as *mut TestSessionsBlob;
        let mut pNew: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        pNew = sqlite3_realloc((*pBlob).p, (*pBlob).n + nData)
            as *mut ::core::ffi::c_char;
        if pNew.is_null() {
            return SQLITE_NOMEM;
        }
        (*pBlob).p = pNew as *mut ::core::ffi::c_void;
        memcpy(
            pNew.offset((*pBlob).n as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            pData,
            nData as size_t,
        );
        (*pBlob).n += nData;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn test_session_cmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut TestSession = clientData as *mut TestSession;
        let mut pSession: *mut sqlite3_session = (*p).pSession;
        static mut aSub: [SessionSubcmd; 13] = [
            SessionSubcmd {
                zSub: b"attach\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"TABLE\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            SessionSubcmd {
                zSub: b"changeset\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            SessionSubcmd {
                zSub: b"delete\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            SessionSubcmd {
                zSub: b"enable\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"BOOL\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            SessionSubcmd {
                zSub: b"indirect\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"BOOL\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            SessionSubcmd {
                zSub: b"isempty\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            SessionSubcmd {
                zSub: b"table_filter\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"SCRIPT\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            SessionSubcmd {
                zSub: b"patchset\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            SessionSubcmd {
                zSub: b"diff\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 2 as ::core::ffi::c_int,
                zMsg: b"FROMDB TBL\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            SessionSubcmd {
                zSub: b"memory_used\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            SessionSubcmd {
                zSub: b"changeset_size\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            SessionSubcmd {
                zSub: b"object_config\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 2 as ::core::ffi::c_int,
                zMsg: b"OPTION INTEGER\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            SessionSubcmd {
                zSub: ::core::ptr::null::<::core::ffi::c_char>(),
                nArg: 0,
                zMsg: ::core::ptr::null::<::core::ffi::c_char>(),
                iSub: 0,
            },
        ];
        let mut iSub: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SUBCOMMAND ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        rc = Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut aSub as *mut SessionSubcmd as *const ::core::ffi::c_void,
            ::core::mem::size_of::<SessionSubcmd>() as ::core::ffi::c_int,
            b"sub-command\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iSub,
        );
        if rc != TCL_OK {
            return rc;
        }
        if objc != 2 as ::core::ffi::c_int + aSub[iSub as usize].nArg {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                aSub[iSub as usize].zMsg,
            );
            return TCL_ERROR;
        }
        let mut c2rust_current_block_78: u64;
        match iSub {
            0 => {
                let mut zArg: *mut ::core::ffi::c_char = Tcl_GetString(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                );
                if *zArg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '*' as i32
                    && *zArg.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == '\0' as i32
                {
                    zArg = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                rc = sqlite3session_attach(pSession, zArg);
                if rc != SQLITE_OK {
                    return test_session_error(
                        interp,
                        rc,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                }
                c2rust_current_block_78 = 3736434875406665187;
            }
            7 => {
                c2rust_current_block_78 = 9606288038608642794;
            }
            1 => {
                c2rust_current_block_78 = 9606288038608642794;
            }
            2 => {
                Tcl_DeleteCommand(
                    interp,
                    Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                );
                c2rust_current_block_78 = 3736434875406665187;
            }
            3 => {
                let mut val: ::core::ffi::c_int = 0;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut val,
                ) != 0
                {
                    return TCL_ERROR;
                }
                val = sqlite3session_enable(pSession, val);
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewIntObj((val != 0 as ::core::ffi::c_int) as ::core::ffi::c_int),
                );
                c2rust_current_block_78 = 3736434875406665187;
            }
            4 => {
                let mut val_0: ::core::ffi::c_int = 0;
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut val_0,
                ) != 0
                {
                    return TCL_ERROR;
                }
                val_0 = sqlite3session_indirect(pSession, val_0);
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewIntObj(
                        (val_0 != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    ),
                );
                c2rust_current_block_78 = 3736434875406665187;
            }
            5 => {
                let mut val_1: ::core::ffi::c_int = 0;
                val_1 = sqlite3session_isempty(pSession);
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewIntObj(
                        (val_1 != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    ),
                );
                c2rust_current_block_78 = 3736434875406665187;
            }
            6 => {
                if !(*p).pFilterScript.is_null() {
                    let mut _objPtr: *mut Tcl_Obj = (*p).pFilterScript;
                    let c2rust_fresh2 = (*_objPtr).refCount;
                    (*_objPtr).refCount = (*_objPtr).refCount - 1;
                    if c2rust_fresh2 <= 1 as ::core::ffi::c_int {
                        TclFreeObj(_objPtr);
                    }
                }
                (*p).interp = interp;
                (*p).pFilterScript = Tcl_DuplicateObj(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                );
                (*(*p).pFilterScript).refCount += 1;
                sqlite3session_table_filter(
                    pSession,
                    Some(
                        test_table_filter
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                    ),
                    clientData,
                );
                c2rust_current_block_78 = 3736434875406665187;
            }
            8 => {
                let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                rc = sqlite3session_diff(
                    pSession,
                    Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                    Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize)),
                    &raw mut zErr,
                );
                if rc != 0 {
                    return test_session_error(interp, rc, zErr);
                }
                c2rust_current_block_78 = 3736434875406665187;
            }
            9 => {
                let mut nMalloc: sqlite3_int64 = sqlite3session_memory_used(pSession);
                Tcl_SetObjResult(interp, Tcl_NewWideIntObj(nMalloc as Tcl_WideInt));
                c2rust_current_block_78 = 3736434875406665187;
            }
            10 => {
                let mut nSize: sqlite3_int64 = sqlite3session_changeset_size(pSession);
                Tcl_SetObjResult(interp, Tcl_NewWideIntObj(nSize as Tcl_WideInt));
                c2rust_current_block_78 = 3736434875406665187;
            }
            11 => {
                let mut aOpt: [ObjConfOpt; 3] = [
                    ObjConfOpt {
                        zName: b"size\0".as_ptr() as *const ::core::ffi::c_char,
                        opt: SQLITE_SESSION_OBJCONFIG_SIZE,
                    },
                    ObjConfOpt {
                        zName: b"rowid\0".as_ptr() as *const ::core::ffi::c_char,
                        opt: SQLITE_SESSION_OBJCONFIG_ROWID,
                    },
                    ObjConfOpt {
                        zName: ::core::ptr::null::<::core::ffi::c_char>(),
                        opt: 0 as ::core::ffi::c_int,
                    },
                ];
                let mut sz: ::core::ffi::c_int = ::core::mem::size_of::<ObjConfOpt>()
                    as ::core::ffi::c_int;
                let mut iArg: ::core::ffi::c_int = 0;
                let mut iOpt: ::core::ffi::c_int = 0;
                if Tcl_GetIndexFromObjStruct(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut aOpt as *mut ObjConfOpt as *const ::core::ffi::c_void,
                    sz,
                    b"option\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                    &raw mut iOpt,
                ) != 0
                {
                    return TCL_ERROR;
                }
                if Tcl_GetIntFromObj(
                    interp,
                    *objv.offset(3 as ::core::ffi::c_int as isize),
                    &raw mut iArg,
                ) != 0
                {
                    return TCL_ERROR;
                }
                rc = sqlite3session_object_config(
                    pSession,
                    aOpt[iOpt as usize].opt,
                    &raw mut iArg as *mut ::core::ffi::c_void,
                );
                if rc != SQLITE_OK {
                    Tcl_SetObjResult(
                        interp,
                        Tcl_NewStringObj(sqlite3ErrName(rc), -1 as ::core::ffi::c_int),
                    );
                } else {
                    Tcl_SetObjResult(interp, Tcl_NewIntObj(iArg));
                }
                c2rust_current_block_78 = 3736434875406665187;
            }
            _ => {
                c2rust_current_block_78 = 3736434875406665187;
            }
        }
        match c2rust_current_block_78 {
            9606288038608642794 => {
                let mut o: TestSessionsBlob = TestSessionsBlob {
                    p: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    n: 0 as ::core::ffi::c_int,
                };
                if test_tcl_integer(interp, SESSION_STREAM_TCL_VAR.as_ptr()) != 0 {
                    let mut pCtx: *mut ::core::ffi::c_void = &raw mut o
                        as *mut ::core::ffi::c_void;
                    if iSub == 7 as ::core::ffi::c_int {
                        rc = sqlite3session_patchset_strm(
                            pSession,
                            Some(
                                testStreamOutput
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const ::core::ffi::c_void,
                                        ::core::ffi::c_int,
                                    ) -> ::core::ffi::c_int,
                            ),
                            pCtx,
                        );
                    } else {
                        rc = sqlite3session_changeset_strm(
                            pSession,
                            Some(
                                testStreamOutput
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const ::core::ffi::c_void,
                                        ::core::ffi::c_int,
                                    ) -> ::core::ffi::c_int,
                            ),
                            pCtx,
                        );
                    }
                } else if iSub == 7 as ::core::ffi::c_int {
                    rc = sqlite3session_patchset(pSession, &raw mut o.n, &raw mut o.p);
                } else {
                    rc = sqlite3session_changeset(pSession, &raw mut o.n, &raw mut o.p);
                }
                if rc == SQLITE_OK {
                    Tcl_SetObjResult(
                        interp,
                        Tcl_NewByteArrayObj(o.p as *const ::core::ffi::c_uchar, o.n),
                    );
                }
                sqlite3_free(o.p);
                if rc != SQLITE_OK {
                    return test_session_error(
                        interp,
                        rc,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                }
            }
            _ => {}
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_session_del(mut clientData: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut TestSession = clientData as *mut TestSession;
        if !(*p).pFilterScript.is_null() {
            let mut _objPtr: *mut Tcl_Obj = (*p).pFilterScript;
            let c2rust_fresh3 = (*_objPtr).refCount;
            (*_objPtr).refCount = (*_objPtr).refCount - 1;
            if c2rust_fresh3 <= 1 as ::core::ffi::c_int {
                TclFreeObj(_objPtr);
            }
        }
        sqlite3session_delete((*p).pSession);
        Tcl_Free(p as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn test_sqlite3session(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut info: Tcl_CmdInfo = Tcl_CmdInfo {
            isNativeObjectProc: 0,
            objProc: None,
            objClientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            proc: None,
            clientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            deleteProc: None,
            deleteData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            namespacePtr: ::core::ptr::null_mut::<Tcl_Namespace>(),
        };
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut TestSession = ::core::ptr::null_mut::<TestSession>();
        let mut iArg: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"CMD DB-HANDLE DB-NAME\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if 0 as ::core::ffi::c_int
            == Tcl_GetCommandInfo(
                interp,
                Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                &raw mut info,
            )
        {
            Tcl_AppendResult(
                interp,
                b"no such handle: \0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                NULL,
            );
            return TCL_ERROR;
        }
        db = *(info.objClientData as *mut *mut sqlite3);
        p = Tcl_Alloc(::core::mem::size_of::<TestSession>() as ::core::ffi::c_uint)
            as *mut ::core::ffi::c_void as *mut TestSession;
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<TestSession>() as size_t,
        );
        rc = sqlite3session_create(
            db,
            Tcl_GetString(*objv.offset(3 as ::core::ffi::c_int as isize)),
            &raw mut (*p).pSession,
        );
        if rc != SQLITE_OK {
            Tcl_Free(p as *mut ::core::ffi::c_char);
            return test_session_error(
                interp,
                rc,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
        }
        sqlite3session_object_config(
            (*p).pSession,
            SQLITE_SESSION_OBJCONFIG_SIZE,
            &raw mut iArg as *mut ::core::ffi::c_void,
        );
        iArg = 1 as ::core::ffi::c_int;
        sqlite3session_object_config(
            (*p).pSession,
            SQLITE_SESSION_OBJCONFIG_SIZE,
            &raw mut iArg as *mut ::core::ffi::c_void,
        );
        Tcl_CreateObjCommand(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            Some(
                test_session_cmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            p as ClientData,
            Some(
                test_session_del as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
        );
        Tcl_SetObjResult(interp, *objv.offset(1 as ::core::ffi::c_int as isize));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_append_value(
    mut pList: *mut Tcl_Obj,
    mut pVal: *mut sqlite3_value,
) {
    unsafe {
        if pVal.is_null() {
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pList,
                Tcl_NewObj(),
            );
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pList,
                Tcl_NewObj(),
            );
        } else {
            let mut pObj: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
            match sqlite3_value_type(pVal) {
                SQLITE_NULL => {
                    Tcl_ListObjAppendElement(
                        ::core::ptr::null_mut::<Tcl_Interp>(),
                        pList,
                        Tcl_NewStringObj(
                            b"n\0".as_ptr() as *const ::core::ffi::c_char,
                            1 as ::core::ffi::c_int,
                        ),
                    );
                    pObj = Tcl_NewObj();
                }
                SQLITE_INTEGER => {
                    Tcl_ListObjAppendElement(
                        ::core::ptr::null_mut::<Tcl_Interp>(),
                        pList,
                        Tcl_NewStringObj(
                            b"i\0".as_ptr() as *const ::core::ffi::c_char,
                            1 as ::core::ffi::c_int,
                        ),
                    );
                    pObj = Tcl_NewWideIntObj(sqlite3_value_int64(pVal) as Tcl_WideInt);
                }
                SQLITE_FLOAT => {
                    Tcl_ListObjAppendElement(
                        ::core::ptr::null_mut::<Tcl_Interp>(),
                        pList,
                        Tcl_NewStringObj(
                            b"f\0".as_ptr() as *const ::core::ffi::c_char,
                            1 as ::core::ffi::c_int,
                        ),
                    );
                    pObj = Tcl_NewDoubleObj(sqlite3_value_double(pVal));
                }
                SQLITE_TEXT => {
                    let mut z: *const ::core::ffi::c_char = sqlite3_value_blob(pVal)
                        as *mut ::core::ffi::c_char;
                    let mut n: ::core::ffi::c_int = sqlite3_value_bytes(pVal);
                    Tcl_ListObjAppendElement(
                        ::core::ptr::null_mut::<Tcl_Interp>(),
                        pList,
                        Tcl_NewStringObj(
                            b"t\0".as_ptr() as *const ::core::ffi::c_char,
                            1 as ::core::ffi::c_int,
                        ),
                    );
                    pObj = Tcl_NewStringObj(z, n);
                }
                _ => {
                    Tcl_ListObjAppendElement(
                        ::core::ptr::null_mut::<Tcl_Interp>(),
                        pList,
                        Tcl_NewStringObj(
                            b"b\0".as_ptr() as *const ::core::ffi::c_char,
                            1 as ::core::ffi::c_int,
                        ),
                    );
                    pObj = Tcl_NewByteArrayObj(
                        sqlite3_value_blob(pVal) as *const ::core::ffi::c_uchar,
                        sqlite3_value_bytes(pVal),
                    );
                }
            }
            Tcl_ListObjAppendElement(::core::ptr::null_mut::<Tcl_Interp>(), pList, pObj);
        };
    }
}
unsafe extern "C" fn test_obj_eq_string(
    mut p: *mut Tcl_Obj,
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut n: ::core::ffi::c_int = 0;
        let mut nObj: ::core::ffi::c_int = 0;
        let mut zObj: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        n = strlen(z) as ::core::ffi::c_int;
        zObj = Tcl_GetStringFromObj(p, &raw mut nObj);
        return (nObj == n
            && (n == 0 as ::core::ffi::c_int
                || 0 as ::core::ffi::c_int
                    == memcmp(
                        zObj as *const ::core::ffi::c_void,
                        z as *const ::core::ffi::c_void,
                        n as size_t,
                    ))) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn testIterData(
    mut pIter: *mut sqlite3_changeset_iter,
) -> *mut Tcl_Obj {
    unsafe {
        let mut pVar: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut nCol: ::core::ffi::c_int = 0;
        let mut nCol2: ::core::ffi::c_int = 0;
        let mut op: ::core::ffi::c_int = 0;
        let mut zTab: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut pOld: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pNew: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut bIndirect: ::core::ffi::c_int = 0;
        let mut zPK: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut abPK: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
            ::core::ffi::c_uchar,
        >();
        let mut i: ::core::ffi::c_int = 0;
        sqlite3changeset_op(
            pIter,
            &raw mut zTab,
            &raw mut nCol,
            &raw mut op,
            &raw mut bIndirect,
        );
        pVar = Tcl_NewObj();
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pVar,
            Tcl_NewStringObj(
                if op == SQLITE_INSERT {
                    b"INSERT\0".as_ptr() as *const ::core::ffi::c_char
                } else if op == SQLITE_UPDATE {
                    b"UPDATE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"DELETE\0".as_ptr() as *const ::core::ffi::c_char
                },
                -1 as ::core::ffi::c_int,
            ),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pVar,
            Tcl_NewStringObj(zTab, -1 as ::core::ffi::c_int),
        );
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pVar,
            Tcl_NewIntObj((bIndirect != 0 as ::core::ffi::c_int) as ::core::ffi::c_int),
        );
        zPK = Tcl_Alloc((nCol + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_char;
        memset(
            zPK as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (nCol + 1 as ::core::ffi::c_int) as size_t,
        );
        sqlite3changeset_pk(pIter, &raw mut abPK, &raw mut nCol2);
        i = 0 as ::core::ffi::c_int;
        while i < nCol {
            *zPK.offset(i as isize) = (if *abPK.offset(i as isize) as ::core::ffi::c_int
                != 0
            {
                'X' as i32
            } else {
                '.' as i32
            }) as ::core::ffi::c_char;
            i += 1;
        }
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pVar,
            Tcl_NewStringObj(zPK, -1 as ::core::ffi::c_int),
        );
        Tcl_Free(zPK);
        pOld = Tcl_NewObj();
        if op != SQLITE_INSERT {
            i = 0 as ::core::ffi::c_int;
            while i < nCol {
                let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                sqlite3changeset_old(pIter, i, &raw mut pVal);
                test_append_value(pOld, pVal);
                i += 1;
            }
        }
        pNew = Tcl_NewObj();
        if op != SQLITE_DELETE {
            i = 0 as ::core::ffi::c_int;
            while i < nCol {
                let mut pVal_0: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                sqlite3changeset_new(pIter, i, &raw mut pVal_0);
                test_append_value(pNew, pVal_0);
                i += 1;
            }
        }
        Tcl_ListObjAppendElement(::core::ptr::null_mut::<Tcl_Interp>(), pVar, pOld);
        Tcl_ListObjAppendElement(::core::ptr::null_mut::<Tcl_Interp>(), pVar, pNew);
        return pVar;
    }
}
unsafe extern "C" fn test_filter_handler(
    mut pCtx: *mut ::core::ffi::c_void,
    mut zTab: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut TestConflictHandler = pCtx as *mut TestConflictHandler;
        let mut res: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut pEval: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut interp: *mut Tcl_Interp = (*p).interp;
        pEval = Tcl_DuplicateObj((*p).pFilterScript);
        (*pEval).refCount += 1;
        if TCL_OK
            != Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pEval,
                Tcl_NewStringObj(zTab, -1 as ::core::ffi::c_int),
            ) || TCL_OK != Tcl_EvalObjEx(interp, pEval, TCL_EVAL_GLOBAL)
            || TCL_OK
                != Tcl_GetIntFromObj(interp, Tcl_GetObjResult(interp), &raw mut res)
        {
            Tcl_BackgroundError(interp);
        }
        let mut _objPtr: *mut Tcl_Obj = pEval;
        let c2rust_fresh4 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh4 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        return res;
    }
}
unsafe extern "C" fn test_filter_v3_handler(
    mut pCtx: *mut ::core::ffi::c_void,
    mut pIter: *mut sqlite3_changeset_iter,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut TestConflictHandler = pCtx as *mut TestConflictHandler;
        let mut res: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut pEval: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut interp: *mut Tcl_Interp = (*p).interp;
        pEval = Tcl_DuplicateObj((*p).pFilterScript);
        (*pEval).refCount += 1;
        Tcl_ListObjAppendElement(
            ::core::ptr::null_mut::<Tcl_Interp>(),
            pEval,
            testIterData(pIter),
        );
        if TCL_OK != Tcl_EvalObjEx(interp, pEval, TCL_EVAL_GLOBAL)
            || TCL_OK
                != Tcl_GetIntFromObj(interp, Tcl_GetObjResult(interp), &raw mut res)
        {
            Tcl_BackgroundError(interp);
        }
        let mut _objPtr: *mut Tcl_Obj = pEval;
        let c2rust_fresh5 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh5 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        return res;
    }
}
unsafe extern "C" fn test_conflict_handler(
    mut pCtx: *mut ::core::ffi::c_void,
    mut eConf: ::core::ffi::c_int,
    mut pIter: *mut sqlite3_changeset_iter,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut TestConflictHandler = pCtx as *mut TestConflictHandler;
        let mut pEval: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut interp: *mut Tcl_Interp = (*p).interp;
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut op: ::core::ffi::c_int = 0;
        let mut zTab: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nCol: ::core::ffi::c_int = 0;
        pEval = Tcl_DuplicateObj((*p).pConflictScript);
        (*pEval).refCount += 1;
        sqlite3changeset_op(
            pIter,
            &raw mut zTab,
            &raw mut nCol,
            &raw mut op,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if eConf == SQLITE_CHANGESET_FOREIGN_KEY {
            let mut nFk: ::core::ffi::c_int = 0;
            sqlite3changeset_fk_conflicts(pIter, &raw mut nFk);
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pEval,
                Tcl_NewStringObj(
                    b"FOREIGN_KEY\0".as_ptr() as *const ::core::ffi::c_char,
                    -1 as ::core::ffi::c_int,
                ),
            );
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pEval,
                Tcl_NewIntObj(nFk),
            );
        } else {
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pEval,
                Tcl_NewStringObj(
                    if op == SQLITE_INSERT {
                        b"INSERT\0".as_ptr() as *const ::core::ffi::c_char
                    } else if op == SQLITE_UPDATE {
                        b"UPDATE\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"DELETE\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    -1 as ::core::ffi::c_int,
                ),
            );
            Tcl_ListObjAppendElement(
                ::core::ptr::null_mut::<Tcl_Interp>(),
                pEval,
                Tcl_NewStringObj(zTab, -1 as ::core::ffi::c_int),
            );
            match eConf {
                SQLITE_CHANGESET_DATA => {
                    Tcl_ListObjAppendElement(
                        interp,
                        pEval,
                        Tcl_NewStringObj(
                            b"DATA\0".as_ptr() as *const ::core::ffi::c_char,
                            -1 as ::core::ffi::c_int,
                        ),
                    );
                }
                SQLITE_CHANGESET_NOTFOUND => {
                    Tcl_ListObjAppendElement(
                        interp,
                        pEval,
                        Tcl_NewStringObj(
                            b"NOTFOUND\0".as_ptr() as *const ::core::ffi::c_char,
                            -1 as ::core::ffi::c_int,
                        ),
                    );
                }
                SQLITE_CHANGESET_CONFLICT => {
                    Tcl_ListObjAppendElement(
                        interp,
                        pEval,
                        Tcl_NewStringObj(
                            b"CONFLICT\0".as_ptr() as *const ::core::ffi::c_char,
                            -1 as ::core::ffi::c_int,
                        ),
                    );
                }
                SQLITE_CHANGESET_CONSTRAINT => {
                    Tcl_ListObjAppendElement(
                        interp,
                        pEval,
                        Tcl_NewStringObj(
                            b"CONSTRAINT\0".as_ptr() as *const ::core::ffi::c_char,
                            -1 as ::core::ffi::c_int,
                        ),
                    );
                }
                _ => {}
            }
            if op != SQLITE_INSERT {
                let mut i: ::core::ffi::c_int = 0;
                let mut pOld: *mut Tcl_Obj = Tcl_NewObj();
                i = 0 as ::core::ffi::c_int;
                while i < nCol {
                    let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<
                        sqlite3_value,
                    >();
                    sqlite3changeset_old(pIter, i, &raw mut pVal);
                    test_append_value(pOld, pVal);
                    i += 1;
                }
                Tcl_ListObjAppendElement(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    pEval,
                    pOld,
                );
            }
            if op != SQLITE_DELETE {
                let mut i_0: ::core::ffi::c_int = 0;
                let mut pNew: *mut Tcl_Obj = Tcl_NewObj();
                i_0 = 0 as ::core::ffi::c_int;
                while i_0 < nCol {
                    let mut pVal_0: *mut sqlite3_value = ::core::ptr::null_mut::<
                        sqlite3_value,
                    >();
                    sqlite3changeset_new(pIter, i_0, &raw mut pVal_0);
                    test_append_value(pNew, pVal_0);
                    i_0 += 1;
                }
                Tcl_ListObjAppendElement(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    pEval,
                    pNew,
                );
            }
            if eConf == SQLITE_CHANGESET_DATA || eConf == SQLITE_CHANGESET_CONFLICT {
                let mut i_1: ::core::ffi::c_int = 0;
                let mut pConflict: *mut Tcl_Obj = Tcl_NewObj();
                i_1 = 0 as ::core::ffi::c_int;
                while i_1 < nCol {
                    let mut rc: ::core::ffi::c_int = 0;
                    let mut pVal_1: *mut sqlite3_value = ::core::ptr::null_mut::<
                        sqlite3_value,
                    >();
                    rc = sqlite3changeset_conflict(pIter, i_1, &raw mut pVal_1);
                    test_append_value(pConflict, pVal_1);
                    i_1 += 1;
                }
                Tcl_ListObjAppendElement(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    pEval,
                    pConflict,
                );
            }
            if eConf == SQLITE_CHANGESET_CONSTRAINT || eConf == SQLITE_CHANGESET_NOTFOUND
            {
                let mut pVal_2: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                let mut rc_0: ::core::ffi::c_int = sqlite3changeset_conflict(
                    pIter,
                    0 as ::core::ffi::c_int,
                    &raw mut pVal_2,
                );
            } else {
                let mut pVal_3: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                let mut rc_1: ::core::ffi::c_int = sqlite3changeset_conflict(
                    pIter,
                    -1 as ::core::ffi::c_int,
                    &raw mut pVal_3,
                );
                rc_1 = sqlite3changeset_conflict(pIter, nCol, &raw mut pVal_3);
            }
            if op == SQLITE_DELETE {
                let mut pVal_4: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                let mut rc_2: ::core::ffi::c_int = sqlite3changeset_new(
                    pIter,
                    0 as ::core::ffi::c_int,
                    &raw mut pVal_4,
                );
            } else {
                let mut pVal_5: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                let mut rc_3: ::core::ffi::c_int = sqlite3changeset_new(
                    pIter,
                    -1 as ::core::ffi::c_int,
                    &raw mut pVal_5,
                );
                rc_3 = sqlite3changeset_new(pIter, nCol, &raw mut pVal_5);
            }
            if op == SQLITE_INSERT {
                let mut pVal_6: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                let mut rc_4: ::core::ffi::c_int = sqlite3changeset_old(
                    pIter,
                    0 as ::core::ffi::c_int,
                    &raw mut pVal_6,
                );
            } else {
                let mut pVal_7: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                let mut rc_5: ::core::ffi::c_int = sqlite3changeset_old(
                    pIter,
                    -1 as ::core::ffi::c_int,
                    &raw mut pVal_7,
                );
                rc_5 = sqlite3changeset_old(pIter, nCol, &raw mut pVal_7);
            }
            if eConf != SQLITE_CHANGESET_FOREIGN_KEY {
                let mut nDummy: ::core::ffi::c_int = 0;
                let mut rc_6: ::core::ffi::c_int = sqlite3changeset_fk_conflicts(
                    pIter,
                    &raw mut nDummy,
                );
            }
        }
        if TCL_OK != Tcl_EvalObjEx(interp, pEval, TCL_EVAL_GLOBAL) {
            Tcl_BackgroundError(interp);
        } else {
            let mut pRes: *mut Tcl_Obj = Tcl_GetObjResult(interp);
            if test_obj_eq_string(pRes, b"OMIT\0".as_ptr() as *const ::core::ffi::c_char)
                != 0
                || test_obj_eq_string(pRes, b"\0".as_ptr() as *const ::core::ffi::c_char)
                    != 0
            {
                ret = SQLITE_CHANGESET_OMIT;
            } else if test_obj_eq_string(
                pRes,
                b"REPLACE\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0
            {
                ret = SQLITE_CHANGESET_REPLACE;
            } else if test_obj_eq_string(
                pRes,
                b"ABORT\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0
            {
                ret = SQLITE_CHANGESET_ABORT;
            } else {
                Tcl_GetIntFromObj(
                    ::core::ptr::null_mut::<Tcl_Interp>(),
                    pRes,
                    &raw mut ret,
                );
            }
        }
        let mut _objPtr: *mut Tcl_Obj = pEval;
        let c2rust_fresh6 = (*_objPtr).refCount;
        (*_objPtr).refCount = (*_objPtr).refCount - 1;
        if c2rust_fresh6 <= 1 as ::core::ffi::c_int {
            TclFreeObj(_objPtr);
        }
        return ret;
    }
}
unsafe extern "C" fn replace_handler(
    mut pCtx: *mut ::core::ffi::c_void,
    mut eConf: ::core::ffi::c_int,
    mut pIter: *mut sqlite3_changeset_iter,
) -> ::core::ffi::c_int {
    unsafe {
        let mut op: ::core::ffi::c_int = 0;
        let mut zTab: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nCol: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        sqlite3changeset_op(
            pIter,
            &raw mut zTab,
            &raw mut nCol,
            &raw mut op,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if op != SQLITE_INSERT {
            i = 0 as ::core::ffi::c_int;
            while i < nCol {
                let mut pVal: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                sqlite3changeset_old(pIter, i, &raw mut pVal);
                sqlite3_value_text16(pVal);
                i += 1;
            }
        }
        if op != SQLITE_DELETE {
            i = 0 as ::core::ffi::c_int;
            while i < nCol {
                let mut pVal_0: *mut sqlite3_value = ::core::ptr::null_mut::<
                    sqlite3_value,
                >();
                sqlite3changeset_new(pIter, i, &raw mut pVal_0);
                sqlite3_value_text16(pVal_0);
                i += 1;
            }
        }
        if eConf == SQLITE_CHANGESET_DATA {
            return SQLITE_CHANGESET_REPLACE;
        }
        return SQLITE_CHANGESET_OMIT;
    }
}
unsafe extern "C" fn testStreamInput(
    mut pCtx: *mut ::core::ffi::c_void,
    mut pData: *mut ::core::ffi::c_void,
    mut pnData: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut TestStreamInput = pCtx as *mut TestStreamInput;
        let mut nReq: ::core::ffi::c_int = *pnData;
        let mut nRem: ::core::ffi::c_int = (*p).nData - (*p).iData;
        let mut nRet: ::core::ffi::c_int = (*p).nStream;
        let mut pAlloc: *mut ::core::ffi::c_void = sqlite3_malloc(
            10 as ::core::ffi::c_int,
        );
        if pAlloc.is_null() {
            return SQLITE_NOMEM;
        }
        sqlite3_free(pAlloc);
        if nRet > nReq {
            nRet = nReq;
        }
        if nRet > nRem {
            nRet = nRem;
        }
        if nRet > 0 as ::core::ffi::c_int {
            memcpy(
                pData,
                (*p).aData.offset((*p).iData as isize) as *mut ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
                nRet as size_t,
            );
            (*p).iData += nRet;
        }
        *pnData = nRet;
        return SQLITE_OK;
    }
}
unsafe extern "C" fn testSqlite3changesetApply(
    mut iVersion: ::core::ffi::c_int,
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut info: Tcl_CmdInfo = Tcl_CmdInfo {
            isNativeObjectProc: 0,
            objProc: None,
            objClientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            proc: None,
            clientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            deleteProc: None,
            deleteData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            namespacePtr: ::core::ptr::null_mut::<Tcl_Namespace>(),
        };
        let mut rc: ::core::ffi::c_int = 0;
        let mut pChangeset: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut nChangeset: ::core::ffi::c_int = 0;
        let mut ctx: TestConflictHandler = TestConflictHandler {
            interp: ::core::ptr::null_mut::<Tcl_Interp>(),
            pConflictScript: ::core::ptr::null_mut::<Tcl_Obj>(),
            pFilterScript: ::core::ptr::null_mut::<Tcl_Obj>(),
        };
        let mut sStr: TestStreamInput = TestStreamInput {
            nStream: 0,
            aData: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            nData: 0,
            iData: 0,
        };
        let mut pRebase: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut nRebase: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        memset(
            &raw mut sStr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<TestStreamInput>() as size_t,
        );
        sStr.nStream = test_tcl_integer(interp, SESSION_STREAM_TCL_VAR.as_ptr());
        if iVersion == 2 as ::core::ffi::c_int || iVersion == 3 as ::core::ffi::c_int {
            while objc > 1 as ::core::ffi::c_int {
                let mut z1: *const ::core::ffi::c_char = Tcl_GetString(
                    *objv.offset(1 as ::core::ffi::c_int as isize),
                );
                let mut n: ::core::ffi::c_int = strlen(z1) as ::core::ffi::c_int;
                if n > 3 as ::core::ffi::c_int && n <= 12 as ::core::ffi::c_int
                    && 0 as ::core::ffi::c_int
                        == sqlite3_strnicmp(
                            b"-nosavepoint\0".as_ptr() as *const ::core::ffi::c_char,
                            z1,
                            n,
                        )
                {
                    flags |= SQLITE_CHANGESETAPPLY_NOSAVEPOINT;
                } else if n > 3 as ::core::ffi::c_int && n <= 9 as ::core::ffi::c_int
                    && 0 as ::core::ffi::c_int
                        == sqlite3_strnicmp(
                            b"-noaction\0".as_ptr() as *const ::core::ffi::c_char,
                            z1,
                            n,
                        )
                {
                    flags |= SQLITE_CHANGESETAPPLY_FKNOACTION;
                } else if n > 2 as ::core::ffi::c_int && n <= 7 as ::core::ffi::c_int
                    && 0 as ::core::ffi::c_int
                        == sqlite3_strnicmp(
                            b"-invert\0".as_ptr() as *const ::core::ffi::c_char,
                            z1,
                            n,
                        )
                {
                    flags |= SQLITE_CHANGESETAPPLY_INVERT;
                } else {
                    if !(n > 2 as ::core::ffi::c_int && n <= 11 as ::core::ffi::c_int
                        && 0 as ::core::ffi::c_int
                            == sqlite3_strnicmp(
                                b"-ignorenoop\0".as_ptr() as *const ::core::ffi::c_char,
                                z1,
                                n,
                            ))
                    {
                        break;
                    }
                    flags |= SQLITE_CHANGESETAPPLY_IGNORENOOP;
                }
                objc -= 1;
                objv = objv.offset(1);
            }
        }
        if objc != 4 as ::core::ffi::c_int && objc != 5 as ::core::ffi::c_int {
            let mut zMsg: *const ::core::ffi::c_char = ::core::ptr::null::<
                ::core::ffi::c_char,
            >();
            if iVersion == 2 as ::core::ffi::c_int || iVersion == 3 as ::core::ffi::c_int
            {
                zMsg = b"?-nosavepoint? ?-inverse? ?-ignorenoop? DB CHANGESET CONFLICT-SCRIPT ?FILTER-SCRIPT?\0"
                    .as_ptr() as *const ::core::ffi::c_char;
            } else {
                zMsg = b"DB CHANGESET CONFLICT-SCRIPT ?FILTER-SCRIPT?\0".as_ptr()
                    as *const ::core::ffi::c_char;
            }
            Tcl_WrongNumArgs(interp, 1 as ::core::ffi::c_int, objv, zMsg);
            return TCL_ERROR;
        }
        if 0 as ::core::ffi::c_int
            == Tcl_GetCommandInfo(
                interp,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                &raw mut info,
            )
        {
            Tcl_AppendResult(
                interp,
                b"no such handle: \0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                NULL,
            );
            return TCL_ERROR;
        }
        db = *(info.objClientData as *mut *mut sqlite3);
        pChangeset = Tcl_GetByteArrayFromObj(
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut nChangeset,
        ) as *mut ::core::ffi::c_void;
        ctx.pConflictScript = *objv.offset(3 as ::core::ffi::c_int as isize);
        ctx.pFilterScript = if objc == 5 as ::core::ffi::c_int {
            *objv.offset(4 as ::core::ffi::c_int as isize)
        } else {
            ::core::ptr::null_mut::<Tcl_Obj>()
        };
        ctx.interp = interp;
        if sStr.nStream == 0 as ::core::ffi::c_int {
            match iVersion {
                1 => {
                    rc = sqlite3changeset_apply(
                        db,
                        nChangeset,
                        pChangeset,
                        if objc == 5 as ::core::ffi::c_int {
                            Some(
                                test_filter_handler
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const ::core::ffi::c_char,
                                    ) -> ::core::ffi::c_int,
                            )
                        } else {
                            None
                        },
                        Some(
                            test_conflict_handler
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *mut sqlite3_changeset_iter,
                                ) -> ::core::ffi::c_int,
                        ),
                        &raw mut ctx as *mut ::core::ffi::c_void,
                    );
                }
                2 => {
                    rc = sqlite3changeset_apply_v2(
                        db,
                        nChangeset,
                        pChangeset,
                        if objc == 5 as ::core::ffi::c_int {
                            Some(
                                test_filter_handler
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const ::core::ffi::c_char,
                                    ) -> ::core::ffi::c_int,
                            )
                        } else {
                            None
                        },
                        Some(
                            test_conflict_handler
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *mut sqlite3_changeset_iter,
                                ) -> ::core::ffi::c_int,
                        ),
                        &raw mut ctx as *mut ::core::ffi::c_void,
                        &raw mut pRebase,
                        &raw mut nRebase,
                        flags,
                    );
                }
                3 => {
                    rc = sqlite3changeset_apply_v3(
                        db,
                        nChangeset,
                        pChangeset,
                        if objc == 5 as ::core::ffi::c_int {
                            Some(
                                test_filter_v3_handler
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *mut sqlite3_changeset_iter,
                                    ) -> ::core::ffi::c_int,
                            )
                        } else {
                            None
                        },
                        Some(
                            test_conflict_handler
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *mut sqlite3_changeset_iter,
                                ) -> ::core::ffi::c_int,
                        ),
                        &raw mut ctx as *mut ::core::ffi::c_void,
                        &raw mut pRebase,
                        &raw mut nRebase,
                        flags,
                    );
                }
                _ => {}
            }
        } else {
            sStr.aData = pChangeset as *mut ::core::ffi::c_uchar;
            sStr.nData = nChangeset;
            match iVersion {
                1 => {
                    rc = sqlite3changeset_apply_strm(
                        db,
                        Some(
                            testStreamInput
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut ::core::ffi::c_void,
                                    *mut ::core::ffi::c_int,
                                ) -> ::core::ffi::c_int,
                        ),
                        &raw mut sStr as *mut ::core::ffi::c_void,
                        if objc == 5 as ::core::ffi::c_int {
                            Some(
                                test_filter_handler
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const ::core::ffi::c_char,
                                    ) -> ::core::ffi::c_int,
                            )
                        } else {
                            None
                        },
                        Some(
                            test_conflict_handler
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *mut sqlite3_changeset_iter,
                                ) -> ::core::ffi::c_int,
                        ),
                        &raw mut ctx as *mut ::core::ffi::c_void,
                    );
                }
                2 => {
                    rc = sqlite3changeset_apply_v2_strm(
                        db,
                        Some(
                            testStreamInput
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut ::core::ffi::c_void,
                                    *mut ::core::ffi::c_int,
                                ) -> ::core::ffi::c_int,
                        ),
                        &raw mut sStr as *mut ::core::ffi::c_void,
                        if objc == 5 as ::core::ffi::c_int {
                            Some(
                                test_filter_handler
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *const ::core::ffi::c_char,
                                    ) -> ::core::ffi::c_int,
                            )
                        } else {
                            None
                        },
                        Some(
                            test_conflict_handler
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *mut sqlite3_changeset_iter,
                                ) -> ::core::ffi::c_int,
                        ),
                        &raw mut ctx as *mut ::core::ffi::c_void,
                        &raw mut pRebase,
                        &raw mut nRebase,
                        flags,
                    );
                }
                3 => {
                    rc = sqlite3changeset_apply_v3_strm(
                        db,
                        Some(
                            testStreamInput
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut ::core::ffi::c_void,
                                    *mut ::core::ffi::c_int,
                                ) -> ::core::ffi::c_int,
                        ),
                        &raw mut sStr as *mut ::core::ffi::c_void,
                        if objc == 5 as ::core::ffi::c_int {
                            Some(
                                test_filter_v3_handler
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        *mut sqlite3_changeset_iter,
                                    ) -> ::core::ffi::c_int,
                            )
                        } else {
                            None
                        },
                        Some(
                            test_conflict_handler
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                    *mut sqlite3_changeset_iter,
                                ) -> ::core::ffi::c_int,
                        ),
                        &raw mut ctx as *mut ::core::ffi::c_void,
                        &raw mut pRebase,
                        &raw mut nRebase,
                        flags,
                    );
                }
                _ => {}
            }
        }
        if rc != SQLITE_OK {
            return test_session_error(
                interp,
                rc,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            )
        } else {
            Tcl_ResetResult(interp);
            if (iVersion == 2 as ::core::ffi::c_int
                || iVersion == 3 as ::core::ffi::c_int) && !pRebase.is_null()
            {
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewByteArrayObj(pRebase as *const ::core::ffi::c_uchar, nRebase),
                );
            }
        }
        sqlite3_free(pRebase);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_sqlite3changeset_apply(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        return testSqlite3changesetApply(
            1 as ::core::ffi::c_int,
            clientData,
            interp,
            objc,
            objv,
        );
    }
}
unsafe extern "C" fn test_sqlite3changeset_apply_v2(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        return testSqlite3changesetApply(
            2 as ::core::ffi::c_int,
            clientData,
            interp,
            objc,
            objv,
        );
    }
}
unsafe extern "C" fn test_sqlite3changeset_apply_v3(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        return testSqlite3changesetApply(
            3 as ::core::ffi::c_int,
            clientData,
            interp,
            objc,
            objv,
        );
    }
}
unsafe extern "C" fn test_sqlite3changeset_apply_replace_all(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
        let mut info: Tcl_CmdInfo = Tcl_CmdInfo {
            isNativeObjectProc: 0,
            objProc: None,
            objClientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            proc: None,
            clientData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            deleteProc: None,
            deleteData: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            namespacePtr: ::core::ptr::null_mut::<Tcl_Namespace>(),
        };
        let mut rc: ::core::ffi::c_int = 0;
        let mut pChangeset: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut nChangeset: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"DB CHANGESET\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if 0 as ::core::ffi::c_int
            == Tcl_GetCommandInfo(
                interp,
                Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
                &raw mut info,
            )
        {
            Tcl_AppendResult(
                interp,
                b"no such handle: \0".as_ptr() as *const ::core::ffi::c_char,
                Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                NULL,
            );
            return TCL_ERROR;
        }
        db = *(info.objClientData as *mut *mut sqlite3);
        pChangeset = Tcl_GetByteArrayFromObj(
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut nChangeset,
        ) as *mut ::core::ffi::c_void;
        rc = sqlite3changeset_apply(
            db,
            nChangeset,
            pChangeset,
            None,
            Some(
                replace_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        ::core::ffi::c_int,
                        *mut sqlite3_changeset_iter,
                    ) -> ::core::ffi::c_int,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        if rc != SQLITE_OK {
            return test_session_error(
                interp,
                rc,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
        }
        Tcl_ResetResult(interp);
        return TCL_OK;
    }
}
unsafe extern "C" fn test_sqlite3changeset_invert(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut nn: ::core::ffi::c_int = 0;
        let mut sIn: TestStreamInput = TestStreamInput {
            nStream: 0,
            aData: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            nData: 0,
            iData: 0,
        };
        let mut sOut: TestSessionsBlob = TestSessionsBlob {
            p: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            n: 0,
        };
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"CHANGESET\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        memset(
            &raw mut sIn as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<TestStreamInput>() as size_t,
        );
        memset(
            &raw mut sOut as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<TestSessionsBlob>() as size_t,
        );
        sIn.nStream = test_tcl_integer(interp, SESSION_STREAM_TCL_VAR.as_ptr());
        sIn.aData = Tcl_GetByteArrayFromObj(
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut nn,
        );
        sIn.nData = nn;
        if sIn.nStream != 0 {
            rc = sqlite3changeset_invert_strm(
                Some(
                    testStreamInput
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                &raw mut sIn as *mut ::core::ffi::c_void,
                Some(
                    testStreamOutput
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                &raw mut sOut as *mut ::core::ffi::c_void,
            );
        } else {
            rc = sqlite3changeset_invert(
                sIn.nData,
                sIn.aData as *const ::core::ffi::c_void,
                &raw mut sOut.n,
                &raw mut sOut.p,
            );
        }
        if rc != SQLITE_OK {
            rc = test_session_error(
                interp,
                rc,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
        } else {
            Tcl_SetObjResult(
                interp,
                Tcl_NewByteArrayObj(sOut.p as *mut ::core::ffi::c_uchar, sOut.n),
            );
        }
        sqlite3_free(sOut.p);
        return rc;
    }
}
unsafe extern "C" fn test_sqlite3changeset_concat(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut nn: ::core::ffi::c_int = 0;
        let mut sLeft: TestStreamInput = TestStreamInput {
            nStream: 0,
            aData: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            nData: 0,
            iData: 0,
        };
        let mut sRight: TestStreamInput = TestStreamInput {
            nStream: 0,
            aData: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            nData: 0,
            iData: 0,
        };
        let mut sOut: TestSessionsBlob = TestSessionsBlob {
            p: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            n: 0 as ::core::ffi::c_int,
        };
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"LEFT RIGHT\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        memset(
            &raw mut sLeft as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<TestStreamInput>() as size_t,
        );
        memset(
            &raw mut sRight as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<TestStreamInput>() as size_t,
        );
        sLeft.aData = Tcl_GetByteArrayFromObj(
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut nn,
        );
        sLeft.nData = nn;
        sRight.aData = Tcl_GetByteArrayFromObj(
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut nn,
        );
        sRight.nData = nn;
        sLeft.nStream = test_tcl_integer(interp, SESSION_STREAM_TCL_VAR.as_ptr());
        sRight.nStream = sLeft.nStream;
        if sLeft.nStream > 0 as ::core::ffi::c_int {
            rc = sqlite3changeset_concat_strm(
                Some(
                    testStreamInput
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                &raw mut sLeft as *mut ::core::ffi::c_void,
                Some(
                    testStreamInput
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                &raw mut sRight as *mut ::core::ffi::c_void,
                Some(
                    testStreamOutput
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_void,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                &raw mut sOut as *mut ::core::ffi::c_void,
            );
        } else {
            rc = sqlite3changeset_concat(
                sLeft.nData,
                sLeft.aData as *mut ::core::ffi::c_void,
                sRight.nData,
                sRight.aData as *mut ::core::ffi::c_void,
                &raw mut sOut.n,
                &raw mut sOut.p,
            );
        }
        if rc != SQLITE_OK {
            rc = test_session_error(
                interp,
                rc,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
        } else {
            Tcl_SetObjResult(
                interp,
                Tcl_NewByteArrayObj(sOut.p as *mut ::core::ffi::c_uchar, sOut.n),
            );
        }
        sqlite3_free(sOut.p);
        return rc;
    }
}
unsafe extern "C" fn test_sqlite3session_foreach(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pChangeset: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut nChangeset: ::core::ffi::c_int = 0;
        let mut pIter: *mut sqlite3_changeset_iter = ::core::ptr::null_mut::<
            sqlite3_changeset_iter,
        >();
        let mut rc: ::core::ffi::c_int = 0;
        let mut pVarname: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pCS: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut pScript: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
        let mut isCheckNext: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut isInvert: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut sStr: TestStreamInput = TestStreamInput {
            nStream: 0,
            aData: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            nData: 0,
            iData: 0,
        };
        memset(
            &raw mut sStr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<TestStreamInput>() as size_t,
        );
        while objc > 1 as ::core::ffi::c_int {
            let mut zOpt: *mut ::core::ffi::c_char = Tcl_GetString(
                *objv.offset(1 as ::core::ffi::c_int as isize),
            );
            let mut nOpt: ::core::ffi::c_int = strlen(zOpt) as ::core::ffi::c_int;
            if *zOpt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '-' as i32
            {
                break;
            }
            if nOpt <= 7 as ::core::ffi::c_int
                && 0 as ::core::ffi::c_int
                    == sqlite3_strnicmp(
                        zOpt,
                        b"-invert\0".as_ptr() as *const ::core::ffi::c_char,
                        nOpt,
                    )
            {
                isInvert = 1 as ::core::ffi::c_int;
            } else {
                if !(nOpt <= 5 as ::core::ffi::c_int
                    && 0 as ::core::ffi::c_int
                        == sqlite3_strnicmp(
                            zOpt,
                            b"-next\0".as_ptr() as *const ::core::ffi::c_char,
                            nOpt,
                        ))
                {
                    break;
                }
                isCheckNext = 1 as ::core::ffi::c_int;
            }
            objv = objv.offset(1);
            objc -= 1;
        }
        if objc != 4 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?-next? ?-invert? VARNAME CHANGESET SCRIPT\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        pVarname = *objv.offset(1 as ::core::ffi::c_int as isize);
        pCS = *objv.offset(2 as ::core::ffi::c_int as isize);
        pScript = *objv.offset(3 as ::core::ffi::c_int as isize);
        pChangeset = Tcl_GetByteArrayFromObj(pCS, &raw mut nChangeset)
            as *mut ::core::ffi::c_void;
        sStr.nStream = test_tcl_integer(interp, SESSION_STREAM_TCL_VAR.as_ptr());
        if isInvert != 0 {
            let mut f: ::core::ffi::c_int = SQLITE_CHANGESETSTART_INVERT;
            if sStr.nStream == 0 as ::core::ffi::c_int {
                rc = sqlite3changeset_start_v2(
                    &raw mut pIter,
                    nChangeset,
                    pChangeset,
                    f,
                );
            } else {
                let mut pCtx: *mut ::core::ffi::c_void = &raw mut sStr
                    as *mut ::core::ffi::c_void;
                sStr.aData = pChangeset as *mut ::core::ffi::c_uchar;
                sStr.nData = nChangeset;
                rc = sqlite3changeset_start_v2_strm(
                    &raw mut pIter,
                    Some(
                        testStreamInput
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut ::core::ffi::c_void,
                                *mut ::core::ffi::c_int,
                            ) -> ::core::ffi::c_int,
                    ),
                    pCtx,
                    f,
                );
            }
        } else if sStr.nStream == 0 as ::core::ffi::c_int {
            rc = sqlite3changeset_start(&raw mut pIter, nChangeset, pChangeset);
        } else {
            sStr.aData = pChangeset as *mut ::core::ffi::c_uchar;
            sStr.nData = nChangeset;
            rc = sqlite3changeset_start_strm(
                &raw mut pIter,
                Some(
                    testStreamInput
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                &raw mut sStr as *mut ::core::ffi::c_void,
            );
        }
        if rc != SQLITE_OK {
            return test_session_error(
                interp,
                rc,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
        }
        while SQLITE_ROW == sqlite3changeset_next(pIter) {
            let mut pVar: *mut Tcl_Obj = ::core::ptr::null_mut::<Tcl_Obj>();
            pVar = testIterData(pIter);
            Tcl_ObjSetVar2(
                interp,
                pVarname,
                ::core::ptr::null_mut::<Tcl_Obj>(),
                pVar,
                0 as ::core::ffi::c_int,
            );
            rc = Tcl_EvalObjEx(interp, pScript, 0 as ::core::ffi::c_int);
            if rc != TCL_OK && rc != TCL_CONTINUE {
                sqlite3changeset_finalize(pIter);
                return if rc == TCL_BREAK { TCL_OK } else { rc };
            }
        }
        if isCheckNext != 0 {
            let mut rc2: ::core::ffi::c_int = sqlite3changeset_next(pIter);
            rc = sqlite3changeset_finalize(pIter);
        } else {
            rc = sqlite3changeset_finalize(pIter);
        }
        if rc != SQLITE_OK {
            return test_session_error(
                interp,
                rc,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_rebaser_cmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aSub: [RebaseSubcmd; 4] = [
            RebaseSubcmd {
                zSub: b"configure\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"REBASE-BLOB\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            RebaseSubcmd {
                zSub: b"delete\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            RebaseSubcmd {
                zSub: b"rebase\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"CHANGESET\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            RebaseSubcmd {
                zSub: ::core::ptr::null::<::core::ffi::c_char>(),
                nArg: 0,
                zMsg: ::core::ptr::null::<::core::ffi::c_char>(),
                iSub: 0,
            },
        ];
        let mut p: *mut sqlite3_rebaser = clientData as *mut sqlite3_rebaser;
        let mut iSub: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SUBCOMMAND ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        rc = Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut aSub as *mut RebaseSubcmd as *const ::core::ffi::c_void,
            ::core::mem::size_of::<RebaseSubcmd>() as ::core::ffi::c_int,
            b"sub-command\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iSub,
        );
        if rc != TCL_OK {
            return rc;
        }
        if objc != 2 as ::core::ffi::c_int + aSub[iSub as usize].nArg {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                aSub[iSub as usize].zMsg,
            );
            return TCL_ERROR;
        }
        match iSub {
            0 => {
                let mut nRebase: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut pRebase: *mut ::core::ffi::c_uchar = Tcl_GetByteArrayFromObj(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut nRebase,
                );
                rc = sqlite3rebaser_configure(
                    p,
                    nRebase,
                    pRebase as *const ::core::ffi::c_void,
                );
            }
            1 => {
                Tcl_DeleteCommand(
                    interp,
                    Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                );
            }
            _ => {
                let mut sStr: TestStreamInput = TestStreamInput {
                    nStream: 0,
                    aData: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                    nData: 0,
                    iData: 0,
                };
                let mut sOut: TestSessionsBlob = TestSessionsBlob {
                    p: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    n: 0,
                };
                let mut nn: ::core::ffi::c_int = 0;
                memset(
                    &raw mut sStr as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<TestStreamInput>() as size_t,
                );
                memset(
                    &raw mut sOut as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<TestSessionsBlob>() as size_t,
                );
                sStr.aData = Tcl_GetByteArrayFromObj(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut nn,
                );
                sStr.nData = nn;
                sStr.nStream = test_tcl_integer(interp, SESSION_STREAM_TCL_VAR.as_ptr());
                if sStr.nStream != 0 {
                    rc = sqlite3rebaser_rebase_strm(
                        p,
                        Some(
                            testStreamInput
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut ::core::ffi::c_void,
                                    *mut ::core::ffi::c_int,
                                ) -> ::core::ffi::c_int,
                        ),
                        &raw mut sStr as *mut ::core::ffi::c_void,
                        Some(
                            testStreamOutput
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *const ::core::ffi::c_void,
                                    ::core::ffi::c_int,
                                ) -> ::core::ffi::c_int,
                        ),
                        &raw mut sOut as *mut ::core::ffi::c_void,
                    );
                } else {
                    rc = sqlite3rebaser_rebase(
                        p,
                        sStr.nData,
                        sStr.aData as *const ::core::ffi::c_void,
                        &raw mut sOut.n,
                        &raw mut sOut.p,
                    );
                }
                if rc == SQLITE_OK {
                    Tcl_SetObjResult(
                        interp,
                        Tcl_NewByteArrayObj(
                            sOut.p as *const ::core::ffi::c_uchar,
                            sOut.n,
                        ),
                    );
                }
                sqlite3_free(sOut.p);
            }
        }
        if rc != SQLITE_OK {
            return test_session_error(
                interp,
                rc,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_rebaser_del(mut clientData: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut sqlite3_rebaser = clientData as *mut sqlite3_rebaser;
        sqlite3rebaser_delete(p);
    }
}
unsafe extern "C" fn test_sqlite3rebaser_create(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut pNew: *mut sqlite3_rebaser = ::core::ptr::null_mut::<sqlite3_rebaser>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"NAME\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return SQLITE_ERROR;
        }
        rc = sqlite3rebaser_create(&raw mut pNew);
        if rc != SQLITE_OK {
            return test_session_error(
                interp,
                rc,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
        }
        Tcl_CreateObjCommand(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            Some(
                test_rebaser_cmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            pNew as ClientData,
            Some(
                test_rebaser_del as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
        );
        Tcl_SetObjResult(interp, *objv.offset(1 as ::core::ffi::c_int as isize));
        return TCL_OK;
    }
}
unsafe extern "C" fn sqlite3_test_changeset(
    mut nChangeset: ::core::ffi::c_int,
    mut pChangeset: *mut ::core::ffi::c_void,
    mut pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pIter: *mut sqlite3_changeset_iter = ::core::ptr::null_mut::<
            sqlite3_changeset_iter,
        >();
        let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut bPatch: ::core::ffi::c_int = (nChangeset > 0 as ::core::ffi::c_int
            && *(pChangeset as *mut ::core::ffi::c_char)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'P' as i32) as ::core::ffi::c_int;
        rc = sqlite3changeset_start(&raw mut pIter, nChangeset, pChangeset);
        if rc == SQLITE_OK {
            let mut rc2: ::core::ffi::c_int = 0;
            while rc == SQLITE_OK && SQLITE_ROW == sqlite3changeset_next(pIter) {
                let mut aPk: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
                    ::core::ffi::c_uchar,
                >();
                let mut nCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut op: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut zTab: *const ::core::ffi::c_char = ::core::ptr::null::<
                    ::core::ffi::c_char,
                >();
                sqlite3changeset_pk(pIter, &raw mut aPk, &raw mut nCol);
                sqlite3changeset_op(
                    pIter,
                    &raw mut zTab,
                    &raw mut nCol,
                    &raw mut op,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if op == SQLITE_UPDATE {
                    let mut iCol: ::core::ffi::c_int = 0;
                    iCol = 0 as ::core::ffi::c_int;
                    while iCol < nCol {
                        let mut pNew: *mut sqlite3_value = ::core::ptr::null_mut::<
                            sqlite3_value,
                        >();
                        let mut pOld: *mut sqlite3_value = ::core::ptr::null_mut::<
                            sqlite3_value,
                        >();
                        sqlite3changeset_new(pIter, iCol, &raw mut pNew);
                        sqlite3changeset_old(pIter, iCol, &raw mut pOld);
                        if *aPk.offset(iCol as isize) != 0 {
                            if pOld.is_null() {
                                rc = SQLITE_ERROR;
                            }
                        } else if bPatch != 0 {
                            if !pOld.is_null() {
                                rc = SQLITE_ERROR;
                            }
                        } else if pOld.is_null() as ::core::ffi::c_int
                            != pNew.is_null() as ::core::ffi::c_int
                        {
                            rc = SQLITE_ERROR;
                        }
                        if rc != SQLITE_OK {
                            zErr = sqlite3_mprintf(
                                b"unexpected SQLITE_UPDATE (bPatch=%d pk=%d pOld=%d pNew=%d)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                bPatch,
                                *aPk.offset(iCol as isize) as ::core::ffi::c_int,
                                !pOld.is_null() as ::core::ffi::c_int,
                                !pNew.is_null() as ::core::ffi::c_int,
                            );
                            break;
                        } else {
                            iCol += 1;
                        }
                    }
                }
            }
            rc2 = sqlite3changeset_finalize(pIter);
            if rc == SQLITE_OK {
                rc = rc2;
            }
        }
        *pzErr = zErr;
        return rc;
    }
}
unsafe extern "C" fn test_changeset(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pChangeset: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut nChangeset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut z: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"CHANGESET\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        pChangeset = Tcl_GetByteArrayFromObj(
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut nChangeset,
        ) as *mut ::core::ffi::c_void;
        Tcl_ResetResult(interp);
        rc = sqlite3_test_changeset(nChangeset, pChangeset, &raw mut z);
        if rc != SQLITE_OK {
            let mut zErr: *mut ::core::ffi::c_char = sqlite3_mprintf(
                b"(%d) - \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                rc,
                z,
            );
            Tcl_SetObjResult(interp, Tcl_NewStringObj(zErr, -1 as ::core::ffi::c_int));
            sqlite3_free(zErr as *mut ::core::ffi::c_void);
        }
        sqlite3_free(z as *mut ::core::ffi::c_void);
        return if rc != 0 { TCL_ERROR } else { TCL_OK };
    }
}
unsafe extern "C" fn test_sqlite3session_config(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aSub: [ConfigOpt; 3] = [
            ConfigOpt {
                zSub: b"strm_size\0".as_ptr() as *const ::core::ffi::c_char,
                op: SQLITE_SESSION_CONFIG_STRMSIZE,
            },
            ConfigOpt {
                zSub: b"invalid\0".as_ptr() as *const ::core::ffi::c_char,
                op: 0 as ::core::ffi::c_int,
            },
            ConfigOpt {
                zSub: ::core::ptr::null::<::core::ffi::c_char>(),
                op: 0,
            },
        ];
        let mut rc: ::core::ffi::c_int = 0;
        let mut iSub: ::core::ffi::c_int = 0;
        let mut iVal: ::core::ffi::c_int = 0;
        if objc != 3 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"OP VALUE\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return SQLITE_ERROR;
        }
        rc = Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut aSub as *mut ConfigOpt as *const ::core::ffi::c_void,
            ::core::mem::size_of::<ConfigOpt>() as ::core::ffi::c_int,
            b"sub-command\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iSub,
        );
        if rc != TCL_OK {
            return rc;
        }
        if Tcl_GetIntFromObj(
            interp,
            *objv.offset(2 as ::core::ffi::c_int as isize),
            &raw mut iVal,
        ) != 0
        {
            return TCL_ERROR;
        }
        rc = sqlite3session_config(
            aSub[iSub as usize].op,
            &raw mut iVal as *mut ::core::ffi::c_void,
        );
        if rc != SQLITE_OK {
            return test_session_error(
                interp,
                rc,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
        }
        Tcl_SetObjResult(interp, Tcl_NewIntObj(iVal));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_changegroup_del(mut clientData: *mut ::core::ffi::c_void) {
    unsafe {
        let mut pGrp: *mut TestChangegroup = clientData as *mut TestChangegroup;
        sqlite3changegroup_delete((*pGrp).pGrp);
        Tcl_Free(pGrp as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn test_changegroup_cmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut TestChangegroup = clientData as *mut TestChangegroup;
        static mut aSub: [ChangegroupCmd; 6] = [
            ChangegroupCmd {
                zSub: b"schema\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 2 as ::core::ffi::c_int,
                zMsg: b"DB DBNAME\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            ChangegroupCmd {
                zSub: b"add\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"CHANGESET\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            ChangegroupCmd {
                zSub: b"output\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            ChangegroupCmd {
                zSub: b"delete\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 0 as ::core::ffi::c_int,
                zMsg: b"\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            ChangegroupCmd {
                zSub: b"add_change\0".as_ptr() as *const ::core::ffi::c_char,
                nArg: 1 as ::core::ffi::c_int,
                zMsg: b"ITERATOR\0".as_ptr() as *const ::core::ffi::c_char,
                iSub: 0,
            },
            ChangegroupCmd {
                zSub: ::core::ptr::null::<::core::ffi::c_char>(),
                nArg: 0,
                zMsg: ::core::ptr::null::<::core::ffi::c_char>(),
                iSub: 0,
            },
        ];
        let mut rc: ::core::ffi::c_int = TCL_OK;
        let mut iSub: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"SUBCOMMAND ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        rc = Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut aSub as *mut ChangegroupCmd as *const ::core::ffi::c_void,
            ::core::mem::size_of::<ChangegroupCmd>() as ::core::ffi::c_int,
            b"sub-command\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iSub,
        );
        if rc != TCL_OK {
            return rc;
        }
        if objc != 2 as ::core::ffi::c_int + aSub[iSub as usize].nArg {
            Tcl_WrongNumArgs(
                interp,
                2 as ::core::ffi::c_int,
                objv,
                aSub[iSub as usize].zMsg,
            );
            return TCL_ERROR;
        }
        match iSub {
            0 => {
                let mut db: *mut sqlite3 = ::core::ptr::null_mut::<sqlite3>();
                let mut zDb: *const ::core::ffi::c_char = Tcl_GetString(
                    *objv.offset(3 as ::core::ffi::c_int as isize),
                );
                if dbHandleFromObj(
                    interp,
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut db,
                ) != 0
                {
                    return TCL_ERROR;
                }
                rc = sqlite3changegroup_schema((*p).pGrp, db, zDb);
                if rc != SQLITE_OK {
                    rc = test_session_error(
                        interp,
                        rc,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                }
            }
            1 => {
                let mut nByte: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut aByte: *const u8_0 = Tcl_GetByteArrayFromObj(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                    &raw mut nByte,
                );
                rc = sqlite3changegroup_add(
                    (*p).pGrp,
                    nByte,
                    aByte as *mut ::core::ffi::c_void,
                );
                if rc != SQLITE_OK {
                    rc = test_session_error(
                        interp,
                        rc,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                }
            }
            2 => {
                let mut nByte_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut aByte_0: *mut u8_0 = ::core::ptr::null_mut::<u8_0>();
                rc = sqlite3changegroup_output(
                    (*p).pGrp,
                    &raw mut nByte_0,
                    &raw mut aByte_0 as *mut *mut ::core::ffi::c_void,
                );
                if rc != SQLITE_OK {
                    rc = test_session_error(
                        interp,
                        rc,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                } else {
                    Tcl_SetObjResult(interp, Tcl_NewByteArrayObj(aByte_0, nByte_0));
                }
                sqlite3_free(aByte_0 as *mut ::core::ffi::c_void);
            }
            4 => {
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
                let mut pIter: *mut TestChangeIter = ::core::ptr::null_mut::<
                    TestChangeIter,
                >();
                let mut zIter: *const ::core::ffi::c_char = Tcl_GetString(
                    *objv.offset(2 as ::core::ffi::c_int as isize),
                );
                if 0 as ::core::ffi::c_int
                    == Tcl_GetCommandInfo(interp, zIter, &raw mut cmdInfo)
                {
                    Tcl_AppendResult(
                        interp,
                        b"no such iter: \0".as_ptr() as *const ::core::ffi::c_char,
                        Tcl_GetString(*objv.offset(2 as ::core::ffi::c_int as isize)),
                        NULL,
                    );
                    return TCL_ERROR;
                }
                pIter = cmdInfo.objClientData as *mut TestChangeIter
                    as *mut TestChangeIter;
                rc = sqlite3changegroup_add_change((*p).pGrp, (*pIter).pIter);
                if rc != SQLITE_OK {
                    rc = test_session_error(
                        interp,
                        rc,
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    );
                }
            }
            _ => {
                Tcl_DeleteCommand(
                    interp,
                    Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                );
            }
        }
        return rc;
    }
}
unsafe extern "C" fn test_sqlite3changegroup(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        let mut p: *mut TestChangegroup = ::core::ptr::null_mut::<TestChangegroup>();
        if objc != 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"CMD\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        p = Tcl_Alloc(::core::mem::size_of::<TestChangegroup>() as ::core::ffi::c_uint)
            as *mut ::core::ffi::c_void as *mut TestChangegroup;
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<TestChangegroup>() as size_t,
        );
        rc = sqlite3changegroup_new(&raw mut (*p).pGrp);
        if rc != SQLITE_OK {
            Tcl_Free(p as *mut ::core::ffi::c_char);
            return test_session_error(
                interp,
                rc,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
        }
        Tcl_CreateObjCommand(
            interp,
            Tcl_GetString(*objv.offset(1 as ::core::ffi::c_int as isize)),
            Some(
                test_changegroup_cmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            p as ClientData,
            Some(
                test_changegroup_del
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
        );
        Tcl_SetObjResult(interp, *objv.offset(1 as ::core::ffi::c_int as isize));
        return TCL_OK;
    }
}
unsafe extern "C" fn test_iter_del(mut clientData: *mut ::core::ffi::c_void) {
    unsafe {
        let mut p: *mut TestChangeIter = clientData as *mut TestChangeIter;
        sqlite3changeset_finalize((*p).pIter);
        Tcl_Free(p as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C" fn test_iter_cmd(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        static mut aSub: [*const ::core::ffi::c_char; 4] = [
            b"next\0".as_ptr() as *const ::core::ffi::c_char,
            b"data\0".as_ptr() as *const ::core::ffi::c_char,
            b"finalize\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        let mut iSub: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut p: *mut TestChangeIter = clientData as *mut TestChangeIter;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        if objc < 2 as ::core::ffi::c_int {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"CMD\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        if Tcl_GetIndexFromObjStruct(
            interp,
            *objv.offset(1 as ::core::ffi::c_int as isize),
            &raw mut aSub as *mut *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>() as ::core::ffi::c_int,
            b"sub-command\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            &raw mut iSub,
        ) != 0
        {
            return TCL_ERROR;
        }
        match iSub {
            0 => {
                rc = sqlite3changeset_next((*p).pIter);
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(sqlite3ErrName(rc), -1 as ::core::ffi::c_int),
                );
            }
            1 => {
                Tcl_SetObjResult(interp, testIterData((*p).pIter));
            }
            2 => {
                rc = sqlite3changeset_finalize((*p).pIter);
                (*p).pIter = ::core::ptr::null_mut::<sqlite3_changeset_iter>();
                Tcl_DeleteCommand(
                    interp,
                    Tcl_GetString(*objv.offset(0 as ::core::ffi::c_int as isize)),
                );
                Tcl_SetObjResult(
                    interp,
                    Tcl_NewStringObj(sqlite3ErrName(rc), -1 as ::core::ffi::c_int),
                );
            }
            _ => {}
        }
        return TCL_OK;
    }
}
unsafe extern "C" fn test_sqlite3changeset_start(
    mut clientData: *mut ::core::ffi::c_void,
    mut interp: *mut Tcl_Interp,
    mut objc: ::core::ffi::c_int,
    mut objv: *const *mut Tcl_Obj,
) -> ::core::ffi::c_int {
    unsafe {
        let mut isInvert: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pChangeset: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut nChangeset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pNew: *mut TestChangeIter = ::core::ptr::null_mut::<TestChangeIter>();
        let mut pIter: *mut sqlite3_changeset_iter = ::core::ptr::null_mut::<
            sqlite3_changeset_iter,
        >();
        let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut nAlloc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        static mut iCmd: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut zCmd: [::core::ffi::c_char; 64] = [0; 64];
        if objc == 3 as ::core::ffi::c_int {
            let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut z: *const ::core::ffi::c_char = Tcl_GetStringFromObj(
                *objv.offset(1 as ::core::ffi::c_int as isize),
                &raw mut n,
            );
            isInvert = (n >= 2 as ::core::ffi::c_int
                && sqlite3_strnicmp(
                    z,
                    b"-invert\0".as_ptr() as *const ::core::ffi::c_char,
                    n,
                ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
        if objc != 2 as ::core::ffi::c_int
            && (objc != 3 as ::core::ffi::c_int || isInvert == 0)
        {
            Tcl_WrongNumArgs(
                interp,
                1 as ::core::ffi::c_int,
                objv,
                b"?-invert? CHANGESET\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return TCL_ERROR;
        }
        pChangeset = Tcl_GetByteArrayFromObj(
            *objv.offset((objc - 1 as ::core::ffi::c_int) as isize),
            &raw mut nChangeset,
        ) as *mut ::core::ffi::c_void;
        flags = if isInvert != 0 {
            SQLITE_CHANGESETSTART_INVERT
        } else {
            0 as ::core::ffi::c_int
        };
        nAlloc = ::core::mem::size_of::<TestChangeIter>() as ::core::ffi::c_int;
        if test_tcl_integer(interp, SESSION_STREAM_TCL_VAR.as_ptr()) != 0 {
            nAlloc += nChangeset;
        }
        pNew = Tcl_Alloc(nAlloc as ::core::ffi::c_uint) as *mut ::core::ffi::c_void
            as *mut TestChangeIter;
        memset(
            pNew as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            nAlloc as size_t,
        );
        if test_tcl_integer(interp, SESSION_STREAM_TCL_VAR.as_ptr()) != 0 {
            (*pNew).in_0.nStream = test_tcl_integer(
                interp,
                SESSION_STREAM_TCL_VAR.as_ptr(),
            );
            (*pNew).in_0.nData = nChangeset;
            (*pNew).in_0.aData = pNew.offset(1 as ::core::ffi::c_int as isize)
                as *mut TestChangeIter as *mut ::core::ffi::c_uchar;
            memcpy(
                (*pNew).in_0.aData as *mut ::core::ffi::c_void,
                pChangeset,
                nChangeset as size_t,
            );
        }
        if (*pNew).in_0.nStream != 0 {
            let mut pCtx: *mut ::core::ffi::c_void = &raw mut (*pNew).in_0
                as *mut ::core::ffi::c_void;
            rc = sqlite3changeset_start_v2_strm(
                &raw mut pIter,
                Some(
                    testStreamInput
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
                pCtx,
                flags,
            );
        } else {
            rc = sqlite3changeset_start_v2(
                &raw mut pIter,
                nChangeset,
                pChangeset,
                flags,
            );
        }
        if rc != SQLITE_OK {
            let mut zErr: *mut ::core::ffi::c_char = sqlite3_mprintf(
                b"error in sqlite3changeset_start_v2() - %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                rc,
            );
            Tcl_AppendResult(
                interp,
                zErr,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
            Tcl_Free(pNew as *mut ::core::ffi::c_char);
            return TCL_ERROR;
        }
        (*pNew).pIter = pIter;
        let c2rust_fresh7 = iCmd;
        iCmd = iCmd + 1;
        sprintf(
            &raw mut zCmd as *mut ::core::ffi::c_char,
            b"csiter%d\0".as_ptr() as *const ::core::ffi::c_char,
            c2rust_fresh7,
        );
        Tcl_CreateObjCommand(
            interp,
            &raw mut zCmd as *mut ::core::ffi::c_char,
            Some(
                test_iter_cmd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut Tcl_Interp,
                        ::core::ffi::c_int,
                        *const *mut Tcl_Obj,
                    ) -> ::core::ffi::c_int,
            ),
            pNew as ClientData,
            Some(test_iter_del as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        Tcl_SetObjResult(
            interp,
            Tcl_NewStringObj(
                &raw mut zCmd as *mut ::core::ffi::c_char,
                -1 as ::core::ffi::c_int,
            ),
        );
        return TCL_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TestSession_Init(
    mut interp: *mut Tcl_Interp,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aCmd: [Cmd; 14] = [
            Cmd {
                zCmd: b"sqlite3session\0".as_ptr() as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sqlite3session
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            Cmd {
                zCmd: b"sqlite3changegroup\0".as_ptr() as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sqlite3changegroup
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            Cmd {
                zCmd: b"sqlite3changeset_start\0".as_ptr() as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sqlite3changeset_start
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            Cmd {
                zCmd: b"sqlite3session_foreach\0".as_ptr() as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sqlite3session_foreach
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            Cmd {
                zCmd: b"sqlite3changeset_invert\0".as_ptr()
                    as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sqlite3changeset_invert
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            Cmd {
                zCmd: b"sqlite3changeset_concat\0".as_ptr()
                    as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sqlite3changeset_concat
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            Cmd {
                zCmd: b"sqlite3changeset_apply\0".as_ptr() as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sqlite3changeset_apply
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            Cmd {
                zCmd: b"sqlite3changeset_apply_v2\0".as_ptr()
                    as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sqlite3changeset_apply_v2
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            Cmd {
                zCmd: b"sqlite3changeset_apply_v3\0".as_ptr()
                    as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sqlite3changeset_apply_v3
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            Cmd {
                zCmd: b"sqlite3changeset_apply_replace_all\0".as_ptr()
                    as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sqlite3changeset_apply_replace_all
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            Cmd {
                zCmd: b"sql_exec_changeset\0".as_ptr() as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sql_exec_changeset
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            Cmd {
                zCmd: b"sqlite3rebaser_create\0".as_ptr() as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sqlite3rebaser_create
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            Cmd {
                zCmd: b"sqlite3session_config\0".as_ptr() as *const ::core::ffi::c_char,
                xProc: Some(
                    test_sqlite3session_config
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
            Cmd {
                zCmd: b"test_changeset\0".as_ptr() as *const ::core::ffi::c_char,
                xProc: Some(
                    test_changeset
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut Tcl_Interp,
                            ::core::ffi::c_int,
                            *const *mut Tcl_Obj,
                        ) -> ::core::ffi::c_int,
                ),
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while (i as usize)
            < (::core::mem::size_of::<[Cmd; 14]>() as usize)
                .wrapping_div(::core::mem::size_of::<Cmd>() as usize)
        {
            let mut p: *mut Cmd = (&raw mut aCmd as *mut Cmd).offset(i as isize)
                as *mut Cmd;
            Tcl_CreateObjCommand(
                interp,
                (*p).zCmd,
                (*p).xProc,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
            );
            i += 1;
        }
        return TCL_OK;
    }
}
