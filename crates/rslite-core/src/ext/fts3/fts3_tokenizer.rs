// =============== BEGIN fts3_tokenizer_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer_module {
    pub iVersion: ::core::ffi::c_int,
    pub xCreate: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *const *const ::core::ffi::c_char,
            *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
        ) -> ::core::ffi::c_int,
    >,
    pub xDestroy: Option<
        unsafe extern "C" fn(
            *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
        ) -> ::core::ffi::c_int,
    >,
    pub xOpen: Option<
        unsafe extern "C" fn(
            *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xClose: Option<
        unsafe extern "C" fn(
            *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub xNext: Option<
        unsafe extern "C" fn(
            *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub xLanguageid: Option<
        unsafe extern "C" fn(
            *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer {
    pub pModule: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3_tokenizer_cursor {
    pub pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
}
pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_ptrdiff_t_h::PtrdiffT;
pub use crate::__stddef_size_t_h::SizeT;

pub use crate::src::ext::fts3::fts3_hash::_fts3ht;
pub use crate::src::ext::fts3::fts3_hash::Fts3Hash;
pub use crate::src::ext::fts3::fts3_hash::Fts3HashElem;
pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFind;
pub use crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashInsert;
pub use crate::src::headers::sqlite3_h::SQLITE_BLOB;
pub use crate::src::headers::sqlite3_h::SQLITE_DIRECTONLY;
pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;
pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;
pub use crate::src::headers::sqlite3_h::SQLITE_OK;
pub use crate::src::headers::sqlite3_h::SQLITE_ROW;
pub use crate::src::headers::sqlite3_h::SQLITE_STATIC;
pub use crate::src::headers::sqlite3_h::SQLITE_UTF8;
pub use crate::src::headers::sqlite3_h::SqliteInt64;
pub use crate::src::headers::sqlite3_h::SqliteUint64;
pub use crate::src::headers::sqlite3_h::Sqlite3DestructorType;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::Sqlite3Stmt;
pub use crate::src::headers::sqlite3_h::Sqlite3Uint64;
pub use crate::src::headers::sqliteInt_h::sqlite3;
pub use crate::src::headers::stdlib::C2RustUnnamed;
pub use crate::src::headers::stdlib::C2RustUnnamed_0;
pub use crate::src::headers::vdbeInt_h::sqlite3_context;
pub use crate::src::headers::vdbeInt_h::sqlite3_value;
pub use crate::src::src::main::sqlite3_create_function;
pub use crate::src::src::malloc::sqlite3_free;
pub use crate::src::src::malloc::sqlite3_realloc64;
pub use crate::src::src::prepare::sqlite3_prepare_v2;
pub use crate::src::src::vdbeapi::sqlite3_bind_blob;
pub use crate::src::src::vdbeapi::sqlite3_bind_text;
pub use crate::src::src::vdbeapi::sqlite3_column_blob;
pub use crate::src::src::vdbeapi::sqlite3_column_bytes;
pub use crate::src::src::vdbeapi::sqlite3_column_type;
pub use crate::src::src::vdbeapi::sqlite3_context_db_handle;
pub use crate::src::src::vdbeapi::sqlite3_finalize;
pub use crate::src::src::vdbeapi::sqlite3_result_blob;
pub use crate::src::src::vdbeapi::sqlite3_result_error;
pub use crate::src::src::vdbeapi::sqlite3_result_text;
pub use crate::src::src::vdbeapi::sqlite3_step;
pub use crate::src::src::vdbeapi::sqlite3_user_data;
pub use crate::src::src::vdbeapi::sqlite3_value_blob;
pub use crate::src::src::vdbeapi::sqlite3_value_bytes;
pub use crate::src::src::vdbeapi::sqlite3_value_frombind;
pub use crate::src::src::vdbeapi::sqlite3_value_text;

#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclDupInternalRepProc;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclFreeInternalRepProc;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclInterp;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::Tcl_Obj;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::Tcl_ObjInternalRep;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::Tcl_ObjType;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclObjTypeGetElements;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclObjTypeInOperatorProc;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclObjTypeIndexProc;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclObjTypeLengthProc;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclObjTypeReplaceProc;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclObjTypeReverseProc;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclObjTypeSetElement;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclObjTypeSliceProc;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclSetFromAnyProc;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclSize;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclUpdateStringProc;
#[cfg(feature = "test")]
pub use crate::src::headers::stdlib::TclWideInt;
pub use crate::src::ext::fts3::fts3_tokenizer1::sqlite3Fts3SimpleTokenizerModule;

unsafe extern "C" fn fts3TokenizerEnabled(
    context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
) -> ::core::ffi::c_int {
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 =
        crate::src::src::vdbeapi::sqlite3_context_db_handle(context);
    let mut isEnabled: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let args: [u64; 3] = [
        -(1 as ::core::ffi::c_int) as u64,
        (&raw mut isEnabled) as usize as u64,
        0,
    ];
    crate::src::src::main::sqlite3_db_config_args(
        db,
        crate::src::headers::sqlite3_h::SqliteDbConfig::EnableFts3Tokenizer as ::core::ffi::c_int,
        args.as_ptr(),
    );
    isEnabled
}

unsafe extern "C" fn fts3TokenizerFunc(
    context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    argc: ::core::ffi::c_int,
    argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    
    let mut pPtr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    
    
    let pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash = crate::src::src::vdbeapi::sqlite3_user_data(context)
        as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash;
    let zName: *const ::core::ffi::c_uchar = crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(0_isize));
    let nName: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_value_bytes(*argv.offset(0_isize))
        + 1 as ::core::ffi::c_int;
    if argc == 2 as ::core::ffi::c_int {
        if fts3TokenizerEnabled(context) != 0
            || crate::src::src::vdbeapi::sqlite3_value_frombind(*argv.offset(1_isize)) != 0
        {
            
            let n: ::core::ffi::c_int =
                crate::src::src::vdbeapi::sqlite3_value_bytes(*argv.offset(1_isize));
            if zName.is_null()
                || n as usize != ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
            {
                crate::src::src::vdbeapi::sqlite3_result_error(
                    context,
                    b"argument type mismatch\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int),
                );
                return;
            }
            pPtr = *(crate::src::src::vdbeapi::sqlite3_value_blob(*argv.offset(1_isize))
                as *mut *mut ::core::ffi::c_void);
            let pOld: *mut ::core::ffi::c_void = crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashInsert(
                pHash as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
                zName as *mut ::core::ffi::c_void,
                nName,
                pPtr,
            );
            if pOld == pPtr {
                crate::src::src::vdbeapi::sqlite3_result_error(
                    context,
                    b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int),
                );
            }
        } else {
            crate::src::src::vdbeapi::sqlite3_result_error(
                context,
                b"fts3tokenize disabled\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int),
            );
            return;
        }
    } else {
        if !zName.is_null() {
            pPtr = crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFind(
                pHash as *const crate::src::ext::fts3::fts3_hash::Fts3Hash,
                zName as *const ::core::ffi::c_void,
                nName,
            );
        }
        if pPtr.is_null() {
            let zErr: *mut ::core::ffi::c_char =
                crate::sqlite_printf!("unknown tokenizer: %s", zName as *const ::core::ffi::c_char);
            crate::src::src::vdbeapi::sqlite3_result_error(
                context,
                zErr,
                -(1 as ::core::ffi::c_int),
            );
            crate::src::src::malloc::sqlite3_free(zErr as *mut ::core::ffi::c_void);
            return;
        }
    }
    if fts3TokenizerEnabled(context) != 0
        || crate::src::src::vdbeapi::sqlite3_value_frombind(*argv.offset(0_isize)) != 0
    {
        crate::src::src::vdbeapi::sqlite3_result_blob(
            context,
            &raw mut pPtr as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<*mut ::core::ffi::c_void>() as ::core::ffi::c_int,
            ::core::mem::transmute::<
                crate::src::headers::stdlib::IntptrT,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
        );
    }
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3Fts3IsIdChar(c: ::core::ffi::c_char) -> ::core::ffi::c_int {
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
    (c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0
        || isFtsIdChar[c as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3Fts3NextToken(
    zStr: *const ::core::ffi::c_char,
    pn: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut z1: *const ::core::ffi::c_char;
    let mut z2: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    z1 = zStr;
    while z2.is_null() {
        let c: ::core::ffi::c_char = *z1;
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
                z2 = z1.offset(1_isize) as *const ::core::ffi::c_char;
                while *z2 as ::core::ffi::c_int != 0
                    && *z2.offset(0_isize) as ::core::ffi::c_int != ']' as i32
                {
                    z2 = z2.offset(1);
                }
                if *z2 != 0 {
                    z2 = z2.offset(1);
                }
            }
            _ => {
                if sqlite3Fts3IsIdChar(*z1) != 0 {
                    z2 = z1.offset(1_isize) as *const ::core::ffi::c_char;
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
    z1
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3Fts3InitTokenizer(
    pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
    zArg: *const ::core::ffi::c_char,
    ppTok: *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
    pzErr: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let rc: ::core::ffi::c_int;
    let mut z: *mut ::core::ffi::c_char;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    
    
    
    let zCopy: *mut ::core::ffi::c_char = crate::sqlite_printf!("%s", zArg);
    if zCopy.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    let zEnd: *mut ::core::ffi::c_char = zCopy.add((::libc::strlen
        as unsafe extern "C" fn(*const ::core::ffi::c_char) -> crate::__stddef_size_t_h::SizeT)(
        zCopy,
    )) as *mut ::core::ffi::c_char;
    z = sqlite3Fts3NextToken(zCopy, &raw mut n) as *mut ::core::ffi::c_char;
    if z.is_null() {
        z = zCopy;
    }
    *z.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    crate::src::ext::fts3::fts3::sqlite3Fts3Dequote(z);
    let m: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFind(
        pHash as *const crate::src::ext::fts3::fts3_hash::Fts3Hash,
        z as *const ::core::ffi::c_void,
        ::libc::strlen(z) as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
    ) as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;
    if m.is_null() {
        crate::src::ext::fts3::fts3::sqlite3Fts3ErrMsg(
            pzErr,
            b"unknown tokenizer: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::src::src::printf::PrintfArg::Str(
                z as *mut ::core::ffi::c_char,
            )],
        );
        rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
    } else {
        let mut aArg: *mut *const ::core::ffi::c_char =
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        let mut iArg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        z = z.offset((n + 1 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_char;
        while z < zEnd && {
            z = sqlite3Fts3NextToken(z, &raw mut n) as *mut ::core::ffi::c_char;
            !z.is_null()
        } {
            let nNew: crate::src::headers::sqlite3_h::Sqlite3Int64 =
                (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                    .wrapping_mul((iArg + 1 as ::core::ffi::c_int) as usize)
                    as crate::src::headers::sqlite3_h::Sqlite3Int64;
            let aNew: *mut *const ::core::ffi::c_char =
                crate::src::src::malloc::sqlite3_realloc64(
                    aArg as *mut ::core::ffi::c_void,
                    nNew as crate::src::headers::sqlite3_h::Sqlite3Uint64,
                ) as *mut *const ::core::ffi::c_char;
            if aNew.is_null() {
                crate::src::src::malloc::sqlite3_free(zCopy as *mut ::core::ffi::c_void);
                crate::src::src::malloc::sqlite3_free(aArg as *mut ::core::ffi::c_void);
                return crate::src::headers::sqlite3_h::SQLITE_NOMEM;
            }
            aArg = aNew;
            let fresh1 = iArg;
            iArg += 1;
            let fresh2 = &mut *aArg.offset(fresh1 as isize);
            *fresh2 = z;
            *z.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
            crate::src::ext::fts3::fts3::sqlite3Fts3Dequote(z);
            z = z.offset((n + 1 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_char;
        }
        rc = (*m).xCreate.expect("non-null function pointer")(iArg, aArg, ppTok);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            crate::src::ext::fts3::fts3::sqlite3Fts3ErrMsg(
                pzErr,
                b"unknown tokenizer\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        } else {
            (**ppTok).pModule = m;
        }
        crate::src::src::malloc::sqlite3_free(aArg as *mut ::core::ffi::c_void);
    }
    crate::src::src::malloc::sqlite3_free(zCopy as *mut ::core::ffi::c_void);
    rc
}

#[cfg(feature = "test")]
unsafe extern "C" fn testFunc(
    context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    argc: ::core::ffi::c_int,
    argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    
    
    let mut pTokenizer: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer =
        ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer>();
    let mut pCsr: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor =
        ::core::ptr::null_mut::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor>();
    let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    
    
    
    
    let mut azArg: [*const ::core::ffi::c_char; 64] =
        [::core::ptr::null::<::core::ffi::c_char>(); 64];
    let mut zToken: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nToken: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iStart: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iEnd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut iPos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int;
    
    if argc < 2 as ::core::ffi::c_int {
        crate::src::src::vdbeapi::sqlite3_result_error(
            context,
            b"insufficient arguments\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int),
        );
        return;
    }
    let nName: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_value_bytes(*argv.offset(0_isize));
    let zName: *const ::core::ffi::c_char = crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(0_isize))
        as *const ::core::ffi::c_char;
    let nInput: ::core::ffi::c_int = crate::src::src::vdbeapi::sqlite3_value_bytes(
        *argv.offset((argc - 1 as ::core::ffi::c_int) as isize),
    );
    let zInput: *const ::core::ffi::c_char = crate::src::src::vdbeapi::sqlite3_value_text(
        *argv.offset((argc - 1 as ::core::ffi::c_int) as isize),
    ) as *const ::core::ffi::c_char;
    let pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash = crate::src::src::vdbeapi::sqlite3_user_data(context)
        as *mut crate::src::ext::fts3::fts3_hash::Fts3Hash;
    let p: *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module = crate::src::ext::fts3::fts3_hash::sqlite3Fts3HashFind(
        pHash as *const crate::src::ext::fts3::fts3_hash::Fts3Hash,
        zName as *const ::core::ffi::c_void,
        nName + 1 as ::core::ffi::c_int,
    ) as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module;
    if p.is_null() {
        let zErr2: *mut ::core::ffi::c_char =
            crate::sqlite_printf!("unknown tokenizer: %s", zName);
        crate::src::src::vdbeapi::sqlite3_result_error(context, zErr2, -(1 as ::core::ffi::c_int));
        crate::src::src::malloc::sqlite3_free(zErr2 as *mut ::core::ffi::c_void);
        return;
    }
    let pRet: *mut crate::src::headers::stdlib::Tcl_Obj = crate::src::headers::stdlib::Tcl_NewObj();
    (*pRet).refCount += 1;
    i = 1 as ::core::ffi::c_int;
    while i < argc - 1 as ::core::ffi::c_int {
        azArg[(i - 1 as ::core::ffi::c_int) as usize] =
            crate::src::src::vdbeapi::sqlite3_value_text(*argv.offset(i as isize))
                as *const ::core::ffi::c_char;
        i += 1;
    }
    if crate::src::headers::sqlite3_h::SQLITE_OK
        != (*p).xCreate.expect("non-null function pointer")(
            argc - 2 as ::core::ffi::c_int,
            &raw mut azArg as *mut *const ::core::ffi::c_char,
            &raw mut pTokenizer,
        )
    {
        zErr = b"error in xCreate()\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        (*pTokenizer).pModule = p;
        if crate::src::ext::fts3::fts3_expr::sqlite3Fts3OpenTokenizer(
            pTokenizer as *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer,
            0 as ::core::ffi::c_int,
            zInput,
            nInput,
            &raw mut pCsr as *mut _
                as *mut *mut crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_cursor,
        ) != 0
        {
            zErr = b"error in xOpen()\0" as *const u8 as *const ::core::ffi::c_char;
        } else {
            let __p_ref = unsafe { &mut *p };
            while crate::src::headers::sqlite3_h::SQLITE_OK
                == __p_ref.xNext.expect("non-null function pointer")(
                    pCsr,
                    &raw mut zToken,
                    &raw mut nToken,
                    &raw mut iStart,
                    &raw mut iEnd,
                    &raw mut iPos,
                )
            {
                crate::src::headers::stdlib::Tcl_ListObjAppendElement(
                    ::core::ptr::null_mut::<crate::src::headers::stdlib::TclInterp>(),
                    pRet,
                    crate::src::headers::stdlib::Tcl_NewWideIntObj(
                        iPos as crate::src::headers::stdlib::TclWideInt,
                    ),
                );
                crate::src::headers::stdlib::Tcl_ListObjAppendElement(
                    ::core::ptr::null_mut::<crate::src::headers::stdlib::TclInterp>(),
                    pRet,
                    crate::src::headers::stdlib::Tcl_NewStringObj(
                        zToken,
                        nToken as crate::src::headers::stdlib::TclSize,
                    ),
                );
                zToken = zInput.offset(iStart as isize) as *const ::core::ffi::c_char;
                nToken = iEnd - iStart;
                crate::src::headers::stdlib::Tcl_ListObjAppendElement(
                    ::core::ptr::null_mut::<crate::src::headers::stdlib::TclInterp>(),
                    pRet,
                    crate::src::headers::stdlib::Tcl_NewStringObj(
                        zToken,
                        nToken as crate::src::headers::stdlib::TclSize,
                    ),
                );
            }
            if crate::src::headers::sqlite3_h::SQLITE_OK
                != __p_ref.xClose.expect("non-null function pointer")(pCsr)
            {
                zErr = b"error in xClose()\0" as *const u8 as *const ::core::ffi::c_char;
            } else if crate::src::headers::sqlite3_h::SQLITE_OK
                != __p_ref.xDestroy.expect("non-null function pointer")(pTokenizer)
            {
                zErr = b"error in xDestroy()\0" as *const u8 as *const ::core::ffi::c_char;
            }
        }
    }
    if !zErr.is_null() {
        crate::src::src::vdbeapi::sqlite3_result_error(context, zErr, -(1 as ::core::ffi::c_int));
    } else {
        crate::src::src::vdbeapi::sqlite3_result_text(
            context,
            crate::src::headers::stdlib::Tcl_GetStringFromObj(
                pRet,
                crate::__stddef_null_h::NULL as *mut crate::src::headers::stdlib::TclSize,
            ),
            -(1 as ::core::ffi::c_int),
            ::core::mem::transmute::<
                crate::src::headers::stdlib::IntptrT,
                Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            >(-(1 as ::core::ffi::c_int) as crate::src::headers::stdlib::IntptrT),
        );
    }
    let mut _objPtr: *mut crate::src::headers::stdlib::Tcl_Obj = pRet;
    let ___objPtr_ref = unsafe { &mut *_objPtr };
    let fresh0 = ___objPtr_ref.refCount;
    ___objPtr_ref.refCount -= 1;
    if fresh0 <= 1 as crate::src::headers::stdlib::TclSize {
        crate::src::headers::stdlib::TclFreeObj(_objPtr);
    }
}
unsafe extern "C" fn registerTokenizer(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zName: *mut ::core::ffi::c_char,
    mut p: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module,
) -> ::core::ffi::c_int {
    
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let zSql: [::core::ffi::c_char; 28] = ::core::mem::transmute::<
        [u8; 28],
        [::core::ffi::c_char; 28],
    >(*b"SELECT fts3_tokenizer(?, ?)\0");
    let rc: ::core::ffi::c_int = crate::src::src::prepare::sqlite3_prepare_v2(
        db,
        &raw const zSql as *const ::core::ffi::c_char,
        -(1 as ::core::ffi::c_int),
        &raw mut pStmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    crate::src::src::vdbeapi::sqlite3_bind_text(
        pStmt,
        1 as ::core::ffi::c_int,
        zName,
        -(1 as ::core::ffi::c_int),
        crate::src::headers::sqlite3_h::SQLITE_STATIC,
    );
    crate::src::src::vdbeapi::sqlite3_bind_blob(
        pStmt,
        2 as ::core::ffi::c_int,
        &raw mut p as *const ::core::ffi::c_void,
        ::core::mem::size_of::<*const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module>(
        ) as ::core::ffi::c_int,
        crate::src::headers::sqlite3_h::SQLITE_STATIC,
    );
    crate::src::src::vdbeapi::sqlite3_step(pStmt);
    crate::src::src::vdbeapi::sqlite3_finalize(pStmt)
}

unsafe extern "C" fn queryTokenizer(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    zName: *mut ::core::ffi::c_char,
    pp: *mut *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module,
) -> ::core::ffi::c_int {
    
    let mut pStmt: *mut crate::src::headers::sqlite3_h::Sqlite3Stmt =
        ::core::ptr::null_mut::<crate::src::headers::sqlite3_h::Sqlite3Stmt>();
    let zSql: [::core::ffi::c_char; 25] = ::core::mem::transmute::<
        [u8; 25],
        [::core::ffi::c_char; 25],
    >(*b"SELECT fts3_tokenizer(?)\0");
    *pp = ::core::ptr::null::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module>();
    let rc: ::core::ffi::c_int = crate::src::src::prepare::sqlite3_prepare_v2(
        db,
        &raw const zSql as *const ::core::ffi::c_char,
        -(1 as ::core::ffi::c_int),
        &raw mut pStmt,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    );
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        return rc;
    }
    crate::src::src::vdbeapi::sqlite3_bind_text(
        pStmt,
        1 as ::core::ffi::c_int,
        zName,
        -(1 as ::core::ffi::c_int),
        crate::src::headers::sqlite3_h::SQLITE_STATIC,
    );
    if crate::src::headers::sqlite3_h::SQLITE_ROW == crate::src::src::vdbeapi::sqlite3_step(pStmt)
        && crate::src::src::vdbeapi::sqlite3_column_type(pStmt, 0 as ::core::ffi::c_int)
            == crate::src::headers::sqlite3_h::SQLITE_BLOB
            && crate::src::src::vdbeapi::sqlite3_column_bytes(pStmt, 0 as ::core::ffi::c_int)
                as usize
                == ::core::mem::size_of::<
                    *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module,
                >() as usize
        {
            ::libc::memcpy(
                pp as *mut ::core::ffi::c_void,
                crate::src::src::vdbeapi::sqlite3_column_blob(pStmt, 0 as ::core::ffi::c_int),
                ::core::mem::size_of::<
                    *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module,
                >() as crate::__stddef_size_t_h::SizeT,
            );
        }
    crate::src::src::vdbeapi::sqlite3_finalize(pStmt)
}

unsafe extern "C" fn intTestFunc(
    context: *mut crate::src::headers::vdbeInt_h::sqlite3_context,
    mut _argc: ::core::ffi::c_int,
    mut _argv: *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
) {
    let rc: ::core::ffi::c_int = 0;
    let p1: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module =
        ::core::ptr::null::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module>();
    let mut p2: *const crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module =
        ::core::ptr::null::<crate::src::ext::fts3::fts3_tokenizer::sqlite3_tokenizer_module>();
    let db: *mut crate::src::headers::sqliteInt_h::sqlite3 =
        ::core::ptr::null_mut::<crate::src::headers::sqliteInt_h::sqlite3>();
    if fts3TokenizerEnabled(context) != 0 {
        registerTokenizer(
            db,
            b"nosuchtokenizer\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            p1,
        );
        queryTokenizer(
            db,
            b"nosuchtokenizer\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw mut p2,
        );
    }
    crate::src::src::vdbeapi::sqlite3_result_text(
        context,
        b"ok\0" as *const u8 as *const ::core::ffi::c_char,
        -(1 as ::core::ffi::c_int),
        crate::src::headers::sqlite3_h::SQLITE_STATIC,
    );
}

#[cfg_attr(feature = "test", unsafe(no_mangle))]
pub unsafe extern "C" fn sqlite3Fts3InitHashTable(
    db: *mut crate::src::headers::sqliteInt_h::sqlite3,
    pHash: *mut crate::src::ext::fts3::fts3_hash::Fts3Hash,
    zName: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let p: *mut ::core::ffi::c_void = pHash as *mut ::core::ffi::c_void;
    let any: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_UTF8
        | crate::src::headers::sqlite3_h::SQLITE_DIRECTONLY;
    
    
    let pdb: *mut ::core::ffi::c_void = db as *mut ::core::ffi::c_void;
    let zTest: *mut ::core::ffi::c_char = crate::sqlite_printf!("%s_test", zName);
    let zTest2: *mut ::core::ffi::c_char = crate::sqlite_printf!("%s_internal_test", zName);
    if zTest.is_null() || zTest2.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_NOMEM;
    }
    if crate::src::headers::sqlite3_h::SQLITE_OK == rc {
        rc = crate::src::src::main::sqlite3_create_function(
            db,
            zName,
            1 as ::core::ffi::c_int,
            any,
            p,
            Some(
                fts3TokenizerFunc
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
    }
    if crate::src::headers::sqlite3_h::SQLITE_OK == rc {
        rc = crate::src::src::main::sqlite3_create_function(
            db,
            zName,
            2 as ::core::ffi::c_int,
            any,
            p,
            Some(
                fts3TokenizerFunc
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
    }
    if crate::src::headers::sqlite3_h::SQLITE_OK == rc {
        #[cfg(feature = "test")]
        {
            rc = crate::src::src::main::sqlite3_create_function(
                db,
                zTest,
                -(1 as ::core::ffi::c_int),
                any,
                p,
                Some(
                    testFunc
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                            ::core::ffi::c_int,
                            *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                        ) -> (),
                ),
                None,
                None,
            );
        }

        #[cfg(not(feature = "test"))]
        {
            rc = crate::src::src::main::sqlite3_create_function(
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
    if crate::src::headers::sqlite3_h::SQLITE_OK == rc {
        rc = crate::src::src::main::sqlite3_create_function(
            db,
            zTest2,
            0 as ::core::ffi::c_int,
            any,
            pdb,
            Some(
                intTestFunc
                    as unsafe extern "C" fn(
                        *mut crate::src::headers::vdbeInt_h::sqlite3_context,
                        ::core::ffi::c_int,
                        *mut *mut crate::src::headers::vdbeInt_h::sqlite3_value,
                    ) -> (),
            ),
            None,
            None,
        );
    }
    crate::src::src::malloc::sqlite3_free(zTest as *mut ::core::ffi::c_void);
    crate::src::src::malloc::sqlite3_free(zTest2 as *mut ::core::ffi::c_void);
    rc
}
