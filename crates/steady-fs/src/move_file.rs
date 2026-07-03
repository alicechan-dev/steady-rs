use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::{Error, Result};

pub fn move_file<P, Q>(from: P, to: Q) -> MoveFile
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    MoveFile {
        from: from.as_ref().to_path_buf(),
        to: to.as_ref().to_path_buf(),
        create_parent_dirs: false,
        overwrite: true,
        backup_existing: false,
        backup_suffix: ".bak".to_owned(),
        dry_run: false,
        fallback_copy_delete: false,
    }
}

#[derive(Debug, Clone)]
pub struct MoveFile {
    from: PathBuf,
    to: PathBuf,
    create_parent_dirs: bool,
    overwrite: bool,
    backup_existing: bool,
    backup_suffix: String,
    dry_run: bool,
    fallback_copy_delete: bool,
}

impl MoveFile {
    pub fn create_parent_dirs(mut self, create_parent_dirs: bool) -> Self {
        self.create_parent_dirs = create_parent_dirs;
        self
    }

    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    pub fn backup_existing(mut self, backup_existing: bool) -> Self {
        self.backup_existing = backup_existing;
        self
    }

    pub fn backup_suffix(mut self, backup_suffix: impl Into<String>) -> Self {
        self.backup_suffix = backup_suffix.into();
        self
    }

    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    pub fn fallback_copy_delete(mut self, fallback_copy_delete: bool) -> Self {
        self.fallback_copy_delete = fallback_copy_delete;
        self
    }

    pub fn run(self) -> Result<()> {
        if !self.overwrite && self.to.exists() {
            return Err(Error::AlreadyExists { path: self.to });
        }

        let parent = self.parent()?;

        if self.dry_run {
            return Ok(());
        }

        self.validate_source_is_file()?;

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

        if self.to.exists() {
            std::fs::remove_file(&self.to).map_err(|source| Error::MoveFile {
                from: self.from.clone(),
                to: self.to.clone(),
                source,
            })?;
        }

        match std::fs::rename(&self.from, &self.to) {
            Ok(()) => Ok(()),
            Err(_) if self.fallback_copy_delete => {
                self.copy_delete().map_err(|source| Error::MoveFile {
                    from: self.from,
                    to: self.to,
                    source,
                })
            }
            Err(source) => Err(Error::MoveFile {
                from: self.from,
                to: self.to,
                source,
            }),
        }
    }

    fn validate_source_is_file(&self) -> Result<()> {
        let metadata = std::fs::metadata(&self.from).map_err(|source| Error::MoveFile {
            from: self.from.clone(),
            to: self.to.clone(),
            source,
        })?;

        if metadata.is_file() {
            Ok(())
        } else {
            let source = std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "source path is not a file",
            );
            Err(Error::MoveFile {
                from: self.from.clone(),
                to: self.to.clone(),
                source,
            })
        }
    }

    fn copy_delete(&self) -> std::io::Result<()> {
        std::fs::copy(&self.from, &self.to)?;
        std::fs::remove_file(&self.from)
    }

    fn parent(&self) -> Result<&Path> {
        self.to.parent().ok_or_else(|| Error::MissingParent {
            path: self.to.clone(),
        })
    }

    fn backup_path(&self) -> PathBuf {
        let mut path = OsString::from(self.to.as_os_str());
        path.push(&self.backup_suffix);
        PathBuf::from(path)
    }
}
