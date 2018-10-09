//! Database-related operations.

#[path="rocksdb/mod.rs"]
mod impls;

pub use self::impls::{open_db, restoration_db_handler, migrate};

#[cfg(feature = "secretstore")]
pub use self::impls::open_secretstore_db;
