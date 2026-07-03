# Error Design

`steady-fs` should use a custom error enum for its public API.

The crate should use `thiserror` for error implementations. It should not expose `anyhow` from the library API. Applications using `steady-fs` can still convert errors into `anyhow::Error` at their own boundary.

Errors should include path context and preserve the original `std::io::Error` as the source where possible.

## Draft error type

```rust
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
    AlreadyExists {
        path: PathBuf,
    },

    #[error("failed to backup existing file `{path}` to `{backup_path}`")]
    Backup {
        path: PathBuf,
        backup_path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to copy file from `{from}` to `{to}`")]
    CopyFile {
        from: PathBuf,
        to: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to move file from `{from}` to `{to}`")]
    MoveFile {
        from: PathBuf,
        to: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("path `{path}` has no parent directory")]
    MissingParent {
        path: PathBuf,
    },

    #[error("refusing to clean dangerous directory `{path}`")]
    DangerousCleanDir {
        path: PathBuf,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
```

This is a draft and can change during implementation.

## Error principles

- Every filesystem error should identify the relevant path.
- Operations with two paths should include both paths.
- Original `std::io::Error` values should be kept as sources where possible.
- Public errors should be specific enough for CLI tools to print useful messages.
- The error type should stay small enough to understand without reading the implementation.
