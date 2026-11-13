//! A reusable opaque-byte database. Domain permission and record meaning remain
//! with its caller. See the crate README for publication and durability scope.
mod backend;
mod index;
mod objects;
mod store;
pub use backend::{Backend, NativeBackend, Storage};
pub use store::*;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub(crate) type Id = String;
pub const MAX_OBJECT: usize = 1024 * 1024;
pub const MAX_CHANGES: usize = 256;
pub const MAX_TRANSACTION_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_READERS: usize = 8;
pub(crate) fn digest(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}
