use steady_fs::prelude::*;

fn main() -> steady_fs::Result<()> {
    let temp = tempfile::tempdir().expect("create temporary example directory");
    std::env::set_current_dir(temp.path()).expect("enter temporary example directory");

    atomic_write("config.toml", "name = \"demo\"\n").write()?;

    copy_file("config.toml", "backup/config.toml")
        .create_parent_dirs(true)
        .overwrite(false)
        .run()?;

    copy_file("config.toml", "backup/config.toml")
        .overwrite(true)
        .run()?;

    Ok(())
}
