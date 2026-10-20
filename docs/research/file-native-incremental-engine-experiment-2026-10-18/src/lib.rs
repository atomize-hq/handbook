//! Private I1 experiment. No production API/format stability promise.
pub mod io;
pub mod disk;
pub mod objects;
#[path="baseline_model.rs"]
pub mod model;
pub mod graph;
pub mod store;
pub mod projection;
mod navigation;
mod pack;
mod packing;
mod physical_lifetime;
pub mod lifetime;
pub mod api;
pub mod differential;
pub use objects::{Objects,Id};
pub use io::{Result,Metrics};
pub const MAX_OBJECT:usize=1024*1024;
#[cfg(test)]
mod bounds_tests;
