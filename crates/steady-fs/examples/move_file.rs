use steady_fs::prelude::*;

fn main() -> steady_fs::Result<()> {
    let temp = tempfile::tempdir().expect("create temporary example directory");
    std::env::set_current_dir(temp.path()).expect("enter temporary example directory");

    atomic_write("download.tmp", "downloaded data\n").write()?;

    move_file("download.tmp", "archive/download.txt")
        .create_parent_dirs(true)
        .fallback_copy_delete(true)
        .run()?;

    Ok(())
}
