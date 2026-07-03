use std::path::Path;

use crate::{Error, Result};

/// Creates a directory and any missing parent directories.
///
/// Succeeds if the directory already exists.
pub fn ensure_dir<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    std::fs::create_dir_all(path).map_err(|source| Error::CreateDir {
        path: path.to_path_buf(),
        source,
    })
}
