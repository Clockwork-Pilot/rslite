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
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BenignMallocHooks(
    mut xBenignBegin: Option<unsafe extern "C" fn() -> ()>,
    mut xBenignEnd: Option<unsafe extern "C" fn() -> ()>,
) {
    sqlite3Hooks.xBenignBegin = xBenignBegin;
    sqlite3Hooks.xBenignEnd = xBenignEnd;
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3BeginBenignMalloc() {
    if (&raw const sqlite3Hooks.xBenignBegin).read().is_some() {
        (&raw const sqlite3Hooks.xBenignBegin)
            .read()
            .expect("non-null function pointer")();
    }
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3EndBenignMalloc() {
    if (&raw const sqlite3Hooks.xBenignEnd).read().is_some() {
        (&raw const sqlite3Hooks.xBenignEnd)
            .read()
            .expect("non-null function pointer")();
    }
}
