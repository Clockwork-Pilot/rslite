































pub mod os_common_h {
    
    pub unsafe extern "C" fn local_ioerr() {
        crate::src::src::os::sqlite3_io_error_hit += 1;
        if crate::src::src::os::sqlite3_io_error_benign == 0 {
            crate::src::src::os::sqlite3_io_error_hardhit += 1;
        }
    }
    
}


pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::src::headers::stdlib::__S_IFMT;






pub use crate::src::headers::stdlib::C2RustUnnamed_1;pub use crate::src::headers::stdlib::_SC_2_CHAR_TERM;pub use crate::src::headers::stdlib::_SC_2_C_BIND;pub use crate::src::headers::stdlib::_SC_2_C_DEV;pub use crate::src::headers::stdlib::_SC_2_C_VERSION;pub use crate::src::headers::stdlib::_SC_2_FORT_DEV;pub use crate::src::headers::stdlib::_SC_2_FORT_RUN;pub use crate::src::headers::stdlib::_SC_2_LOCALEDEF;pub use crate::src::headers::stdlib::_SC_2_PBS;pub use crate::src::headers::stdlib::_SC_2_PBS_ACCOUNTING;pub use crate::src::headers::stdlib::_SC_2_PBS_CHECKPOINT;pub use crate::src::headers::stdlib::_SC_2_PBS_LOCATE;pub use crate::src::headers::stdlib::_SC_2_PBS_MESSAGE;pub use crate::src::headers::stdlib::_SC_2_PBS_TRACK;pub use crate::src::headers::stdlib::_SC_2_SW_DEV;pub use crate::src::headers::stdlib::_SC_2_UPE;pub use crate::src::headers::stdlib::_SC_2_VERSION;pub use crate::src::headers::stdlib::_SC_ADVISORY_INFO;pub use crate::src::headers::stdlib::_SC_AIO_LISTIO_MAX;pub use crate::src::headers::stdlib::_SC_AIO_MAX;pub use crate::src::headers::stdlib::_SC_AIO_PRIO_DELTA_MAX;pub use crate::src::headers::stdlib::_SC_ARG_MAX;pub use crate::src::headers::stdlib::_SC_ASYNCHRONOUS_IO;pub use crate::src::headers::stdlib::_SC_ATEXIT_MAX;pub use crate::src::headers::stdlib::_SC_AVPHYS_PAGES;pub use crate::src::headers::stdlib::_SC_BARRIERS;pub use crate::src::headers::stdlib::_SC_BASE;pub use crate::src::headers::stdlib::_SC_BC_BASE_MAX;pub use crate::src::headers::stdlib::_SC_BC_DIM_MAX;pub use crate::src::headers::stdlib::_SC_BC_SCALE_MAX;pub use crate::src::headers::stdlib::_SC_BC_STRING_MAX;pub use crate::src::headers::stdlib::_SC_CHARCLASS_NAME_MAX;pub use crate::src::headers::stdlib::_SC_CHAR_BIT;pub use crate::src::headers::stdlib::_SC_CHAR_MAX;pub use crate::src::headers::stdlib::_SC_CHAR_MIN;pub use crate::src::headers::stdlib::_SC_CHILD_MAX;pub use crate::src::headers::stdlib::_SC_CLK_TCK;pub use crate::src::headers::stdlib::_SC_CLOCK_SELECTION;pub use crate::src::headers::stdlib::_SC_COLL_WEIGHTS_MAX;pub use crate::src::headers::stdlib::_SC_CPUTIME;pub use crate::src::headers::stdlib::_SC_C_LANG_SUPPORT;pub use crate::src::headers::stdlib::_SC_C_LANG_SUPPORT_R;pub use crate::src::headers::stdlib::_SC_DELAYTIMER_MAX;pub use crate::src::headers::stdlib::_SC_DEVICE_IO;pub use crate::src::headers::stdlib::_SC_DEVICE_SPECIFIC;pub use crate::src::headers::stdlib::_SC_DEVICE_SPECIFIC_R;pub use crate::src::headers::stdlib::_SC_EQUIV_CLASS_MAX;pub use crate::src::headers::stdlib::_SC_EXPR_NEST_MAX;pub use crate::src::headers::stdlib::_SC_FD_MGMT;pub use crate::src::headers::stdlib::_SC_FIFO;pub use crate::src::headers::stdlib::_SC_FILE_ATTRIBUTES;pub use crate::src::headers::stdlib::_SC_FILE_LOCKING;pub use crate::src::headers::stdlib::_SC_FILE_SYSTEM;pub use crate::src::headers::stdlib::_SC_FSYNC;pub use crate::src::headers::stdlib::_SC_GETGR_R_SIZE_MAX;pub use crate::src::headers::stdlib::_SC_GETPW_R_SIZE_MAX;pub use crate::src::headers::stdlib::_SC_HOST_NAME_MAX;pub use crate::src::headers::stdlib::_SC_INT_MAX;pub use crate::src::headers::stdlib::_SC_INT_MIN;pub use crate::src::headers::stdlib::_SC_IOV_MAX;pub use crate::src::headers::stdlib::_SC_IPV6;pub use crate::src::headers::stdlib::_SC_JOB_CONTROL;pub use crate::src::headers::stdlib::_SC_LEVEL1_DCACHE_ASSOC;pub use crate::src::headers::stdlib::_SC_LEVEL1_DCACHE_LINESIZE;pub use crate::src::headers::stdlib::_SC_LEVEL1_DCACHE_SIZE;pub use crate::src::headers::stdlib::_SC_LEVEL1_ICACHE_ASSOC;pub use crate::src::headers::stdlib::_SC_LEVEL1_ICACHE_LINESIZE;pub use crate::src::headers::stdlib::_SC_LEVEL1_ICACHE_SIZE;pub use crate::src::headers::stdlib::_SC_LEVEL2_CACHE_ASSOC;pub use crate::src::headers::stdlib::_SC_LEVEL2_CACHE_LINESIZE;pub use crate::src::headers::stdlib::_SC_LEVEL2_CACHE_SIZE;pub use crate::src::headers::stdlib::_SC_LEVEL3_CACHE_ASSOC;pub use crate::src::headers::stdlib::_SC_LEVEL3_CACHE_LINESIZE;pub use crate::src::headers::stdlib::_SC_LEVEL3_CACHE_SIZE;pub use crate::src::headers::stdlib::_SC_LEVEL4_CACHE_ASSOC;pub use crate::src::headers::stdlib::_SC_LEVEL4_CACHE_LINESIZE;pub use crate::src::headers::stdlib::_SC_LEVEL4_CACHE_SIZE;pub use crate::src::headers::stdlib::_SC_LINE_MAX;pub use crate::src::headers::stdlib::_SC_LOGIN_NAME_MAX;pub use crate::src::headers::stdlib::_SC_LONG_BIT;pub use crate::src::headers::stdlib::_SC_MAPPED_FILES;pub use crate::src::headers::stdlib::_SC_MB_LEN_MAX;pub use crate::src::headers::stdlib::_SC_MEMLOCK;pub use crate::src::headers::stdlib::_SC_MEMLOCK_RANGE;pub use crate::src::headers::stdlib::_SC_MEMORY_PROTECTION;pub use crate::src::headers::stdlib::_SC_MESSAGE_PASSING;pub use crate::src::headers::stdlib::_SC_MINSIGSTKSZ;pub use crate::src::headers::stdlib::_SC_MONOTONIC_CLOCK;pub use crate::src::headers::stdlib::_SC_MQ_OPEN_MAX;pub use crate::src::headers::stdlib::_SC_MQ_PRIO_MAX;pub use crate::src::headers::stdlib::_SC_MULTI_PROCESS;pub use crate::src::headers::stdlib::_SC_NETWORKING;pub use crate::src::headers::stdlib::_SC_NGROUPS_MAX;pub use crate::src::headers::stdlib::_SC_NL_ARGMAX;pub use crate::src::headers::stdlib::_SC_NL_LANGMAX;pub use crate::src::headers::stdlib::_SC_NL_MSGMAX;pub use crate::src::headers::stdlib::_SC_NL_NMAX;pub use crate::src::headers::stdlib::_SC_NL_SETMAX;pub use crate::src::headers::stdlib::_SC_NL_TEXTMAX;pub use crate::src::headers::stdlib::_SC_NPROCESSORS_CONF;pub use crate::src::headers::stdlib::_SC_NPROCESSORS_ONLN;pub use crate::src::headers::stdlib::_SC_NZERO;pub use crate::src::headers::stdlib::_SC_OPEN_MAX;pub use crate::src::headers::stdlib::_SC_PAGESIZE;pub use crate::src::headers::stdlib::_SC_PASS_MAX;pub use crate::src::headers::stdlib::_SC_PHYS_PAGES;pub use crate::src::headers::stdlib::_SC_PII;pub use crate::src::headers::stdlib::_SC_PII_INTERNET;pub use crate::src::headers::stdlib::_SC_PII_INTERNET_DGRAM;pub use crate::src::headers::stdlib::_SC_PII_INTERNET_STREAM;pub use crate::src::headers::stdlib::_SC_PII_OSI;pub use crate::src::headers::stdlib::_SC_PII_OSI_CLTS;pub use crate::src::headers::stdlib::_SC_PII_OSI_COTS;pub use crate::src::headers::stdlib::_SC_PII_OSI_M;pub use crate::src::headers::stdlib::_SC_PII_SOCKET;pub use crate::src::headers::stdlib::_SC_PII_XTI;pub use crate::src::headers::stdlib::_SC_PIPE;pub use crate::src::headers::stdlib::_SC_POLL;pub use crate::src::headers::stdlib::_SC_PRIORITIZED_IO;pub use crate::src::headers::stdlib::_SC_PRIORITY_SCHEDULING;pub use crate::src::headers::stdlib::_SC_RAW_SOCKETS;pub use crate::src::headers::stdlib::_SC_READER_WRITER_LOCKS;pub use crate::src::headers::stdlib::_SC_REALTIME_SIGNALS;pub use crate::src::headers::stdlib::_SC_REGEXP;pub use crate::src::headers::stdlib::_SC_REGEX_VERSION;pub use crate::src::headers::stdlib::_SC_RE_DUP_MAX;pub use crate::src::headers::stdlib::_SC_RTSIG_MAX;pub use crate::src::headers::stdlib::_SC_SAVED_IDS;pub use crate::src::headers::stdlib::_SC_SCHAR_MAX;pub use crate::src::headers::stdlib::_SC_SCHAR_MIN;pub use crate::src::headers::stdlib::_SC_SELECT;pub use crate::src::headers::stdlib::_SC_SEMAPHORES;pub use crate::src::headers::stdlib::_SC_SEM_NSEMS_MAX;pub use crate::src::headers::stdlib::_SC_SEM_VALUE_MAX;pub use crate::src::headers::stdlib::_SC_SHARED_MEMORY_OBJECTS;pub use crate::src::headers::stdlib::_SC_SHELL;pub use crate::src::headers::stdlib::_SC_SHRT_MAX;pub use crate::src::headers::stdlib::_SC_SHRT_MIN;pub use crate::src::headers::stdlib::_SC_SIGNALS;pub use crate::src::headers::stdlib::_SC_SIGQUEUE_MAX;pub use crate::src::headers::stdlib::_SC_SIGSTKSZ;pub use crate::src::headers::stdlib::_SC_SINGLE_PROCESS;pub use crate::src::headers::stdlib::_SC_SPAWN;pub use crate::src::headers::stdlib::_SC_SPIN_LOCKS;pub use crate::src::headers::stdlib::_SC_SPORADIC_SERVER;pub use crate::src::headers::stdlib::_SC_SSIZE_MAX;pub use crate::src::headers::stdlib::_SC_SS_REPL_MAX;pub use crate::src::headers::stdlib::_SC_STREAMS;pub use crate::src::headers::stdlib::_SC_STREAM_MAX;pub use crate::src::headers::stdlib::_SC_SYMLOOP_MAX;pub use crate::src::headers::stdlib::_SC_SYNCHRONIZED_IO;pub use crate::src::headers::stdlib::_SC_SYSTEM_DATABASE;pub use crate::src::headers::stdlib::_SC_SYSTEM_DATABASE_R;pub use crate::src::headers::stdlib::_SC_THREADS;pub use crate::src::headers::stdlib::_SC_THREAD_ATTR_STACKADDR;pub use crate::src::headers::stdlib::_SC_THREAD_ATTR_STACKSIZE;pub use crate::src::headers::stdlib::_SC_THREAD_CPUTIME;pub use crate::src::headers::stdlib::_SC_THREAD_DESTRUCTOR_ITERATIONS;pub use crate::src::headers::stdlib::_SC_THREAD_KEYS_MAX;pub use crate::src::headers::stdlib::_SC_THREAD_PRIORITY_SCHEDULING;pub use crate::src::headers::stdlib::_SC_THREAD_PRIO_INHERIT;pub use crate::src::headers::stdlib::_SC_THREAD_PRIO_PROTECT;pub use crate::src::headers::stdlib::_SC_THREAD_PROCESS_SHARED;pub use crate::src::headers::stdlib::_SC_THREAD_ROBUST_PRIO_INHERIT;pub use crate::src::headers::stdlib::_SC_THREAD_ROBUST_PRIO_PROTECT;pub use crate::src::headers::stdlib::_SC_THREAD_SAFE_FUNCTIONS;pub use crate::src::headers::stdlib::_SC_THREAD_SPORADIC_SERVER;pub use crate::src::headers::stdlib::_SC_THREAD_STACK_MIN;pub use crate::src::headers::stdlib::_SC_THREAD_THREADS_MAX;pub use crate::src::headers::stdlib::_SC_TIMEOUTS;pub use crate::src::headers::stdlib::_SC_TIMERS;pub use crate::src::headers::stdlib::_SC_TIMER_MAX;pub use crate::src::headers::stdlib::_SC_TRACE;pub use crate::src::headers::stdlib::_SC_TRACE_EVENT_FILTER;pub use crate::src::headers::stdlib::_SC_TRACE_EVENT_NAME_MAX;pub use crate::src::headers::stdlib::_SC_TRACE_INHERIT;pub use crate::src::headers::stdlib::_SC_TRACE_LOG;pub use crate::src::headers::stdlib::_SC_TRACE_NAME_MAX;pub use crate::src::headers::stdlib::_SC_TRACE_SYS_MAX;pub use crate::src::headers::stdlib::_SC_TRACE_USER_EVENT_MAX;pub use crate::src::headers::stdlib::_SC_TTY_NAME_MAX;pub use crate::src::headers::stdlib::_SC_TYPED_MEMORY_OBJECTS;pub use crate::src::headers::stdlib::_SC_TZNAME_MAX;pub use crate::src::headers::stdlib::_SC_T_IOV_MAX;pub use crate::src::headers::stdlib::_SC_UCHAR_MAX;pub use crate::src::headers::stdlib::_SC_UINT_MAX;pub use crate::src::headers::stdlib::_SC_UIO_MAXIOV;pub use crate::src::headers::stdlib::_SC_ULONG_MAX;pub use crate::src::headers::stdlib::_SC_USER_GROUPS;pub use crate::src::headers::stdlib::_SC_USER_GROUPS_R;pub use crate::src::headers::stdlib::_SC_USHRT_MAX;pub use crate::src::headers::stdlib::_SC_V6_ILP32_OFF32;pub use crate::src::headers::stdlib::_SC_V6_ILP32_OFFBIG;pub use crate::src::headers::stdlib::_SC_V6_LP64_OFF64;pub use crate::src::headers::stdlib::_SC_V6_LPBIG_OFFBIG;pub use crate::src::headers::stdlib::_SC_V7_ILP32_OFF32;pub use crate::src::headers::stdlib::_SC_V7_ILP32_OFFBIG;pub use crate::src::headers::stdlib::_SC_V7_LP64_OFF64;pub use crate::src::headers::stdlib::_SC_V7_LPBIG_OFFBIG;pub use crate::src::headers::stdlib::_SC_VERSION;pub use crate::src::headers::stdlib::_SC_WORD_BIT;pub use crate::src::headers::stdlib::_SC_XBS5_ILP32_OFF32;pub use crate::src::headers::stdlib::_SC_XBS5_ILP32_OFFBIG;pub use crate::src::headers::stdlib::_SC_XBS5_LP64_OFF64;pub use crate::src::headers::stdlib::_SC_XBS5_LPBIG_OFFBIG;pub use crate::src::headers::stdlib::_SC_XOPEN_CRYPT;pub use crate::src::headers::stdlib::_SC_XOPEN_ENH_I18N;pub use crate::src::headers::stdlib::_SC_XOPEN_LEGACY;pub use crate::src::headers::stdlib::_SC_XOPEN_REALTIME;pub use crate::src::headers::stdlib::_SC_XOPEN_REALTIME_THREADS;pub use crate::src::headers::stdlib::_SC_XOPEN_SHM;pub use crate::src::headers::stdlib::_SC_XOPEN_STREAMS;pub use crate::src::headers::stdlib::_SC_XOPEN_UNIX;pub use crate::src::headers::stdlib::_SC_XOPEN_VERSION;pub use crate::src::headers::stdlib::_SC_XOPEN_XCU_VERSION;pub use crate::src::headers::stdlib::_SC_XOPEN_XPG2;pub use crate::src::headers::stdlib::_SC_XOPEN_XPG3;pub use crate::src::headers::stdlib::_SC_XOPEN_XPG4;pub use ::libc::RTLD_GLOBAL;pub use ::libc::RTLD_NOW;pub use ::libc::EACCES;pub use ::libc::EAGAIN;pub use ::libc::EBUSY;pub use ::libc::EEXIST;pub use ::libc::EINTR;pub use ::libc::EIO;pub use ::libc::EISDIR;pub use ::libc::ENOENT;pub use ::libc::ENOSPC;pub use ::libc::ENXIO;pub use ::libc::EPERM;pub use ::libc::ERANGE;pub use ::libc::ENOLCK;pub use ::libc::ETIMEDOUT;pub use ::libc::flock;pub use crate::src::headers::stdlib::F_GETLK64;pub use crate::src::headers::stdlib::F_SETLK64;pub use crate::src::headers::stdlib::__O_LARGEFILE;pub use ::libc::F_GETLK;pub use ::libc::F_RDLCK;pub use ::libc::F_SETLK;pub use ::libc::F_UNLCK;pub use ::libc::F_WRLCK;pub use ::libc::O_CLOEXEC;pub use ::libc::O_CREAT;pub use ::libc::O_EXCL;pub use ::libc::O_LARGEFILE;pub use ::libc::O_NOFOLLOW;pub use ::libc::O_RDONLY;pub use ::libc::O_RDWR;pub use crate::src::headers::stdlib::__O_CLOEXEC;pub use crate::src::headers::stdlib::__O_NOFOLLOW;

pub use ::libc::fcntl;pub use ::libc::open;pub use ::libc::SEEK_SET;


pub use ::libc::mmap;pub use ::libc::mremap;pub use ::libc::munmap;pub use ::libc::MAP_FAILED;pub use ::libc::MAP_SHARED;pub use ::libc::PROT_READ;pub use ::libc::PROT_WRITE;
pub use ::libc::MREMAP_MAYMOVE;






pub use crate::src::src::os_unix::os_common_h::local_ioerr;pub use crate::src::src::os::sqlite3_diskfull;pub use crate::src::src::os::sqlite3_diskfull_pending;pub use crate::src::src::os::sqlite3_io_error_benign;pub use crate::src::src::os::sqlite3_io_error_hardhit;pub use crate::src::src::os::sqlite3_io_error_hit;pub use crate::src::src::os::sqlite3_io_error_pending;pub use crate::src::src::os::sqlite3_io_error_persist;pub use crate::src::src::os::sqlite3_open_file_count;pub use crate::src::src::os::EXCLUSIVE_LOCK;pub use crate::src::src::os::NO_LOCK;pub use crate::src::src::os::PENDING_LOCK;pub use crate::src::src::os::RESERVED_LOCK;pub use crate::src::src::os::SHARED_LOCK;pub use crate::src::src::os::SHARED_SIZE;pub use crate::src::src::os::SQLITE_DEFAULT_SECTOR_SIZE;pub use crate::src::src::os::SQLITE_MAX_SYMLINK;pub use crate::src::headers::sqlite3_h::sqlite3_file;pub use crate::src::headers::sqlite3_h::sqlite3_filename;pub use crate::src::src::malloc::sqlite3_free;pub use crate::src::headers::sqlite3_h::sqlite3_int64;pub use crate::src::headers::sqlite3_h::sqlite3_io_methods;pub use crate::src::src::printf::sqlite3_log;pub use crate::src::src::malloc::sqlite3_malloc64;pub use crate::src::headers::sqlite3_h::sqlite3_mem_methods;pub use crate::src::src::printf::sqlite3_mprintf;pub use crate::src::src::mutex_unix::sqlite3_mutex;pub use crate::src::src::mutex::sqlite3_mutex_alloc;pub use crate::src::src::mutex::sqlite3_mutex_enter;pub use crate::src::src::mutex::sqlite3_mutex_free;pub use crate::src::src::mutex::sqlite3_mutex_leave;pub use crate::src::headers::sqlite3_h::sqlite3_mutex_methods;pub use crate::src::headers::sqlite3_h::sqlite3_pcache;pub use crate::src::headers::sqlite3_h::sqlite3_pcache_methods2;pub use crate::src::headers::sqlite3_h::sqlite3_pcache_page;pub use crate::src::src::random::sqlite3_randomness;pub use crate::src::src::malloc::sqlite3_realloc;pub use crate::src::src::printf::sqlite3_snprintf;pub use crate::src::headers::sqlite3_h::sqlite3_syscall_ptr;pub use crate::src::src::main::sqlite3_temp_directory;pub use crate::src::headers::sqlite3_h::sqlite3_uint64;pub use crate::src::src::main::sqlite3_uri_boolean;pub use crate::src::src::main::sqlite3_uri_parameter;pub use crate::src::headers::sqlite3_h::sqlite3_vfs;pub use crate::src::src::os::sqlite3_vfs_register;pub use crate::src::headers::sqlite3_h::sqlite_int64;pub use crate::src::headers::sqlite3_h::sqlite_uint64;pub use crate::src::headers::sqlite3_h::SQLITE_ACCESS_EXISTS;pub use crate::src::headers::sqlite3_h::SQLITE_BUSY;pub use crate::src::headers::sqlite3_h::SQLITE_ERROR;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_CHUNK_SIZE;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_EXTERNAL_READER;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_HAS_MOVED;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_LAST_ERRNO;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_LOCKSTATE;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_MMAP_SIZE;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_NULL_IO;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_PERSIST_WAL;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_POWERSAFE_OVERWRITE;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_SIZE_HINT;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_TEMPFILENAME;pub use crate::src::headers::sqlite3_h::SQLITE_FCNTL_VFSNAME_1;pub use crate::src::headers::sqlite3_h::SQLITE_FULL;pub use crate::src::headers::sqlite3_h::SQLITE_IOCAP_POWERSAFE_OVERWRITE;pub use crate::src::headers::sqlite3_h::SQLITE_IOCAP_SUBPAGE_READ;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_CHECKRESERVEDLOCK_1;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_CLOSE_1;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_CORRUPTFS;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_DELETE_NOENT_1;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_FSTAT_1;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_GETTEMPPATH_1;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_LOCK_1;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_NOMEM;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_RDLOCK_1;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_READ_1;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_SHMLOCK_1;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_SHMSIZE_1;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_SHORT_READ_1;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_UNLOCK_1;pub use crate::src::headers::sqlite3_h::SQLITE_IOERR_WRITE_1;pub use crate::src::headers::sqlite3_h::SQLITE_MUTEX_FAST;pub use crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_VFS1;pub use crate::src::headers::sqlite3_h::SQLITE_NOMEM;pub use crate::src::headers::sqlite3_h::SQLITE_NOTFOUND;pub use crate::src::headers::sqlite3_h::SQLITE_OK;pub use crate::src::headers::sqlite3_h::SQLITE_OK_SYMLINK;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_CREATE;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_DELETEONCLOSE;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_EXCLUSIVE;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_DB;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_JOURNAL;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_SUPER_JOURNAL;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_URI;pub use crate::src::headers::sqlite3_h::SQLITE_OPEN_WAL;pub use crate::src::headers::sqlite3_h::SQLITE_PERM;pub use crate::src::headers::sqlite3_h::SQLITE_READONLY;pub use crate::src::headers::sqlite3_h::SQLITE_READONLY_CANTINIT_1;pub use crate::src::headers::sqlite3_h::SQLITE_READONLY_DIRECTORY_1;pub use crate::src::headers::sqlite3_h::SQLITE_SHM_EXCLUSIVE;pub use crate::src::headers::sqlite3_h::SQLITE_SHM_LOCK;pub use crate::src::headers::sqlite3_h::SQLITE_SHM_NLOCK;pub use crate::src::headers::sqlite3_h::SQLITE_SHM_SHARED;pub use crate::src::headers::sqlite3_h::SQLITE_SHM_UNLOCK;pub use crate::src::headers::sqlite3_h::SQLITE_SYNC_DATAONLY;pub use crate::src::headers::sqlite3_h::SQLITE_SYNC_FULL;pub use crate::src::headers::sqlite3_h::SQLITE_WARNING;pub use crate::src::ext::rtree::rtree::i64_0;pub use crate::src::src::main::sqlite3CantopenError;pub use crate::src::src::global::sqlite3Config;pub use crate::src::src::printf::sqlite3DebugPrintf;pub use crate::src::src::mutex_unix::sqlite3MemoryBarrier;pub use crate::src::src::mutex::sqlite3MutexAlloc;pub use crate::src::src::main::sqlite3OSTrace;pub use crate::src::src::global::sqlite3PendingByte;pub use crate::src::src::util::sqlite3Strlen30;pub use crate::src::fts5::u16_0;pub use crate::src::ext::rtree::rtree::u32_0;pub use crate::src::ext::rtree::rtree::u64_0;pub use crate::src::ext::rtree::rtree::u8_0;pub use crate::src::headers::sqliteInt_h::Sqlite3Config;pub use crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;pub use crate::src::headers::sqliteInt_h::SQLITE_MUTEX_STATIC_TEMPDIR;pub use crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;pub use crate::src::headers::sqliteInt_h::SQLITE_POWERSAFE_OVERWRITE;pub use crate::src::headers::stdlib::uint16_t;pub use crate::src::headers::stdlib::uint32_t;pub use crate::src::headers::stdlib::uint8_t;pub use crate::src::headers::stdlib::off64_t;pub use crate::src::headers::stdlib::off_t;pub use crate::src::headers::stdlib::ssize_t;



pub use ::libc::timespec;
pub use ::libc::timeval;
pub use crate::src::headers::stdlib::dev_t;pub use crate::src::headers::stdlib::gid_t;pub use crate::src::headers::stdlib::mode_t;pub use crate::src::headers::stdlib::pid_t;pub use crate::src::headers::stdlib::uid_t;



pub use crate::src::headers::stdlib::__blkcnt_t;pub use crate::src::headers::stdlib::__blksize_t;pub use crate::src::headers::stdlib::__dev_t;pub use crate::src::headers::stdlib::__gid_t;pub use crate::src::headers::stdlib::__ino_t;pub use crate::src::headers::stdlib::__mode_t;pub use crate::src::headers::stdlib::__nlink_t;pub use crate::src::headers::stdlib::__off64_t;pub use crate::src::headers::stdlib::__off_t;pub use crate::src::headers::stdlib::__pid_t;pub use crate::src::headers::stdlib::__ssize_t;pub use crate::src::headers::stdlib::__suseconds_t;pub use crate::src::headers::stdlib::__syscall_slong_t;pub use crate::src::headers::stdlib::__time_t;pub use crate::src::headers::stdlib::__uid_t;pub use crate::src::headers::stdlib::__uint16_t;pub use crate::src::headers::stdlib::__uint32_t;pub use crate::src::headers::stdlib::__uint8_t;pub use ::libc::access;pub use ::libc::close;pub use ::libc::fchown;pub use ::libc::ftruncate;pub use ::libc::getcwd;pub use ::libc::geteuid;pub use ::libc::getpid;pub use crate::src::headers::stdlib::pread64;pub use crate::src::headers::stdlib::pwrite64;pub use crate::src::headers::stdlib::read;pub use crate::src::headers::stdlib::readlink;pub use ::libc::rmdir;pub use ::libc::sysconf;pub use ::libc::unlink;pub use crate::src::headers::stdlib::write;pub use ::libc::F_OK;pub use ::libc::R_OK;pub use ::libc::W_OK;pub use ::libc::utimbuf;pub use ::libc::utime;
pub use crate::vxworks_h::OS_VXWORKS;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct unix_syscall {
    pub zName: *const ::core::ffi::c_char,
    pub pCurrent: crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
    pub pDefault: crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct DbPath {
    pub rc: ::core::ffi::c_int,
    pub nSymlink: ::core::ffi::c_int,
    pub zOut: *mut ::core::ffi::c_char,
    pub nOut: ::core::ffi::c_int,
    pub nUsed: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct unixFile {
    pub pMethod: *const crate::src::headers::sqlite3_h::sqlite3_io_methods,
    pub pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    pub pInode: *mut unixInodeInfo,
    pub h: ::core::ffi::c_int,
    pub eFileLock: ::core::ffi::c_uchar,
    pub ctrlFlags: ::core::ffi::c_ushort,
    pub lastErrno: ::core::ffi::c_int,
    pub lockingContext: *mut ::core::ffi::c_void,
    pub pPreallocatedUnused: *mut UnixUnusedFd,
    pub zPath: *const ::core::ffi::c_char,
    pub pShm: *mut unixShm,
    pub szChunk: ::core::ffi::c_int,
    pub nFetchOut: ::core::ffi::c_int,
    pub mmapSize: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub mmapSizeActual: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub mmapSizeMax: crate::src::headers::sqlite3_h::sqlite3_int64,
    pub pMapRegion: *mut ::core::ffi::c_void,
    pub sectorSize: ::core::ffi::c_int,
    pub deviceCharacteristics: ::core::ffi::c_int,
    pub aPadding: [::core::ffi::c_char; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct unixShm {
    pub pShmNode: *mut unixShmNode,
    pub pNext: *mut unixShm,
    pub hasMutex: crate::src::ext::rtree::rtree::u8_0,
    pub id: crate::src::ext::rtree::rtree::u8_0,
    pub sharedMask: crate::src::fts5::u16_0,
    pub exclMask: crate::src::fts5::u16_0,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct unixShmNode {
    pub pInode: *mut unixInodeInfo,
    pub pShmMutex: *mut crate::src::src::mutex_unix::sqlite3_mutex,
    pub zFilename: *mut ::core::ffi::c_char,
    pub hShm: ::core::ffi::c_int,
    pub szRegion: ::core::ffi::c_int,
    pub nRegion: crate::src::fts5::u16_0,
    pub isReadonly: crate::src::ext::rtree::rtree::u8_0,
    pub isUnlocked: crate::src::ext::rtree::rtree::u8_0,
    pub apRegion: *mut *mut ::core::ffi::c_char,
    pub nRef: ::core::ffi::c_int,
    pub pFirst: *mut unixShm,
    pub aLock: [::core::ffi::c_int; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct unixInodeInfo {
    pub fileId: unixFileId,
    pub pLockMutex: *mut crate::src::src::mutex_unix::sqlite3_mutex,
    pub nShared: ::core::ffi::c_int,
    pub nLock: ::core::ffi::c_int,
    pub eFileLock: ::core::ffi::c_uchar,
    pub bProcessLock: ::core::ffi::c_uchar,
    pub pUnused: *mut UnixUnusedFd,
    pub nRef: ::core::ffi::c_int,
    pub pShmNode: *mut unixShmNode,
    pub pNext: *mut unixInodeInfo,
    pub pPrev: *mut unixInodeInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct UnixUnusedFd {
    pub fd: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub pNext: *mut UnixUnusedFd,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct unixFileId {
    pub dev: crate::src::headers::stdlib::dev_t,
    pub ino: crate::src::ext::rtree::rtree::u64_0,
}

pub type finder_type = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, *mut unixFile) -> *const crate::src::headers::sqlite3_h::sqlite3_io_methods,
>;

pub const SQLITE_DEFAULT_FILE_PERMISSIONS: ::core::ffi::c_int = 0o644 as ::core::ffi::c_int;

pub const MAX_PATHNAME: ::core::ffi::c_int = 512 as ::core::ffi::c_int;

static mut randomnessPid: crate::src::headers::stdlib::pid_t = 0 as crate::src::headers::stdlib::pid_t;

pub const UNIXFILE_EXCL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;

pub const UNIXFILE_RDONLY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;

pub const UNIXFILE_PERSIST_WAL: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;

pub const UNIXFILE_DIRSYNC: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const UNIXFILE_PSOW: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;

pub const UNIXFILE_DELETE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;

pub const UNIXFILE_URI: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;

pub const UNIXFILE_NOLOCK: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;

pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

unsafe extern "C" fn posixOpen(
    mut zFile: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    ::libc::open(zFile, flags, mode)
}

static mut aSyscall: [unix_syscall; 29] = unsafe {
    [
        unix_syscall {
            zName: b"open\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                posixOpen
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"close\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                ::libc::close as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"access\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                ::libc::access
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"getcwd\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        crate::__stddef_size_t_h::size_t,
                    ) -> *mut ::core::ffi::c_char,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                ::libc::getcwd
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        crate::__stddef_size_t_h::size_t,
                    ) -> *mut ::core::ffi::c_char,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"stat\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut crate::src::headers::stdlib::stat,
                    ) -> ::core::ffi::c_int,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                crate::src::headers::stdlib::stat as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut crate::src::headers::stdlib::stat,
                ) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"fstat\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_int, *mut crate::src::headers::stdlib::stat) -> ::core::ffi::c_int>,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                crate::src::headers::stdlib::fstat as unsafe extern "C" fn(::core::ffi::c_int, *mut crate::src::headers::stdlib::stat) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"ftruncate\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_int, crate::src::headers::stdlib::__off64_t) -> ::core::ffi::c_int>,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                ::libc::ftruncate
                    as unsafe extern "C" fn(::core::ffi::c_int, crate::src::headers::stdlib::__off64_t) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"fcntl\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ...
                    ) -> ::core::ffi::c_int,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                ::libc::fcntl
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ...
                    ) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"read\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_void,
                        crate::__stddef_size_t_h::size_t,
                    ) -> crate::src::headers::stdlib::ssize_t,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                crate::src::headers::stdlib::read as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                    crate::__stddef_size_t_h::size_t,
                ) -> crate::src::headers::stdlib::ssize_t,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"pread\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: None,
            pDefault: None,
        },
        unix_syscall {
            zName: b"pread64\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_void,
                        crate::__stddef_size_t_h::size_t,
                        crate::src::headers::stdlib::__off64_t,
                    ) -> crate::src::headers::stdlib::ssize_t,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                crate::src::headers::stdlib::pread64
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_void,
                        crate::__stddef_size_t_h::size_t,
                        crate::src::headers::stdlib::__off64_t,
                    ) -> crate::src::headers::stdlib::ssize_t,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"write\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        crate::__stddef_size_t_h::size_t,
                    ) -> crate::src::headers::stdlib::ssize_t,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                crate::src::headers::stdlib::write
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        crate::__stddef_size_t_h::size_t,
                    ) -> crate::src::headers::stdlib::ssize_t,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"pwrite\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: None,
            pDefault: None,
        },
        unix_syscall {
            zName: b"pwrite64\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        crate::__stddef_size_t_h::size_t,
                        crate::src::headers::stdlib::__off64_t,
                    ) -> crate::src::headers::stdlib::ssize_t,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                crate::src::headers::stdlib::pwrite64
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        crate::__stddef_size_t_h::size_t,
                        crate::src::headers::stdlib::__off64_t,
                    ) -> crate::src::headers::stdlib::ssize_t,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"fchmod\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(::core::ffi::c_int, crate::src::headers::stdlib::__mode_t) -> ::core::ffi::c_int>,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                ::libc::fchmod as unsafe extern "C" fn(::core::ffi::c_int, crate::src::headers::stdlib::__mode_t) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"fallocate\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: None,
            pDefault: None,
        },
        unix_syscall {
            zName: b"unlink\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                ::libc::unlink as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"openDirectory\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                openDirectory
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"mkdir\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        crate::src::headers::stdlib::__mode_t,
                    ) -> ::core::ffi::c_int,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                ::libc::mkdir
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        crate::src::headers::stdlib::__mode_t,
                    ) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"rmdir\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                ::libc::rmdir as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"fchown\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        crate::src::headers::stdlib::__uid_t,
                        crate::src::headers::stdlib::__gid_t,
                    ) -> ::core::ffi::c_int,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                ::libc::fchown
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        crate::src::headers::stdlib::__uid_t,
                        crate::src::headers::stdlib::__gid_t,
                    ) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"geteuid\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> crate::src::headers::stdlib::__uid_t>,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(::libc::geteuid as unsafe extern "C" fn() -> crate::src::headers::stdlib::__uid_t)),
            pDefault: None,
        },
        unix_syscall {
            zName: b"mmap\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        crate::__stddef_size_t_h::size_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        crate::src::headers::stdlib::__off64_t,
                    ) -> *mut ::core::ffi::c_void,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                ::libc::mmap as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    crate::__stddef_size_t_h::size_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    crate::src::headers::stdlib::__off64_t,
                ) -> *mut ::core::ffi::c_void,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"munmap\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ::core::ffi::c_int,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                ::libc::munmap
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"mremap\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        crate::__stddef_size_t_h::size_t,
                        crate::__stddef_size_t_h::size_t,
                        ::core::ffi::c_int,
                        ...
                    ) -> *mut ::core::ffi::c_void,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                ::libc::mremap
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        crate::__stddef_size_t_h::size_t,
                        crate::__stddef_size_t_h::size_t,
                        ::core::ffi::c_int,
                        ...
                    ) -> *mut ::core::ffi::c_void,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"getpagesize\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                unixGetpagesize as unsafe extern "C" fn() -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"readlink\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_char,
                        crate::__stddef_size_t_h::size_t,
                    ) -> crate::src::headers::stdlib::ssize_t,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                crate::src::headers::stdlib::readlink
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_char,
                        crate::__stddef_size_t_h::size_t,
                    ) -> crate::src::headers::stdlib::ssize_t,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"lstat\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut crate::src::headers::stdlib::stat,
                    ) -> ::core::ffi::c_int,
                >,
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            >(Some(
                crate::src::headers::stdlib::lstat
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut crate::src::headers::stdlib::stat,
                    ) -> ::core::ffi::c_int,
            )),
            pDefault: None,
        },
        unix_syscall {
            zName: b"ioctl\0" as *const u8 as *const ::core::ffi::c_char,
            pCurrent: None,
            pDefault: None,
        },
    ]
};

unsafe extern "C" fn robustFchown(
    mut fd: ::core::ffi::c_int,
    mut uid: crate::src::headers::stdlib::uid_t,
    mut gid: crate::src::headers::stdlib::gid_t,
) -> ::core::ffi::c_int {
    if ::core::mem::transmute::<
        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
        Option<unsafe extern "C" fn() -> crate::src::headers::stdlib::uid_t>,
    >(aSyscall[21 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")() != 0
    {
        0 as ::core::ffi::c_int
    } else {
        ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    crate::src::headers::stdlib::uid_t,
                    crate::src::headers::stdlib::gid_t,
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[20 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(fd, uid, gid)
    }
}

unsafe extern "C" fn unixSetSystemCall(
    mut _pNotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
    mut pNewFunc: crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_uint = 0;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_NOTFOUND;
    if zName.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        i = 0 as ::core::ffi::c_uint;
        while (i as usize)
            < (::core::mem::size_of::<[unix_syscall; 29]>() as usize)
                .wrapping_div(::core::mem::size_of::<unix_syscall>() as usize)
        {
            if aSyscall[i as usize].pDefault.is_some() {
                aSyscall[i as usize].pCurrent = aSyscall[i as usize].pDefault;
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = 0 as ::core::ffi::c_uint;
        while (i as usize)
            < (::core::mem::size_of::<[unix_syscall; 29]>() as usize)
                .wrapping_div(::core::mem::size_of::<unix_syscall>() as usize)
        {
            if ::libc::strcmp(zName, aSyscall[i as usize].zName) == 0 as ::core::ffi::c_int {
                if aSyscall[i as usize].pDefault.is_none() {
                    aSyscall[i as usize].pDefault = aSyscall[i as usize].pCurrent;
                }
                rc = crate::src::headers::sqlite3_h::SQLITE_OK;
                if pNewFunc.is_none() {
                    pNewFunc = aSyscall[i as usize].pDefault;
                }
                aSyscall[i as usize].pCurrent = pNewFunc;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
    }
    rc
}

unsafe extern "C" fn unixGetSystemCall(
    mut _pNotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
) -> crate::src::headers::sqlite3_h::sqlite3_syscall_ptr {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while (i as usize)
        < (::core::mem::size_of::<[unix_syscall; 29]>() as usize)
            .wrapping_div(::core::mem::size_of::<unix_syscall>() as usize)
    {
        if ::libc::strcmp(zName, aSyscall[i as usize].zName) == 0 as ::core::ffi::c_int {
            return aSyscall[i as usize].pCurrent;
        }
        i = i.wrapping_add(1);
    }
    None
}

unsafe extern "C" fn unixNextSystemCall(
    mut _p: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zName: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if !zName.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i
            < (::core::mem::size_of::<[unix_syscall; 29]>() as usize)
                .wrapping_div(::core::mem::size_of::<unix_syscall>() as usize)
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
        {
            if ::libc::strcmp(zName, aSyscall[i as usize].zName) == 0 as ::core::ffi::c_int {
                break;
            }
            i += 1;
        }
    }
    i += 1;
    while i
        < (::core::mem::size_of::<[unix_syscall; 29]>() as usize)
            .wrapping_div(::core::mem::size_of::<unix_syscall>() as usize)
            as ::core::ffi::c_int
    {
        if aSyscall[i as usize].pCurrent.is_some() {
            return aSyscall[i as usize].zName;
        }
        i += 1;
    }
    ::core::ptr::null::<::core::ffi::c_char>()
}

pub const SQLITE_MINIMUM_FILE_DESCRIPTOR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

unsafe extern "C" fn robust_open(
    mut z: *const ::core::ffi::c_char,
    mut f: ::core::ffi::c_int,
    mut m: crate::src::headers::stdlib::mode_t,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    let mut m2: crate::src::headers::stdlib::mode_t = if m != 0 {
        m
    } else {
        SQLITE_DEFAULT_FILE_PERMISSIONS as crate::src::headers::stdlib::mode_t
    };
    loop {
        fd = ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[0 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(
            z, f | ::libc::O_CLOEXEC, m2 as ::core::ffi::c_int
        );
        if fd < 0 as ::core::ffi::c_int {
            if !(*::libc::__errno_location() == ::libc::EINTR) {
                break;
            }
        } else {
            if fd >= SQLITE_MINIMUM_FILE_DESCRIPTOR {
                break;
            }
            if f & (::libc::O_EXCL | ::libc::O_CREAT) == ::libc::O_EXCL | ::libc::O_CREAT {
                ::core::mem::transmute::<
                    crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                    Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
                >(aSyscall[16 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")(z);
            }
            ::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
            >(aSyscall[1 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(fd);
            crate::src::src::printf::sqlite3_log(
                crate::src::headers::sqlite3_h::SQLITE_WARNING,
                b"attempt to open \"%s\" as file descriptor %d\0" as *const u8
                    as *const ::core::ffi::c_char,
                z,
                fd,
            );
            fd = -(1 as ::core::ffi::c_int);
            if ::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[0 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(
                b"/dev/null\0" as *const u8 as *const ::core::ffi::c_char,
                ::libc::O_RDONLY,
                m as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                break;
            }
        }
    }
    if fd >= 0 as ::core::ffi::c_int {
        if m != 0 as crate::src::headers::stdlib::mode_t {
            let mut statbuf: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
            if ::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<unsafe extern "C" fn(::core::ffi::c_int, *mut crate::src::headers::stdlib::stat) -> ::core::ffi::c_int>,
            >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(fd, &raw mut statbuf)
                == 0 as ::core::ffi::c_int
                && statbuf.st_size == 0 as crate::src::headers::stdlib::__off_t
                && statbuf.st_mode & 0o777 as crate::src::headers::stdlib::__mode_t != m
            {
                ::core::mem::transmute::<
                    crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                    Option<unsafe extern "C" fn(::core::ffi::c_int, crate::src::headers::stdlib::mode_t) -> ::core::ffi::c_int>,
                >(aSyscall[14 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")(fd, m);
            }
        }
    }
    fd
}

static mut unixBigLock: *mut crate::src::src::mutex_unix::sqlite3_mutex =
    ::core::ptr::null::<crate::src::src::mutex_unix::sqlite3_mutex>() as *mut crate::src::src::mutex_unix::sqlite3_mutex;

unsafe extern "C" fn unixEnterMutex() {
    crate::src::src::mutex::sqlite3_mutex_enter(unixBigLock);
}

unsafe extern "C" fn unixLeaveMutex() {
    crate::src::src::mutex::sqlite3_mutex_leave(unixBigLock);
}

unsafe extern "C" fn azFileLock(mut eFileLock: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    match  eFileLock {
    crate::src::src::os::NO_LOCK =>
         return b"NONE\0" as *const u8 as *const ::core::ffi::c_char,
    crate::src::src::os::SHARED_LOCK =>
         return b"SHARED\0" as *const u8 as *const ::core::ffi::c_char,
    crate::src::src::os::RESERVED_LOCK =>
         return b"RESERVED\0" as *const u8 as *const ::core::ffi::c_char,
    crate::src::src::os::PENDING_LOCK =>
         return b"PENDING\0" as *const u8 as *const ::core::ffi::c_char,
    crate::src::src::os::EXCLUSIVE_LOCK =>  {
            return b"EXCLUSIVE\0" as *const u8 as *const ::core::ffi::c_char;
        }
    _ =>  {}
}
    b"ERROR\0" as *const u8 as *const ::core::ffi::c_char
}

unsafe extern "C" fn robust_ftruncate(
    mut h: ::core::ffi::c_int,
    mut sz: crate::src::headers::sqlite3_h::sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    loop {
        rc = ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<unsafe extern "C" fn(::core::ffi::c_int, crate::src::headers::stdlib::off_t) -> ::core::ffi::c_int>,
        >(aSyscall[6 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(h, sz as crate::src::headers::stdlib::off_t);
        if !(rc < 0 as ::core::ffi::c_int && *::libc::__errno_location() == ::libc::EINTR) {
            break;
        }
    }
    rc
}

unsafe extern "C" fn sqliteErrorFromPosixError(
    mut posixError: ::core::ffi::c_int,
    mut sqliteIOErr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match  posixError {
    ::libc::EACCES | ::libc::EAGAIN | ::libc::ETIMEDOUT | ::libc::EBUSY |
        ::libc::EINTR | ::libc::ENOLCK =>
         return crate::src::headers::sqlite3_h::SQLITE_BUSY,
    ::libc::EPERM =>  return crate::src::headers::sqlite3_h::SQLITE_PERM,
    _ =>  return sqliteIOErr,
};
}

static mut inodeList: *mut unixInodeInfo =
    ::core::ptr::null::<unixInodeInfo>() as *mut unixInodeInfo;

unsafe extern "C" fn unixLogErrorAtLine(
    mut errcode: ::core::ffi::c_int,
    mut zFunc: *const ::core::ffi::c_char,
    mut zPath: *const ::core::ffi::c_char,
    mut iLine: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut zErr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut iErrno: ::core::ffi::c_int = *::libc::__errno_location();
    zErr = b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    if zPath.is_null() {
        zPath = b"\0" as *const u8 as *const ::core::ffi::c_char;
    }
    crate::src::src::printf::sqlite3_log(
        errcode,
        b"os_unix.c:%d: (%d) %s(%s) - %s\0" as *const u8 as *const ::core::ffi::c_char,
        iLine,
        iErrno,
        zFunc,
        zPath,
        zErr,
    );
    errcode
}

unsafe extern "C" fn robust_close(
    mut pFile: *mut unixFile,
    mut h: ::core::ffi::c_int,
    mut lineno: ::core::ffi::c_int,
) {
    if ::core::mem::transmute::<
        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
        Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    >(aSyscall[1 as ::core::ffi::c_int as usize].pCurrent)
    .expect("non-null function pointer")(h)
        != 0
    {
        unixLogErrorAtLine(
            crate::src::headers::sqlite3_h::SQLITE_IOERR_CLOSE_1,
            b"close\0" as *const u8 as *const ::core::ffi::c_char,
            if !pFile.is_null() {
                (*pFile).zPath
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            },
            lineno,
        );
    }
}

unsafe extern "C" fn storeLastErrno(mut pFile: *mut unixFile, mut error: ::core::ffi::c_int) {
    (*pFile).lastErrno = error;
}

unsafe extern "C" fn closePendingFds(mut pFile: *mut unixFile) {
    let mut pInode: *mut unixInodeInfo = (*pFile).pInode;
    let mut p: *mut UnixUnusedFd = ::core::ptr::null_mut::<UnixUnusedFd>();
    let mut pNext: *mut UnixUnusedFd = ::core::ptr::null_mut::<UnixUnusedFd>();
    p = (*pInode).pUnused;
    while !p.is_null() {
        pNext = (*p).pNext;
        robust_close(pFile, (*p).fd, 1478 as ::core::ffi::c_int);
        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
        p = pNext;
    }
    (*pInode).pUnused = ::core::ptr::null_mut::<UnixUnusedFd>();
}

unsafe extern "C" fn releaseInodeInfo(mut pFile: *mut unixFile) {
    let mut pInode: *mut unixInodeInfo = (*pFile).pInode;
    if !pInode.is_null() {
        (*pInode).nRef -= 1;
        if (*pInode).nRef == 0 as ::core::ffi::c_int {
            let __pInode_ref = unsafe { &mut *pInode };
            crate::src::src::mutex::sqlite3_mutex_enter(__pInode_ref.pLockMutex);
            closePendingFds(pFile);
            crate::src::src::mutex::sqlite3_mutex_leave(__pInode_ref.pLockMutex);
            if !__pInode_ref.pPrev.is_null() {
                (*__pInode_ref.pPrev).pNext = __pInode_ref.pNext;
            } else {
                inodeList = __pInode_ref.pNext;
            }
            if !__pInode_ref.pNext.is_null() {
                (*__pInode_ref.pNext).pPrev = __pInode_ref.pPrev;
            }
            crate::src::src::mutex::sqlite3_mutex_free(__pInode_ref.pLockMutex);
            crate::src::src::malloc::sqlite3_free(pInode as *mut ::core::ffi::c_void);
        }
    }
}

unsafe extern "C" fn findInodeInfo(
    mut pFile: *mut unixFile,
    mut ppInode: *mut *mut unixInodeInfo,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut fd: ::core::ffi::c_int = 0;
    let mut fileId: unixFileId = unsafe { ::core::mem::zeroed() };
    let mut statbuf: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
    let mut pInode: *mut unixInodeInfo = ::core::ptr::null_mut::<unixInodeInfo>();
    fd = (*pFile).h;
    rc = ::core::mem::transmute::<
        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
        Option<unsafe extern "C" fn(::core::ffi::c_int, *mut crate::src::headers::stdlib::stat) -> ::core::ffi::c_int>,
    >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
    .expect("non-null function pointer")(fd, &raw mut statbuf);
    if rc != 0 as ::core::ffi::c_int {
        storeLastErrno(pFile, *::libc::__errno_location());
        return crate::src::headers::sqlite3_h::SQLITE_IOERR;
    }
    fileId.dev = statbuf.st_dev as crate::src::headers::stdlib::dev_t;
    fileId.ino = statbuf.st_ino as crate::src::ext::rtree::rtree::u64_0;
    pInode = inodeList;
    while !pInode.is_null()
        && ::libc::memcmp(
            &raw mut fileId as *const ::core::ffi::c_void,
            &raw mut (*pInode).fileId as *const ::core::ffi::c_void,
            ::core::mem::size_of::<unixFileId>() as crate::__stddef_size_t_h::size_t,
        ) != 0
    {
        pInode = (*pInode).pNext;
    }
    if pInode.is_null() {
        pInode = crate::src::src::malloc::sqlite3_malloc64(::core::mem::size_of::<unixInodeInfo>() as crate::src::headers::sqlite3_h::sqlite3_uint64)
            as *mut unixInodeInfo;
        if pInode.is_null() {
            return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        }
        ::libc::memset(
            pInode as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<unixInodeInfo>() as crate::__stddef_size_t_h::size_t,
        );
        ::core::ptr::copy_nonoverlapping(
                    &raw mut fileId as *const u8,
                    &raw mut (*pInode).fileId as *mut u8,
                    ::core::mem::size_of::<unixFileId>() as usize,
                );
        if crate::src::src::global::sqlite3Config.bCoreMutex != 0 {
            (*pInode).pLockMutex = crate::src::src::mutex::sqlite3_mutex_alloc(crate::src::headers::sqlite3_h::SQLITE_MUTEX_FAST);
            if (*pInode).pLockMutex.is_null() {
                crate::src::src::malloc::sqlite3_free(pInode as *mut ::core::ffi::c_void);
                return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
            }
        }
        (*pInode).nRef = 1 as ::core::ffi::c_int;
        (*pInode).pNext = inodeList;
        (*pInode).pPrev = ::core::ptr::null_mut::<unixInodeInfo>();
        if !inodeList.is_null() {
            (*inodeList).pPrev = pInode;
        }
        inodeList = pInode;
    } else {
        (*pInode).nRef += 1;
    }
    *ppInode = pInode;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fileHasMoved(mut pFile: *mut unixFile) -> ::core::ffi::c_int {
    let mut buf: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
    let __pFile_ref = unsafe { &mut *pFile };
    (!__pFile_ref.pInode.is_null()
        && (::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(*const ::core::ffi::c_char, *mut crate::src::headers::stdlib::stat) -> ::core::ffi::c_int,
            >,
        >(aSyscall[4 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(__pFile_ref.zPath, &raw mut buf)
            != 0 as ::core::ffi::c_int
            || buf.st_ino as crate::src::ext::rtree::rtree::u64_0 != (*__pFile_ref.pInode).fileId.ino))
        as ::core::ffi::c_int
}

unsafe extern "C" fn verifyDbFile(mut pFile: *mut unixFile) {
    let mut buf: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
    let mut rc: ::core::ffi::c_int = 0;
    if (*pFile).ctrlFlags as ::core::ffi::c_int & UNIXFILE_NOLOCK != 0 {
        return;
    }
    rc = ::core::mem::transmute::<
        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
        Option<unsafe extern "C" fn(::core::ffi::c_int, *mut crate::src::headers::stdlib::stat) -> ::core::ffi::c_int>,
    >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
    .expect("non-null function pointer")((*pFile).h, &raw mut buf);
    if rc != 0 as ::core::ffi::c_int {
        crate::src::src::printf::sqlite3_log(
            crate::src::headers::sqlite3_h::SQLITE_WARNING,
            b"cannot fstat db file %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*pFile).zPath,
        );
        return;
    }
    if buf.st_nlink == 0 as crate::src::headers::stdlib::__nlink_t {
        crate::src::src::printf::sqlite3_log(
            crate::src::headers::sqlite3_h::SQLITE_WARNING,
            b"file unlinked while open: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*pFile).zPath,
        );
        return;
    }
    if buf.st_nlink > 1 as crate::src::headers::stdlib::__nlink_t {
        crate::src::src::printf::sqlite3_log(
            crate::src::headers::sqlite3_h::SQLITE_WARNING,
            b"multiple links to file: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*pFile).zPath,
        );
        return;
    }
    if fileHasMoved(pFile) != 0 {
        crate::src::src::printf::sqlite3_log(
            crate::src::headers::sqlite3_h::SQLITE_WARNING,
            b"file renamed while open: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*pFile).zPath,
        );
        return;
    }
}

unsafe extern "C" fn unixCheckReservedLock(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut reserved: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pFile: *mut unixFile = id as *mut unixFile;
    if crate::src::src::os::sqlite3_io_error_persist != 0 && crate::src::src::os::sqlite3_io_error_hit != 0 || {
        let fresh19 = crate::src::src::os::sqlite3_io_error_pending;
        crate::src::src::os::sqlite3_io_error_pending -= 1;
        fresh19 == 1 as ::core::ffi::c_int
    } {
        local_ioerr();
        return 10 as ::core::ffi::c_int | (14 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    }
    let __pFile_ref = unsafe { &*pFile };
    crate::src::src::mutex::sqlite3_mutex_enter((*__pFile_ref.pInode).pLockMutex);
    if (*__pFile_ref.pInode).eFileLock as ::core::ffi::c_int > crate::src::src::os::SHARED_LOCK {
        reserved = 1 as ::core::ffi::c_int;
    }
    if reserved == 0 && (*__pFile_ref.pInode).bProcessLock == 0 {
        let mut lock: ::libc::flock = unsafe { ::core::mem::zeroed() };
        lock.l_whence = ::libc::SEEK_SET as ::core::ffi::c_short;
        lock.l_start = (crate::src::src::global::sqlite3PendingByte + 1 as ::core::ffi::c_int) as crate::src::headers::stdlib::__off64_t;
        lock.l_len = 1 as crate::src::headers::stdlib::__off64_t;
        lock.l_type = ::libc::F_WRLCK as ::core::ffi::c_short;
        if ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ...
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(__pFile_ref.h, ::libc::F_GETLK, &raw mut lock)
            != 0
        {
            rc = crate::src::headers::sqlite3_h::SQLITE_IOERR_CHECKRESERVEDLOCK_1;
            storeLastErrno(pFile, *::libc::__errno_location());
        } else if lock.l_type as ::core::ffi::c_int != ::libc::F_UNLCK {
            reserved = 1 as ::core::ffi::c_int;
        }
    }
    crate::src::src::mutex::sqlite3_mutex_leave((*__pFile_ref.pInode).pLockMutex);
    if crate::src::src::main::sqlite3OSTrace != 0 {
        crate::src::src::printf::sqlite3DebugPrintf(
            b"TEST WR-LOCK %d %d %d (unix)\n\0" as *const u8 as *const ::core::ffi::c_char,
            __pFile_ref.h,
            rc,
            reserved,
        );
    }
    *pResOut = reserved;
    rc
}

unsafe extern "C" fn unixFileLock(
    mut pFile: *mut unixFile,
    mut pLock: *mut ::libc::flock,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pInode: *mut unixInodeInfo = (*pFile).pInode;
    if (*pFile).ctrlFlags as ::core::ffi::c_int & (UNIXFILE_EXCL | UNIXFILE_RDONLY) == UNIXFILE_EXCL
    {
        if (*pInode).bProcessLock as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut lock: ::libc::flock = unsafe { ::core::mem::zeroed() };
            lock.l_whence = ::libc::SEEK_SET as ::core::ffi::c_short;
            lock.l_start = (crate::src::src::global::sqlite3PendingByte + 2 as ::core::ffi::c_int) as crate::src::headers::stdlib::__off64_t;
            lock.l_len = crate::src::src::os::SHARED_SIZE as crate::src::headers::stdlib::__off64_t;
            lock.l_type = ::libc::F_WRLCK as ::core::ffi::c_short;
            rc = ::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ...
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(
                (*pFile).h, ::libc::F_SETLK, &raw mut lock
            );
            if rc < 0 as ::core::ffi::c_int {
                return rc;
            }
            (*pInode).bProcessLock = 1 as ::core::ffi::c_uchar;
            (*pInode).nLock += 1;
        } else {
            rc = 0 as ::core::ffi::c_int;
        }
    } else {
        rc = ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ...
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")((*pFile).h, ::libc::F_SETLK, pLock);
    }
    rc
}

unsafe extern "C" fn unixLock(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut eFileLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut pInode: *mut unixInodeInfo = ::core::ptr::null_mut::<unixInodeInfo>();
    let mut lock: ::libc::flock = unsafe { ::core::mem::zeroed() };
    let mut tErrno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let __pFile_ref = unsafe { &mut *pFile };
    if crate::src::src::main::sqlite3OSTrace != 0 {
        crate::src::src::printf::sqlite3DebugPrintf(
            b"LOCK    %d %s was %s(%s,%d) pid=%d (unix)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            __pFile_ref.h,
            azFileLock(eFileLock),
            azFileLock(__pFile_ref.eFileLock as ::core::ffi::c_int),
            azFileLock((*__pFile_ref.pInode).eFileLock as ::core::ffi::c_int),
            (*__pFile_ref.pInode).nShared,
            ::libc::getpid(),
        );
    }
    if __pFile_ref.eFileLock as ::core::ffi::c_int >= eFileLock {
        if crate::src::src::main::sqlite3OSTrace != 0 {
            crate::src::src::printf::sqlite3DebugPrintf(
                b"LOCK    %d %s ok (already held) (unix)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                __pFile_ref.h,
                azFileLock(eFileLock),
            );
        }
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    pInode = __pFile_ref.pInode;
    crate::src::src::mutex::sqlite3_mutex_enter((*pInode).pLockMutex);
    if __pFile_ref.eFileLock as ::core::ffi::c_int != (*pInode).eFileLock as ::core::ffi::c_int
        && ((*pInode).eFileLock as ::core::ffi::c_int >= crate::src::src::os::PENDING_LOCK || eFileLock > crate::src::src::os::SHARED_LOCK)
    {
        rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
    } else if eFileLock == crate::src::src::os::SHARED_LOCK
        && ((*pInode).eFileLock as ::core::ffi::c_int == crate::src::src::os::SHARED_LOCK
            || (*pInode).eFileLock as ::core::ffi::c_int == crate::src::src::os::RESERVED_LOCK)
    {
        __pFile_ref.eFileLock = crate::src::src::os::SHARED_LOCK as ::core::ffi::c_uchar;
        (*pInode).nShared += 1;
        (*pInode).nLock += 1;
    } else {
        lock.l_len = 1 as ::core::ffi::c_long as crate::src::headers::stdlib::__off64_t;
        lock.l_whence = ::libc::SEEK_SET as ::core::ffi::c_short;
        if eFileLock == crate::src::src::os::SHARED_LOCK
            || eFileLock == crate::src::src::os::EXCLUSIVE_LOCK
                && __pFile_ref.eFileLock as ::core::ffi::c_int == crate::src::src::os::RESERVED_LOCK
        {
            lock.l_type = (if eFileLock == crate::src::src::os::SHARED_LOCK {
                ::libc::F_RDLCK
            } else {
                ::libc::F_WRLCK
            }) as ::core::ffi::c_short;
            lock.l_start = crate::src::src::global::sqlite3PendingByte as crate::src::headers::stdlib::__off64_t;
            if unixFileLock(pFile, &raw mut lock) != 0 {
                tErrno = *::libc::__errno_location();
                rc = sqliteErrorFromPosixError(tErrno, crate::src::headers::sqlite3_h::SQLITE_IOERR_LOCK_1);
                if rc != crate::src::headers::sqlite3_h::SQLITE_BUSY {
                    storeLastErrno(pFile, tErrno);
                }
                current_block = 13757627988896076780;
            } else {
                if eFileLock == crate::src::src::os::EXCLUSIVE_LOCK {
                    __pFile_ref.eFileLock = crate::src::src::os::PENDING_LOCK as ::core::ffi::c_uchar;
                    (*pInode).eFileLock = crate::src::src::os::PENDING_LOCK as ::core::ffi::c_uchar;
                }
                current_block = 4488286894823169796;
            }
        } else {
            current_block = 4488286894823169796;
        }
        match current_block {
            13757627988896076780 => {}
            _ => {
                if eFileLock == crate::src::src::os::SHARED_LOCK {
                    lock.l_start = (crate::src::src::global::sqlite3PendingByte + 2 as ::core::ffi::c_int) as crate::src::headers::stdlib::__off64_t;
                    lock.l_len = crate::src::src::os::SHARED_SIZE as crate::src::headers::stdlib::__off64_t;
                    if unixFileLock(pFile, &raw mut lock) != 0 {
                        tErrno = *::libc::__errno_location();
                        rc = sqliteErrorFromPosixError(tErrno, crate::src::headers::sqlite3_h::SQLITE_IOERR_LOCK_1);
                    }
                    lock.l_start = crate::src::src::global::sqlite3PendingByte as crate::src::headers::stdlib::__off64_t;
                    lock.l_len = 1 as ::core::ffi::c_long as crate::src::headers::stdlib::__off64_t;
                    lock.l_type = ::libc::F_UNLCK as ::core::ffi::c_short;
                    if unixFileLock(pFile, &raw mut lock) != 0 && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        tErrno = *::libc::__errno_location();
                        rc = crate::src::headers::sqlite3_h::SQLITE_IOERR_UNLOCK_1;
                    }
                    if rc != 0 {
                        if rc != crate::src::headers::sqlite3_h::SQLITE_BUSY {
                            storeLastErrno(pFile, tErrno);
                        }
                        current_block = 13757627988896076780;
                    } else {
                        __pFile_ref.eFileLock = crate::src::src::os::SHARED_LOCK as ::core::ffi::c_uchar;
                        (*pInode).nLock += 1;
                        (*pInode).nShared = 1 as ::core::ffi::c_int;
                        current_block = 12758904613967585247;
                    }
                } else {
                    if eFileLock == crate::src::src::os::EXCLUSIVE_LOCK && (*pInode).nShared > 1 as ::core::ffi::c_int {
                        rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
                    } else if unixIsSharingShmNode(pFile) != 0 {
                        rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
                    } else {
                        lock.l_type = ::libc::F_WRLCK as ::core::ffi::c_short;
                        if eFileLock == crate::src::src::os::RESERVED_LOCK {
                            lock.l_start =
                                (crate::src::src::global::sqlite3PendingByte + 1 as ::core::ffi::c_int) as crate::src::headers::stdlib::__off64_t;
                            lock.l_len = 1 as ::core::ffi::c_long as crate::src::headers::stdlib::__off64_t;
                        } else {
                            lock.l_start =
                                (crate::src::src::global::sqlite3PendingByte + 2 as ::core::ffi::c_int) as crate::src::headers::stdlib::__off64_t;
                            lock.l_len = crate::src::src::os::SHARED_SIZE as crate::src::headers::stdlib::__off64_t;
                        }
                        if unixFileLock(pFile, &raw mut lock) != 0 {
                            tErrno = *::libc::__errno_location();
                            rc = sqliteErrorFromPosixError(tErrno, crate::src::headers::sqlite3_h::SQLITE_IOERR_LOCK_1);
                            if rc != crate::src::headers::sqlite3_h::SQLITE_BUSY {
                                storeLastErrno(pFile, tErrno);
                            }
                        }
                    }
                    current_block = 12758904613967585247;
                }
                match current_block {
                    13757627988896076780 => {}
                    _ => {
                        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                            __pFile_ref.eFileLock = eFileLock as ::core::ffi::c_uchar;
                            (*pInode).eFileLock = eFileLock as ::core::ffi::c_uchar;
                        }
                    }
                }
            }
        }
    }
    crate::src::src::mutex::sqlite3_mutex_leave((*pInode).pLockMutex);
    if crate::src::src::main::sqlite3OSTrace != 0 {
        crate::src::src::printf::sqlite3DebugPrintf(
            b"LOCK    %d %s %s (unix)\n\0" as *const u8 as *const ::core::ffi::c_char,
            __pFile_ref.h,
            azFileLock(eFileLock),
            if rc == 0 as ::core::ffi::c_int {
                b"ok\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"failed\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
    }
    rc
}

unsafe extern "C" fn setPendingFd(mut pFile: *mut unixFile) {
    let __pFile_ref = unsafe { &mut *pFile };
    let mut pInode: *mut unixInodeInfo = __pFile_ref.pInode;
    let mut p: *mut UnixUnusedFd = __pFile_ref.pPreallocatedUnused;
    (*p).pNext = (*pInode).pUnused;
    (*pInode).pUnused = p;
    __pFile_ref.h = -(1 as ::core::ffi::c_int);
    __pFile_ref.pPreallocatedUnused = ::core::ptr::null_mut::<UnixUnusedFd>();
}

unsafe extern "C" fn posixUnlock(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut eFileLock: ::core::ffi::c_int,
    mut _handleNFSUnlock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut pInode: *mut unixInodeInfo = ::core::ptr::null_mut::<unixInodeInfo>();
    let mut lock: ::libc::flock = unsafe { ::core::mem::zeroed() };
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let __pFile_ref = unsafe { &mut *pFile };
    if crate::src::src::main::sqlite3OSTrace != 0 {
        crate::src::src::printf::sqlite3DebugPrintf(
            b"UNLOCK  %d %d was %d(%d,%d) pid=%d (unix)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            __pFile_ref.h,
            eFileLock,
            __pFile_ref.eFileLock as ::core::ffi::c_int,
            (*__pFile_ref.pInode).eFileLock as ::core::ffi::c_int,
            (*__pFile_ref.pInode).nShared,
            ::libc::getpid(),
        );
    }
    if __pFile_ref.eFileLock as ::core::ffi::c_int <= eFileLock {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    pInode = __pFile_ref.pInode;
    crate::src::src::mutex::sqlite3_mutex_enter((*pInode).pLockMutex);
    if __pFile_ref.eFileLock as ::core::ffi::c_int > crate::src::src::os::SHARED_LOCK {
        if eFileLock == crate::src::src::os::SHARED_LOCK {
            lock.l_type = ::libc::F_RDLCK as ::core::ffi::c_short;
            lock.l_whence = ::libc::SEEK_SET as ::core::ffi::c_short;
            lock.l_start = (crate::src::src::global::sqlite3PendingByte + 2 as ::core::ffi::c_int) as crate::src::headers::stdlib::__off64_t;
            lock.l_len = crate::src::src::os::SHARED_SIZE as crate::src::headers::stdlib::__off64_t;
            if unixFileLock(pFile, &raw mut lock) != 0 {
                rc = crate::src::headers::sqlite3_h::SQLITE_IOERR_RDLOCK_1;
                storeLastErrno(pFile, *::libc::__errno_location());
                current_block = 7847358768392935724;
            } else {
                current_block = 15652330335145281839;
            }
        } else {
            current_block = 15652330335145281839;
        }
        match current_block {
            7847358768392935724 => {}
            _ => {
                lock.l_type = ::libc::F_UNLCK as ::core::ffi::c_short;
                lock.l_whence = ::libc::SEEK_SET as ::core::ffi::c_short;
                lock.l_start = crate::src::src::global::sqlite3PendingByte as crate::src::headers::stdlib::__off64_t;
                lock.l_len = 2 as ::core::ffi::c_long as crate::src::headers::stdlib::__off64_t;
                if unixFileLock(pFile, &raw mut lock) == 0 as ::core::ffi::c_int {
                    (*pInode).eFileLock = crate::src::src::os::SHARED_LOCK as ::core::ffi::c_uchar;
                    current_block = 16203760046146113240;
                } else {
                    rc = crate::src::headers::sqlite3_h::SQLITE_IOERR_UNLOCK_1;
                    storeLastErrno(pFile, *::libc::__errno_location());
                    current_block = 7847358768392935724;
                }
            }
        }
    } else {
        current_block = 16203760046146113240;
    }
    match current_block {
        16203760046146113240 => {
            if eFileLock == crate::src::src::os::NO_LOCK {
                let __pInode_ref = unsafe { &mut *pInode };
                __pInode_ref.nShared -= 1;
                if __pInode_ref.nShared == 0 as ::core::ffi::c_int {
                    lock.l_type = ::libc::F_UNLCK as ::core::ffi::c_short;
                    lock.l_whence = ::libc::SEEK_SET as ::core::ffi::c_short;
                    lock.l_len = 0 as ::core::ffi::c_long as crate::src::headers::stdlib::__off64_t;
                    lock.l_start = lock.l_len;
                    if unixFileLock(pFile, &raw mut lock) == 0 as ::core::ffi::c_int {
                        __pInode_ref.eFileLock = crate::src::src::os::NO_LOCK as ::core::ffi::c_uchar;
                    } else {
                        rc = crate::src::headers::sqlite3_h::SQLITE_IOERR_UNLOCK_1;
                        storeLastErrno(pFile, *::libc::__errno_location());
                        __pInode_ref.eFileLock = crate::src::src::os::NO_LOCK as ::core::ffi::c_uchar;
                        __pFile_ref.eFileLock = crate::src::src::os::NO_LOCK as ::core::ffi::c_uchar;
                    }
                }
                __pInode_ref.nLock -= 1;
                if __pInode_ref.nLock == 0 as ::core::ffi::c_int {
                    closePendingFds(pFile);
                }
            }
        }
        _ => {}
    }
    crate::src::src::mutex::sqlite3_mutex_leave((*pInode).pLockMutex);
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        __pFile_ref.eFileLock = eFileLock as ::core::ffi::c_uchar;
    }
    rc
}

unsafe extern "C" fn unixUnlock(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut eFileLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    posixUnlock(id, eFileLock, 0 as ::core::ffi::c_int)
}

unsafe extern "C" fn closeUnixFile(mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    unixUnmapfile(pFile);
    if (*pFile).h >= 0 as ::core::ffi::c_int {
        robust_close(pFile, (*pFile).h, 2312 as ::core::ffi::c_int);
        (*pFile).h = -(1 as ::core::ffi::c_int);
    }
    if crate::src::src::main::sqlite3OSTrace != 0 {
        crate::src::src::printf::sqlite3DebugPrintf(
            b"CLOSE   %-3d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*pFile).h,
        );
    }
    crate::src::src::os::sqlite3_open_file_count += -(1 as ::core::ffi::c_int);
    crate::src::src::malloc::sqlite3_free((*pFile).pPreallocatedUnused as *mut ::core::ffi::c_void);
    ::libc::memset(
        pFile as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<unixFile>() as crate::__stddef_size_t_h::size_t,
    );
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn unixClose(mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut pInode: *mut unixInodeInfo = (*pFile).pInode;
    verifyDbFile(pFile);
    unixUnlock(id, crate::src::src::os::NO_LOCK);
    unixEnterMutex();
    let __pInode_ref = unsafe { &*pInode };
    crate::src::src::mutex::sqlite3_mutex_enter(__pInode_ref.pLockMutex);
    if __pInode_ref.nLock != 0 {
        setPendingFd(pFile);
    }
    crate::src::src::mutex::sqlite3_mutex_leave(__pInode_ref.pLockMutex);
    releaseInodeInfo(pFile);
    rc = closeUnixFile(id);
    unixLeaveMutex();
    rc
}

unsafe extern "C" fn nolockCheckReservedLock(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    *pResOut = 0 as ::core::ffi::c_int;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn nolockLock(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut _NotUsed2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn nolockUnlock(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut _NotUsed2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn nolockClose(mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int {
    closeUnixFile(id)
}

unsafe extern "C" fn dotlockCheckReservedLock(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    if crate::src::src::os::sqlite3_io_error_persist != 0 && crate::src::src::os::sqlite3_io_error_hit != 0 || {
        let fresh12 = crate::src::src::os::sqlite3_io_error_pending;
        crate::src::src::os::sqlite3_io_error_pending -= 1;
        fresh12 == 1 as ::core::ffi::c_int
    } {
        local_ioerr();
        return 10 as ::core::ffi::c_int | (14 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    }
    if (*pFile).eFileLock as ::core::ffi::c_int >= crate::src::src::os::SHARED_LOCK {
        *pResOut = 0 as ::core::ffi::c_int;
    } else {
        *pResOut = (::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[2 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(
            (*pFile).lockingContext as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    if crate::src::src::main::sqlite3OSTrace != 0 {
        crate::src::src::printf::sqlite3DebugPrintf(
            b"TEST WR-LOCK %d %d %d (dotlock)\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*pFile).h,
            0 as ::core::ffi::c_int,
            *pResOut,
        );
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn dotlockLock(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut eFileLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let __pFile_ref = unsafe { &mut *pFile };
    let mut zLockFile: *mut ::core::ffi::c_char =
        __pFile_ref.lockingContext as *mut ::core::ffi::c_char;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if __pFile_ref.eFileLock as ::core::ffi::c_int > crate::src::src::os::NO_LOCK {
        __pFile_ref.eFileLock = eFileLock as ::core::ffi::c_uchar;
        ::libc::utime(zLockFile,  ::core::ptr::null::<::libc::utimbuf>() as *const ::libc::utimbuf);
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    rc = ::core::mem::transmute::<
        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, crate::src::headers::stdlib::mode_t) -> ::core::ffi::c_int>,
    >(aSyscall[18 as ::core::ffi::c_int as usize].pCurrent)
    .expect("non-null function pointer")(zLockFile, 0o777 as crate::src::headers::stdlib::mode_t);
    if rc < 0 as ::core::ffi::c_int {
        let mut tErrno: ::core::ffi::c_int = *::libc::__errno_location();
        if ::libc::EEXIST == tErrno {
            rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
        } else {
            rc = sqliteErrorFromPosixError(tErrno, crate::src::headers::sqlite3_h::SQLITE_IOERR_LOCK_1);
            if rc != crate::src::headers::sqlite3_h::SQLITE_BUSY {
                storeLastErrno(pFile, tErrno);
            }
        }
        return rc;
    }
    __pFile_ref.eFileLock = eFileLock as ::core::ffi::c_uchar;
    rc
}

unsafe extern "C" fn dotlockUnlock(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut eFileLock: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let __pFile_ref = unsafe { &mut *pFile };
    let mut zLockFile: *mut ::core::ffi::c_char =
        __pFile_ref.lockingContext as *mut ::core::ffi::c_char;
    let mut rc: ::core::ffi::c_int = 0;
    if crate::src::src::main::sqlite3OSTrace != 0 {
        crate::src::src::printf::sqlite3DebugPrintf(
            b"UNLOCK  %d %d was %d pid=%d (dotlock)\n\0" as *const u8 as *const ::core::ffi::c_char,
            __pFile_ref.h,
            eFileLock,
            __pFile_ref.eFileLock as ::core::ffi::c_int,
            ::libc::getpid(),
        );
    }
    if __pFile_ref.eFileLock as ::core::ffi::c_int == eFileLock {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if eFileLock == crate::src::src::os::SHARED_LOCK {
        __pFile_ref.eFileLock = crate::src::src::os::SHARED_LOCK as ::core::ffi::c_uchar;
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    rc = ::core::mem::transmute::<
        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    >(aSyscall[19 as ::core::ffi::c_int as usize].pCurrent)
    .expect("non-null function pointer")(zLockFile);
    if rc < 0 as ::core::ffi::c_int {
        let mut tErrno: ::core::ffi::c_int = *::libc::__errno_location();
        if tErrno == ::libc::ENOENT {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_IOERR_UNLOCK_1;
            storeLastErrno(pFile, tErrno);
        }
        return rc;
    }
    __pFile_ref.eFileLock = crate::src::src::os::NO_LOCK as ::core::ffi::c_uchar;
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn dotlockClose(mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    dotlockUnlock(id, crate::src::src::os::NO_LOCK);
    crate::src::src::malloc::sqlite3_free((*pFile).lockingContext);
    closeUnixFile(id)
}

unsafe extern "C" fn seekAndRead(
    mut id: *mut unixFile,
    mut offset: crate::src::headers::sqlite3_h::sqlite3_int64,
    mut pBuf: *mut ::core::ffi::c_void,
    mut cnt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut got: ::core::ffi::c_int = 0;
    let mut prior: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        got = ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                    crate::__stddef_size_t_h::size_t,
                    crate::src::headers::stdlib::off64_t,
                ) -> crate::src::headers::stdlib::ssize_t,
            >,
        >(aSyscall[10 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(
            (*id).h, pBuf, cnt as crate::__stddef_size_t_h::size_t, offset as crate::src::headers::stdlib::off64_t
        ) as ::core::ffi::c_int;
        if crate::src::src::os::sqlite3_io_error_persist != 0 && crate::src::src::os::sqlite3_io_error_hit != 0 || {
            let fresh17 = crate::src::src::os::sqlite3_io_error_pending;
            crate::src::src::os::sqlite3_io_error_pending -= 1;
            fresh17 == 1 as ::core::ffi::c_int
        } {
            local_ioerr();
            got = -(1 as ::core::ffi::c_int);
        }
        if got == cnt {
            break;
        }
        if got < 0 as ::core::ffi::c_int {
            if *::libc::__errno_location() == ::libc::EINTR {
                got = 1 as ::core::ffi::c_int;
            } else {
                prior = 0 as ::core::ffi::c_int;
                storeLastErrno(id, *::libc::__errno_location());
                break;
            }
        } else if got > 0 as ::core::ffi::c_int {
            cnt -= got;
            offset += got as crate::src::headers::sqlite3_h::sqlite3_int64;
            prior += got;
            pBuf =
                (pBuf as *mut ::core::ffi::c_char).offset(got as isize) as *mut ::core::ffi::c_void;
        }
        if !(got > 0 as ::core::ffi::c_int) {
            break;
        }
    }
    if crate::src::src::main::sqlite3OSTrace != 0 {
        crate::src::src::printf::sqlite3DebugPrintf(
            b"READ    %-3d %5d %7lld %llu\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*id).h,
            got + prior,
            offset - prior as crate::src::headers::sqlite3_h::sqlite3_int64,
            0 as ::core::ffi::c_int as crate::src::headers::sqlite3_h::sqlite_uint64,
        );
    }
    got + prior
}

unsafe extern "C" fn unixRead(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut pBuf: *mut ::core::ffi::c_void,
    mut amt: ::core::ffi::c_int,
    mut offset: crate::src::headers::sqlite3_h::sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut got: ::core::ffi::c_int = 0;
    if offset < (*pFile).mmapSize {
        if offset + amt as crate::src::headers::sqlite3_h::sqlite3_int64 <= (*pFile).mmapSize {
            ::libc::memcpy(
                pBuf,
                ((*pFile).pMapRegion as *mut crate::src::ext::rtree::rtree::u8_0).offset(offset as isize) as *mut crate::src::ext::rtree::rtree::u8_0
                    as *const ::core::ffi::c_void,
                amt as crate::__stddef_size_t_h::size_t,
            );
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        } else {
            let mut nCopy: ::core::ffi::c_int = ((*pFile).mmapSize - offset) as ::core::ffi::c_int;
            ::libc::memcpy(
                pBuf,
                ((*pFile).pMapRegion as *mut crate::src::ext::rtree::rtree::u8_0).offset(offset as isize) as *mut crate::src::ext::rtree::rtree::u8_0
                    as *const ::core::ffi::c_void,
                nCopy as crate::__stddef_size_t_h::size_t,
            );
            pBuf =
                (pBuf as *mut crate::src::ext::rtree::rtree::u8_0).offset(nCopy as isize) as *mut crate::src::ext::rtree::rtree::u8_0 as *mut ::core::ffi::c_void;
            amt -= nCopy;
            offset += nCopy as crate::src::headers::sqlite3_h::sqlite3_int64;
        }
    }
    got = seekAndRead(pFile, offset, pBuf, amt);
    if got == amt {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    } else if got < 0 as ::core::ffi::c_int {
        match  (*pFile).lastErrno {
    ::libc::ERANGE | ::libc::EIO | ::libc::ENXIO =>
         return crate::src::headers::sqlite3_h::SQLITE_IOERR_CORRUPTFS,
    _ =>  {}
}
        return crate::src::headers::sqlite3_h::SQLITE_IOERR_READ_1;
    } else {
        storeLastErrno(pFile, 0 as ::core::ffi::c_int);
        ::libc::memset(
            (pBuf as *mut ::core::ffi::c_char).offset(got as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (amt - got) as crate::__stddef_size_t_h::size_t,
        );
        return crate::src::headers::sqlite3_h::SQLITE_IOERR_SHORT_READ_1;
    };
}

unsafe extern "C" fn seekAndWriteFd(
    mut fd: ::core::ffi::c_int,
    mut iOff: crate::src::ext::rtree::rtree::i64_0,
    mut pBuf: *const ::core::ffi::c_void,
    mut nBuf: ::core::ffi::c_int,
    mut piErrno: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    nBuf &= 0x1ffff as ::core::ffi::c_int;
    loop {
        rc = ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                    crate::__stddef_size_t_h::size_t,
                    crate::src::headers::stdlib::off64_t,
                ) -> crate::src::headers::stdlib::ssize_t,
            >,
        >(aSyscall[13 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(fd, pBuf, nBuf as crate::__stddef_size_t_h::size_t, iOff as crate::src::headers::stdlib::off64_t)
            as ::core::ffi::c_int;
        if !(rc < 0 as ::core::ffi::c_int && *::libc::__errno_location() == ::libc::EINTR) {
            break;
        }
    }
    if crate::src::src::main::sqlite3OSTrace != 0 {
        crate::src::src::printf::sqlite3DebugPrintf(
            b"WRITE   %-3d %5d %7lld %llu\n\0" as *const u8 as *const ::core::ffi::c_char,
            fd,
            rc,
            iOff,
            0 as ::core::ffi::c_int as crate::src::headers::sqlite3_h::sqlite_uint64,
        );
    }
    if rc < 0 as ::core::ffi::c_int {
        *piErrno = *::libc::__errno_location();
    }
    rc
}

unsafe extern "C" fn seekAndWrite(
    mut id: *mut unixFile,
    mut offset: crate::src::ext::rtree::rtree::i64_0,
    mut pBuf: *const ::core::ffi::c_void,
    mut cnt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    seekAndWriteFd((*id).h, offset, pBuf, cnt, &raw mut (*id).lastErrno)
}

unsafe extern "C" fn unixWrite(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut pBuf: *const ::core::ffi::c_void,
    mut amt: ::core::ffi::c_int,
    mut offset: crate::src::headers::sqlite3_h::sqlite3_int64,
) -> ::core::ffi::c_int {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut wrote: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        wrote = seekAndWrite(pFile, offset as crate::src::ext::rtree::rtree::i64_0, pBuf, amt);
        if !(wrote < amt && wrote > 0 as ::core::ffi::c_int) {
            break;
        }
        amt -= wrote;
        offset += wrote as crate::src::headers::sqlite3_h::sqlite3_int64;
        pBuf = (pBuf as *mut ::core::ffi::c_char).offset(wrote as isize) as *mut ::core::ffi::c_char
            as *const ::core::ffi::c_void;
    }
    if crate::src::src::os::sqlite3_io_error_persist != 0 && crate::src::src::os::sqlite3_io_error_hit != 0 || {
        let fresh16 = crate::src::src::os::sqlite3_io_error_pending;
        crate::src::src::os::sqlite3_io_error_pending -= 1;
        fresh16 == 1 as ::core::ffi::c_int
    } {
        local_ioerr();
        wrote = -(1 as ::core::ffi::c_int);
        amt = 1 as ::core::ffi::c_int;
    }
    if crate::src::src::os::sqlite3_diskfull_pending != 0 {
        if crate::src::src::os::sqlite3_diskfull_pending == 1 as ::core::ffi::c_int {
            local_ioerr();
            crate::src::src::os::sqlite3_diskfull = 1 as ::core::ffi::c_int;
            crate::src::src::os::sqlite3_io_error_hit = 1 as ::core::ffi::c_int;
            wrote = 0 as ::core::ffi::c_int;
            amt = 1 as ::core::ffi::c_int;
        } else {
            crate::src::src::os::sqlite3_diskfull_pending -= 1;
        }
    }
    if amt > wrote {
        if wrote < 0 as ::core::ffi::c_int && (*pFile).lastErrno != ::libc::ENOSPC {
            return crate::src::headers::sqlite3_h::SQLITE_IOERR_WRITE_1;
        } else {
            storeLastErrno(pFile, 0 as ::core::ffi::c_int);
            return crate::src::headers::sqlite3_h::SQLITE_FULL;
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[unsafe(no_mangle)]

pub static mut sqlite3_sync_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]

pub static mut sqlite3_fullsync_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

unsafe extern "C" fn full_fsync(
    mut fd: ::core::ffi::c_int,
    mut fullSync: ::core::ffi::c_int,
    mut _dataOnly: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    if fullSync != 0 {
        sqlite3_fullsync_count += 1;
    }
    sqlite3_sync_count += 1;
    let mut buf: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
    rc = ::core::mem::transmute::<
        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
        Option<unsafe extern "C" fn(::core::ffi::c_int, *mut crate::src::headers::stdlib::stat) -> ::core::ffi::c_int>,
    >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
    .expect("non-null function pointer")(fd, &raw mut buf);
    if crate::vxworks_h::OS_VXWORKS != 0 && rc != -(1 as ::core::ffi::c_int) {
        rc = 0 as ::core::ffi::c_int;
    }
    rc
}

unsafe extern "C" fn openDirectory(
    mut zFilename: *const ::core::ffi::c_char,
    mut pFd: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ii: ::core::ffi::c_int = 0;
    let mut fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut zDirname: [::core::ffi::c_char; 513] = [0; 513];
    crate::src::src::printf::sqlite3_snprintf(
        MAX_PATHNAME,
        &raw mut zDirname as *mut ::core::ffi::c_char,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        zFilename,
    );
    ii = ::libc::strlen(&raw mut zDirname as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
    while ii > 0 as ::core::ffi::c_int && zDirname[ii as usize] as ::core::ffi::c_int != '/' as i32
    {
        ii -= 1;
    }
    if ii > 0 as ::core::ffi::c_int {
        zDirname[ii as usize] = '\0' as i32 as ::core::ffi::c_char;
    } else {
        if zDirname[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '/' as i32 {
            zDirname[0 as ::core::ffi::c_int as usize] = '.' as i32 as ::core::ffi::c_char;
        }
        zDirname[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    }
    fd = robust_open(
        &raw mut zDirname as *mut ::core::ffi::c_char,
        ::libc::O_RDONLY | O_BINARY,
        0 as crate::src::headers::stdlib::mode_t,
    );
    if fd >= 0 as ::core::ffi::c_int {
        if crate::src::src::main::sqlite3OSTrace != 0 {
            crate::src::src::printf::sqlite3DebugPrintf(
                b"OPENDIR %-3d %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                fd,
                &raw mut zDirname as *mut ::core::ffi::c_char,
            );
        }
    }
    *pFd = fd;
    if fd >= 0 as ::core::ffi::c_int {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    unixLogErrorAtLine(
        crate::src::src::main::sqlite3CantopenError(3893 as ::core::ffi::c_int),
        b"openDirectory\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut zDirname as *mut ::core::ffi::c_char,
        3893 as ::core::ffi::c_int,
    )
}

unsafe extern "C" fn unixSync(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut isDataOnly: ::core::ffi::c_int = flags & crate::src::headers::sqlite3_h::SQLITE_SYNC_DATAONLY;
    let mut isFullsync: ::core::ffi::c_int =
        (flags & 0xf as ::core::ffi::c_int == crate::src::headers::sqlite3_h::SQLITE_SYNC_FULL) as ::core::ffi::c_int;
    if crate::src::src::os::sqlite3_diskfull_pending != 0 {
        if crate::src::src::os::sqlite3_diskfull_pending == 1 as ::core::ffi::c_int {
            local_ioerr();
            crate::src::src::os::sqlite3_diskfull = 1 as ::core::ffi::c_int;
            crate::src::src::os::sqlite3_io_error_hit = 1 as ::core::ffi::c_int;
            return 13 as ::core::ffi::c_int;
        } else {
            crate::src::src::os::sqlite3_diskfull_pending -= 1;
        }
    }
    if crate::src::src::main::sqlite3OSTrace != 0 {
        crate::src::src::printf::sqlite3DebugPrintf(
            b"SYNC    %-3d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*pFile).h,
        );
    }
    rc = full_fsync((*pFile).h, isFullsync, isDataOnly);
    if crate::src::src::os::sqlite3_io_error_persist != 0 && crate::src::src::os::sqlite3_io_error_hit != 0 || {
        let fresh14 = crate::src::src::os::sqlite3_io_error_pending;
        crate::src::src::os::sqlite3_io_error_pending -= 1;
        fresh14 == 1 as ::core::ffi::c_int
    } {
        local_ioerr();
        rc = 1 as ::core::ffi::c_int;
    }
    if rc != 0 {
        storeLastErrno(pFile, *::libc::__errno_location());
        return unixLogErrorAtLine(
            10 as ::core::ffi::c_int | (4 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            b"full_fsync\0" as *const u8 as *const ::core::ffi::c_char,
            (*pFile).zPath,
            3934 as ::core::ffi::c_int,
        );
    }
    if (*pFile).ctrlFlags as ::core::ffi::c_int & UNIXFILE_DIRSYNC != 0 {
        let mut dirfd: ::core::ffi::c_int = 0;
        let __pFile_ref = unsafe { &mut *pFile };
        if crate::src::src::main::sqlite3OSTrace != 0 {
            crate::src::src::printf::sqlite3DebugPrintf(
                b"DIRSYNC %s (have_fullfsync=%d fullsync=%d)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                __pFile_ref.zPath,
                0 as ::core::ffi::c_int,
                isFullsync,
            );
        }
        rc = ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[17 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(__pFile_ref.zPath, &raw mut dirfd);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            full_fsync(dirfd, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
            robust_close(pFile, dirfd, 3948 as ::core::ffi::c_int);
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
        __pFile_ref.ctrlFlags =
            (__pFile_ref.ctrlFlags as ::core::ffi::c_int & !UNIXFILE_DIRSYNC) as ::core::ffi::c_ushort;
    }
    rc
}

unsafe extern "C" fn unixTruncate(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut nByte: crate::src::ext::rtree::rtree::i64_0,
) -> ::core::ffi::c_int {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    let mut rc: ::core::ffi::c_int = 0;
    if crate::src::src::os::sqlite3_io_error_persist != 0 && crate::src::src::os::sqlite3_io_error_hit != 0 || {
        let fresh15 = crate::src::src::os::sqlite3_io_error_pending;
        crate::src::src::os::sqlite3_io_error_pending -= 1;
        fresh15 == 1 as ::core::ffi::c_int
    } {
        local_ioerr();
        return 10 as ::core::ffi::c_int | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    }
    if (*pFile).szChunk > 0 as ::core::ffi::c_int {
        let __pFile_ref = unsafe { &*pFile };
        nByte = (nByte + __pFile_ref.szChunk as crate::src::ext::rtree::rtree::i64_0 - 1 as crate::src::ext::rtree::rtree::i64_0) / __pFile_ref.szChunk as crate::src::ext::rtree::rtree::i64_0
            * __pFile_ref.szChunk as crate::src::ext::rtree::rtree::i64_0;
    }
    rc = robust_ftruncate((*pFile).h, nByte as crate::src::headers::sqlite3_h::sqlite3_int64);
    if rc != 0 {
        storeLastErrno(pFile, *::libc::__errno_location());
        return unixLogErrorAtLine(
            10 as ::core::ffi::c_int | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
            b"ftruncate\0" as *const u8 as *const ::core::ffi::c_char,
            (*pFile).zPath,
            3979 as ::core::ffi::c_int,
        );
    } else {
        if nByte < (*pFile).mmapSize {
            (*pFile).mmapSize = nByte as crate::src::headers::sqlite3_h::sqlite3_int64;
        }
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    };
}

unsafe extern "C" fn unixFileSize(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut pSize: *mut crate::src::ext::rtree::rtree::i64_0,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut buf: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
    rc = ::core::mem::transmute::<
        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
        Option<unsafe extern "C" fn(::core::ffi::c_int, *mut crate::src::headers::stdlib::stat) -> ::core::ffi::c_int>,
    >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
    .expect("non-null function pointer")((*(id as *mut unixFile)).h, &raw mut buf);
    if crate::src::src::os::sqlite3_io_error_persist != 0 && crate::src::src::os::sqlite3_io_error_hit != 0 || {
        let fresh13 = crate::src::src::os::sqlite3_io_error_pending;
        crate::src::src::os::sqlite3_io_error_pending -= 1;
        fresh13 == 1 as ::core::ffi::c_int
    } {
        local_ioerr();
        rc = 1 as ::core::ffi::c_int;
    }
    if rc != 0 as ::core::ffi::c_int {
        storeLastErrno(id as *mut unixFile, *::libc::__errno_location());
        return crate::src::headers::sqlite3_h::SQLITE_IOERR_FSTAT_1;
    }
    *pSize = buf.st_size as crate::src::ext::rtree::rtree::i64_0;
    if *pSize == 1 as crate::src::ext::rtree::rtree::i64_0 {
        *pSize = 0 as crate::src::ext::rtree::rtree::i64_0;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn fcntlSizeHint(
    mut pFile: *mut unixFile,
    mut nByte: crate::src::ext::rtree::rtree::i64_0,
) -> ::core::ffi::c_int {
    let __pFile_ref = unsafe { &*pFile };
    if __pFile_ref.szChunk > 0 as ::core::ffi::c_int {
        let mut nSize: crate::src::ext::rtree::rtree::i64_0 = 0;
        let mut buf: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
        if ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<unsafe extern "C" fn(::core::ffi::c_int, *mut crate::src::headers::stdlib::stat) -> ::core::ffi::c_int>,
        >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(__pFile_ref.h, &raw mut buf)
            != 0
        {
            return crate::src::headers::sqlite3_h::SQLITE_IOERR_FSTAT_1;
        }
        nSize = (nByte + __pFile_ref.szChunk as crate::src::ext::rtree::rtree::i64_0 - 1 as crate::src::ext::rtree::rtree::i64_0) / __pFile_ref.szChunk as crate::src::ext::rtree::rtree::i64_0
            * __pFile_ref.szChunk as crate::src::ext::rtree::rtree::i64_0;
        if nSize > buf.st_size as crate::src::ext::rtree::rtree::i64_0 {
            let mut nBlk: ::core::ffi::c_int = buf.st_blksize as ::core::ffi::c_int;
            let mut nWrite: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut iWrite: crate::src::ext::rtree::rtree::i64_0 = 0;
            iWrite = (buf.st_size / nBlk as crate::src::headers::stdlib::__off_t * nBlk as crate::src::headers::stdlib::__off_t + nBlk as crate::src::headers::stdlib::__off_t
                - 1 as crate::src::headers::stdlib::__off_t) as crate::src::ext::rtree::rtree::i64_0;
            while iWrite < nSize + nBlk as crate::src::ext::rtree::rtree::i64_0 - 1 as crate::src::ext::rtree::rtree::i64_0 {
                if iWrite >= nSize {
                    iWrite = nSize - 1 as crate::src::ext::rtree::rtree::i64_0;
                }
                nWrite = seekAndWrite(
                    pFile,
                    iWrite,
                    b"\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                    1 as ::core::ffi::c_int,
                );
                if nWrite != 1 as ::core::ffi::c_int {
                    return crate::src::headers::sqlite3_h::SQLITE_IOERR_WRITE_1;
                }
                iWrite += nBlk as crate::src::ext::rtree::rtree::i64_0;
            }
        }
    }
    if __pFile_ref.mmapSizeMax > 0 as crate::src::headers::sqlite3_h::sqlite3_int64 && nByte > __pFile_ref.mmapSize {
        let mut rc: ::core::ffi::c_int = 0;
        if __pFile_ref.szChunk <= 0 as ::core::ffi::c_int {
            if robust_ftruncate(__pFile_ref.h, nByte as crate::src::headers::sqlite3_h::sqlite3_int64) != 0 {
                storeLastErrno(pFile, *::libc::__errno_location());
                return unixLogErrorAtLine(
                    10 as ::core::ffi::c_int | (6 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                    b"ftruncate\0" as *const u8 as *const ::core::ffi::c_char,
                    __pFile_ref.zPath,
                    4100 as ::core::ffi::c_int,
                );
            }
        }
        rc = unixMapfile(pFile, nByte);
        return rc;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn unixModeBit(
    mut pFile: *mut unixFile,
    mut mask: ::core::ffi::c_uchar,
    mut pArg: *mut ::core::ffi::c_int,
) {
    if *pArg < 0 as ::core::ffi::c_int {
        *pArg = ((*pFile).ctrlFlags as ::core::ffi::c_int & mask as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    } else if *pArg == 0 as ::core::ffi::c_int {
        (*pFile).ctrlFlags = ((*pFile).ctrlFlags as ::core::ffi::c_int
            & !(mask as ::core::ffi::c_int)) as ::core::ffi::c_ushort;
    } else {
        (*pFile).ctrlFlags = ((*pFile).ctrlFlags as ::core::ffi::c_int | mask as ::core::ffi::c_int)
            as ::core::ffi::c_ushort;
    };
}

unsafe extern "C" fn unixFileControl(
    mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut op: ::core::ffi::c_int,
    mut pArg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pFile: *mut unixFile = id as *mut unixFile;
    match  op {
    crate::src::headers::sqlite3_h::SQLITE_FCNTL_NULL_IO =>  {
            ::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
            >(aSyscall[1 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")((*pFile).h);
            (*pFile).h = -(1 as ::core::ffi::c_int);
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    crate::src::headers::sqlite3_h::SQLITE_FCNTL_LOCKSTATE =>  {
            *(pArg as *mut ::core::ffi::c_int) = (*pFile).eFileLock as ::core::ffi::c_int;
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    crate::src::headers::sqlite3_h::SQLITE_FCNTL_LAST_ERRNO =>  {
            *(pArg as *mut ::core::ffi::c_int) = (*pFile).lastErrno;
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    crate::src::headers::sqlite3_h::SQLITE_FCNTL_CHUNK_SIZE =>  {
            (*pFile).szChunk = *(pArg as *mut ::core::ffi::c_int);
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    crate::src::headers::sqlite3_h::SQLITE_FCNTL_SIZE_HINT =>  {
            let mut rc: ::core::ffi::c_int = 0;
            crate::src::src::os::sqlite3_io_error_benign = 1 as ::core::ffi::c_int;
            rc = fcntlSizeHint(pFile, *(pArg as *mut crate::src::ext::rtree::rtree::i64_0));
            crate::src::src::os::sqlite3_io_error_benign = 0 as ::core::ffi::c_int;
            return rc;
        }
    crate::src::headers::sqlite3_h::SQLITE_FCNTL_PERSIST_WAL =>  {
            unixModeBit(
                pFile,
                UNIXFILE_PERSIST_WAL as ::core::ffi::c_uchar,
                pArg as *mut ::core::ffi::c_int,
            );
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    crate::src::headers::sqlite3_h::SQLITE_FCNTL_POWERSAFE_OVERWRITE =>  {
            unixModeBit(
                pFile,
                UNIXFILE_PSOW as ::core::ffi::c_uchar,
                pArg as *mut ::core::ffi::c_int,
            );
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    crate::src::headers::sqlite3_h::SQLITE_FCNTL_VFSNAME_1 =>  {
            let ref mut fresh7 = *(pArg as *mut *mut ::core::ffi::c_char);
            *fresh7 = crate::src::src::printf::sqlite3_mprintf(
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                (*(*pFile).pVfs).zName,
            );
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    crate::src::headers::sqlite3_h::SQLITE_FCNTL_TEMPFILENAME =>  {
            let mut zTFile: *mut ::core::ffi::c_char =
                crate::src::src::malloc::sqlite3_malloc64((*(*pFile).pVfs).mxPathname as crate::src::headers::sqlite3_h::sqlite3_uint64)
                    as *mut ::core::ffi::c_char;
            if !zTFile.is_null() {
                unixGetTempname((*(*pFile).pVfs).mxPathname, zTFile);
                let ref mut fresh8 = *(pArg as *mut *mut ::core::ffi::c_char);
                *fresh8 = zTFile;
            }
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    crate::src::headers::sqlite3_h::SQLITE_FCNTL_HAS_MOVED =>  {
            *(pArg as *mut ::core::ffi::c_int) = fileHasMoved(pFile);
            return crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    crate::src::headers::sqlite3_h::SQLITE_FCNTL_MMAP_SIZE =>  {
            let mut newLimit: crate::src::ext::rtree::rtree::i64_0 = *(pArg as *mut crate::src::ext::rtree::rtree::i64_0);
            let mut rc_0: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
            if newLimit > crate::src::src::global::sqlite3Config.mxMmap {
                newLimit = crate::src::src::global::sqlite3Config.mxMmap as crate::src::ext::rtree::rtree::i64_0;
            }
            if newLimit > 0 as crate::src::ext::rtree::rtree::i64_0 && (::core::mem::size_of::<crate::__stddef_size_t_h::size_t>() as usize) < 8 as usize {
                newLimit &= 0x7fffffff as crate::src::ext::rtree::rtree::i64_0;
            }
            let __pFile_ref = unsafe { &mut *pFile };
            *(pArg as *mut crate::src::ext::rtree::rtree::i64_0) = __pFile_ref.mmapSizeMax as crate::src::ext::rtree::rtree::i64_0;
            if newLimit >= 0 as crate::src::ext::rtree::rtree::i64_0
                && newLimit != __pFile_ref.mmapSizeMax
                && __pFile_ref.nFetchOut == 0 as ::core::ffi::c_int
            {
                __pFile_ref.mmapSizeMax = newLimit as crate::src::headers::sqlite3_h::sqlite3_int64;
                if __pFile_ref.mmapSize > 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
                    unixUnmapfile(pFile);
                    rc_0 = unixMapfile(pFile, -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0);
                }
            }
            return rc_0;
        }
    crate::src::headers::sqlite3_h::SQLITE_FCNTL_EXTERNAL_READER =>  {
            return unixFcntlExternalReader(id as *mut unixFile, pArg as *mut ::core::ffi::c_int);
        }
    _ =>  {}
}
    crate::src::headers::sqlite3_h::SQLITE_NOTFOUND
}

unsafe extern "C" fn setDeviceCharacteristics(mut pFd: *mut unixFile) {
    if (*pFd).sectorSize == 0 as ::core::ffi::c_int {
        let __pFd_ref = unsafe { &mut *pFd };
        if __pFd_ref.ctrlFlags as ::core::ffi::c_int & UNIXFILE_PSOW != 0 {
            __pFd_ref.deviceCharacteristics |= crate::src::headers::sqlite3_h::SQLITE_IOCAP_POWERSAFE_OVERWRITE;
        }
        __pFd_ref.deviceCharacteristics |= crate::src::headers::sqlite3_h::SQLITE_IOCAP_SUBPAGE_READ;
        __pFd_ref.sectorSize = crate::src::src::os::SQLITE_DEFAULT_SECTOR_SIZE;
    }
}

unsafe extern "C" fn unixSectorSize(mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int {
    let mut pFd: *mut unixFile = id as *mut unixFile;
    setDeviceCharacteristics(pFd);
    (*pFd).sectorSize
}

unsafe extern "C" fn unixDeviceCharacteristics(mut id: *mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int {
    let mut pFd: *mut unixFile = id as *mut unixFile;
    setDeviceCharacteristics(pFd);
    (*pFd).deviceCharacteristics
}

unsafe extern "C" fn unixGetpagesize() -> ::core::ffi::c_int {
    ::libc::sysconf(crate::src::headers::stdlib::_SC_PAGESIZE as ::core::ffi::c_int) as ::core::ffi::c_int
}

pub const UNIX_SHM_BASE: ::core::ffi::c_int =
    (22 as ::core::ffi::c_int + crate::src::headers::sqlite3_h::SQLITE_SHM_NLOCK) * 4 as ::core::ffi::c_int;

pub const UNIX_SHM_DMS: ::core::ffi::c_int = UNIX_SHM_BASE + crate::src::headers::sqlite3_h::SQLITE_SHM_NLOCK;

unsafe extern "C" fn unixFcntlExternalReader(
    mut pFile: *mut unixFile,
    mut piOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    *piOut = 0 as ::core::ffi::c_int;
    if !(*pFile).pShm.is_null() {
        let mut pShmNode: *mut unixShmNode = (*(*pFile).pShm).pShmNode;
        let mut f: ::libc::flock = unsafe { ::core::mem::zeroed() };
        f.l_type = ::libc::F_WRLCK as ::core::ffi::c_short;
        f.l_whence = ::libc::SEEK_SET as ::core::ffi::c_short;
        f.l_start = (UNIX_SHM_BASE + 3 as ::core::ffi::c_int) as crate::src::headers::stdlib::__off64_t;
        f.l_len = (crate::src::headers::sqlite3_h::SQLITE_SHM_NLOCK - 3 as ::core::ffi::c_int) as crate::src::headers::stdlib::__off64_t;
        let __pShmNode_ref = unsafe { &*pShmNode };
        crate::src::src::mutex::sqlite3_mutex_enter(__pShmNode_ref.pShmMutex);
        if ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ...
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(__pShmNode_ref.hShm, ::libc::F_GETLK, &raw mut f)
            < 0 as ::core::ffi::c_int
        {
            rc = crate::src::headers::sqlite3_h::SQLITE_IOERR_LOCK_1;
        } else {
            *piOut = (f.l_type as ::core::ffi::c_int != ::libc::F_UNLCK) as ::core::ffi::c_int;
        }
        crate::src::src::mutex::sqlite3_mutex_leave(__pShmNode_ref.pShmMutex);
    }
    rc
}

unsafe extern "C" fn unixIsSharingShmNode(mut pFile: *mut unixFile) -> ::core::ffi::c_int {
    let mut pShmNode: *mut unixShmNode = ::core::ptr::null_mut::<unixShmNode>();
    let mut lock: ::libc::flock = unsafe { ::core::mem::zeroed() };
    let __pFile_ref = unsafe { &mut *pFile };
    if __pFile_ref.pShm.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if __pFile_ref.ctrlFlags as ::core::ffi::c_int & UNIXFILE_EXCL != 0 {
        return 0 as ::core::ffi::c_int;
    }
    pShmNode = (*__pFile_ref.pShm).pShmNode;
    lock.l_whence = ::libc::SEEK_SET as ::core::ffi::c_short;
    lock.l_start = UNIX_SHM_DMS as crate::src::headers::stdlib::__off64_t;
    lock.l_len = 1 as crate::src::headers::stdlib::__off64_t;
    lock.l_type = ::libc::F_WRLCK as ::core::ffi::c_short;
    ::core::mem::transmute::<
        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
        Option<
            unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int, ...) -> ::core::ffi::c_int,
        >,
    >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
    .expect("non-null function pointer")((*pShmNode).hShm, ::libc::F_GETLK, &raw mut lock);
    (lock.l_type as ::core::ffi::c_int != ::libc::F_UNLCK) as ::core::ffi::c_int
}

unsafe extern "C" fn unixShmSystemLock(
    mut pFile: *mut unixFile,
    mut lockType: ::core::ffi::c_int,
    mut ofst: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pShmNode: *mut unixShmNode = ::core::ptr::null_mut::<unixShmNode>();
    let mut f: ::libc::flock = unsafe { ::core::mem::zeroed() };
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    pShmNode = (*(*pFile).pInode).pShmNode;
    ofst == UNIX_SHM_DMS;
    if (*pShmNode).hShm >= 0 as ::core::ffi::c_int {
        let mut res: ::core::ffi::c_int = 0;
        f.l_type = lockType as ::core::ffi::c_short;
        f.l_whence = ::libc::SEEK_SET as ::core::ffi::c_short;
        f.l_start = ofst as crate::src::headers::stdlib::__off64_t;
        f.l_len = n as crate::src::headers::stdlib::__off64_t;
        res = ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ...
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")((*pShmNode).hShm, ::libc::F_SETLK, &raw mut f);
        if res == -(1 as ::core::ffi::c_int) {
            rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
        }
    }
    rc
}

unsafe extern "C" fn unixShmRegionPerMap() -> ::core::ffi::c_int {
    let mut shmsz: ::core::ffi::c_int = 32 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
    let mut pgsz: ::core::ffi::c_int = ::core::mem::transmute::<
        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
        Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    >(aSyscall[25 as ::core::ffi::c_int as usize].pCurrent)
    .expect("non-null function pointer")();
    if pgsz < shmsz {
        return 1 as ::core::ffi::c_int;
    }
    pgsz / shmsz
}

unsafe extern "C" fn unixShmPurge(mut pFd: *mut unixFile) {
    let mut p: *mut unixShmNode = (*(*pFd).pInode).pShmNode;
    if !p.is_null() && (*p).nRef == 0 as ::core::ffi::c_int {
        let mut nShmPerMap: ::core::ffi::c_int = unixShmRegionPerMap();
        let mut i: ::core::ffi::c_int = 0;
        let __p_ref = unsafe { &mut *p };
        crate::src::src::mutex::sqlite3_mutex_free(__p_ref.pShmMutex);
        i = 0 as ::core::ffi::c_int;
        while i < __p_ref.nRegion as ::core::ffi::c_int {
            if __p_ref.hShm >= 0 as ::core::ffi::c_int {
                ::core::mem::transmute::<
                    crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            crate::__stddef_size_t_h::size_t,
                        ) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[23 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")(
                    *__p_ref.apRegion.offset(i as isize) as *mut ::core::ffi::c_void,
                    __p_ref.szRegion as crate::__stddef_size_t_h::size_t,
                );
            } else {
                crate::src::src::malloc::sqlite3_free(*__p_ref.apRegion.offset(i as isize) as *mut ::core::ffi::c_void);
            }
            i += nShmPerMap;
        }
        crate::src::src::malloc::sqlite3_free(__p_ref.apRegion as *mut ::core::ffi::c_void);
        if __p_ref.hShm >= 0 as ::core::ffi::c_int {
            robust_close(pFd, __p_ref.hShm, 4833 as ::core::ffi::c_int);
            __p_ref.hShm = -(1 as ::core::ffi::c_int);
        }
        (*__p_ref.pInode).pShmNode = ::core::ptr::null_mut::<unixShmNode>();
        crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
    }
}

unsafe extern "C" fn unixLockSharedMemory(
    mut pDbFd: *mut unixFile,
    mut pShmNode: *mut unixShmNode,
) -> ::core::ffi::c_int {
    let mut lock: ::libc::flock = unsafe { ::core::mem::zeroed() };
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    lock.l_whence = ::libc::SEEK_SET as ::core::ffi::c_short;
    lock.l_start = UNIX_SHM_DMS as crate::src::headers::stdlib::__off64_t;
    lock.l_len = 1 as crate::src::headers::stdlib::__off64_t;
    lock.l_type = ::libc::F_WRLCK as ::core::ffi::c_short;
    if ::core::mem::transmute::<
        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
        Option<
            unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int, ...) -> ::core::ffi::c_int,
        >,
    >(aSyscall[7 as ::core::ffi::c_int as usize].pCurrent)
    .expect("non-null function pointer")((*pShmNode).hShm, ::libc::F_GETLK, &raw mut lock)
        != 0 as ::core::ffi::c_int
    {
        rc = crate::src::headers::sqlite3_h::SQLITE_IOERR_LOCK_1;
    } else if lock.l_type as ::core::ffi::c_int == ::libc::F_UNLCK {
        if (*pShmNode).isReadonly != 0 {
            (*pShmNode).isUnlocked = 1 as crate::src::ext::rtree::rtree::u8_0;
            rc = crate::src::headers::sqlite3_h::SQLITE_READONLY_CANTINIT_1;
        } else {
            rc = unixShmSystemLock(pDbFd, ::libc::F_WRLCK, UNIX_SHM_DMS, 1 as ::core::ffi::c_int);
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK && robust_ftruncate((*pShmNode).hShm, 3 as crate::src::headers::sqlite3_h::sqlite3_int64) != 0 {
                rc = unixLogErrorAtLine(
                    10 as ::core::ffi::c_int
                        | (18 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                    b"ftruncate\0" as *const u8 as *const ::core::ffi::c_char,
                    (*pShmNode).zFilename,
                    4903 as ::core::ffi::c_int,
                );
            }
        }
    } else if lock.l_type as ::core::ffi::c_int == ::libc::F_WRLCK {
        rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
    }
    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = unixShmSystemLock(pDbFd, ::libc::F_RDLCK, UNIX_SHM_DMS, 1 as ::core::ffi::c_int);
    }
    rc
}

unsafe extern "C" fn unixOpenSharedMemory(mut pDbFd: *mut unixFile) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut p: *mut unixShm = ::core::ptr::null_mut::<unixShm>();
    let mut pShmNode: *mut unixShmNode = ::core::ptr::null_mut::<unixShmNode>();
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut pInode: *mut unixInodeInfo = ::core::ptr::null_mut::<unixInodeInfo>();
    let mut zShm: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nShmFilename: ::core::ffi::c_int = 0;
    p = crate::src::src::malloc::sqlite3_malloc64(::core::mem::size_of::<unixShm>() as crate::src::headers::sqlite3_h::sqlite3_uint64) as *mut unixShm;
    if p.is_null() {
        return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
    }
    ::libc::memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<unixShm>() as crate::__stddef_size_t_h::size_t,
    );
    unixEnterMutex();
    pInode = (*pDbFd).pInode;
    pShmNode = (*pInode).pShmNode as *mut unixShmNode;
    if pShmNode.is_null() {
        let mut sStat: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
        let mut zBasePath: *const ::core::ffi::c_char = (*pDbFd).zPath;
        if ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<unsafe extern "C" fn(::core::ffi::c_int, *mut crate::src::headers::stdlib::stat) -> ::core::ffi::c_int>,
        >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")((*pDbFd).h, &raw mut sStat)
            != 0
        {
            rc = crate::src::headers::sqlite3_h::SQLITE_IOERR_FSTAT_1;
            current_block = 15838364395331891352;
        } else {
            nShmFilename = 6 as ::core::ffi::c_int + ::libc::strlen(zBasePath) as ::core::ffi::c_int;
            pShmNode = crate::src::src::malloc::sqlite3_malloc64(
                (::core::mem::size_of::<unixShmNode>() as usize).wrapping_add(nShmFilename as usize)
                    as crate::src::headers::sqlite3_h::sqlite3_uint64,
            ) as *mut unixShmNode;
            if pShmNode.is_null() {
                rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
                current_block = 15838364395331891352;
            } else {
                ::libc::memset(
                    pShmNode as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (::core::mem::size_of::<unixShmNode>() as crate::__stddef_size_t_h::size_t)
                        .wrapping_add(nShmFilename as crate::__stddef_size_t_h::size_t),
                );
                let __pShmNode_ref = unsafe { &mut *pShmNode };
                __pShmNode_ref.zFilename = pShmNode.offset(1 as isize)
                    as *mut unixShmNode
                    as *mut ::core::ffi::c_char;
                zShm = __pShmNode_ref.zFilename;
                crate::src::src::printf::sqlite3_snprintf(
                    nShmFilename,
                    zShm,
                    b"%s-shm\0" as *const u8 as *const ::core::ffi::c_char,
                    zBasePath,
                );
                __pShmNode_ref.hShm = -(1 as ::core::ffi::c_int);
                (*(*pDbFd).pInode).pShmNode = pShmNode as *mut unixShmNode;
                __pShmNode_ref.pInode = (*pDbFd).pInode;
                if crate::src::src::global::sqlite3Config.bCoreMutex != 0 {
                    __pShmNode_ref.pShmMutex = crate::src::src::mutex::sqlite3_mutex_alloc(crate::src::headers::sqlite3_h::SQLITE_MUTEX_FAST);
                    if __pShmNode_ref.pShmMutex.is_null() {
                        rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
                        current_block = 15838364395331891352;
                    } else {
                        current_block = 14576567515993809846;
                    }
                } else {
                    current_block = 14576567515993809846;
                }
                match current_block {
                    15838364395331891352 => {}
                    _ => {
                        if (*pInode).bProcessLock as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            if 0 as ::core::ffi::c_int
                                == crate::src::src::main::sqlite3_uri_boolean(
                                    (*pDbFd).zPath as crate::src::headers::sqlite3_h::sqlite3_filename,
                                    b"readonly_shm\0" as *const u8 as *const ::core::ffi::c_char,
                                    0 as ::core::ffi::c_int,
                                )
                            {
                                __pShmNode_ref.hShm = robust_open(
                                    zShm,
                                    ::libc::O_RDWR | ::libc::O_CREAT | ::libc::O_NOFOLLOW,
                                    sStat.st_mode as crate::src::headers::stdlib::mode_t & 0o777 as crate::src::headers::stdlib::mode_t,
                                );
                            }
                            if __pShmNode_ref.hShm < 0 as ::core::ffi::c_int {
                                __pShmNode_ref.hShm = robust_open(
                                    zShm,
                                    ::libc::O_RDONLY | ::libc::O_NOFOLLOW,
                                    sStat.st_mode as crate::src::headers::stdlib::mode_t & 0o777 as crate::src::headers::stdlib::mode_t,
                                );
                                if __pShmNode_ref.hShm < 0 as ::core::ffi::c_int {
                                    rc = unixLogErrorAtLine(
                                        crate::src::src::main::sqlite3CantopenError(5040 as ::core::ffi::c_int),
                                        b"open\0" as *const u8 as *const ::core::ffi::c_char,
                                        zShm,
                                        5040 as ::core::ffi::c_int,
                                    );
                                    current_block = 15838364395331891352;
                                } else {
                                    __pShmNode_ref.isReadonly = 1 as crate::src::ext::rtree::rtree::u8_0;
                                    current_block = 11932355480408055363;
                                }
                            } else {
                                current_block = 11932355480408055363;
                            }
                            match current_block {
                                15838364395331891352 => {}
                                _ => {
                                    robustFchown(
                                        __pShmNode_ref.hShm,
                                        sStat.st_uid as crate::src::headers::stdlib::uid_t,
                                        sStat.st_gid as crate::src::headers::stdlib::gid_t,
                                    );
                                    rc = unixLockSharedMemory(pDbFd, pShmNode as *mut unixShmNode);
                                    if rc != crate::src::headers::sqlite3_h::SQLITE_OK && rc != crate::src::headers::sqlite3_h::SQLITE_READONLY_CANTINIT_1 {
                                        current_block = 15838364395331891352;
                                    } else {
                                        current_block = 14434620278749266018;
                                    }
                                }
                            }
                        } else {
                            current_block = 14434620278749266018;
                        }
                    }
                }
            }
        }
        match current_block {
            14434620278749266018 => {}
            _ => {
                unixShmPurge(pDbFd);
                crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
                unixLeaveMutex();
                return rc;
            }
        }
    }
    (*p).pShmNode = pShmNode as *mut unixShmNode;
    (*pShmNode).nRef += 1;
    (*pDbFd).pShm = p as *mut unixShm;
    unixLeaveMutex();
    crate::src::src::mutex::sqlite3_mutex_enter((*pShmNode).pShmMutex);
    (*p).pNext = (*pShmNode).pFirst;
    (*pShmNode).pFirst = p as *mut unixShm;
    crate::src::src::mutex::sqlite3_mutex_leave((*pShmNode).pShmMutex);
    rc
}

unsafe extern "C" fn unixShmMap(
    mut fd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut iRegion: ::core::ffi::c_int,
    mut szRegion: ::core::ffi::c_int,
    mut bExtend: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pDbFd: *mut unixFile = fd as *mut unixFile;
    let mut p: *mut unixShm = ::core::ptr::null_mut::<unixShm>();
    let mut pShmNode: *mut unixShmNode = ::core::ptr::null_mut::<unixShmNode>();
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut nShmPerMap: ::core::ffi::c_int = unixShmRegionPerMap();
    let mut nReqRegion: ::core::ffi::c_int = 0;
    if (*pDbFd).pShm.is_null() {
        rc = unixOpenSharedMemory(pDbFd);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
    }
    p = (*pDbFd).pShm;
    pShmNode = (*p).pShmNode;
    crate::src::src::mutex::sqlite3_mutex_enter((*pShmNode).pShmMutex);
    if (*pShmNode).isUnlocked != 0 {
        rc = unixLockSharedMemory(pDbFd, pShmNode);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            current_block = 3940646464161370556;
        } else {
            (*pShmNode).isUnlocked = 0 as crate::src::ext::rtree::rtree::u8_0;
            current_block = 4166486009154926805;
        }
    } else {
        current_block = 4166486009154926805;
    }
    match current_block {
        4166486009154926805 => {
            nReqRegion = (iRegion + nShmPerMap) / nShmPerMap * nShmPerMap;
            if ((*pShmNode).nRegion as ::core::ffi::c_int) < nReqRegion {
                let mut apNew: *mut *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
                let mut nByte: ::core::ffi::c_int = nReqRegion * szRegion;
                let mut sStat: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
                (*pShmNode).szRegion = szRegion;
                if (*pShmNode).hShm >= 0 as ::core::ffi::c_int {
                    if ::core::mem::transmute::<
                        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                        Option<
                            unsafe extern "C" fn(
                                ::core::ffi::c_int,
                                *mut crate::src::headers::stdlib::stat,
                            ) -> ::core::ffi::c_int,
                        >,
                    >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
                    .expect("non-null function pointer")(
                        (*pShmNode).hShm, &raw mut sStat
                    ) != 0
                    {
                        rc = crate::src::headers::sqlite3_h::SQLITE_IOERR_SHMSIZE_1;
                        current_block = 3940646464161370556;
                    } else if sStat.st_size < nByte as crate::src::headers::stdlib::__off_t {
                        if bExtend == 0 {
                            current_block = 3940646464161370556;
                        } else {
                            static mut pgsz: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
                            let mut iPg: ::core::ffi::c_int = 0;
                            iPg = (sStat.st_size / pgsz as crate::src::headers::stdlib::__off_t) as ::core::ffi::c_int;
                            loop {
                                if !(iPg < nByte / pgsz) {
                                    current_block = 8693738493027456495;
                                    break;
                                }
                                let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                if seekAndWriteFd(
                                    (*pShmNode).hShm,
                                    (iPg * pgsz + pgsz - 1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0,
                                    b"\0" as *const u8 as *const ::core::ffi::c_char
                                        as *const ::core::ffi::c_void,
                                    1 as ::core::ffi::c_int,
                                    &raw mut x,
                                ) != 1 as ::core::ffi::c_int
                                {
                                    let mut zFile: *const ::core::ffi::c_char =
                                        (*pShmNode).zFilename;
                                    rc = unixLogErrorAtLine(
                                        10 as ::core::ffi::c_int
                                            | (19 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                                        b"write\0" as *const u8 as *const ::core::ffi::c_char,
                                        zFile,
                                        5184 as ::core::ffi::c_int,
                                    );
                                    current_block = 3940646464161370556;
                                    break;
                                } else {
                                    iPg += 1;
                                }
                            }
                        }
                    } else {
                        current_block = 8693738493027456495;
                    }
                } else {
                    current_block = 8693738493027456495;
                }
                match current_block {
                    3940646464161370556 => {}
                    _ => {
                        apNew = crate::src::src::malloc::sqlite3_realloc(
                            (*pShmNode).apRegion as *mut ::core::ffi::c_void,
                            (nReqRegion as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize
                                ) as ::core::ffi::c_int,
                        ) as *mut *mut ::core::ffi::c_char;
                        if apNew.is_null() {
                            rc = crate::src::headers::sqliteInt_h::SQLITE_IOERR_NOMEM_BKPT;
                        } else {
                            (*pShmNode).apRegion = apNew;
                            while ((*pShmNode).nRegion as ::core::ffi::c_int) < nReqRegion {
                                let mut nMap: ::core::ffi::c_int = szRegion * nShmPerMap;
                                let mut i: ::core::ffi::c_int = 0;
                                let mut pMem: *mut ::core::ffi::c_void =
                                    ::core::ptr::null_mut::<::core::ffi::c_void>();
                                let __pShmNode_ref = unsafe { &mut *pShmNode };
                                if __pShmNode_ref.hShm >= 0 as ::core::ffi::c_int {
                                    pMem = ::core::mem::transmute::<
                                        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                                        Option<
                                            unsafe extern "C" fn(
                                                *mut ::core::ffi::c_void,
                                                crate::__stddef_size_t_h::size_t,
                                                ::core::ffi::c_int,
                                                ::core::ffi::c_int,
                                                ::core::ffi::c_int,
                                                crate::src::headers::stdlib::off_t,
                                            )
                                                -> *mut ::core::ffi::c_void,
                                        >,
                                    >(
                                        aSyscall[22 as ::core::ffi::c_int as usize].pCurrent
                                    )
                                    .expect("non-null function pointer")(
                                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                        nMap as crate::__stddef_size_t_h::size_t,
                                        if __pShmNode_ref.isReadonly as ::core::ffi::c_int != 0 {
                                            ::libc::PROT_READ
                                        } else {
                                            ::libc::PROT_READ | ::libc::PROT_WRITE
                                        },
                                        ::libc::MAP_SHARED,
                                        __pShmNode_ref.hShm,
                                        (szRegion as crate::src::ext::rtree::rtree::i64_0 * __pShmNode_ref.nRegion as crate::src::ext::rtree::rtree::i64_0) as crate::src::headers::stdlib::off_t,
                                    );
                                    if pMem == ::libc::MAP_FAILED {
                                        rc = unixLogErrorAtLine(
                                            10 as ::core::ffi::c_int
                                                | (21 as ::core::ffi::c_int)
                                                    << 8 as ::core::ffi::c_int,
                                            b"mmap\0" as *const u8 as *const ::core::ffi::c_char,
                                            __pShmNode_ref.zFilename,
                                            5211 as ::core::ffi::c_int,
                                        );
                                        break;
                                    }
                                } else {
                                    pMem = crate::src::src::malloc::sqlite3_malloc64(nMap as crate::src::headers::sqlite3_h::sqlite3_uint64);
                                    if pMem.is_null() {
                                        rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
                                        break;
                                    } else {
                                        ::libc::memset(pMem, 0 as ::core::ffi::c_int, nMap as crate::__stddef_size_t_h::size_t);
                                    }
                                }
                                i = 0 as ::core::ffi::c_int;
                                while i < nShmPerMap {
                                    let ref mut fresh18 = *__pShmNode_ref.apRegion.offset(
                                        (__pShmNode_ref.nRegion as ::core::ffi::c_int + i) as isize,
                                    );
                                    *fresh18 = (pMem as *mut ::core::ffi::c_char)
                                        .offset((szRegion * i) as isize)
                                        as *mut ::core::ffi::c_char;
                                    i += 1;
                                }
                                __pShmNode_ref.nRegion = (__pShmNode_ref.nRegion as ::core::ffi::c_int
                                    + nShmPerMap)
                                    as crate::src::fts5::u16_0;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if (*pShmNode).nRegion as ::core::ffi::c_int > iRegion {
        *pp = *(*pShmNode).apRegion.offset(iRegion as isize) as *mut ::core::ffi::c_void;
    } else {
        *pp = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*pShmNode).isReadonly as ::core::ffi::c_int != 0 && rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        rc = crate::src::headers::sqlite3_h::SQLITE_READONLY;
    }
    crate::src::src::mutex::sqlite3_mutex_leave((*pShmNode).pShmMutex);
    rc
}

unsafe extern "C" fn unixShmLock(
    mut fd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut ofst: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pDbFd: *mut unixFile = fd as *mut unixFile;
    let mut p: *mut unixShm = ::core::ptr::null_mut::<unixShm>();
    let mut pShmNode: *mut unixShmNode = ::core::ptr::null_mut::<unixShmNode>();
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut mask: crate::src::fts5::u16_0 =
        (((1 as ::core::ffi::c_int) << ofst + n) - ((1 as ::core::ffi::c_int) << ofst)) as crate::src::fts5::u16_0;
    let mut aLock: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    p = (*pDbFd).pShm;
    if p.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_IOERR_SHMLOCK_1;
    }
    pShmNode = (*p).pShmNode;
    if pShmNode.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_IOERR_SHMLOCK_1;
    }
    aLock = &raw mut (*pShmNode).aLock as *mut ::core::ffi::c_int;
    if flags & crate::src::headers::sqlite3_h::SQLITE_SHM_UNLOCK != 0
        && ((*p).exclMask as ::core::ffi::c_int | (*p).sharedMask as ::core::ffi::c_int)
            & mask as ::core::ffi::c_int
            != 0
        || flags == crate::src::headers::sqlite3_h::SQLITE_SHM_SHARED | crate::src::headers::sqlite3_h::SQLITE_SHM_LOCK
            && 0 as ::core::ffi::c_int
                == (*p).sharedMask as ::core::ffi::c_int & mask as ::core::ffi::c_int
        || flags == crate::src::headers::sqlite3_h::SQLITE_SHM_EXCLUSIVE | crate::src::headers::sqlite3_h::SQLITE_SHM_LOCK
    {
        crate::src::src::mutex::sqlite3_mutex_enter((*pShmNode).pShmMutex);
        if rc == 0 as ::core::ffi::c_int {
            if flags & crate::src::headers::sqlite3_h::SQLITE_SHM_UNLOCK != 0 {
                let mut bUnlock: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                if flags & crate::src::headers::sqlite3_h::SQLITE_SHM_SHARED != 0 {
                    if *aLock.offset(ofst as isize) > 1 as ::core::ffi::c_int {
                        bUnlock = 0 as ::core::ffi::c_int;
                        let ref mut fresh5 = *aLock.offset(ofst as isize);
                        *fresh5 -= 1;
                        (*p).sharedMask = ((*p).sharedMask as ::core::ffi::c_int
                            & !(mask as ::core::ffi::c_int))
                            as crate::src::fts5::u16_0;
                    }
                }
                if bUnlock != 0 {
                    rc = unixShmSystemLock(pDbFd, ::libc::F_UNLCK, ofst + UNIX_SHM_BASE, n);
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        ::libc::memset(
                            aLock.offset(ofst as isize) as *mut ::core::ffi::c_int
                                as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            (::core::mem::size_of::<::core::ffi::c_int>() as crate::__stddef_size_t_h::size_t)
                                .wrapping_mul(n as crate::__stddef_size_t_h::size_t),
                        );
                        let __p_ref = unsafe { &mut *p };
                        __p_ref.sharedMask = (__p_ref.sharedMask as ::core::ffi::c_int
                            & !(mask as ::core::ffi::c_int))
                            as crate::src::fts5::u16_0;
                        __p_ref.exclMask = (__p_ref.exclMask as ::core::ffi::c_int
                            & !(mask as ::core::ffi::c_int))
                            as crate::src::fts5::u16_0;
                    }
                }
            } else if flags & crate::src::headers::sqlite3_h::SQLITE_SHM_SHARED != 0 {
                if *aLock.offset(ofst as isize) < 0 as ::core::ffi::c_int {
                    rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
                } else if *aLock.offset(ofst as isize) == 0 as ::core::ffi::c_int {
                    rc = unixShmSystemLock(pDbFd, ::libc::F_RDLCK, ofst + UNIX_SHM_BASE, n);
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    (*p).sharedMask = ((*p).sharedMask as ::core::ffi::c_int
                        | mask as ::core::ffi::c_int)
                        as crate::src::fts5::u16_0;
                    let ref mut fresh6 = *aLock.offset(ofst as isize);
                    *fresh6 += 1;
                }
            } else {
                let mut ii: ::core::ffi::c_int = 0;
                ii = ofst;
                while ii < ofst + n {
                    if *aLock.offset(ii as isize) != 0 {
                        rc = crate::src::headers::sqlite3_h::SQLITE_BUSY;
                        break;
                    } else {
                        ii += 1;
                    }
                }
                if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                    rc = unixShmSystemLock(pDbFd, ::libc::F_WRLCK, ofst + UNIX_SHM_BASE, n);
                    if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                        (*p).exclMask = ((*p).exclMask as ::core::ffi::c_int
                            | mask as ::core::ffi::c_int)
                            as crate::src::fts5::u16_0;
                        ii = ofst;
                        while ii < ofst + n {
                            *aLock.offset(ii as isize) = -(1 as ::core::ffi::c_int);
                            ii += 1;
                        }
                    }
                }
            }
        }
        crate::src::src::mutex::sqlite3_mutex_leave((*pShmNode).pShmMutex);
    }
    if crate::src::src::main::sqlite3OSTrace != 0 {
        let __p_ref = unsafe { &*p };
        crate::src::src::printf::sqlite3DebugPrintf(
            b"SHM-LOCK shmid-%d, pid-%d got %03x,%03x\n\0" as *const u8
                as *const ::core::ffi::c_char,
            __p_ref.id as ::core::ffi::c_int,
            ::libc::getpid(),
            __p_ref.sharedMask as ::core::ffi::c_int,
            __p_ref.exclMask as ::core::ffi::c_int,
        );
    }
    rc
}

unsafe extern "C" fn unixShmBarrier(mut _fd: *mut crate::src::headers::sqlite3_h::sqlite3_file) {
    crate::src::src::mutex_unix::sqlite3MemoryBarrier();
    unixEnterMutex();
    unixLeaveMutex();
}

unsafe extern "C" fn unixShmUnmap(
    mut fd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut deleteFlag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut unixShm = ::core::ptr::null_mut::<unixShm>();
    let mut pShmNode: *mut unixShmNode = ::core::ptr::null_mut::<unixShmNode>();
    let mut pp: *mut *mut unixShm = ::core::ptr::null_mut::<*mut unixShm>();
    let mut pDbFd: *mut unixFile = ::core::ptr::null_mut::<unixFile>();
    pDbFd = fd as *mut unixFile;
    p = (*pDbFd).pShm;
    if p.is_null() {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    pShmNode = (*p).pShmNode;
    crate::src::src::mutex::sqlite3_mutex_enter((*pShmNode).pShmMutex);
    pp = &raw mut (*pShmNode).pFirst;
    while *pp != p {
        pp = &raw mut (**pp).pNext;
    }
    *pp = (*p).pNext;
    crate::src::src::malloc::sqlite3_free(p as *mut ::core::ffi::c_void);
    (*pDbFd).pShm = ::core::ptr::null_mut::<unixShm>();
    crate::src::src::mutex::sqlite3_mutex_leave((*pShmNode).pShmMutex);
    unixEnterMutex();
    (*pShmNode).nRef -= 1;
    if (*pShmNode).nRef == 0 as ::core::ffi::c_int {
        if deleteFlag != 0 && (*pShmNode).hShm >= 0 as ::core::ffi::c_int {
            ::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
            >(aSyscall[16 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")((*pShmNode).zFilename);
        }
        unixShmPurge(pDbFd);
    }
    unixLeaveMutex();
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn unixUnmapfile(mut pFd: *mut unixFile) {
    if !(*pFd).pMapRegion.is_null() {
        let __pFd_ref = unsafe { &mut *pFd };
        ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ::core::ffi::c_int>,
        >(aSyscall[23 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(
            __pFd_ref.pMapRegion, __pFd_ref.mmapSizeActual as crate::__stddef_size_t_h::size_t
        );
        __pFd_ref.pMapRegion = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __pFd_ref.mmapSize = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
        __pFd_ref.mmapSizeActual = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
    }
}

unsafe extern "C" fn unixRemapfile(mut pFd: *mut unixFile, mut nNew: crate::src::ext::rtree::rtree::i64_0) {
    let mut zErr: *const ::core::ffi::c_char = b"mmap\0" as *const u8 as *const ::core::ffi::c_char;
    let __pFd_ref = unsafe { &mut *pFd };
    let mut h: ::core::ffi::c_int = __pFd_ref.h;
    let mut pOrig: *mut crate::src::ext::rtree::rtree::u8_0 = __pFd_ref.pMapRegion as *mut crate::src::ext::rtree::rtree::u8_0;
    let mut nOrig: crate::src::ext::rtree::rtree::i64_0 = __pFd_ref.mmapSizeActual as crate::src::ext::rtree::rtree::i64_0;
    let mut pNew: *mut crate::src::ext::rtree::rtree::u8_0 = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u8_0>();
    let mut flags: ::core::ffi::c_int = ::libc::PROT_READ;
    if !pOrig.is_null() {
        let mut nReuse: crate::src::ext::rtree::rtree::i64_0 = __pFd_ref.mmapSize as crate::src::ext::rtree::rtree::i64_0;
        let mut pReq: *mut crate::src::ext::rtree::rtree::u8_0 = pOrig.offset(nReuse as isize) as *mut crate::src::ext::rtree::rtree::u8_0;
        if nReuse != nOrig {
            ::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ::core::ffi::c_int,
                >,
            >(aSyscall[23 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(
                pReq as *mut ::core::ffi::c_void,
                (nOrig - nReuse) as crate::__stddef_size_t_h::size_t,
            );
        }
        pNew = ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    crate::__stddef_size_t_h::size_t,
                    crate::__stddef_size_t_h::size_t,
                    ::core::ffi::c_int,
                    ...
                ) -> *mut ::core::ffi::c_void,
            >,
        >(aSyscall[24 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(
            pOrig as *mut ::core::ffi::c_void,
            nReuse as crate::__stddef_size_t_h::size_t,
            nNew as crate::__stddef_size_t_h::size_t,
            ::libc::MREMAP_MAYMOVE,
        ) as *mut crate::src::ext::rtree::rtree::u8_0;
        zErr = b"mremap\0" as *const u8 as *const ::core::ffi::c_char;
        if pNew == ::libc::MAP_FAILED as *mut crate::src::ext::rtree::rtree::u8_0 || pNew.is_null() {
            ::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::__stddef_size_t_h::size_t) -> ::core::ffi::c_int,
                >,
            >(aSyscall[23 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(
                pOrig as *mut ::core::ffi::c_void,
                nReuse as crate::__stddef_size_t_h::size_t,
            );
        }
    }
    if pNew.is_null() {
        pNew = ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    crate::__stddef_size_t_h::size_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    crate::src::headers::stdlib::off_t,
                ) -> *mut ::core::ffi::c_void,
            >,
        >(aSyscall[22 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            nNew as crate::__stddef_size_t_h::size_t,
            flags,
            ::libc::MAP_SHARED,
            h,
            0 as crate::src::headers::stdlib::off_t,
        ) as *mut crate::src::ext::rtree::rtree::u8_0;
    }
    if pNew == ::libc::MAP_FAILED as *mut crate::src::ext::rtree::rtree::u8_0 {
        pNew = ::core::ptr::null_mut::<crate::src::ext::rtree::rtree::u8_0>();
        nNew = 0 as crate::src::ext::rtree::rtree::i64_0;
        unixLogErrorAtLine(
            0 as ::core::ffi::c_int,
            zErr,
            __pFd_ref.zPath,
            5650 as ::core::ffi::c_int,
        );
        __pFd_ref.mmapSizeMax = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
    }
    __pFd_ref.pMapRegion = pNew as *mut ::core::ffi::c_void;
    __pFd_ref.mmapSizeActual = nNew as crate::src::headers::sqlite3_h::sqlite3_int64;
    __pFd_ref.mmapSize = __pFd_ref.mmapSizeActual;
}

unsafe extern "C" fn unixMapfile(mut pFd: *mut unixFile, mut nMap: crate::src::ext::rtree::rtree::i64_0) -> ::core::ffi::c_int {
    let __pFd_ref = unsafe { &*pFd };
    if __pFd_ref.nFetchOut > 0 as ::core::ffi::c_int {
        return crate::src::headers::sqlite3_h::SQLITE_OK;
    }
    if nMap < 0 as crate::src::ext::rtree::rtree::i64_0 {
        let mut statbuf: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
        if ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<unsafe extern "C" fn(::core::ffi::c_int, *mut crate::src::headers::stdlib::stat) -> ::core::ffi::c_int>,
        >(aSyscall[5 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(__pFd_ref.h, &raw mut statbuf)
            != 0
        {
            return crate::src::headers::sqlite3_h::SQLITE_IOERR_FSTAT_1;
        }
        nMap = statbuf.st_size as crate::src::ext::rtree::rtree::i64_0;
    }
    if nMap > __pFd_ref.mmapSizeMax {
        nMap = __pFd_ref.mmapSizeMax as crate::src::ext::rtree::rtree::i64_0;
    }
    if nMap != __pFd_ref.mmapSize {
        unixRemapfile(pFd, nMap);
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn unixFetch(
    mut fd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut iOff: crate::src::ext::rtree::rtree::i64_0,
    mut nAmt: ::core::ffi::c_int,
    mut pp: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pFd: *mut unixFile = fd as *mut unixFile;
    *pp = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if (*pFd).mmapSizeMax > 0 as crate::src::headers::sqlite3_h::sqlite3_int64 {
        let nEofBuffer: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
        if (*pFd).pMapRegion.is_null() {
            let mut rc: ::core::ffi::c_int = unixMapfile(pFd, -(1 as ::core::ffi::c_int) as crate::src::ext::rtree::rtree::i64_0);
            if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
                return rc;
            }
        }
        if (*pFd).mmapSize >= iOff + nAmt as crate::src::ext::rtree::rtree::i64_0 + nEofBuffer as crate::src::ext::rtree::rtree::i64_0 {
            *pp = ((*pFd).pMapRegion as *mut crate::src::ext::rtree::rtree::u8_0).offset(iOff as isize) as *mut crate::src::ext::rtree::rtree::u8_0
                as *mut ::core::ffi::c_void;
            (*pFd).nFetchOut += 1;
        }
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn unixUnfetch(
    mut fd: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut _iOff: crate::src::ext::rtree::rtree::i64_0,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pFd: *mut unixFile = fd as *mut unixFile;
    if !p.is_null() {
        (*pFd).nFetchOut -= 1;
    } else {
        unixUnmapfile(pFd);
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

static mut posixIoMethods: crate::src::headers::sqlite3_h::sqlite3_io_methods = unsafe {
    crate::src::headers::sqlite3_h::sqlite3_io_methods {
    iVersion:  3 as ::core::ffi::c_int,
    xClose:  Some(unixClose as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int),
    xRead:  Some(
            unixRead
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    crate::src::headers::sqlite3_h::sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
    xWrite:  Some(
            unixWrite
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    crate::src::headers::sqlite3_h::sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
    xTruncate:  Some(
            unixTruncate as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file, crate::src::ext::rtree::rtree::i64_0) -> ::core::ffi::c_int,
        ),
    xSync:  Some(
            unixSync
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xFileSize:  Some(
            unixFileSize
                as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file, *mut crate::src::ext::rtree::rtree::i64_0) -> ::core::ffi::c_int,
        ),
    xLock:  Some(
            unixLock
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xUnlock:  Some(
            unixUnlock
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xCheckReservedLock:  Some(
            unixCheckReservedLock
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xFileControl:  Some(
            unixFileControl
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    xSectorSize:  Some(
            unixSectorSize as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int,
        ),
    xDeviceCharacteristics:  Some(
            unixDeviceCharacteristics
                as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int,
        ),
    xShmMap:  Some(
            unixShmMap
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    xShmLock:  Some(
            unixShmLock
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xShmBarrier:  Some(unixShmBarrier as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ()),
    xShmUnmap:  Some(
            unixShmUnmap
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xFetch:  Some(
            unixFetch
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    crate::src::ext::rtree::rtree::i64_0,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    xUnfetch:  Some(
            unixUnfetch
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    crate::src::ext::rtree::rtree::i64_0,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
}
};

static mut posixIoFinder: Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, *mut unixFile) -> *const crate::src::headers::sqlite3_h::sqlite3_io_methods,
> = unsafe {
    Some(
        posixIoFinderImpl
            as unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *mut unixFile,
            ) -> *const crate::src::headers::sqlite3_h::sqlite3_io_methods,
    )
};

unsafe extern "C" fn posixIoFinderImpl(
    mut _z: *const ::core::ffi::c_char,
    mut _p: *mut unixFile,
) -> *const crate::src::headers::sqlite3_h::sqlite3_io_methods {
    &raw const posixIoMethods
}

static mut nolockIoFinder: Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, *mut unixFile) -> *const crate::src::headers::sqlite3_h::sqlite3_io_methods,
> = unsafe {
    Some(
        nolockIoFinderImpl
            as unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *mut unixFile,
            ) -> *const crate::src::headers::sqlite3_h::sqlite3_io_methods,
    )
};

static mut nolockIoMethods: crate::src::headers::sqlite3_h::sqlite3_io_methods = unsafe {
    crate::src::headers::sqlite3_h::sqlite3_io_methods {
    iVersion:  3 as ::core::ffi::c_int,
    xClose:  Some(nolockClose as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int),
    xRead:  Some(
            unixRead
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    crate::src::headers::sqlite3_h::sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
    xWrite:  Some(
            unixWrite
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    crate::src::headers::sqlite3_h::sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
    xTruncate:  Some(
            unixTruncate as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file, crate::src::ext::rtree::rtree::i64_0) -> ::core::ffi::c_int,
        ),
    xSync:  Some(
            unixSync
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xFileSize:  Some(
            unixFileSize
                as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file, *mut crate::src::ext::rtree::rtree::i64_0) -> ::core::ffi::c_int,
        ),
    xLock:  Some(
            nolockLock
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xUnlock:  Some(
            nolockUnlock
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xCheckReservedLock:  Some(
            nolockCheckReservedLock
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xFileControl:  Some(
            unixFileControl
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    xSectorSize:  Some(
            unixSectorSize as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int,
        ),
    xDeviceCharacteristics:  Some(
            unixDeviceCharacteristics
                as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int,
        ),
    xShmMap:  None,
    xShmLock:  Some(
            unixShmLock
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xShmBarrier:  Some(unixShmBarrier as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ()),
    xShmUnmap:  Some(
            unixShmUnmap
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xFetch:  Some(
            unixFetch
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    crate::src::ext::rtree::rtree::i64_0,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    xUnfetch:  Some(
            unixUnfetch
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    crate::src::ext::rtree::rtree::i64_0,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
}
};

unsafe extern "C" fn nolockIoFinderImpl(
    mut _z: *const ::core::ffi::c_char,
    mut _p: *mut unixFile,
) -> *const crate::src::headers::sqlite3_h::sqlite3_io_methods {
    &raw const nolockIoMethods
}

static mut dotlockIoMethods: crate::src::headers::sqlite3_h::sqlite3_io_methods = unsafe {
    crate::src::headers::sqlite3_h::sqlite3_io_methods {
    iVersion:  1 as ::core::ffi::c_int,
    xClose:  Some(dotlockClose as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int),
    xRead:  Some(
            unixRead
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    crate::src::headers::sqlite3_h::sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
    xWrite:  Some(
            unixWrite
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    *const ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    crate::src::headers::sqlite3_h::sqlite3_int64,
                ) -> ::core::ffi::c_int,
        ),
    xTruncate:  Some(
            unixTruncate as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file, crate::src::ext::rtree::rtree::i64_0) -> ::core::ffi::c_int,
        ),
    xSync:  Some(
            unixSync
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xFileSize:  Some(
            unixFileSize
                as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file, *mut crate::src::ext::rtree::rtree::i64_0) -> ::core::ffi::c_int,
        ),
    xLock:  Some(
            dotlockLock
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xUnlock:  Some(
            dotlockUnlock
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xCheckReservedLock:  Some(
            dotlockCheckReservedLock
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xFileControl:  Some(
            unixFileControl
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    xSectorSize:  Some(
            unixSectorSize as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int,
        ),
    xDeviceCharacteristics:  Some(
            unixDeviceCharacteristics
                as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ::core::ffi::c_int,
        ),
    xShmMap:  None,
    xShmLock:  Some(
            unixShmLock
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xShmBarrier:  Some(unixShmBarrier as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_file) -> ()),
    xShmUnmap:  Some(
            unixShmUnmap
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        ),
    xFetch:  Some(
            unixFetch
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    crate::src::ext::rtree::rtree::i64_0,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    xUnfetch:  Some(
            unixUnfetch
                as unsafe extern "C" fn(
                    *mut crate::src::headers::sqlite3_h::sqlite3_file,
                    crate::src::ext::rtree::rtree::i64_0,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
}
};

static mut dotlockIoFinder: Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, *mut unixFile) -> *const crate::src::headers::sqlite3_h::sqlite3_io_methods,
> = unsafe {
    Some(
        dotlockIoFinderImpl
            as unsafe extern "C" fn(
                *const ::core::ffi::c_char,
                *mut unixFile,
            ) -> *const crate::src::headers::sqlite3_h::sqlite3_io_methods,
    )
};

unsafe extern "C" fn dotlockIoFinderImpl(
    mut _z: *const ::core::ffi::c_char,
    mut _p: *mut unixFile,
) -> *const crate::src::headers::sqlite3_h::sqlite3_io_methods {
    &raw const dotlockIoMethods
}

unsafe extern "C" fn fillInUnixFile(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut h: ::core::ffi::c_int,
    mut pId: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut zFilename: *const ::core::ffi::c_char,
    mut ctrlFlags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pLockingStyle: *const crate::src::headers::sqlite3_h::sqlite3_io_methods = ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_io_methods>();
    let mut pNew: *mut unixFile = pId as *mut unixFile;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if crate::src::src::main::sqlite3OSTrace != 0 {
        crate::src::src::printf::sqlite3DebugPrintf(
            b"OPEN    %-3d %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            h,
            zFilename,
        );
    }
    let __pNew_ref = unsafe { &mut *pNew };
    __pNew_ref.h = h;
    __pNew_ref.pVfs = pVfs;
    __pNew_ref.zPath = zFilename;
    __pNew_ref.ctrlFlags = ctrlFlags as crate::src::ext::rtree::rtree::u8_0 as ::core::ffi::c_ushort;
    __pNew_ref.mmapSizeMax = crate::src::src::global::sqlite3Config.szMmap;
    if crate::src::src::main::sqlite3_uri_boolean(
        if ctrlFlags & UNIXFILE_URI != 0 {
            zFilename as crate::src::headers::sqlite3_h::sqlite3_filename
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
        b"psow\0" as *const u8 as *const ::core::ffi::c_char,
        crate::src::headers::sqliteInt_h::SQLITE_POWERSAFE_OVERWRITE,
    ) != 0
    {
        __pNew_ref.ctrlFlags =
            (__pNew_ref.ctrlFlags as ::core::ffi::c_int | UNIXFILE_PSOW) as ::core::ffi::c_ushort;
    }
    if ::libc::strcmp(
        (*pVfs).zName,
        b"unix-excl\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        __pNew_ref.ctrlFlags =
            (__pNew_ref.ctrlFlags as ::core::ffi::c_int | UNIXFILE_EXCL) as ::core::ffi::c_ushort;
    }
    if ctrlFlags & UNIXFILE_NOLOCK != 0 {
        pLockingStyle = &raw const nolockIoMethods;
    } else {
        pLockingStyle =
            Some((*((*pVfs).pAppData as *mut finder_type)).expect("non-null function pointer"))
                .expect("non-null function pointer")(zFilename, pNew);
    }
    if pLockingStyle == &raw const posixIoMethods {
        unixEnterMutex();
        rc = findInodeInfo(pNew, &raw mut __pNew_ref.pInode);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            robust_close(pNew, h, 6158 as ::core::ffi::c_int);
            h = -(1 as ::core::ffi::c_int);
        }
        unixLeaveMutex();
    } else if pLockingStyle == &raw const dotlockIoMethods {
        let mut zLockFile: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut nFilename: ::core::ffi::c_int = 0;
        nFilename = ::libc::strlen(zFilename) as ::core::ffi::c_int + 6 as ::core::ffi::c_int;
        zLockFile = crate::src::src::malloc::sqlite3_malloc64(nFilename as crate::src::headers::sqlite3_h::sqlite3_uint64) as *mut ::core::ffi::c_char;
        if zLockFile.is_null() {
            rc = crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
        } else {
            crate::src::src::printf::sqlite3_snprintf(
                nFilename,
                zLockFile,
                b"%s.lock\0" as *const u8 as *const ::core::ffi::c_char,
                zFilename,
            );
        }
        __pNew_ref.lockingContext = zLockFile as *mut ::core::ffi::c_void;
    }
    storeLastErrno(pNew, 0 as ::core::ffi::c_int);
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        if h >= 0 as ::core::ffi::c_int {
            robust_close(pNew, h, 6250 as ::core::ffi::c_int);
        }
    } else {
        (*pId).pMethods = pLockingStyle as *const crate::src::headers::sqlite3_h::sqlite3_io_methods;
        crate::src::src::os::sqlite3_open_file_count += 1 as ::core::ffi::c_int;
        verifyDbFile(pNew);
    }
    rc
}

static mut azTempDirs: [*const ::core::ffi::c_char; 6] = [
    ::core::ptr::null::<::core::ffi::c_char>(),
    ::core::ptr::null::<::core::ffi::c_char>(),
    b"/var/tmp\0" as *const u8 as *const ::core::ffi::c_char,
    b"/usr/tmp\0" as *const u8 as *const ::core::ffi::c_char,
    b"/tmp\0" as *const u8 as *const ::core::ffi::c_char,
    b".\0" as *const u8 as *const ::core::ffi::c_char,
];

unsafe extern "C" fn unixTempFileInit() {
    azTempDirs[0 as ::core::ffi::c_int as usize] =
        ::libc::getenv(b"SQLITE_TMPDIR\0" as *const u8 as *const ::core::ffi::c_char);
    azTempDirs[1 as ::core::ffi::c_int as usize] =
        ::libc::getenv(b"TMPDIR\0" as *const u8 as *const ::core::ffi::c_char);
}

unsafe extern "C" fn unixTempFileDir() -> *const ::core::ffi::c_char {
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut buf: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
    let mut zDir: *const ::core::ffi::c_char = crate::src::src::main::sqlite3_temp_directory;
    loop {
        if !zDir.is_null()
            && ::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut crate::src::headers::stdlib::stat,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[4 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(zDir, &raw mut buf)
                == 0 as ::core::ffi::c_int
            && buf.st_mode & crate::src::headers::stdlib::__S_IFMT as crate::src::headers::stdlib::__mode_t == 0o40000 as crate::src::headers::stdlib::__mode_t
            && ::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[2 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(zDir, 0o3 as ::core::ffi::c_int)
                == 0 as ::core::ffi::c_int
        {
            return zDir;
        }
        if i as usize
            >= (::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>() as usize)
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
        {
            break;
        }
        let fresh11 = i;
        i = i.wrapping_add(1);
        zDir = azTempDirs[fresh11 as usize];
    }
    ::core::ptr::null::<::core::ffi::c_char>()
}

unsafe extern "C" fn unixGetTempname(
    mut nBuf: ::core::ffi::c_int,
    mut zBuf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut zDir: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut iLimit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    *zBuf.offset(0 as isize) = 0 as ::core::ffi::c_char;
    if crate::src::src::os::sqlite3_io_error_persist != 0 && crate::src::src::os::sqlite3_io_error_hit != 0 || {
        let fresh9 = crate::src::src::os::sqlite3_io_error_pending;
        crate::src::src::os::sqlite3_io_error_pending -= 1;
        fresh9 == 1 as ::core::ffi::c_int
    } {
        local_ioerr();
        return 10 as ::core::ffi::c_int;
    }
    crate::src::src::mutex::sqlite3_mutex_enter(crate::src::src::mutex::sqlite3MutexAlloc(crate::src::headers::sqliteInt_h::SQLITE_MUTEX_STATIC_TEMPDIR));
    zDir = unixTempFileDir();
    if zDir.is_null() {
        rc = crate::src::headers::sqlite3_h::SQLITE_IOERR_GETTEMPPATH_1;
    } else {
        loop {
            let mut r: crate::src::ext::rtree::rtree::u64_0 = 0;
            crate::src::src::random::sqlite3_randomness(
                ::core::mem::size_of::<crate::src::ext::rtree::rtree::u64_0>() as ::core::ffi::c_int,
                &raw mut r as *mut ::core::ffi::c_void,
            );
            *zBuf.offset((nBuf - 2 as ::core::ffi::c_int) as isize) = 0 as ::core::ffi::c_char;
            crate::src::src::printf::sqlite3_snprintf(
                nBuf,
                zBuf,
                b"%s/etilqs_%llx%c\0" as *const u8 as *const ::core::ffi::c_char,
                zDir,
                r,
                0 as ::core::ffi::c_int,
            );
            if *zBuf.offset((nBuf - 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
                || {
                    let fresh10 = iLimit;
                    iLimit += 1;
                    fresh10 > 10 as ::core::ffi::c_int
                }
            {
                rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
                break;
            } else if !(::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[2 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(
                zBuf, 0 as ::core::ffi::c_int
            ) == 0 as ::core::ffi::c_int)
            {
                break;
            }
        }
    }
    crate::src::src::mutex::sqlite3_mutex_leave(crate::src::src::mutex::sqlite3MutexAlloc(crate::src::headers::sqliteInt_h::SQLITE_MUTEX_STATIC_TEMPDIR));
    rc
}

unsafe extern "C" fn findReusableFd(
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> *mut UnixUnusedFd {
    let mut pUnused: *mut UnixUnusedFd = ::core::ptr::null_mut::<UnixUnusedFd>();
    let mut sStat: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
    unixEnterMutex();
    if !inodeList.is_null()
        && 0 as ::core::ffi::c_int
            == ::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut crate::src::headers::stdlib::stat,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[4 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(zPath, &raw mut sStat)
    {
        let mut pInode: *mut unixInodeInfo = ::core::ptr::null_mut::<unixInodeInfo>();
        pInode = inodeList;
        while !pInode.is_null()
            && ((*pInode).fileId.dev != sStat.st_dev
                || (*pInode).fileId.ino != sStat.st_ino as crate::src::ext::rtree::rtree::u64_0)
        {
            pInode = (*pInode).pNext;
        }
        if !pInode.is_null() {
            let mut pp: *mut *mut UnixUnusedFd = ::core::ptr::null_mut::<*mut UnixUnusedFd>();
            let __pInode_ref = unsafe { &mut *pInode };
            crate::src::src::mutex::sqlite3_mutex_enter(__pInode_ref.pLockMutex);
            flags &= crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY | crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE;
            pp = &raw mut __pInode_ref.pUnused;
            while !(*pp).is_null() && (**pp).flags != flags {
                pp = &raw mut (**pp).pNext;
            }
            pUnused = *pp;
            if !pUnused.is_null() {
                *pp = (*pUnused).pNext;
            }
            crate::src::src::mutex::sqlite3_mutex_leave(__pInode_ref.pLockMutex);
        }
    }
    unixLeaveMutex();
    pUnused
}

unsafe extern "C" fn getFileMode(
    mut zFile: *const ::core::ffi::c_char,
    mut pMode: *mut crate::src::headers::stdlib::mode_t,
    mut pUid: *mut crate::src::headers::stdlib::uid_t,
    mut pGid: *mut crate::src::headers::stdlib::gid_t,
) -> ::core::ffi::c_int {
    let mut sStat: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if 0 as ::core::ffi::c_int
        == ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(*const ::core::ffi::c_char, *mut crate::src::headers::stdlib::stat) -> ::core::ffi::c_int,
            >,
        >(aSyscall[4 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(zFile, &raw mut sStat)
    {
        *pMode = (sStat.st_mode & 0o777 as crate::src::headers::stdlib::__mode_t) as crate::src::headers::stdlib::mode_t;
        *pUid = sStat.st_uid as crate::src::headers::stdlib::uid_t;
        *pGid = sStat.st_gid as crate::src::headers::stdlib::gid_t;
    } else {
        rc = crate::src::headers::sqlite3_h::SQLITE_IOERR_FSTAT_1;
    }
    rc
}

unsafe extern "C" fn findCreateFileMode(
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pMode: *mut crate::src::headers::stdlib::mode_t,
    mut pUid: *mut crate::src::headers::stdlib::uid_t,
    mut pGid: *mut crate::src::headers::stdlib::gid_t,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    *pMode = 0 as crate::src::headers::stdlib::mode_t;
    *pUid = 0 as crate::src::headers::stdlib::uid_t;
    *pGid = 0 as crate::src::headers::stdlib::gid_t;
    if flags & (crate::src::headers::sqlite3_h::SQLITE_OPEN_WAL | crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_JOURNAL) != 0 {
        let mut zDb: [::core::ffi::c_char; 513] = [0; 513];
        let mut nDb: ::core::ffi::c_int = 0;
        nDb = crate::src::src::util::sqlite3Strlen30(zPath) - 1 as ::core::ffi::c_int;
        while nDb > 0 as ::core::ffi::c_int
            && *zPath.offset(nDb as isize) as ::core::ffi::c_int != '.' as i32
        {
            if *zPath.offset(nDb as isize) as ::core::ffi::c_int == '-' as i32 {
                ::core::ptr::copy_nonoverlapping(
                    zPath as *const u8,
                    &raw mut zDb as *mut ::core::ffi::c_char as *mut u8,
                    nDb as usize,
                );
                zDb[nDb as usize] = '\0' as i32 as ::core::ffi::c_char;
                rc = getFileMode(&raw mut zDb as *mut ::core::ffi::c_char, pMode, pUid, pGid);
                break;
            } else {
                nDb -= 1;
            }
        }
    } else if flags & crate::src::headers::sqlite3_h::SQLITE_OPEN_DELETEONCLOSE != 0 {
        *pMode = 0o600 as crate::src::headers::stdlib::mode_t;
    } else if flags & crate::src::headers::sqlite3_h::SQLITE_OPEN_URI != 0 {
        let mut z: *const ::core::ffi::c_char = crate::src::src::main::sqlite3_uri_parameter(
            zPath as crate::src::headers::sqlite3_h::sqlite3_filename,
            b"modeof\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !z.is_null() {
            rc = getFileMode(z, pMode, pUid, pGid);
        }
    }
    rc
}

unsafe extern "C" fn unixOpen(
    mut pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut pFile: *mut crate::src::headers::sqlite3_h::sqlite3_file,
    mut flags: ::core::ffi::c_int,
    mut pOutFlags: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut p: *mut unixFile = pFile as *mut unixFile;
    let mut fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut openFlags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut eType: ::core::ffi::c_int = flags & 0xfff00 as ::core::ffi::c_int;
    let mut noLock: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut ctrlFlags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut isExclusive: ::core::ffi::c_int = flags & crate::src::headers::sqlite3_h::SQLITE_OPEN_EXCLUSIVE;
    let mut isDelete: ::core::ffi::c_int = flags & crate::src::headers::sqlite3_h::SQLITE_OPEN_DELETEONCLOSE;
    let mut isCreate: ::core::ffi::c_int = flags & crate::src::headers::sqlite3_h::SQLITE_OPEN_CREATE;
    let mut isReadonly: ::core::ffi::c_int = flags & crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY;
    let mut isReadWrite: ::core::ffi::c_int = flags & crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE;
    let mut isNewJrnl: ::core::ffi::c_int = (isCreate != 0
        && (eType == crate::src::headers::sqlite3_h::SQLITE_OPEN_SUPER_JOURNAL
            || eType == crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_JOURNAL
            || eType == crate::src::headers::sqlite3_h::SQLITE_OPEN_WAL))
        as ::core::ffi::c_int;
    let mut zTmpname: [::core::ffi::c_char; 514] = [0; 514];
    let mut zName: *const ::core::ffi::c_char = zPath;
    if randomnessPid != ::libc::getpid() {
        randomnessPid = ::libc::getpid();
        crate::src::src::random::sqlite3_randomness(
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
    }
    ::libc::memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<unixFile>() as crate::__stddef_size_t_h::size_t,
    );
    if eType == crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_DB {
        let mut pUnused: *mut UnixUnusedFd = ::core::ptr::null_mut::<UnixUnusedFd>();
        pUnused = findReusableFd(zName, flags);
        if !pUnused.is_null() {
            fd = (*pUnused).fd;
        } else {
            pUnused = crate::src::src::malloc::sqlite3_malloc64(::core::mem::size_of::<UnixUnusedFd>() as crate::src::headers::sqlite3_h::sqlite3_uint64)
                as *mut UnixUnusedFd;
            if pUnused.is_null() {
                return crate::src::headers::sqliteInt_h::SQLITE_NOMEM_BKPT;
            }
        }
        (*p).pPreallocatedUnused = pUnused;
    } else if zName.is_null() {
        rc = unixGetTempname(
            (*pVfs).mxPathname,
            &raw mut zTmpname as *mut ::core::ffi::c_char,
        );
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        zName = &raw mut zTmpname as *mut ::core::ffi::c_char;
    }
    if isReadonly != 0 {
        openFlags |= ::libc::O_RDONLY;
    }
    if isReadWrite != 0 {
        openFlags |= ::libc::O_RDWR;
    }
    if isCreate != 0 {
        openFlags |= ::libc::O_CREAT;
    }
    if isExclusive != 0 {
        openFlags |= ::libc::O_EXCL | ::libc::O_NOFOLLOW;
    }
    openFlags |= ::libc::O_LARGEFILE | O_BINARY | ::libc::O_NOFOLLOW;
    if fd < 0 as ::core::ffi::c_int {
        let mut openMode: crate::src::headers::stdlib::mode_t = 0;
        let mut uid: crate::src::headers::stdlib::uid_t = 0;
        let mut gid: crate::src::headers::stdlib::gid_t = 0;
        rc = findCreateFileMode(zName, flags, &raw mut openMode, &raw mut uid, &raw mut gid);
        if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
            return rc;
        }
        fd = robust_open(zName, openFlags, openMode);
        if crate::src::src::main::sqlite3OSTrace != 0 {
            crate::src::src::printf::sqlite3DebugPrintf(
                b"OPENX   %-3d %s 0%o\n\0" as *const u8 as *const ::core::ffi::c_char,
                fd,
                zName,
                openFlags,
            );
        }
        if fd < 0 as ::core::ffi::c_int {
            if isNewJrnl != 0
                && *::libc::__errno_location() == ::libc::EACCES
                && ::core::mem::transmute::<
                    crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                    Option<
                        unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                    >,
                >(aSyscall[2 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")(zName, ::libc::F_OK)
                    != 0
            {
                rc = crate::src::headers::sqlite3_h::SQLITE_READONLY_DIRECTORY_1;
            } else if *::libc::__errno_location() != ::libc::EISDIR && isReadWrite != 0 {
                let mut pReadonly: *mut UnixUnusedFd = ::core::ptr::null_mut::<UnixUnusedFd>();
                flags &= !(crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE | crate::src::headers::sqlite3_h::SQLITE_OPEN_CREATE);
                openFlags &= !(::libc::O_RDWR | ::libc::O_CREAT);
                flags |= crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY;
                openFlags |= ::libc::O_RDONLY;
                isReadonly = 1 as ::core::ffi::c_int;
                pReadonly = findReusableFd(zName, flags);
                if !pReadonly.is_null() {
                    fd = (*pReadonly).fd;
                    crate::src::src::malloc::sqlite3_free(pReadonly as *mut ::core::ffi::c_void);
                } else {
                    fd = robust_open(zName, openFlags, openMode);
                }
            }
        }
        if fd < 0 as ::core::ffi::c_int {
            let mut rc2: ::core::ffi::c_int = unixLogErrorAtLine(
                crate::src::src::main::sqlite3CantopenError(6707 as ::core::ffi::c_int),
                b"open\0" as *const u8 as *const ::core::ffi::c_char,
                zName,
                6707 as ::core::ffi::c_int,
            );
            if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
                rc = rc2;
            }
            current_block = 6176415489515291600;
        } else {
            if openMode != 0
                && flags & (crate::src::headers::sqlite3_h::SQLITE_OPEN_WAL | crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_JOURNAL) != 0 as ::core::ffi::c_int
            {
                robustFchown(fd, uid, gid);
            }
            current_block = 6281126495347172768;
        }
    } else {
        current_block = 6281126495347172768;
    }
    match current_block {
        6281126495347172768 => {
            if !pOutFlags.is_null() {
                *pOutFlags = flags;
            }
            if !(*p).pPreallocatedUnused.is_null() {
                let __pPreallocatedUnused_ref = &mut *(*p).pPreallocatedUnused;
                __pPreallocatedUnused_ref.fd = fd;
                __pPreallocatedUnused_ref.flags =
                    flags & (crate::src::headers::sqlite3_h::SQLITE_OPEN_READONLY | crate::src::headers::sqlite3_h::SQLITE_OPEN_READWRITE);
            }
            if isDelete != 0 {
                ::core::mem::transmute::<
                    crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                    Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
                >(aSyscall[16 as ::core::ffi::c_int as usize].pCurrent)
                .expect("non-null function pointer")(zName);
            }
            if isDelete != 0 {
                ctrlFlags |= UNIXFILE_DELETE;
            }
            if isReadonly != 0 {
                ctrlFlags |= UNIXFILE_RDONLY;
            }
            noLock = (eType != crate::src::headers::sqlite3_h::SQLITE_OPEN_MAIN_DB) as ::core::ffi::c_int;
            if noLock != 0 {
                ctrlFlags |= UNIXFILE_NOLOCK;
            }
            if isNewJrnl != 0 {
                ctrlFlags |= UNIXFILE_DIRSYNC;
            }
            if flags & crate::src::headers::sqlite3_h::SQLITE_OPEN_URI != 0 {
                ctrlFlags |= UNIXFILE_URI;
            }
            rc = fillInUnixFile(pVfs, fd, pFile, zPath, ctrlFlags);
        }
        _ => {}
    }
    if rc != crate::src::headers::sqlite3_h::SQLITE_OK {
        crate::src::src::malloc::sqlite3_free((*p).pPreallocatedUnused as *mut ::core::ffi::c_void);
    }
    rc
}

unsafe extern "C" fn unixDelete(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut dirSync: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    if crate::src::src::os::sqlite3_io_error_persist != 0 && crate::src::src::os::sqlite3_io_error_hit != 0 || {
        let fresh4 = crate::src::src::os::sqlite3_io_error_pending;
        crate::src::src::os::sqlite3_io_error_pending -= 1;
        fresh4 == 1 as ::core::ffi::c_int
    } {
        local_ioerr();
        return 10 as ::core::ffi::c_int | (10 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    }
    if ::core::mem::transmute::<
        crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    >(aSyscall[16 as ::core::ffi::c_int as usize].pCurrent)
    .expect("non-null function pointer")(zPath)
        == -(1 as ::core::ffi::c_int)
    {
        if *::libc::__errno_location() == ::libc::ENOENT {
            rc = crate::src::headers::sqlite3_h::SQLITE_IOERR_DELETE_NOENT_1;
        } else {
            rc = unixLogErrorAtLine(
                10 as ::core::ffi::c_int | (10 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                b"unlink\0" as *const u8 as *const ::core::ffi::c_char,
                zPath,
                6849 as ::core::ffi::c_int,
            );
        }
        return rc;
    }
    if dirSync & 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        let mut fd: ::core::ffi::c_int = 0;
        rc = ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[17 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(zPath, &raw mut fd);
        if rc == crate::src::headers::sqlite3_h::SQLITE_OK {
            if full_fsync(fd, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int) != 0 {
                rc = unixLogErrorAtLine(
                    10 as ::core::ffi::c_int | (5 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int,
                    b"fsync\0" as *const u8 as *const ::core::ffi::c_char,
                    zPath,
                    6859 as ::core::ffi::c_int,
                );
            }
            robust_close(
                ::core::ptr::null_mut::<unixFile>(),
                fd,
                6861 as ::core::ffi::c_int,
            );
        } else {
            rc = crate::src::headers::sqlite3_h::SQLITE_OK;
        }
    }
    rc
}

unsafe extern "C" fn unixAccess(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut pResOut: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if crate::src::src::os::sqlite3_io_error_persist != 0 && crate::src::src::os::sqlite3_io_error_hit != 0 || {
        let fresh3 = crate::src::src::os::sqlite3_io_error_pending;
        crate::src::src::os::sqlite3_io_error_pending -= 1;
        fresh3 == 1 as ::core::ffi::c_int
    } {
        local_ioerr();
        return 10 as ::core::ffi::c_int | (13 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
    }
    if flags == crate::src::headers::sqlite3_h::SQLITE_ACCESS_EXISTS {
        let mut buf: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
        *pResOut = (0 as ::core::ffi::c_int
            == ::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut crate::src::headers::stdlib::stat,
                    ) -> ::core::ffi::c_int,
                >,
            >(aSyscall[4 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(zPath, &raw mut buf)
            && (!(buf.st_mode & crate::src::headers::stdlib::__S_IFMT as crate::src::headers::stdlib::__mode_t == 0o100000 as crate::src::headers::stdlib::__mode_t)
                || buf.st_size > 0 as crate::src::headers::stdlib::__off_t)) as ::core::ffi::c_int;
    } else {
        *pResOut = (::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
        >(aSyscall[2 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(zPath, ::libc::W_OK | ::libc::R_OK)
            == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn appendOnePathElement(
    mut pPath: *mut DbPath,
    mut zName: *const ::core::ffi::c_char,
    mut nName: ::core::ffi::c_int,
) {
    let __pPath_ref = unsafe { &mut *pPath };
    if *zName.offset(0 as isize) as ::core::ffi::c_int == '.' as i32 {
        if nName == 1 as ::core::ffi::c_int {
            return;
        }
        if *zName.offset(1 as isize) as ::core::ffi::c_int == '.' as i32
            && nName == 2 as ::core::ffi::c_int
        {
            if __pPath_ref.nUsed > 1 as ::core::ffi::c_int {
                loop {
                    __pPath_ref.nUsed -= 1;
                    if !(*__pPath_ref.zOut.offset(__pPath_ref.nUsed as isize) as ::core::ffi::c_int
                        != '/' as i32)
                    {
                        break;
                    }
                }
            }
            return;
        }
    }
    if __pPath_ref.nUsed + nName + 2 as ::core::ffi::c_int >= __pPath_ref.nOut {
        __pPath_ref.rc = crate::src::headers::sqlite3_h::SQLITE_ERROR;
        return;
    }
    let fresh1 = __pPath_ref.nUsed;
    __pPath_ref.nUsed += 1;
    *__pPath_ref.zOut.offset(fresh1 as isize) = '/' as i32 as ::core::ffi::c_char;
    ::core::ptr::copy_nonoverlapping(
                    zName as *const u8,
                    __pPath_ref.zOut.offset(__pPath_ref.nUsed as isize) as *mut ::core::ffi::c_char as *mut u8,
                    nName as usize,
                );
    __pPath_ref.nUsed += nName;
    if __pPath_ref.rc == crate::src::headers::sqlite3_h::SQLITE_OK {
        let mut zIn: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut buf: crate::src::headers::stdlib::stat = unsafe { ::core::mem::zeroed() };
        *__pPath_ref.zOut.offset(__pPath_ref.nUsed as isize) = 0 as ::core::ffi::c_char;
        zIn = __pPath_ref.zOut;
        if ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(*const ::core::ffi::c_char, *mut crate::src::headers::stdlib::stat) -> ::core::ffi::c_int,
            >,
        >(aSyscall[27 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(zIn, &raw mut buf)
            != 0 as ::core::ffi::c_int
        {
            if *::libc::__errno_location() != ::libc::ENOENT {
                __pPath_ref.rc = unixLogErrorAtLine(
                    crate::src::src::main::sqlite3CantopenError(6955 as ::core::ffi::c_int),
                    b"lstat\0" as *const u8 as *const ::core::ffi::c_char,
                    zIn,
                    6955 as ::core::ffi::c_int,
                );
            }
        } else if buf.st_mode & crate::src::headers::stdlib::__S_IFMT as crate::src::headers::stdlib::__mode_t == 0o120000 as crate::src::headers::stdlib::__mode_t {
            let mut got: crate::src::headers::stdlib::ssize_t = 0;
            let mut zLnk: [::core::ffi::c_char; 4098] = [0; 4098];
            let fresh2 = __pPath_ref.nSymlink;
            __pPath_ref.nSymlink += 1;
            if fresh2 > crate::src::src::os::SQLITE_MAX_SYMLINK {
                __pPath_ref.rc = crate::src::src::main::sqlite3CantopenError(6961 as ::core::ffi::c_int);
                return;
            }
            got = ::core::mem::transmute::<
                crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_char,
                        crate::__stddef_size_t_h::size_t,
                    ) -> crate::src::headers::stdlib::ssize_t,
                >,
            >(aSyscall[26 as ::core::ffi::c_int as usize].pCurrent)
            .expect("non-null function pointer")(
                zIn,
                &raw mut zLnk as *mut ::core::ffi::c_char,
                (::core::mem::size_of::<[::core::ffi::c_char; 4098]>() as crate::__stddef_size_t_h::size_t)
                    .wrapping_sub(2 as crate::__stddef_size_t_h::size_t),
            );
            if got <= 0 as crate::src::headers::stdlib::ssize_t
                || got
                    >= ::core::mem::size_of::<[::core::ffi::c_char; 4098]>() as crate::src::headers::stdlib::ssize_t
                        - 2 as crate::src::headers::stdlib::ssize_t
            {
                __pPath_ref.rc = unixLogErrorAtLine(
                    crate::src::src::main::sqlite3CantopenError(6966 as ::core::ffi::c_int),
                    b"readlink\0" as *const u8 as *const ::core::ffi::c_char,
                    zIn,
                    6966 as ::core::ffi::c_int,
                );
                return;
            }
            zLnk[got as usize] = 0 as ::core::ffi::c_char;
            if zLnk[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '/' as i32 {
                __pPath_ref.nUsed = 0 as ::core::ffi::c_int;
            } else {
                __pPath_ref.nUsed -= nName + 1 as ::core::ffi::c_int;
            }
            appendAllPathElements(pPath, &raw mut zLnk as *mut ::core::ffi::c_char);
        }
    }
}

unsafe extern "C" fn appendAllPathElements(
    mut pPath: *mut DbPath,
    mut zPath: *const ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        while *zPath.offset(i as isize) as ::core::ffi::c_int != 0
            && *zPath.offset(i as isize) as ::core::ffi::c_int != '/' as i32
        {
            i += 1;
        }
        if i > j {
            appendOnePathElement(
                pPath,
                zPath.offset(j as isize) as *const ::core::ffi::c_char,
                i - j,
            );
        }
        j = i + 1 as ::core::ffi::c_int;
        let fresh0 = i;
        i += 1;
        if !(*zPath.offset(fresh0 as isize) != 0) {
            break;
        }
    }
}

unsafe extern "C" fn unixFullPathname(
    mut _pVfs: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zPath: *const ::core::ffi::c_char,
    mut nOut: ::core::ffi::c_int,
    mut zOut: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut path: DbPath = unsafe { ::core::mem::zeroed() };
    path.rc = 0 as ::core::ffi::c_int;
    path.nUsed = 0 as ::core::ffi::c_int;
    path.nSymlink = 0 as ::core::ffi::c_int;
    path.nOut = nOut;
    path.zOut = zOut;
    if *zPath.offset(0 as isize) as ::core::ffi::c_int != '/' as i32 {
        let mut zPwd: [::core::ffi::c_char; 4098] = [0; 4098];
        if ::core::mem::transmute::<
            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
            Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_char, crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_char,
            >,
        >(aSyscall[3 as ::core::ffi::c_int as usize].pCurrent)
        .expect("non-null function pointer")(
            &raw mut zPwd as *mut ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 4098]>() as crate::__stddef_size_t_h::size_t)
                .wrapping_sub(2 as crate::__stddef_size_t_h::size_t),
        )
        .is_null()
        {
            return unixLogErrorAtLine(
                crate::src::src::main::sqlite3CantopenError(7024 as ::core::ffi::c_int),
                b"getcwd\0" as *const u8 as *const ::core::ffi::c_char,
                zPath,
                7024 as ::core::ffi::c_int,
            );
        }
        appendAllPathElements(&raw mut path, &raw mut zPwd as *mut ::core::ffi::c_char);
    }
    appendAllPathElements(&raw mut path, zPath);
    *zOut.offset(path.nUsed as isize) = 0 as ::core::ffi::c_char;
    if path.rc != 0 || path.nUsed < 2 as ::core::ffi::c_int {
        return crate::src::src::main::sqlite3CantopenError(7030 as ::core::ffi::c_int);
    }
    if path.nSymlink != 0 {
        return crate::src::headers::sqlite3_h::SQLITE_OK_SYMLINK;
    }
    crate::src::headers::sqlite3_h::SQLITE_OK
}

unsafe extern "C" fn unixDlOpen(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut zFilename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    ::libc::dlopen(zFilename, ::libc::RTLD_NOW | ::libc::RTLD_GLOBAL)
}

unsafe extern "C" fn unixDlError(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut nBuf: ::core::ffi::c_int,
    mut zBufOut: *mut ::core::ffi::c_char,
) {
    let mut zErr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    unixEnterMutex();
    zErr = ::libc::dlerror();
    if !zErr.is_null() {
        crate::src::src::printf::sqlite3_snprintf(
            nBuf,
            zBufOut,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            zErr,
        );
    }
    unixLeaveMutex();
}

unsafe extern "C" fn unixDlSym(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut p: *mut ::core::ffi::c_void,
    mut zSym: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn() -> ()> {
    let mut x: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> Option<unsafe extern "C" fn() -> ()>,
    > = None;
    x = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_void,
        >,
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> Option<unsafe extern "C" fn() -> ()>,
        >,
    >(Some(
        ::libc::dlsym
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
            ) -> *mut ::core::ffi::c_void,
    ));
    Some(x.expect("non-null function pointer")).expect("non-null function pointer")(
        p, zSym,
    )
}

unsafe extern "C" fn unixDlClose(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut pHandle: *mut ::core::ffi::c_void,
) {
    ::libc::dlclose(pHandle);
}

unsafe extern "C" fn unixRandomness(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut nBuf: ::core::ffi::c_int,
    mut zBuf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    ::libc::memset(
        zBuf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        nBuf as crate::__stddef_size_t_h::size_t,
    );
    randomnessPid = ::libc::getpid();
    nBuf
}

unsafe extern "C" fn unixSleep(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut microseconds: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sp: ::libc::timespec = unsafe { ::core::mem::zeroed() };
    sp.tv_sec = (microseconds / 1000000 as ::core::ffi::c_int) as crate::src::headers::stdlib::__time_t;
    sp.tv_nsec = (microseconds % 1000000 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int)
        as crate::src::headers::stdlib::__syscall_slong_t;
    ::libc::nanosleep(&raw mut sp as *mut _ as *const ::libc::timespec,  ::core::ptr::null_mut::<::libc::timespec>() as *mut ::libc::timespec);
    microseconds
}
#[unsafe(no_mangle)]

pub static mut sqlite3_current_time: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

unsafe extern "C" fn unixCurrentTimeInt64(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut piNow: *mut crate::src::headers::sqlite3_h::sqlite3_int64,
) -> ::core::ffi::c_int {
    static mut unixEpoch: crate::src::headers::sqlite3_h::sqlite3_int64 =
        24405875 as crate::src::headers::sqlite3_h::sqlite3_int64 * 8640000 as ::core::ffi::c_int as crate::src::headers::sqlite3_h::sqlite3_int64;
    let mut rc: ::core::ffi::c_int = crate::src::headers::sqlite3_h::SQLITE_OK;
    let mut sNow: ::libc::timeval = unsafe { ::core::mem::zeroed() };
    crate::src::headers::stdlib::gettimeofday(
        &raw mut sNow,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    *piNow = unixEpoch
        + 1000 as crate::src::headers::sqlite3_h::sqlite3_int64 * sNow.tv_sec as crate::src::headers::sqlite3_h::sqlite3_int64
        + (sNow.tv_usec / 1000 as crate::src::headers::stdlib::__suseconds_t) as crate::src::headers::sqlite3_h::sqlite3_int64;
    if sqlite3_current_time != 0 {
        *piNow = 1000 as crate::src::headers::sqlite3_h::sqlite3_int64 * sqlite3_current_time as crate::src::headers::sqlite3_h::sqlite3_int64 + unixEpoch;
    }
    rc
}

unsafe extern "C" fn unixCurrentTime(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut prNow: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut i: crate::src::headers::sqlite3_h::sqlite3_int64 = 0 as crate::src::headers::sqlite3_h::sqlite3_int64;
    let mut rc: ::core::ffi::c_int = 0;
    rc = unixCurrentTimeInt64(::core::ptr::null_mut::<crate::src::headers::sqlite3_h::sqlite3_vfs>(), &raw mut i);
    *prNow = i as ::core::ffi::c_double / 86400000.0f64;
    rc
}

unsafe extern "C" fn unixGetLastError(
    mut _NotUsed: *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    mut _NotUsed2: ::core::ffi::c_int,
    mut _NotUsed3: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    *::libc::__errno_location()
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3_os_init() -> ::core::ffi::c_int {
    static mut aVfs: [crate::src::headers::sqlite3_h::sqlite3_vfs; 4] = unsafe {
        [
            crate::src::headers::sqlite3_h::sqlite3_vfs {
    iVersion:  3 as ::core::ffi::c_int,
    szOsFile:  ::core::mem::size_of::<unixFile>() as ::core::ffi::c_int,
    mxPathname:  MAX_PATHNAME,
    pNext:  ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_vfs>() as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    zName:  b"unix\0" as *const u8 as *const ::core::ffi::c_char,
    pAppData:  &raw const posixIoFinder as *mut ::core::ffi::c_void,
    xOpen:  Some(
                    unixOpen
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            *mut crate::src::headers::sqlite3_h::sqlite3_file,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xDelete:  Some(
                    unixDelete
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xAccess:  Some(
                    unixAccess
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xFullPathname:  Some(
                    unixFullPathname
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
    xDlOpen:  Some(
                    unixDlOpen
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        )
                            -> *mut ::core::ffi::c_void,
                ),
    xDlError:  Some(
                    unixDlError
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> (),
                ),
    xDlSym:  Some(
                    unixDlSym
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                        )
                            -> Option<unsafe extern "C" fn() -> ()>,
                ),
    xDlClose:  Some(
                    unixDlClose
                        as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
                ),
    xRandomness:  Some(
                    unixRandomness
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
    xSleep:  Some(
                    unixSleep
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xCurrentTime:  Some(
                    unixCurrentTime
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *mut ::core::ffi::c_double,
                        ) -> ::core::ffi::c_int,
                ),
    xGetLastError:  Some(
                    unixGetLastError
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
    xCurrentTimeInt64:  Some(
                    unixCurrentTimeInt64
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *mut crate::src::headers::sqlite3_h::sqlite3_int64,
                        ) -> ::core::ffi::c_int,
                ),
    xSetSystemCall:  Some(
                    unixSetSystemCall
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                        ) -> ::core::ffi::c_int,
                ),
    xGetSystemCall:  Some(
                    unixGetSystemCall
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        ) -> crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                ),
    xNextSystemCall:  Some(
                    unixNextSystemCall
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        )
                            -> *const ::core::ffi::c_char,
                ),
},
            crate::src::headers::sqlite3_h::sqlite3_vfs {
    iVersion:  3 as ::core::ffi::c_int,
    szOsFile:  ::core::mem::size_of::<unixFile>() as ::core::ffi::c_int,
    mxPathname:  MAX_PATHNAME,
    pNext:  ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_vfs>() as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    zName:  b"unix-none\0" as *const u8 as *const ::core::ffi::c_char,
    pAppData:  &raw const nolockIoFinder as *mut ::core::ffi::c_void,
    xOpen:  Some(
                    unixOpen
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            *mut crate::src::headers::sqlite3_h::sqlite3_file,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xDelete:  Some(
                    unixDelete
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xAccess:  Some(
                    unixAccess
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xFullPathname:  Some(
                    unixFullPathname
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
    xDlOpen:  Some(
                    unixDlOpen
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        )
                            -> *mut ::core::ffi::c_void,
                ),
    xDlError:  Some(
                    unixDlError
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> (),
                ),
    xDlSym:  Some(
                    unixDlSym
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                        )
                            -> Option<unsafe extern "C" fn() -> ()>,
                ),
    xDlClose:  Some(
                    unixDlClose
                        as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
                ),
    xRandomness:  Some(
                    unixRandomness
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
    xSleep:  Some(
                    unixSleep
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xCurrentTime:  Some(
                    unixCurrentTime
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *mut ::core::ffi::c_double,
                        ) -> ::core::ffi::c_int,
                ),
    xGetLastError:  Some(
                    unixGetLastError
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
    xCurrentTimeInt64:  Some(
                    unixCurrentTimeInt64
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *mut crate::src::headers::sqlite3_h::sqlite3_int64,
                        ) -> ::core::ffi::c_int,
                ),
    xSetSystemCall:  Some(
                    unixSetSystemCall
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                        ) -> ::core::ffi::c_int,
                ),
    xGetSystemCall:  Some(
                    unixGetSystemCall
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        ) -> crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                ),
    xNextSystemCall:  Some(
                    unixNextSystemCall
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        )
                            -> *const ::core::ffi::c_char,
                ),
},
            crate::src::headers::sqlite3_h::sqlite3_vfs {
    iVersion:  3 as ::core::ffi::c_int,
    szOsFile:  ::core::mem::size_of::<unixFile>() as ::core::ffi::c_int,
    mxPathname:  MAX_PATHNAME,
    pNext:  ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_vfs>() as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    zName:  b"unix-dotfile\0" as *const u8 as *const ::core::ffi::c_char,
    pAppData:  &raw const dotlockIoFinder as *mut ::core::ffi::c_void,
    xOpen:  Some(
                    unixOpen
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            *mut crate::src::headers::sqlite3_h::sqlite3_file,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xDelete:  Some(
                    unixDelete
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xAccess:  Some(
                    unixAccess
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xFullPathname:  Some(
                    unixFullPathname
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
    xDlOpen:  Some(
                    unixDlOpen
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        )
                            -> *mut ::core::ffi::c_void,
                ),
    xDlError:  Some(
                    unixDlError
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> (),
                ),
    xDlSym:  Some(
                    unixDlSym
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                        )
                            -> Option<unsafe extern "C" fn() -> ()>,
                ),
    xDlClose:  Some(
                    unixDlClose
                        as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
                ),
    xRandomness:  Some(
                    unixRandomness
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
    xSleep:  Some(
                    unixSleep
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xCurrentTime:  Some(
                    unixCurrentTime
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *mut ::core::ffi::c_double,
                        ) -> ::core::ffi::c_int,
                ),
    xGetLastError:  Some(
                    unixGetLastError
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
    xCurrentTimeInt64:  Some(
                    unixCurrentTimeInt64
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *mut crate::src::headers::sqlite3_h::sqlite3_int64,
                        ) -> ::core::ffi::c_int,
                ),
    xSetSystemCall:  Some(
                    unixSetSystemCall
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                        ) -> ::core::ffi::c_int,
                ),
    xGetSystemCall:  Some(
                    unixGetSystemCall
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        ) -> crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                ),
    xNextSystemCall:  Some(
                    unixNextSystemCall
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        )
                            -> *const ::core::ffi::c_char,
                ),
},
            crate::src::headers::sqlite3_h::sqlite3_vfs {
    iVersion:  3 as ::core::ffi::c_int,
    szOsFile:  ::core::mem::size_of::<unixFile>() as ::core::ffi::c_int,
    mxPathname:  MAX_PATHNAME,
    pNext:  ::core::ptr::null::<crate::src::headers::sqlite3_h::sqlite3_vfs>() as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
    zName:  b"unix-excl\0" as *const u8 as *const ::core::ffi::c_char,
    pAppData:  &raw const posixIoFinder as *mut ::core::ffi::c_void,
    xOpen:  Some(
                    unixOpen
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            *mut crate::src::headers::sqlite3_h::sqlite3_file,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xDelete:  Some(
                    unixDelete
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xAccess:  Some(
                    unixAccess
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xFullPathname:  Some(
                    unixFullPathname
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
    xDlOpen:  Some(
                    unixDlOpen
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        )
                            -> *mut ::core::ffi::c_void,
                ),
    xDlError:  Some(
                    unixDlError
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> (),
                ),
    xDlSym:  Some(
                    unixDlSym
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *mut ::core::ffi::c_void,
                            *const ::core::ffi::c_char,
                        )
                            -> Option<unsafe extern "C" fn() -> ()>,
                ),
    xDlClose:  Some(
                    unixDlClose
                        as unsafe extern "C" fn(*mut crate::src::headers::sqlite3_h::sqlite3_vfs, *mut ::core::ffi::c_void) -> (),
                ),
    xRandomness:  Some(
                    unixRandomness
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
    xSleep:  Some(
                    unixSleep
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                ),
    xCurrentTime:  Some(
                    unixCurrentTime
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *mut ::core::ffi::c_double,
                        ) -> ::core::ffi::c_int,
                ),
    xGetLastError:  Some(
                    unixGetLastError
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            ::core::ffi::c_int,
                            *mut ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
    xCurrentTimeInt64:  Some(
                    unixCurrentTimeInt64
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *mut crate::src::headers::sqlite3_h::sqlite3_int64,
                        ) -> ::core::ffi::c_int,
                ),
    xSetSystemCall:  Some(
                    unixSetSystemCall
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                            crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                        ) -> ::core::ffi::c_int,
                ),
    xGetSystemCall:  Some(
                    unixGetSystemCall
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        ) -> crate::src::headers::sqlite3_h::sqlite3_syscall_ptr,
                ),
    xNextSystemCall:  Some(
                    unixNextSystemCall
                        as unsafe extern "C" fn(
                            *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
                            *const ::core::ffi::c_char,
                        )
                            -> *const ::core::ffi::c_char,
                ),
},
        ]
    };
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while (i as usize)
        < (::core::mem::size_of::<[crate::src::headers::sqlite3_h::sqlite3_vfs; 4]>() as usize)
            .wrapping_div(::core::mem::size_of::<crate::src::headers::sqlite3_h::sqlite3_vfs>() as usize)
    {
        crate::src::src::os::sqlite3_vfs_register(
            
            (&raw mut aVfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs).offset(i as isize) as *mut crate::src::headers::sqlite3_h::sqlite3_vfs as *mut crate::src::headers::sqlite3_h::sqlite3_vfs,
            (i == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        );
        i = i.wrapping_add(1);
    }
    unixBigLock = crate::src::src::mutex::sqlite3MutexAlloc(crate::src::headers::sqlite3_h::SQLITE_MUTEX_STATIC_VFS1);
    unixTempFileInit();
    crate::src::headers::sqlite3_h::SQLITE_OK
}
#[unsafe(no_mangle)]

pub unsafe extern "C" fn sqlite3_os_end() -> ::core::ffi::c_int {
    unixBigLock = ::core::ptr::null_mut::<crate::src::src::mutex_unix::sqlite3_mutex>();
    crate::src::headers::sqlite3_h::SQLITE_OK
}
