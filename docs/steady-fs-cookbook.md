# steady-fs Cookbook

This cookbook shows small, practical `steady-fs` workflows.

The examples use `steady_fs::prelude::*` and return `steady_fs::Result<()>`.

## Write a generated file

```rust
use steady_fs::prelude::*;

fn main() -> steady_fs::Result<()> {
    atomic_write("output/report.txt", "hello\n")
        .create_parent_dirs(true)
        .write()?;

    Ok(())
}
```

Use this when a tool writes a generated file and should create missing parent directories.

## Back up a file before replacing it

```rust
use steady_fs::prelude::*;

fn main() -> steady_fs::Result<()> {
    atomic_write("output/report.txt", "updated\n")
        .create_parent_dirs(true)
        .backup_existing(true)
        .backup_suffix(".bak")
        .write()?;

    Ok(())
}
```

Backups append the suffix to the full destination path. For example, `report.txt` becomes `report.txt.bak`.

## Clean a temporary directory

```rust
use steady_fs::prelude::*;

fn main() -> steady_fs::Result<()> {
    clean_dir("target/tmp").run()?;

    Ok(())
}
```

`clean_dir` removes the contents of the directory and leaves the directory itself in place.

## Preview a clean operation

```rust
use steady_fs::prelude::*;

fn main() -> steady_fs::Result<()> {
    clean_dir("target/tmp").dry_run(true).run()?;

    Ok(())
}
```

Dry-run validates the operation but does not remove files.

## Copy a file without overwriting

```rust
use steady_fs::prelude::*;

fn main() -> steady_fs::Result<()> {
    copy_file("config.toml", "backup/config.toml")
        .create_parent_dirs(true)
        .overwrite(false)
        .run()?;

    Ok(())
}
```

This is useful for CLI tools that should refuse to replace user-owned files unless explicitly told to.

## Copy a file and allow replacement

```rust
use steady_fs::prelude::*;

fn main() -> steady_fs::Result<()> {
    copy_file("config.toml", "backup/config.toml")
        .create_parent_dirs(true)
        .overwrite(true)
        .backup_existing(true)
        .run()?;

    Ok(())
}
```

When `backup_existing(true)` is enabled, an existing destination is copied to a backup path before replacement.

## Move a downloaded file into an archive

```rust
use steady_fs::prelude::*;

fn main() -> steady_fs::Result<()> {
    move_file("download.tmp", "archive/download.txt")
        .create_parent_dirs(true)
        .overwrite(false)
        .fallback_copy_delete(true)
        .run()?;

    Ok(())
}
```

`fallback_copy_delete(true)` allows the helper to fall back to copy-then-delete if a rename fails, such as across filesystem boundaries.

## Example Files

Runnable examples live under `crates/steady-fs/examples/`:

- `atomic_write.rs`
- `clean_dir.rs`
- `copy_file.rs`
- `move_file.rs`

Run one with:

```bash
cargo run -p steady-fs --example atomic_write
```
