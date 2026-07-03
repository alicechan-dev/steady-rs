use steady_fs::prelude::*;

fn main() -> steady_fs::Result<()> {
    let temp = tempfile::tempdir().expect("create temporary example directory");
    std::env::set_current_dir(temp.path()).expect("enter temporary example directory");

    atomic_write("target/tmp/cache.txt", "cached data\n")
        .create_parent_dirs(true)
        .write()?;

    clean_dir("target/tmp").dry_run(true).run()?;
    clean_dir("target/tmp").run()?;

    Ok(())
}
