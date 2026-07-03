//! Small, boring, reliable filesystem helpers for Rust CLI tools and developer apps.

mod dir;
mod error;

pub mod prelude;

pub use dir::ensure_dir;
pub use error::{Error, Result};
