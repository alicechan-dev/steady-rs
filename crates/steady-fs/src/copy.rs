use std::path::{Path, PathBuf};

use crate::backup::backup_path_for;
use crate::{Error, Result};

/// Starts a file copy from `from` to `to`.
///
/// This helper copies a single file only. It does not recursively copy
/// directories.
pub fn copy_file<P, Q>(from: P, to: Q) -> CopyFile
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    CopyFile {
        from: from.as_ref().to_path_buf(),
        to: to.as_ref().to_path_buf(),
        create_parent_dirs: false,
        overwrite: true,
        backup_existing: false,
        backup_suffix: ".bak".to_owned(),
        dry_run: false,
    }
}

/// Builder for [`copy_file`].
///
/// By default, parent directories are not created, existing files may be
/// overwritten, backups are disabled, and dry-run mode is disabled.
#[derive(Debug, Clone)]
pub struct CopyFile {
    from: PathBuf,
    to: PathBuf,
    create_parent_dirs: bool,
    overwrite: bool,
    backup_existing: bool,
    backup_suffix: String,
    dry_run: bool,
}

impl CopyFile {
    /// Sets whether missing parent directories for the destination should be created.
    pub fn create_parent_dirs(mut self, create_parent_dirs: bool) -> Self {
        self.create_parent_dirs = create_parent_dirs;
        self
    }

    /// Sets whether an existing destination file may be overwritten.
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    /// Sets whether an existing destination file should be backed up first.
    pub fn backup_existing(mut self, backup_existing: bool) -> Self {
        self.backup_existing = backup_existing;
        self
    }

    /// Sets the suffix appended to the destination path for backups.
    pub fn backup_suffix(mut self, backup_suffix: impl Into<String>) -> Self {
        self.backup_suffix = backup_suffix.into();
        self
    }

    /// Sets whether the operation should validate policy but skip filesystem writes.
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Copies the source file to the destination and returns the copied byte count.
    pub fn run(self) -> Result<u64> {
        if !self.overwrite && self.to.exists() {
            return Err(Error::AlreadyExists { path: self.to });
        }

        let parent = self.parent()?;

        if self.dry_run {
            return Ok(0);
        }

        if self.create_parent_dirs {
            std::fs::create_dir_all(parent).map_err(|source| Error::CreateParentDir {
                path: self.to.clone(),
                parent: parent.to_path_buf(),
                source,
            })?;
        }

        if self.backup_existing && self.to.exists() {
            let backup_path = self.backup_path();
            std::fs::copy(&self.to, &backup_path).map_err(|source| Error::Backup {
                path: self.to.clone(),
                backup_path,
                source,
            })?;
        }

        std::fs::copy(&self.from, &self.to).map_err(|source| Error::CopyFile {
            from: self.from,
            to: self.to,
            source,
        })
    }

    fn parent(&self) -> Result<&Path> {
        self.to.parent().ok_or_else(|| Error::MissingParent {
            path: self.to.clone(),
        })
    }

    fn backup_path(&self) -> PathBuf {
        backup_path_for(&self.to, &self.backup_suffix)
    }
}
