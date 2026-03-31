#[derive(Copy, Clone)]
#[repr(C)]

pub struct BenignMallocHooks {
    pub xBenignBegin: Option<unsafe extern "C" fn() -> ()>,
    pub xBenignEnd: Option<unsafe extern "C" fn() -> ()>,
}

static mut sqlite3Hooks: BenignMallocHooks = BenignMallocHooks {
    xBenignBegin: None,
    xBenignEnd: None,
};
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3BenignMallocHooks(
    mut xBenignBegin: Option<unsafe extern "C" fn() -> ()>,
    mut xBenignEnd: Option<unsafe extern "C" fn() -> ()>,
) {
    sqlite3Hooks.xBenignBegin = xBenignBegin;
    sqlite3Hooks.xBenignEnd = xBenignEnd;
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3BeginBenignMalloc() {
    if sqlite3Hooks.xBenignBegin.is_some() {
        sqlite3Hooks
            .xBenignBegin
            .expect("non-null function pointer")();
    }
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3EndBenignMalloc() {
    if sqlite3Hooks.xBenignEnd.is_some() {
        sqlite3Hooks.xBenignEnd.expect("non-null function pointer")();
    }
}
