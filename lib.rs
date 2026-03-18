#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(raw_ref_op)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod src {
    pub mod ctime;
    pub mod ext {
        pub mod fts3 {
            pub mod fts3;
            pub mod fts3_aux;
            pub mod fts3_expr;
            pub mod fts3_hash;
            pub mod fts3_icu;
            pub mod fts3_porter;
            pub mod fts3_snippet;
            pub mod fts3_tokenize_vtab;
            pub mod fts3_tokenizer;
            pub mod fts3_tokenizer1;
            pub mod fts3_unicode;
            pub mod fts3_unicode2;
            pub mod fts3_write;
        } // mod fts3
        pub mod icu {
            pub mod icu;
        } // mod icu
        pub mod misc {
            pub mod stmt;
        } // mod misc
        pub mod rbu {
            pub mod sqlite3rbu;
        } // mod rbu
        pub mod rtree {
            pub mod rtree;
        } // mod rtree
        pub mod session {
            pub mod sqlite3session;
        } // mod session
    } // mod ext
    pub mod fts5;
    pub mod opcodes;
    pub mod parse;
    pub mod src {
        pub mod alter;
        pub mod analyze;
        pub mod attach;
        pub mod auth;
        pub mod backup;
        pub mod bitvec;
        pub mod btmutex;
        pub mod btree;
        pub mod build;
        pub mod callback;
        pub mod carray;
        pub mod complete;
        pub mod date;
        pub mod dbpage;
        pub mod dbstat;
        pub mod delete;
        pub mod expr;
        pub mod fault;
        pub mod fkey;
        pub mod func;
        pub mod global;
        pub mod hash;
        pub mod insert;
        pub mod json;
        pub mod legacy;
        pub mod loadext;
        pub mod main;
        pub mod malloc;
        pub mod mem0;
        pub mod mem1;
        pub mod mem2;
        pub mod mem3;
        pub mod mem5;
        pub mod memdb;
        pub mod memjournal;
        pub mod mutex;
        pub mod mutex_noop;
        pub mod mutex_unix;
        pub mod mutex_w32;
        pub mod notify;
        pub mod os;
        pub mod os_kv;
        pub mod os_unix;
        pub mod os_win;
        pub mod pager;
        pub mod pcache;
        pub mod pcache1;
        pub mod pragma;
        pub mod prepare;
        pub mod printf;
        pub mod random;
        pub mod resolve;
        pub mod rowset;
        pub mod select;
        pub mod status;
        pub mod table;
        pub mod threads;
        pub mod tokenize;
        pub mod treeview;
        pub mod trigger;
        pub mod update;
        pub mod upsert;
        pub mod utf;
        pub mod util;
        pub mod vacuum;
        pub mod vdbe;
        pub mod vdbeapi;
        pub mod vdbeaux;
        pub mod vdbeblob;
        pub mod vdbemem;
        pub mod vdbesort;
        pub mod vdbetrace;
        pub mod vdbevtab;
        pub mod vtab;
        pub mod wal;
        pub mod walker;
        pub mod r#where;
        pub mod wherecode;
        pub mod whereexpr;
        pub mod window;
    } // mod src
} // mod src
