//! Small, boring, reliable filesystem helpers for Rust CLI tools and developer apps.

mod atomic;
mod dir;
mod error;

pub mod prelude;

pub use atomic::{atomic_write, AtomicWrite};
pub use dir::ensure_dir;
pub use error::{Error, Result};
