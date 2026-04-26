pub use crate::__stddef_size_t_h::SizeT;

pub use crate::src::ext::rtree::rtree::U8_0;
pub use crate::src::ext::rtree::rtree::U32_0;
pub use crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_PRNG;
pub use crate::src::headers::sqlite3_h::SqliteInt64;
pub use crate::src::headers::sqlite3_h::sqlite3_file;
pub use crate::src::headers::sqlite3_h::Sqlite3Filename;
pub use crate::src::headers::sqlite3_h::Sqlite3Int64;
pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;
pub use crate::src::headers::sqlite3_h::Sqlite3SyscallPtr;
pub use crate::src::headers::sqlite3_h::sqlite3_vfs;
pub use crate::src::headers::stdlib::Uint8T;
pub use crate::src::headers::stdlib::Uint32T;
pub use crate::src::src::main::sqlite3_initialize;
pub use crate::src::src::mutex::sqlite3_mutex_enter;
pub use crate::src::src::mutex::sqlite3_mutex_leave;
pub use crate::src::src::mutex::sqlite3MutexAlloc;
pub use crate::src::src::mutex_unix::sqlite3_mutex;
pub use crate::src::src::os::sqlite3_vfs_find;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sqlite3PrngType {
    pub s: [crate::src::ext::rtree::rtree::U32_0; 16],
    pub out: [crate::src::ext::rtree::rtree::U8_0; 64],
    pub n: crate::src::ext::rtree::rtree::U8_0,
}

static mut sqlite3Prng: sqlite3PrngType = sqlite3PrngType {
    s: [0; 16],
    out: [0; 64],
    n: 0,
};

unsafe extern "C" fn chacha_block(
    out: *mut crate::src::ext::rtree::rtree::U32_0,
    in_0: *const crate::src::ext::rtree::rtree::U32_0,
) {
    let mut i: ::core::ffi::c_int;
    let mut x: [crate::src::ext::rtree::rtree::U32_0; 16] = [0; 16];
    ::core::ptr::copy_nonoverlapping(
        in_0 as *const u8,
        &raw mut x as *mut crate::src::ext::rtree::rtree::U32_0 as *mut u8,
        64 as usize,
    );
    i = 0 as ::core::ffi::c_int;
    while i < 10 as ::core::ffi::c_int {
        x[0 as ::core::ffi::c_int as usize] =
            x[0 as ::core::ffi::c_int as usize].wrapping_add(x[4 as ::core::ffi::c_int as usize]);
        x[12 as ::core::ffi::c_int as usize] ^= x[0 as ::core::ffi::c_int as usize];
        x[12 as ::core::ffi::c_int as usize] = x[12 as ::core::ffi::c_int as usize]
            << 16 as ::core::ffi::c_int
            | x[12 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        x[8 as ::core::ffi::c_int as usize] =
            x[8 as ::core::ffi::c_int as usize].wrapping_add(x[12 as ::core::ffi::c_int as usize]);
        x[4 as ::core::ffi::c_int as usize] ^= x[8 as ::core::ffi::c_int as usize];
        x[4 as ::core::ffi::c_int as usize] = x[4 as ::core::ffi::c_int as usize]
            << 12 as ::core::ffi::c_int
            | x[4 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        x[0 as ::core::ffi::c_int as usize] =
            x[0 as ::core::ffi::c_int as usize].wrapping_add(x[4 as ::core::ffi::c_int as usize]);
        x[12 as ::core::ffi::c_int as usize] ^= x[0 as ::core::ffi::c_int as usize];
        x[12 as ::core::ffi::c_int as usize] = x[12 as ::core::ffi::c_int as usize]
            << 8 as ::core::ffi::c_int
            | x[12 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
        x[8 as ::core::ffi::c_int as usize] =
            x[8 as ::core::ffi::c_int as usize].wrapping_add(x[12 as ::core::ffi::c_int as usize]);
        x[4 as ::core::ffi::c_int as usize] ^= x[8 as ::core::ffi::c_int as usize];
        x[4 as ::core::ffi::c_int as usize] = x[4 as ::core::ffi::c_int as usize]
            << 7 as ::core::ffi::c_int
            | x[4 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        x[1 as ::core::ffi::c_int as usize] =
            x[1 as ::core::ffi::c_int as usize].wrapping_add(x[5 as ::core::ffi::c_int as usize]);
        x[13 as ::core::ffi::c_int as usize] ^= x[1 as ::core::ffi::c_int as usize];
        x[13 as ::core::ffi::c_int as usize] = x[13 as ::core::ffi::c_int as usize]
            << 16 as ::core::ffi::c_int
            | x[13 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        x[9 as ::core::ffi::c_int as usize] =
            x[9 as ::core::ffi::c_int as usize].wrapping_add(x[13 as ::core::ffi::c_int as usize]);
        x[5 as ::core::ffi::c_int as usize] ^= x[9 as ::core::ffi::c_int as usize];
        x[5 as ::core::ffi::c_int as usize] = x[5 as ::core::ffi::c_int as usize]
            << 12 as ::core::ffi::c_int
            | x[5 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        x[1 as ::core::ffi::c_int as usize] =
            x[1 as ::core::ffi::c_int as usize].wrapping_add(x[5 as ::core::ffi::c_int as usize]);
        x[13 as ::core::ffi::c_int as usize] ^= x[1 as ::core::ffi::c_int as usize];
        x[13 as ::core::ffi::c_int as usize] = x[13 as ::core::ffi::c_int as usize]
            << 8 as ::core::ffi::c_int
            | x[13 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
        x[9 as ::core::ffi::c_int as usize] =
            x[9 as ::core::ffi::c_int as usize].wrapping_add(x[13 as ::core::ffi::c_int as usize]);
        x[5 as ::core::ffi::c_int as usize] ^= x[9 as ::core::ffi::c_int as usize];
        x[5 as ::core::ffi::c_int as usize] = x[5 as ::core::ffi::c_int as usize]
            << 7 as ::core::ffi::c_int
            | x[5 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        x[2 as ::core::ffi::c_int as usize] =
            x[2 as ::core::ffi::c_int as usize].wrapping_add(x[6 as ::core::ffi::c_int as usize]);
        x[14 as ::core::ffi::c_int as usize] ^= x[2 as ::core::ffi::c_int as usize];
        x[14 as ::core::ffi::c_int as usize] = x[14 as ::core::ffi::c_int as usize]
            << 16 as ::core::ffi::c_int
            | x[14 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        x[10 as ::core::ffi::c_int as usize] =
            x[10 as ::core::ffi::c_int as usize].wrapping_add(x[14 as ::core::ffi::c_int as usize]);
        x[6 as ::core::ffi::c_int as usize] ^= x[10 as ::core::ffi::c_int as usize];
        x[6 as ::core::ffi::c_int as usize] = x[6 as ::core::ffi::c_int as usize]
            << 12 as ::core::ffi::c_int
            | x[6 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        x[2 as ::core::ffi::c_int as usize] =
            x[2 as ::core::ffi::c_int as usize].wrapping_add(x[6 as ::core::ffi::c_int as usize]);
        x[14 as ::core::ffi::c_int as usize] ^= x[2 as ::core::ffi::c_int as usize];
        x[14 as ::core::ffi::c_int as usize] = x[14 as ::core::ffi::c_int as usize]
            << 8 as ::core::ffi::c_int
            | x[14 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
        x[10 as ::core::ffi::c_int as usize] =
            x[10 as ::core::ffi::c_int as usize].wrapping_add(x[14 as ::core::ffi::c_int as usize]);
        x[6 as ::core::ffi::c_int as usize] ^= x[10 as ::core::ffi::c_int as usize];
        x[6 as ::core::ffi::c_int as usize] = x[6 as ::core::ffi::c_int as usize]
            << 7 as ::core::ffi::c_int
            | x[6 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        x[3 as ::core::ffi::c_int as usize] =
            x[3 as ::core::ffi::c_int as usize].wrapping_add(x[7 as ::core::ffi::c_int as usize]);
        x[15 as ::core::ffi::c_int as usize] ^= x[3 as ::core::ffi::c_int as usize];
        x[15 as ::core::ffi::c_int as usize] = x[15 as ::core::ffi::c_int as usize]
            << 16 as ::core::ffi::c_int
            | x[15 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        x[11 as ::core::ffi::c_int as usize] =
            x[11 as ::core::ffi::c_int as usize].wrapping_add(x[15 as ::core::ffi::c_int as usize]);
        x[7 as ::core::ffi::c_int as usize] ^= x[11 as ::core::ffi::c_int as usize];
        x[7 as ::core::ffi::c_int as usize] = x[7 as ::core::ffi::c_int as usize]
            << 12 as ::core::ffi::c_int
            | x[7 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        x[3 as ::core::ffi::c_int as usize] =
            x[3 as ::core::ffi::c_int as usize].wrapping_add(x[7 as ::core::ffi::c_int as usize]);
        x[15 as ::core::ffi::c_int as usize] ^= x[3 as ::core::ffi::c_int as usize];
        x[15 as ::core::ffi::c_int as usize] = x[15 as ::core::ffi::c_int as usize]
            << 8 as ::core::ffi::c_int
            | x[15 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
        x[11 as ::core::ffi::c_int as usize] =
            x[11 as ::core::ffi::c_int as usize].wrapping_add(x[15 as ::core::ffi::c_int as usize]);
        x[7 as ::core::ffi::c_int as usize] ^= x[11 as ::core::ffi::c_int as usize];
        x[7 as ::core::ffi::c_int as usize] = x[7 as ::core::ffi::c_int as usize]
            << 7 as ::core::ffi::c_int
            | x[7 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        x[0 as ::core::ffi::c_int as usize] =
            x[0 as ::core::ffi::c_int as usize].wrapping_add(x[5 as ::core::ffi::c_int as usize]);
        x[15 as ::core::ffi::c_int as usize] ^= x[0 as ::core::ffi::c_int as usize];
        x[15 as ::core::ffi::c_int as usize] = x[15 as ::core::ffi::c_int as usize]
            << 16 as ::core::ffi::c_int
            | x[15 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        x[10 as ::core::ffi::c_int as usize] =
            x[10 as ::core::ffi::c_int as usize].wrapping_add(x[15 as ::core::ffi::c_int as usize]);
        x[5 as ::core::ffi::c_int as usize] ^= x[10 as ::core::ffi::c_int as usize];
        x[5 as ::core::ffi::c_int as usize] = x[5 as ::core::ffi::c_int as usize]
            << 12 as ::core::ffi::c_int
            | x[5 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        x[0 as ::core::ffi::c_int as usize] =
            x[0 as ::core::ffi::c_int as usize].wrapping_add(x[5 as ::core::ffi::c_int as usize]);
        x[15 as ::core::ffi::c_int as usize] ^= x[0 as ::core::ffi::c_int as usize];
        x[15 as ::core::ffi::c_int as usize] = x[15 as ::core::ffi::c_int as usize]
            << 8 as ::core::ffi::c_int
            | x[15 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
        x[10 as ::core::ffi::c_int as usize] =
            x[10 as ::core::ffi::c_int as usize].wrapping_add(x[15 as ::core::ffi::c_int as usize]);
        x[5 as ::core::ffi::c_int as usize] ^= x[10 as ::core::ffi::c_int as usize];
        x[5 as ::core::ffi::c_int as usize] = x[5 as ::core::ffi::c_int as usize]
            << 7 as ::core::ffi::c_int
            | x[5 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        x[1 as ::core::ffi::c_int as usize] =
            x[1 as ::core::ffi::c_int as usize].wrapping_add(x[6 as ::core::ffi::c_int as usize]);
        x[12 as ::core::ffi::c_int as usize] ^= x[1 as ::core::ffi::c_int as usize];
        x[12 as ::core::ffi::c_int as usize] = x[12 as ::core::ffi::c_int as usize]
            << 16 as ::core::ffi::c_int
            | x[12 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        x[11 as ::core::ffi::c_int as usize] =
            x[11 as ::core::ffi::c_int as usize].wrapping_add(x[12 as ::core::ffi::c_int as usize]);
        x[6 as ::core::ffi::c_int as usize] ^= x[11 as ::core::ffi::c_int as usize];
        x[6 as ::core::ffi::c_int as usize] = x[6 as ::core::ffi::c_int as usize]
            << 12 as ::core::ffi::c_int
            | x[6 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        x[1 as ::core::ffi::c_int as usize] =
            x[1 as ::core::ffi::c_int as usize].wrapping_add(x[6 as ::core::ffi::c_int as usize]);
        x[12 as ::core::ffi::c_int as usize] ^= x[1 as ::core::ffi::c_int as usize];
        x[12 as ::core::ffi::c_int as usize] = x[12 as ::core::ffi::c_int as usize]
            << 8 as ::core::ffi::c_int
            | x[12 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
        x[11 as ::core::ffi::c_int as usize] =
            x[11 as ::core::ffi::c_int as usize].wrapping_add(x[12 as ::core::ffi::c_int as usize]);
        x[6 as ::core::ffi::c_int as usize] ^= x[11 as ::core::ffi::c_int as usize];
        x[6 as ::core::ffi::c_int as usize] = x[6 as ::core::ffi::c_int as usize]
            << 7 as ::core::ffi::c_int
            | x[6 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        x[2 as ::core::ffi::c_int as usize] =
            x[2 as ::core::ffi::c_int as usize].wrapping_add(x[7 as ::core::ffi::c_int as usize]);
        x[13 as ::core::ffi::c_int as usize] ^= x[2 as ::core::ffi::c_int as usize];
        x[13 as ::core::ffi::c_int as usize] = x[13 as ::core::ffi::c_int as usize]
            << 16 as ::core::ffi::c_int
            | x[13 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        x[8 as ::core::ffi::c_int as usize] =
            x[8 as ::core::ffi::c_int as usize].wrapping_add(x[13 as ::core::ffi::c_int as usize]);
        x[7 as ::core::ffi::c_int as usize] ^= x[8 as ::core::ffi::c_int as usize];
        x[7 as ::core::ffi::c_int as usize] = x[7 as ::core::ffi::c_int as usize]
            << 12 as ::core::ffi::c_int
            | x[7 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        x[2 as ::core::ffi::c_int as usize] =
            x[2 as ::core::ffi::c_int as usize].wrapping_add(x[7 as ::core::ffi::c_int as usize]);
        x[13 as ::core::ffi::c_int as usize] ^= x[2 as ::core::ffi::c_int as usize];
        x[13 as ::core::ffi::c_int as usize] = x[13 as ::core::ffi::c_int as usize]
            << 8 as ::core::ffi::c_int
            | x[13 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
        x[8 as ::core::ffi::c_int as usize] =
            x[8 as ::core::ffi::c_int as usize].wrapping_add(x[13 as ::core::ffi::c_int as usize]);
        x[7 as ::core::ffi::c_int as usize] ^= x[8 as ::core::ffi::c_int as usize];
        x[7 as ::core::ffi::c_int as usize] = x[7 as ::core::ffi::c_int as usize]
            << 7 as ::core::ffi::c_int
            | x[7 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        x[3 as ::core::ffi::c_int as usize] =
            x[3 as ::core::ffi::c_int as usize].wrapping_add(x[4 as ::core::ffi::c_int as usize]);
        x[14 as ::core::ffi::c_int as usize] ^= x[3 as ::core::ffi::c_int as usize];
        x[14 as ::core::ffi::c_int as usize] = x[14 as ::core::ffi::c_int as usize]
            << 16 as ::core::ffi::c_int
            | x[14 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
        x[9 as ::core::ffi::c_int as usize] =
            x[9 as ::core::ffi::c_int as usize].wrapping_add(x[14 as ::core::ffi::c_int as usize]);
        x[4 as ::core::ffi::c_int as usize] ^= x[9 as ::core::ffi::c_int as usize];
        x[4 as ::core::ffi::c_int as usize] = x[4 as ::core::ffi::c_int as usize]
            << 12 as ::core::ffi::c_int
            | x[4 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
        x[3 as ::core::ffi::c_int as usize] =
            x[3 as ::core::ffi::c_int as usize].wrapping_add(x[4 as ::core::ffi::c_int as usize]);
        x[14 as ::core::ffi::c_int as usize] ^= x[3 as ::core::ffi::c_int as usize];
        x[14 as ::core::ffi::c_int as usize] = x[14 as ::core::ffi::c_int as usize]
            << 8 as ::core::ffi::c_int
            | x[14 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
        x[9 as ::core::ffi::c_int as usize] =
            x[9 as ::core::ffi::c_int as usize].wrapping_add(x[14 as ::core::ffi::c_int as usize]);
        x[4 as ::core::ffi::c_int as usize] ^= x[9 as ::core::ffi::c_int as usize];
        x[4 as ::core::ffi::c_int as usize] = x[4 as ::core::ffi::c_int as usize]
            << 7 as ::core::ffi::c_int
            | x[4 as ::core::ffi::c_int as usize]
                >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        *out.offset(i as isize) = x[i as usize].wrapping_add(*in_0.offset(i as isize));
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_randomness(
    mut N: ::core::ffi::c_int,
    pBuf: *mut ::core::ffi::c_void,
) {
    let mut zBuf: *mut ::core::ffi::c_uchar = pBuf as *mut ::core::ffi::c_uchar;
    let mutex: *mut crate::src::src::mutex_unix::sqlite3_mutex;
    if crate::src::src::main::sqlite3_initialize() != 0 {
        return;
    }
    mutex = crate::src::src::mutex::sqlite3MutexAlloc(
        crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_PRNG,
    );
    crate::src::src::mutex::sqlite3_mutex_enter(mutex);
    if N <= 0 as ::core::ffi::c_int || pBuf.is_null() {
        sqlite3Prng.s[0 as ::core::ffi::c_int as usize] = 0 as crate::src::ext::rtree::rtree::U32_0;
        crate::src::src::mutex::sqlite3_mutex_leave(mutex);
        return;
    }
    if sqlite3Prng.s[0 as ::core::ffi::c_int as usize] == 0 as crate::src::ext::rtree::rtree::U32_0
    {
        let pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs =
            crate::src::src::os::sqlite3_vfs_find(::core::ptr::null::<::core::ffi::c_char>())
                as *mut crate::src::headers::sqlite3_h::sqlite3_vfs;
        static mut chacha20_init: [crate::src::ext::rtree::rtree::U32_0; 4] = [
            0x61707865 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0,
            0x3320646e as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0,
            0x79622d32 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0,
            0x6b206574 as ::core::ffi::c_int as crate::src::ext::rtree::rtree::U32_0,
        ];
        ::core::ptr::copy_nonoverlapping(
            &raw const chacha20_init as *const crate::src::ext::rtree::rtree::U32_0 as *const u8,
            (&raw mut sqlite3Prng.s as *mut crate::src::ext::rtree::rtree::U32_0).offset(0 as isize)
                as *mut crate::src::ext::rtree::rtree::U32_0 as *mut u8,
            16 as usize,
        );
        if pVfs.is_null() {
            ::libc::memset(
                (&raw mut sqlite3Prng.s as *mut crate::src::ext::rtree::rtree::U32_0)
                    .offset(4 as isize) as *mut crate::src::ext::rtree::rtree::U32_0
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                44 as crate::__stddef_size_t_h::SizeT,
            );
        } else {
            crate::src::src::os::sqlite3OsRandomness(
                pVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                44 as ::core::ffi::c_int,
                (&raw mut sqlite3Prng.s as *mut crate::src::ext::rtree::rtree::U32_0)
                    .offset(4 as isize) as *mut crate::src::ext::rtree::rtree::U32_0
                    as *mut ::core::ffi::c_char,
            );
        }
        sqlite3Prng.s[15 as ::core::ffi::c_int as usize] =
            sqlite3Prng.s[12 as ::core::ffi::c_int as usize];
        sqlite3Prng.s[12 as ::core::ffi::c_int as usize] =
            0 as crate::src::ext::rtree::rtree::U32_0;
        sqlite3Prng.n = 0 as crate::src::ext::rtree::rtree::U8_0;
    }
    loop {
        if N <= sqlite3Prng.n as ::core::ffi::c_int {
            ::core::ptr::copy_nonoverlapping(
                (&raw mut sqlite3Prng.out as *mut crate::src::ext::rtree::rtree::U8_0)
                    .offset((sqlite3Prng.n as ::core::ffi::c_int - N) as isize)
                    as *mut crate::src::ext::rtree::rtree::U8_0 as *const u8,
                zBuf as *mut u8,
                N as usize,
            );
            sqlite3Prng.n =
                (sqlite3Prng.n as ::core::ffi::c_int - N) as crate::src::ext::rtree::rtree::U8_0;
            break;
        } else {
            if sqlite3Prng.n as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                ::core::ptr::copy_nonoverlapping(
                    &raw mut sqlite3Prng.out as *mut crate::src::ext::rtree::rtree::U8_0
                        as *const u8,
                    zBuf as *mut u8,
                    sqlite3Prng.n as usize,
                );
                N -= sqlite3Prng.n as ::core::ffi::c_int;
                zBuf = zBuf.offset(sqlite3Prng.n as ::core::ffi::c_int as isize);
            }
            sqlite3Prng.s[12 as ::core::ffi::c_int as usize] =
                sqlite3Prng.s[12 as ::core::ffi::c_int as usize].wrapping_add(1);
            chacha_block(
                &raw mut sqlite3Prng.out as *mut crate::src::ext::rtree::rtree::U8_0
                    as *mut crate::src::ext::rtree::rtree::U32_0,
                &raw mut sqlite3Prng.s as *mut crate::src::ext::rtree::rtree::U32_0,
            );
            sqlite3Prng.n = 64 as crate::src::ext::rtree::rtree::U8_0;
        }
    }
    crate::src::src::mutex::sqlite3_mutex_leave(mutex);
}

static mut sqlite3SavedPrng: sqlite3PrngType = sqlite3PrngType {
    s: [0; 16],
    out: [0; 64],
    n: 0,
};
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3PrngSaveState() {
    ::core::ptr::copy_nonoverlapping(
        &raw mut sqlite3Prng as *const u8,
        &raw mut sqlite3SavedPrng as *mut u8,
        ::core::mem::size_of::<sqlite3PrngType>() as usize,
    );
}
#[cfg_attr(feature = "test", unsafe(no_mangle))]

pub unsafe extern "C" fn sqlite3PrngRestoreState() {
    ::core::ptr::copy_nonoverlapping(
        &raw mut sqlite3SavedPrng as *const u8,
        &raw mut sqlite3Prng as *mut u8,
        ::core::mem::size_of::<sqlite3PrngType>() as usize,
    );
}
