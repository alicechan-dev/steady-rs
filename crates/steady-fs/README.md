# steady-fs

Small, boring, reliable filesystem helpers for Rust CLI tools and developer apps.

`steady-fs` does not replace `std::fs`. It wraps common error-prone filesystem workflows into safer, clearer helpers.

This crate is currently in the design and documentation phase. The examples below describe the intended API.

## Before

```rust
std::fs::write("output/report.txt", data)?;
```

Problems:

- parent directory may not exist
- crash can leave partial file
- overwrite behavior is easy to forget
- error may lack useful path context

## After

```rust
use steady_fs::prelude::*;

atomic_write("output/report.txt", data)
    .create_parent_dirs(true)
    .backup_existing(true)
    .write()?;
```

## Intended MVP API

```rust
use steady_fs::prelude::*;

ensure_dir("cache")?;

clean_dir("target/tmp").run()?;

atomic_write("output/report.txt", data)
    .create_parent_dirs(true)
    .backup_existing(true)
    .write()?;

copy_file("config.toml", "backup/config.toml")
    .create_parent_dirs(true)
    .overwrite(false)
    .run()?;

move_file("download.tmp", "archive/download.txt")
    .create_parent_dirs(true)
    .overwrite(false)
    .run()?;
```

## Intended features

- `atomic_write`
- `ensure_dir`
- `clean_dir`
- `copy_file`
- `move_file`
- backup-before-overwrite
- dry-run mode
- path-aware errors

## Non-goals

- Not a framework.
- Not async-first.
- Not a virtual filesystem.
- Not a config library.
- Not a directory watcher.
- Not a globbing or walking crate.
- Not a replacement for `std::fs`.

## Status

Design first. Implementation comes after the API shape, error model, and platform behavior are clear.
