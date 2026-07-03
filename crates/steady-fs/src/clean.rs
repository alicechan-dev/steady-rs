use std::path::{Component, Path, PathBuf};

use crate::{Error, Result};

pub fn clean_dir<P>(path: P) -> CleanDir
where
    P: AsRef<Path>,
{
    CleanDir {
        path: path.as_ref().to_path_buf(),
        dry_run: false,
    }
}

#[derive(Debug, Clone)]
pub struct CleanDir {
    path: PathBuf,
    dry_run: bool,
}

impl CleanDir {
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    pub fn run(self) -> Result<()> {
        if is_dangerous_path(&self.path) {
            return Err(Error::DangerousCleanDir { path: self.path });
        }

        if self.dry_run {
            return Ok(());
        }

        let entries = std::fs::read_dir(&self.path).map_err(|source| Error::CleanDir {
            path: self.path.clone(),
            source,
        })?;

        for entry in entries {
            let entry = entry.map_err(|source| Error::CleanDir {
                path: self.path.clone(),
                source,
            })?;
            let entry_path = entry.path();
            let file_type = entry.file_type().map_err(|source| Error::CleanDir {
                path: self.path.clone(),
                source,
            })?;

            if file_type.is_dir() {
                std::fs::remove_dir_all(&entry_path).map_err(|source| Error::CleanDir {
                    path: self.path.clone(),
                    source,
                })?;
            } else {
                std::fs::remove_file(&entry_path).map_err(|source| Error::CleanDir {
                    path: self.path.clone(),
                    source,
                })?;
            }
        }

        Ok(())
    }
}

fn is_dangerous_path(path: &Path) -> bool {
    if path.as_os_str().is_empty() {
        return true;
    }

    let mut has_normal = false;

    for component in path.components() {
        match component {
            Component::Normal(_) => has_normal = true,
            Component::ParentDir | Component::CurDir => return true,
            Component::RootDir | Component::Prefix(_) => {}
        }
    }

    !has_normal
}
