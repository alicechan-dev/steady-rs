use std::io::Write;
use std::path::{Path, PathBuf};

use crate::backup::backup_path_for;
use crate::{Error, Result};

/// Starts an atomic-style file write to `path`.
///
/// The data is written to a temporary file in the destination directory and
/// then persisted to the final path when [`AtomicWrite::write`] is called.
pub fn atomic_write<P, D>(path: P, data: D) -> AtomicWrite
where
    P: AsRef<Path>,
    D: AsRef<[u8]>,
{
    AtomicWrite {
        path: path.as_ref().to_path_buf(),
        data: data.as_ref().to_vec(),
        create_parent_dirs: false,
        overwrite: true,
        backup_existing: false,
        backup_suffix: ".bak".to_owned(),
        dry_run: false,
    }
}

/// Builder for [`atomic_write`].
///
/// By default, parent directories are not created, existing files may be
/// overwritten, backups are disabled, and dry-run mode is disabled.
#[derive(Debug, Clone)]
pub struct AtomicWrite {
    path: PathBuf,
    data: Vec<u8>,
    create_parent_dirs: bool,
    overwrite: bool,
    backup_existing: bool,
    backup_suffix: String,
    dry_run: bool,
}

impl AtomicWrite {
    /// Sets whether missing parent directories should be created.
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

    /// Writes the data to the destination path.
    pub fn write(self) -> Result<()> {
        if !self.overwrite && self.path.exists() {
            return Err(Error::AlreadyExists { path: self.path });
        }

        let parent = self.parent()?;

        if self.dry_run {
            return Ok(());
        }

        if self.create_parent_dirs {
            std::fs::create_dir_all(parent).map_err(|source| Error::CreateParentDir {
                path: self.path.clone(),
                parent: parent.to_path_buf(),
                source,
            })?;
        }

        let mut temp =
            tempfile::NamedTempFile::new_in(parent).map_err(|source| Error::WriteTemp {
                path: self.path.clone(),
                temp_path: None,
                source,
            })?;

        temp.write_all(&self.data)
            .map_err(|source| Error::WriteTemp {
                path: self.path.clone(),
                temp_path: Some(temp.path().to_path_buf()),
                source,
            })?;

        temp.flush().map_err(|source| Error::WriteTemp {
            path: self.path.clone(),
            temp_path: Some(temp.path().to_path_buf()),
            source,
        })?;

        temp.as_file()
            .sync_all()
            .map_err(|source| Error::WriteTemp {
                path: self.path.clone(),
                temp_path: Some(temp.path().to_path_buf()),
                source,
            })?;

        if self.backup_existing && self.path.exists() {
            let backup_path = self.backup_path();
            std::fs::copy(&self.path, &backup_path).map_err(|source| Error::Backup {
                path: self.path.clone(),
                backup_path,
                source,
            })?;
        }

        let temp_path = temp.path().to_path_buf();
        temp.persist(&self.path).map_err(|err| Error::PersistTemp {
            temp_path,
            path: self.path,
            source: err.error,
        })?;

        Ok(())
    }

    fn parent(&self) -> Result<&Path> {
        self.path.parent().ok_or_else(|| Error::MissingParent {
            path: self.path.clone(),
        })
    }

    fn backup_path(&self) -> PathBuf {
        backup_path_for(&self.path, &self.backup_suffix)
    }
}
