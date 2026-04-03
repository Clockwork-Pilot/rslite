use ::libc;
unsafe extern "C" {
    pub type sqlite3;
    pub type sqlite3_stmt;
    pub type sqlite3_value;
    pub type sqlite3_context;
    fn sqlite3_mprintf(_: *const ::core::ffi::c_char, ...) -> *mut ::core::ffi::c_char;
    fn sqlite3_vmprintf(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    fn sqlite3_malloc(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn sqlite3_free(_: *mut ::core::ffi::c_void);
    fn sqlite3_errcode(db: *mut sqlite3) -> ::core::ffi::c_int;
    fn sqlite3_errmsg(_: *mut sqlite3) -> *const ::core::ffi::c_char;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const ::core::ffi::c_char,
        nByte: ::core::ffi::c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_text(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    fn sqlite3_bind_value(
        _: *mut sqlite3_stmt,
        _: ::core::ffi::c_int,
        _: *const sqlite3_value,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_name(
        _: *mut sqlite3_stmt,
        N: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn sqlite3_step(_: *mut sqlite3_stmt) -> ::core::ffi::c_int;
    fn sqlite3_column_int(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sqlite3_column_text(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_uchar;
    fn sqlite3_column_value(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> *mut sqlite3_value;
    fn sqlite3_column_type(
        _: *mut sqlite3_stmt,
        iCol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn sqlite3_value_int(_: *mut sqlite3_value) -> ::core::ffi::c_int;
    fn sqlite3_value_text(_: *mut sqlite3_value) -> *const ::core::ffi::c_uchar;
    fn sqlite3_result_text(
        _: *mut sqlite3_context,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn sqlite3_stricmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sqlite3_strnicmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
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
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type va_list = __builtin_va_list;
pub type sqlite3_destructor_type = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_intck {
    pub db: *mut sqlite3,
    pub zDb: *const ::core::ffi::c_char,
    pub zObj: *mut ::core::ffi::c_char,
    pub pCheck: *mut sqlite3_stmt,
    pub zKey: *mut ::core::ffi::c_char,
    pub nKeyVal: ::core::ffi::c_int,
    pub zMessage: *mut ::core::ffi::c_char,
    pub bCorruptSchema: ::core::ffi::c_int,
    pub rc: ::core::ffi::c_int,
    pub zErr: *mut ::core::ffi::c_char,
    pub zTestSql: *mut ::core::ffi::c_char,
}
pub type size_t = usize;
pub const SQLITE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SQLITE_NOMEM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SQLITE_CORRUPT: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SQLITE_ROW: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const SQLITE_DONE: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const SQLITE_NULL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SQLITE_UTF8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn intckSaveErrmsg(mut p: *mut sqlite3_intck) {
    unsafe {
        (*p).rc = sqlite3_errcode((*p).db);
        sqlite3_free((*p).zErr as *mut ::core::ffi::c_void);
        (*p).zErr = sqlite3_mprintf(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            sqlite3_errmsg((*p).db),
        );
    }
}
unsafe extern "C" fn intckPrepare(
    mut p: *mut sqlite3_intck,
    mut zSql: *const ::core::ffi::c_char,
) -> *mut sqlite3_stmt {
    unsafe {
        let mut pRet: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        if (*p).rc == SQLITE_OK {
            (*p).rc = sqlite3_prepare_v2(
                (*p).db,
                zSql,
                -1 as ::core::ffi::c_int,
                &raw mut pRet,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            );
            if (*p).rc != SQLITE_OK {
                intckSaveErrmsg(p);
            }
        }
        return pRet;
    }
}
unsafe extern "C" fn intckPrepareFmt(
    mut p: *mut sqlite3_intck,
    mut zFmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut sqlite3_stmt {
    unsafe {
        let mut pRet: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut ap = c2rust_args.clone();
        zSql = sqlite3_vmprintf(zFmt, ap);
        if (*p).rc == SQLITE_OK && zSql.is_null() {
            (*p).rc = SQLITE_NOMEM;
        }
        pRet = intckPrepare(p, zSql);
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        return pRet;
    }
}
unsafe extern "C" fn intckFinalize(
    mut p: *mut sqlite3_intck,
    mut pStmt: *mut sqlite3_stmt,
) {
    unsafe {
        let mut rc: ::core::ffi::c_int = sqlite3_finalize(pStmt);
        if (*p).rc == SQLITE_OK && rc != SQLITE_OK {
            intckSaveErrmsg(p);
        }
    }
}
unsafe extern "C" fn intckStep(
    mut p: *mut sqlite3_intck,
    mut pStmt: *mut sqlite3_stmt,
) -> ::core::ffi::c_int {
    unsafe {
        if (*p).rc != 0 {
            return (*p).rc;
        }
        return sqlite3_step(pStmt);
    }
}
unsafe extern "C" fn intckExec(
    mut p: *mut sqlite3_intck,
    mut zSql: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        pStmt = intckPrepare(p, zSql);
        intckStep(p, pStmt);
        intckFinalize(p, pStmt);
    }
}
unsafe extern "C" fn intckMprintf(
    mut p: *mut sqlite3_intck,
    mut zFmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut ap = c2rust_args.clone();
        zRet = sqlite3_vmprintf(zFmt, ap);
        if (*p).rc == SQLITE_OK {
            if zRet.is_null() {
                (*p).rc = SQLITE_NOMEM;
            }
        } else {
            sqlite3_free(zRet as *mut ::core::ffi::c_void);
            zRet = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return zRet;
    }
}
unsafe extern "C" fn intckSaveKey(mut p: *mut sqlite3_intck) {
    unsafe {
        let mut ii: ::core::ffi::c_int = 0;
        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut pXinfo: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zDir: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        pXinfo = intckPrepareFmt(
            p,
            b"SELECT group_concat(desc, '') FROM %Q.sqlite_schema s, pragma_index_xinfo(%Q, %Q) WHERE s.type='index' AND s.name=%Q\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*p).zDb,
            (*p).zObj,
            (*p).zDb,
            (*p).zObj,
        );
        if (*p).rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pXinfo) {
            zDir = sqlite3_column_text(pXinfo, 0 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
        }
        if zDir.is_null() {
            let mut zSep: *const ::core::ffi::c_char = b"SELECT '(' || \0".as_ptr()
                as *const ::core::ffi::c_char;
            ii = 0 as ::core::ffi::c_int;
            while ii < (*p).nKeyVal {
                zSql = intckMprintf(
                    p,
                    b"%z%squote(?)\0".as_ptr() as *const ::core::ffi::c_char,
                    zSql,
                    zSep,
                );
                zSep = b" || ', ' || \0".as_ptr() as *const ::core::ffi::c_char;
                ii += 1;
            }
            zSql = intckMprintf(
                p,
                b"%z || ')'\0".as_ptr() as *const ::core::ffi::c_char,
                zSql,
            );
        } else {
            let mut c2rust_current_block_29: u64;
            ii = (*p).nKeyVal;
            while ii > 0 as ::core::ffi::c_int {
                let mut bLastIsDesc: ::core::ffi::c_int = (*zDir
                    .offset((ii - 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int == '1' as i32) as ::core::ffi::c_int;
                let mut bLastIsNull: ::core::ffi::c_int = (sqlite3_column_type(
                    (*p).pCheck,
                    ii,
                ) == SQLITE_NULL) as ::core::ffi::c_int;
                let mut zLast: *const ::core::ffi::c_char = sqlite3_column_name(
                    (*p).pCheck,
                    ii,
                );
                let mut zLhs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut zRhs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut zWhere: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                if bLastIsNull != 0 {
                    if bLastIsDesc != 0 {
                        c2rust_current_block_29 = 12599329904712511516;
                    } else {
                        zWhere = intckMprintf(
                            p,
                            b"'%s IS NOT NULL'\0".as_ptr() as *const ::core::ffi::c_char,
                            zLast,
                        );
                        c2rust_current_block_29 = 14576567515993809846;
                    }
                } else {
                    let mut zOp: *const ::core::ffi::c_char = if bLastIsDesc != 0 {
                        b"<\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b">\0".as_ptr() as *const ::core::ffi::c_char
                    };
                    zWhere = intckMprintf(
                        p,
                        b"'%s %s ' || quote(?%d)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        zLast,
                        zOp,
                        ii,
                    );
                    c2rust_current_block_29 = 14576567515993809846;
                }
                match c2rust_current_block_29 {
                    14576567515993809846 => {
                        if ii > 1 as ::core::ffi::c_int {
                            let mut zLhsSep: *const ::core::ffi::c_char = b"\0".as_ptr()
                                as *const ::core::ffi::c_char;
                            let mut zRhsSep: *const ::core::ffi::c_char = b"\0".as_ptr()
                                as *const ::core::ffi::c_char;
                            let mut jj: ::core::ffi::c_int = 0;
                            jj = 0 as ::core::ffi::c_int;
                            while jj < ii - 1 as ::core::ffi::c_int {
                                let mut zAlias: *const ::core::ffi::c_char = sqlite3_column_name(
                                    (*p).pCheck,
                                    jj + 1 as ::core::ffi::c_int,
                                );
                                zLhs = intckMprintf(
                                    p,
                                    b"%z%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                                    zLhs,
                                    zLhsSep,
                                    zAlias,
                                );
                                zRhs = intckMprintf(
                                    p,
                                    b"%z%squote(?%d)\0".as_ptr() as *const ::core::ffi::c_char,
                                    zRhs,
                                    zRhsSep,
                                    jj + 1 as ::core::ffi::c_int,
                                );
                                zLhsSep = b",\0".as_ptr() as *const ::core::ffi::c_char;
                                zRhsSep = b" || ',' || \0".as_ptr()
                                    as *const ::core::ffi::c_char;
                                jj += 1;
                            }
                            zWhere = intckMprintf(
                                p,
                                b"'(%z) IS (' || %z || ') AND ' || %z\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                zLhs,
                                zRhs,
                                zWhere,
                            );
                        }
                        zWhere = intckMprintf(
                            p,
                            b"'WHERE ' || %z\0".as_ptr() as *const ::core::ffi::c_char,
                            zWhere,
                        );
                        zSql = intckMprintf(
                            p,
                            b"%z%s(quote( %z ) )\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            zSql,
                            if zSql.is_null() {
                                b"VALUES\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b",\n      \0".as_ptr() as *const ::core::ffi::c_char
                            },
                            zWhere,
                        );
                    }
                    _ => {}
                }
                ii -= 1;
            }
            zSql = intckMprintf(
                p,
                b"WITH wc(q) AS (\n%z\n)SELECT 'VALUES' || group_concat('(' || q || ')', ',\n      ') FROM wc\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                zSql,
            );
        }
        pStmt = intckPrepare(p, zSql);
        if (*p).rc == SQLITE_OK {
            ii = 0 as ::core::ffi::c_int;
            while ii < (*p).nKeyVal {
                sqlite3_bind_value(
                    pStmt,
                    ii + 1 as ::core::ffi::c_int,
                    sqlite3_column_value((*p).pCheck, ii + 1 as ::core::ffi::c_int),
                );
                ii += 1;
            }
            if SQLITE_ROW == sqlite3_step(pStmt) {
                (*p).zKey = intckMprintf(
                    p,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_column_text(pStmt, 0 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char,
                );
            }
            intckFinalize(p, pStmt);
        }
        sqlite3_free(zSql as *mut ::core::ffi::c_void);
        intckFinalize(p, pXinfo);
    }
}
unsafe extern "C" fn intckFindObject(mut p: *mut sqlite3_intck) {
    unsafe {
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut zPrev: *mut ::core::ffi::c_char = (*p).zObj;
        (*p).zObj = ::core::ptr::null_mut::<::core::ffi::c_char>();
        pStmt = intckPrepareFmt(
            p,
            b"WITH tables(table_name) AS (  SELECT name  FROM %Q.sqlite_schema WHERE (type='table' OR type='index') AND rootpage  UNION ALL   SELECT 'sqlite_schema')SELECT table_name FROM tables WHERE ?1 IS NULL OR table_name%s?1 ORDER BY 1\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*p).zDb,
            if !(*p).zKey.is_null() {
                b">=\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b">\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if (*p).rc == SQLITE_OK {
            sqlite3_bind_text(
                pStmt,
                1 as ::core::ffi::c_int,
                zPrev,
                -1 as ::core::ffi::c_int,
                ::core::mem::transmute::<
                    ::libc::intptr_t,
                    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
                >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
            );
            if sqlite3_step(pStmt) == SQLITE_ROW {
                (*p).zObj = intckMprintf(
                    p,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    sqlite3_column_text(pStmt, 0 as ::core::ffi::c_int)
                        as *const ::core::ffi::c_char,
                );
            }
        }
        intckFinalize(p, pStmt);
        if sqlite3_stricmp((*p).zObj, zPrev) != 0 {
            sqlite3_free((*p).zKey as *mut ::core::ffi::c_void);
            (*p).zKey = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        sqlite3_free(zPrev as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn intckGetToken(
    mut z: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: ::core::ffi::c_char = *z.offset(0 as ::core::ffi::c_int as isize);
        let mut iRet: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if c as ::core::ffi::c_int == '\'' as i32
            || c as ::core::ffi::c_int == '"' as i32
            || c as ::core::ffi::c_int == '`' as i32
        {
            loop {
                if *z.offset(iRet as isize) as ::core::ffi::c_int
                    == c as ::core::ffi::c_int
                {
                    iRet += 1;
                    if *z.offset(iRet as isize) as ::core::ffi::c_int
                        != c as ::core::ffi::c_int
                    {
                        break;
                    }
                }
                iRet += 1;
            }
        } else if c as ::core::ffi::c_int == '[' as i32 {
            loop {
                let c2rust_fresh0 = iRet;
                iRet = iRet + 1;
                if !(*z.offset(c2rust_fresh0 as isize) as ::core::ffi::c_int
                    != ']' as i32 && *z.offset(iRet as isize) as ::core::ffi::c_int != 0)
                {
                    break;
                }
            }
        } else if c as ::core::ffi::c_int >= 'A' as i32
            && c as ::core::ffi::c_int <= 'Z' as i32
            || c as ::core::ffi::c_int >= 'a' as i32
                && c as ::core::ffi::c_int <= 'z' as i32
        {
            while *z.offset(iRet as isize) as ::core::ffi::c_int >= 'A' as i32
                && *z.offset(iRet as isize) as ::core::ffi::c_int <= 'Z' as i32
                || *z.offset(iRet as isize) as ::core::ffi::c_int >= 'a' as i32
                    && *z.offset(iRet as isize) as ::core::ffi::c_int <= 'z' as i32
            {
                iRet += 1;
            }
        }
        return iRet;
    }
}
unsafe extern "C" fn intckIsSpace(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    unsafe {
        return (c as ::core::ffi::c_int == ' ' as i32
            || c as ::core::ffi::c_int == '\t' as i32
            || c as ::core::ffi::c_int == '\n' as i32
            || c as ::core::ffi::c_int == '\r' as i32) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn intckParseCreateIndex(
    mut z: *const ::core::ffi::c_char,
    mut iCol: ::core::ffi::c_int,
    mut pnByte: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut iOff: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iThisCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iStart: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut nOpen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zRet: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nRet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut iEndOfCol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while *z.offset(iOff as isize) as ::core::ffi::c_int != '(' as i32 {
            iOff += intckGetToken(z.offset(iOff as isize) as *const ::core::ffi::c_char);
            if *z.offset(iOff as isize) as ::core::ffi::c_int == '\0' as i32 {
                return ::core::ptr::null::<::core::ffi::c_char>();
            }
        }
        nOpen = 1 as ::core::ffi::c_int;
        iOff += 1;
        iStart = iOff;
        while *z.offset(iOff as isize) != 0 {
            let mut zToken: *const ::core::ffi::c_char = z.offset(iOff as isize)
                as *const ::core::ffi::c_char;
            let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if nOpen == 1 as ::core::ffi::c_int {
                if *z.offset(iOff as isize) as ::core::ffi::c_int == ',' as i32
                    || *z.offset(iOff as isize) as ::core::ffi::c_int == ')' as i32
                {
                    if iCol == iThisCol {
                        let mut iEnd: ::core::ffi::c_int = if iEndOfCol != 0 {
                            iEndOfCol
                        } else {
                            iOff
                        };
                        nRet = iEnd - iStart;
                        zRet = z.offset(iStart as isize) as *const ::core::ffi::c_char;
                        break;
                    } else {
                        iStart = iOff + 1 as ::core::ffi::c_int;
                        while intckIsSpace(*z.offset(iStart as isize)) != 0 {
                            iStart += 1;
                        }
                        iThisCol += 1;
                    }
                }
                if *z.offset(iOff as isize) as ::core::ffi::c_int == ')' as i32 {
                    break;
                }
            }
            if *z.offset(iOff as isize) as ::core::ffi::c_int == '(' as i32 {
                nOpen += 1;
            }
            if *z.offset(iOff as isize) as ::core::ffi::c_int == ')' as i32 {
                nOpen -= 1;
            }
            nToken = intckGetToken(zToken);
            if nToken == 3 as ::core::ffi::c_int
                && 0 as ::core::ffi::c_int
                    == sqlite3_strnicmp(
                        zToken,
                        b"ASC\0".as_ptr() as *const ::core::ffi::c_char,
                        nToken,
                    )
                || nToken == 4 as ::core::ffi::c_int
                    && 0 as ::core::ffi::c_int
                        == sqlite3_strnicmp(
                            zToken,
                            b"DESC\0".as_ptr() as *const ::core::ffi::c_char,
                            nToken,
                        )
            {
                iEndOfCol = iOff;
            } else if 0 as ::core::ffi::c_int
                == intckIsSpace(*zToken.offset(0 as ::core::ffi::c_int as isize))
            {
                iEndOfCol = 0 as ::core::ffi::c_int;
            }
            iOff += nToken;
        }
        while zRet.is_null() && *z.offset(iOff as isize) as ::core::ffi::c_int != 0 {
            let mut n: ::core::ffi::c_int = intckGetToken(
                z.offset(iOff as isize) as *const ::core::ffi::c_char,
            );
            if n == 5 as ::core::ffi::c_int
                && 0 as ::core::ffi::c_int
                    == sqlite3_strnicmp(
                        z.offset(iOff as isize) as *const ::core::ffi::c_char,
                        b"where\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    )
            {
                zRet = z.offset((iOff + 5 as ::core::ffi::c_int) as isize)
                    as *const ::core::ffi::c_char;
                nRet = strlen(zRet) as ::core::ffi::c_int;
            }
            iOff += n;
        }
        if !zRet.is_null() {
            while intckIsSpace(*zRet.offset(0 as ::core::ffi::c_int as isize)) != 0 {
                nRet -= 1;
                zRet = zRet.offset(1);
            }
            while nRet > 0 as ::core::ffi::c_int
                && intckIsSpace(*zRet.offset((nRet - 1 as ::core::ffi::c_int) as isize))
                    != 0
            {
                nRet -= 1;
            }
        }
        *pnByte = nRet;
        return zRet;
    }
}
unsafe extern "C" fn intckParseCreateIndexFunc(
    mut pCtx: *mut sqlite3_context,
    mut nVal: ::core::ffi::c_int,
    mut apVal: *mut *mut sqlite3_value,
) {
    unsafe {
        let mut zSql: *const ::core::ffi::c_char = sqlite3_value_text(
            *apVal.offset(0 as ::core::ffi::c_int as isize),
        ) as *const ::core::ffi::c_char;
        let mut idx: ::core::ffi::c_int = sqlite3_value_int(
            *apVal.offset(1 as ::core::ffi::c_int as isize),
        );
        let mut zRes: *const ::core::ffi::c_char = ::core::ptr::null::<
            ::core::ffi::c_char,
        >();
        let mut nRes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !zSql.is_null() {
            zRes = intckParseCreateIndex(zSql, idx, &raw mut nRes);
        }
        sqlite3_result_text(
            pCtx,
            zRes,
            nRes,
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-1 as ::core::ffi::c_int as ::libc::intptr_t),
        );
    }
}
unsafe extern "C" fn intckGetAutoIndex(mut p: *mut sqlite3_intck) -> ::core::ffi::c_int {
    unsafe {
        let mut bRet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        pStmt = intckPrepare(
            p,
            b"PRAGMA automatic_index\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if SQLITE_ROW == intckStep(p, pStmt) {
            bRet = sqlite3_column_int(pStmt, 0 as ::core::ffi::c_int);
        }
        intckFinalize(p, pStmt);
        return bRet;
    }
}
unsafe extern "C" fn intckIsIndex(
    mut p: *mut sqlite3_intck,
    mut zObj: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut bRet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        pStmt = intckPrepareFmt(
            p,
            b"SELECT 1 FROM %Q.sqlite_schema WHERE name=%Q AND type='index'\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*p).zDb,
            zObj,
        );
        if (*p).rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pStmt) {
            bRet = 1 as ::core::ffi::c_int;
        }
        intckFinalize(p, pStmt);
        return bRet;
    }
}
unsafe extern "C" fn intckCheckObjectSql(
    mut p: *mut sqlite3_intck,
    mut zObj: *const ::core::ffi::c_char,
    mut zPrev: *const ::core::ffi::c_char,
    mut pnKeyVal: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut zRet: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut pStmt: *mut sqlite3_stmt = ::core::ptr::null_mut::<sqlite3_stmt>();
        let mut bAutoIndex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bIsIndex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut zCommon: *const ::core::ffi::c_char = b", without_rowid(b) AS (  SELECT EXISTS (    SELECT 1 FROM tabname, pragma_index_list(tab, db) AS l      WHERE origin='pk'       AND NOT EXISTS (SELECT 1 FROM sqlite_schema WHERE name=l.name)  )), idx_cols(idx_name, idx_ispk, col_name, col_expr, col_alias) AS (  SELECT l.name, (l.origin=='pk' AND w.b), i.name, COALESCE((    SELECT parse_create_index(sql, i.seqno) FROM     sqlite_schema WHERE name = l.name  ), format('\"%w\"', i.name) || ' COLLATE ' || quote(i.coll)),  'c' || row_number() OVER ()  FROM       tabname t,      without_rowid w,      pragma_index_list(t.tab, t.db) l,      pragma_index_xinfo(l.name) i      WHERE i.key  UNION ALL  SELECT '', 1, '_rowid_', '_rowid_', 'r1' FROM without_rowid WHERE b=0), tabpk(db, tab, idx, o_pk, i_pk, q_pk, eq_pk, ps_pk, pk_pk, n_pk) AS (    WITH pkfields(f, a) AS (      SELECT i.col_name, i.col_alias FROM idx_cols i WHERE i.idx_ispk    )    SELECT t.db, t.tab, t.idx,            group_concat(a, ', '),            group_concat('i.'||quote(f), ', '),            group_concat('quote(o.'||a||')', ' || '','' || '),             format('(%s)==(%s)',               group_concat('o.'||a, ', '),                group_concat(format('\"%w\"', f), ', ')           ),           group_concat('%s', ','),           group_concat('quote('||a||')', ', '),             count(*)    FROM tabname t, pkfields), idx(name, match_expr, partial, partial_alias, idx_ps, idx_idx) AS (  SELECT idx_name,    format('(%s,%s) IS (%s,%s)',            group_concat(i.col_expr, ', '), i_pk,           group_concat('o.'||i.col_alias, ', '), o_pk    ),     parse_create_index(        (SELECT sql FROM sqlite_schema WHERE name=idx_name), -1    ),    'cond' || row_number() OVER ()    , group_concat('%s', ',')    , group_concat('quote('||i.col_alias||')', ', ')  FROM tabpk t,        without_rowid w,       idx_cols i  WHERE i.idx_ispk==0   GROUP BY idx_name), wrapper_with(s) AS (  SELECT 'intck_wrapper AS (\n  SELECT\n    ' || (      WITH f(a, b) AS (        SELECT col_expr, col_alias FROM idx_cols          UNION ALL         SELECT partial, partial_alias FROM idx WHERE partial IS NOT NULL      )      SELECT group_concat(format('%s AS %s', a, b), ',\n    ') FROM f    )    || format('\n  FROM %Q.%Q ', t.db, t.tab)    || CASE WHEN t.idx IS NULL THEN         'NOT INDEXED'       ELSE        format('INDEXED BY %Q%s', t.idx, ' WHERE '||i.partial)       END    || '\n)'    FROM tabname t LEFT JOIN idx i ON (i.name=t.idx))\0"
            .as_ptr() as *const ::core::ffi::c_char;
        bAutoIndex = intckGetAutoIndex(p);
        if bAutoIndex != 0 {
            intckExec(
                p,
                b"PRAGMA automatic_index = 0\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        bIsIndex = intckIsIndex(p, zObj);
        if bIsIndex != 0 {
            pStmt = intckPrepareFmt(
                p,
                b"WITH tabname(db, tab, idx) AS (  SELECT %Q, (SELECT tbl_name FROM %Q.sqlite_schema WHERE name=%Q), %Q ), whereclause(w_c) AS (%s)%s, case_statement(c) AS (  SELECT     'CASE WHEN (' || group_concat(col_alias, ', ') || ', 1) IS (\n'     || '      SELECT ' || group_concat(col_expr, ', ') || ', 1 FROM '    || format('%%Q.%%Q NOT INDEXED WHERE %%s\n', t.db, t.tab, p.eq_pk)    || '    )\n  THEN NULL\n    '    || 'ELSE format(''surplus entry ('    ||   group_concat('%%s', ',') || ',' || p.ps_pk    || ') in index ' || t.idx || ''', '     ||   group_concat('quote('||i.col_alias||')', ', ') || ', ' || p.pk_pk    || ')'    || '\n  END AS error_message'  FROM tabname t, tabpk p, idx_cols i WHERE i.idx_name=t.idx), thiskey(k, n) AS (    SELECT group_concat(i.col_alias, ', ') || ', ' || p.o_pk,            count(*) + p.n_pk     FROM tabpk p, idx_cols i WHERE i.idx_name=p.idx), main_select(m, n) AS (  SELECT format(      'WITH %%s\n' ||      ', idx_checker AS (\n' ||      '  SELECT %%s,\n' ||      '  %%s\n' ||       '  FROM intck_wrapper AS o\n' ||      ')\n',      ww.s, c, t.k  ), t.n  FROM case_statement, wrapper_with ww, thiskey t)SELECT m ||     group_concat('SELECT * FROM idx_checker ' || w_c, ' UNION ALL '), n FROM main_select, whereclause \0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*p).zDb,
                (*p).zDb,
                zObj,
                zObj,
                if !zPrev.is_null() {
                    zPrev
                } else {
                    b"VALUES('')\0".as_ptr() as *const ::core::ffi::c_char
                },
                zCommon,
            );
        } else {
            pStmt = intckPrepareFmt(
                p,
                b"WITH tabname(db, tab, idx, prev) AS (SELECT %Q, %Q, NULL, %Q)%s, expr(e, p) AS (  SELECT format('CASE WHEN EXISTS \n    (SELECT 1 FROM %%Q.%%Q AS i INDEXED BY %%Q WHERE %%s%%s)\n    THEN NULL\n    ELSE format(''entry (%%s,%%s) missing from index %%s'', %%s, %%s)\n  END\n'    , t.db, t.tab, i.name, i.match_expr, ' AND (' || partial || ')',      i.idx_ps, t.ps_pk, i.name, i.idx_idx, t.pk_pk),    CASE WHEN partial IS NULL THEN NULL ELSE i.partial_alias END  FROM tabpk t, idx i), numbered(ii, cond, e) AS (  SELECT 0, 'n.ii=0', 'NULL'    UNION ALL   SELECT row_number() OVER (),      '(n.ii='||row_number() OVER ()||COALESCE(' AND '||p||')', ')'), e  FROM expr), counter_with(w) AS (    SELECT 'WITH intck_counter(ii) AS (\n  ' ||        group_concat('SELECT '||ii, ' UNION ALL\n  ')     || '\n)' FROM numbered), case_statement(c) AS (    SELECT 'CASE ' ||     group_concat(format('\n  WHEN %%s THEN (%%s)', cond, e), '') ||    '\nEND AS error_message'    FROM numbered), thiskey(k, n) AS (    SELECT o_pk || ', ii', n_pk+1 FROM tabpk), whereclause(w_c) AS (    SELECT CASE WHEN prev!='' THEN     '\nWHERE (' || o_pk ||', n.ii) > ' || prev    ELSE ''    END    FROM tabpk, tabname), main_select(m, n) AS (  SELECT format(      '%%s, %%s\nSELECT %%s,\n%%s\nFROM intck_wrapper AS o, intck_counter AS n%%s\nORDER BY %%s',       w, ww.s, c, thiskey.k, whereclause.w_c, t.o_pk  ), thiskey.n  FROM case_statement, tabpk t, counter_with,        wrapper_with ww, thiskey, whereclause)SELECT m, n FROM main_select\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*p).zDb,
                zObj,
                zPrev,
                zCommon,
            );
        }
        while (*p).rc == SQLITE_OK && SQLITE_ROW == sqlite3_step(pStmt) {
            zRet = intckMprintf(
                p,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                sqlite3_column_text(pStmt, 0 as ::core::ffi::c_int)
                    as *const ::core::ffi::c_char,
            );
            if !pnKeyVal.is_null() {
                *pnKeyVal = sqlite3_column_int(pStmt, 1 as ::core::ffi::c_int);
            }
        }
        intckFinalize(p, pStmt);
        if bAutoIndex != 0 {
            intckExec(
                p,
                b"PRAGMA automatic_index = 1\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return zRet;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_intck_open(
    mut db: *mut sqlite3,
    mut zDbArg: *const ::core::ffi::c_char,
    mut ppOut: *mut *mut sqlite3_intck,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pNew: *mut sqlite3_intck = ::core::ptr::null_mut::<sqlite3_intck>();
        let mut rc: ::core::ffi::c_int = SQLITE_OK;
        let mut zDb: *const ::core::ffi::c_char = if !zDbArg.is_null() {
            zDbArg
        } else {
            b"main\0".as_ptr() as *const ::core::ffi::c_char
        };
        let mut nDb: ::core::ffi::c_int = strlen(zDb) as ::core::ffi::c_int;
        pNew = sqlite3_malloc(
            (::core::mem::size_of::<sqlite3_intck>() as usize)
                .wrapping_add(nDb as usize)
                .wrapping_add(1 as usize) as ::core::ffi::c_int,
        ) as *mut sqlite3_intck;
        if pNew.is_null() {
            rc = SQLITE_NOMEM;
        } else {
            memset(
                pNew as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<sqlite3_intck>() as size_t,
            );
            (*pNew).db = db;
            (*pNew).zDb = pNew.offset(1 as ::core::ffi::c_int as isize)
                as *mut sqlite3_intck as *const ::core::ffi::c_char;
            memcpy(
                pNew.offset(1 as ::core::ffi::c_int as isize) as *mut sqlite3_intck
                    as *mut ::core::ffi::c_void,
                zDb as *const ::core::ffi::c_void,
                (nDb + 1 as ::core::ffi::c_int) as size_t,
            );
            rc = sqlite3_create_function(
                db,
                b"parse_create_index\0".as_ptr() as *const ::core::ffi::c_char,
                2 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                Some(
                    intckParseCreateIndexFunc
                        as unsafe extern "C" fn(
                            *mut sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut sqlite3_value,
                        ) -> (),
                ),
                None,
                None,
            );
            if rc != SQLITE_OK {
                sqlite3_intck_close(pNew);
                pNew = ::core::ptr::null_mut::<sqlite3_intck>();
            }
        }
        *ppOut = pNew;
        return rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_intck_close(mut p: *mut sqlite3_intck) {
    unsafe {
        if !p.is_null() {
            sqlite3_finalize((*p).pCheck);
            sqlite3_create_function(
                (*p).db,
                b"parse_create_index\0".as_ptr() as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                SQLITE_UTF8,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                None,
                None,
                None,
            );
            sqlite3_free((*p).zObj as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zKey as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zTestSql as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zErr as *mut ::core::ffi::c_void);
            sqlite3_free((*p).zMessage as *mut ::core::ffi::c_void);
            sqlite3_free(p as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_intck_step(
    mut p: *mut sqlite3_intck,
) -> ::core::ffi::c_int {
    unsafe {
        if (*p).rc == SQLITE_OK {
            if !(*p).zMessage.is_null() {
                sqlite3_free((*p).zMessage as *mut ::core::ffi::c_void);
                (*p).zMessage = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            if (*p).bCorruptSchema != 0 {
                (*p).rc = SQLITE_DONE;
            } else if (*p).pCheck.is_null() {
                intckFindObject(p);
                if (*p).rc == SQLITE_OK {
                    if !(*p).zObj.is_null() {
                        let mut zSql: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                            ::core::ffi::c_char,
                        >();
                        zSql = intckCheckObjectSql(
                            p,
                            (*p).zObj,
                            (*p).zKey,
                            &raw mut (*p).nKeyVal,
                        );
                        (*p).pCheck = intckPrepare(p, zSql);
                        sqlite3_free(zSql as *mut ::core::ffi::c_void);
                        sqlite3_free((*p).zKey as *mut ::core::ffi::c_void);
                        (*p).zKey = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    } else {
                        (*p).rc = SQLITE_DONE;
                    }
                } else if (*p).rc == SQLITE_CORRUPT {
                    (*p).rc = SQLITE_OK;
                    (*p).zMessage = intckMprintf(
                        p,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        b"corruption found while reading database schema\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*p).bCorruptSchema = 1 as ::core::ffi::c_int;
                }
            }
            if !(*p).pCheck.is_null() {
                if !(sqlite3_step((*p).pCheck) == SQLITE_ROW) {
                    intckFinalize(p, (*p).pCheck);
                    (*p).pCheck = ::core::ptr::null_mut::<sqlite3_stmt>();
                    (*p).nKeyVal = 0 as ::core::ffi::c_int;
                    if (*p).rc == SQLITE_CORRUPT {
                        (*p).rc = SQLITE_OK;
                        (*p).zMessage = intckMprintf(
                            p,
                            b"corruption found while scanning database object %s\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            (*p).zObj,
                        );
                    }
                }
            }
        }
        return (*p).rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_intck_message(
    mut p: *mut sqlite3_intck,
) -> *const ::core::ffi::c_char {
    unsafe {
        if !(*p).zMessage.is_null() {
            return (*p).zMessage;
        }
        if !(*p).pCheck.is_null() {
            return sqlite3_column_text((*p).pCheck, 0 as ::core::ffi::c_int)
                as *const ::core::ffi::c_char;
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_intck_error(
    mut p: *mut sqlite3_intck,
    mut pzErr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if !pzErr.is_null() {
            *pzErr = (*p).zErr;
        }
        return if (*p).rc == SQLITE_DONE { SQLITE_OK } else { (*p).rc };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_intck_unlock(
    mut p: *mut sqlite3_intck,
) -> ::core::ffi::c_int {
    unsafe {
        if (*p).rc == SQLITE_OK && !(*p).pCheck.is_null() {
            intckSaveKey(p);
            intckFinalize(p, (*p).pCheck);
            (*p).pCheck = ::core::ptr::null_mut::<sqlite3_stmt>();
        }
        return (*p).rc;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_intck_test_sql(
    mut p: *mut sqlite3_intck,
    mut zObj: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        sqlite3_free((*p).zTestSql as *mut ::core::ffi::c_void);
        if !zObj.is_null() {
            (*p).zTestSql = intckCheckObjectSql(
                p,
                zObj,
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
        } else if !(*p).zObj.is_null() {
            (*p).zTestSql = intckCheckObjectSql(
                p,
                (*p).zObj,
                (*p).zKey,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
        } else {
            sqlite3_free((*p).zTestSql as *mut ::core::ffi::c_void);
            (*p).zTestSql = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return (*p).zTestSql;
    }
}
