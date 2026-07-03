# steady-fs

Small, boring, reliable filesystem helpers for Rust CLI tools and developer apps.

`steady-fs` does not replace `std::fs`. It wraps common error-prone filesystem workflows into safer, clearer helpers.

The crate is intentionally small. It currently focuses on common file and directory operations used by CLI tools, build scripts, code generators, and developer apps.

For more task-oriented snippets, see the [steady-fs cookbook](../../docs/steady-fs-cookbook.md). Runnable examples live in [`examples/`](examples/).

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

## API

```rust
use steady_fs::prelude::*;

ensure_dir("cache")?;

clean_dir("target/tmp").run()?;

atomic_write("output/report.txt", data)
    .create_parent_dirs(true)
    .backup_existing(true)
    .backup_suffix(".bak")
    .dry_run(false)
    .write()?;

copy_file("config.toml", "backup/config.toml")
    .create_parent_dirs(true)
    .overwrite(false)
    .backup_existing(false)
    .dry_run(false)
    .run()?;

move_file("download.tmp", "archive/download.txt")
    .create_parent_dirs(true)
    .overwrite(false)
    .backup_existing(false)
    .dry_run(false)
    .fallback_copy_delete(true)
    .run()?;
```

## Features

- `atomic_write`
- `ensure_dir`
- `clean_dir`
- `copy_file`
- `move_file`
- backup-before-overwrite
- dry-run mode
- path-aware errors

## Examples

Run the example programs with Cargo:

```bash
cargo run -p steady-fs --example atomic_write
cargo run -p steady-fs --example clean_dir
cargo run -p steady-fs --example copy_file
cargo run -p steady-fs --example move_file
```

## Notes

`atomic_write` writes through a temporary file in the destination directory and persists it to the final path. It avoids promising perfect crash-proof behavior on every filesystem.

`clean_dir` removes the contents of a directory while leaving the directory itself in place. It refuses obviously dangerous paths such as roots, `.`, and paths containing `..`.

`copy_file` and `move_file` operate on files only. They do not recursively copy or move directories.

## Non-goals

- Not a framework.
- Not async-first.
- Not a virtual filesystem.
- Not a config library.
- Not a directory watcher.
- Not a globbing or walking crate.
- Not a replacement for `std::fs`.

## Status

Early implementation. The API is useful, but still pre-1.0 and expected to stay conservative.

## Release status

`steady-fs` is currently preparing for its first `0.1.0` release.

The API is usable for experimentation, but the crate should still be treated as early-stage until published and tested in real projects.
