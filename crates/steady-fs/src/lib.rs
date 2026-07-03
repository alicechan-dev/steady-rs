//! Small, boring, reliable filesystem helpers for Rust CLI tools and developer apps.

mod atomic;
mod clean;
mod copy;
mod dir;
mod error;
mod move_file;

pub mod prelude;

pub use atomic::{atomic_write, AtomicWrite};
pub use clean::{clean_dir, CleanDir};
pub use copy::{copy_file, CopyFile};
pub use dir::ensure_dir;
pub use error::{Error, Result};
pub use move_file::{move_file, MoveFile};
