use std::io::Write;
use std::path::{Path, PathBuf};

use crate::{Error, Result};

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
    }
}

#[derive(Debug, Clone)]
pub struct AtomicWrite {
    path: PathBuf,
    data: Vec<u8>,
    create_parent_dirs: bool,
    overwrite: bool,
}

impl AtomicWrite {
    pub fn create_parent_dirs(mut self, create_parent_dirs: bool) -> Self {
        self.create_parent_dirs = create_parent_dirs;
        self
    }

    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    pub fn write(self) -> Result<()> {
        if !self.overwrite && self.path.exists() {
            return Err(Error::AlreadyExists { path: self.path });
        }

        let parent = self.parent()?;

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
}
