use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to create directory `{path}`")]
    CreateDir {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to create parent directory `{parent}` for `{path}`")]
    CreateParentDir {
        path: PathBuf,
        parent: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to write temporary file for `{path}`")]
    WriteTemp {
        path: PathBuf,
        temp_path: Option<PathBuf>,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to persist temporary file `{temp_path}` to `{path}`")]
    PersistTemp {
        temp_path: PathBuf,
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("refusing to overwrite existing file `{path}`")]
    AlreadyExists { path: PathBuf },

    #[error("path `{path}` has no parent directory")]
    MissingParent { path: PathBuf },
}
