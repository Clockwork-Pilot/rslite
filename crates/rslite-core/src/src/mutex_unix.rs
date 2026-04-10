// Thin shim — all implementation lives in crate::src::mutex_pl.
pub use crate::src::mutex_pl::Sqlite3Mutex as sqlite3_mutex;
pub use crate::src::mutex_pl::sqlite3DefaultMutex;
pub use crate::src::mutex_pl::sqlite3MemoryBarrier;
