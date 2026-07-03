use steady_fs::prelude::*;

fn main() -> steady_fs::Result<()> {
    let temp = tempfile::tempdir().expect("create temporary example directory");
    std::env::set_current_dir(temp.path()).expect("enter temporary example directory");

    atomic_write("output/report.txt", "first report\n")
        .create_parent_dirs(true)
        .write()?;

    atomic_write("output/report.txt", "updated report\n")
        .backup_existing(true)
        .write()?;

    Ok(())
}
