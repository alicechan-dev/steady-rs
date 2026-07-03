use std::path::{Path, PathBuf};

pub(crate) fn backup_path_for(path: &Path, suffix: &str) -> PathBuf {
    let mut os = path.as_os_str().to_os_string();
    os.push(suffix);
    PathBuf::from(os)
}
