# steady-fs Design

## Purpose

`steady-fs` is a small Rust crate for common filesystem workflows in CLI tools and developer apps.

It exists for operations that are simple in concept but easy to implement inconsistently: writing files safely, creating parent directories, copying and moving files with explicit overwrite behavior, cleaning directories, and reporting errors with useful path context.

## Target users

`steady-fs` is aimed at:

- CLI tools
- build scripts
- developer tools
- small desktop and developer apps
- code generators
- project scaffolding tools

## Design philosophy

- Boring but reliable.
- Small public API.
- Sane defaults.
- Cross-platform behavior where practical.
- Clear path-aware errors.
- No framework shape.
- No magic soup.
- Good interop with `std::fs` and existing crates.

The crate should make common workflows easier without pretending the filesystem is simpler than it is.

## MVP feature list

- `atomic_write`
- `ensure_dir`
- `clean_dir`
- `copy_file`
- `move_file`
- backup-before-overwrite
- dry-run mode
- path-aware errors

## Future feature list

- lock file helpers
- safer move behavior across filesystems
- path normalization helpers
- safe path display helpers
- optional `camino` support
- more platform-specific tests

## Non-goals

- Not a framework.
- Not async-first.
- Not a virtual filesystem.
- Not a config library.
- Not a directory watcher.
- Not a globbing or walking crate.
- Not a replacement for `std::fs`.

## Example workflows

Write a generated report:

```rust
use steady_fs::prelude::*;

atomic_write("output/report.txt", data)
    .create_parent_dirs(true)
    .backup_existing(true)
    .write()?;
```

Prepare and clean a cache directory:

```rust
use steady_fs::prelude::*;

ensure_dir("cache")?;
clean_dir("cache/tmp").run()?;
```

Copy a config file without silently overwriting:

```rust
use steady_fs::prelude::*;

copy_file("config.toml", "backup/config.toml")
    .create_parent_dirs(true)
    .overwrite(false)
    .run()?;
```

Move a downloaded file into an archive:

```rust
use steady_fs::prelude::*;

move_file("download.tmp", "archive/download.txt")
    .create_parent_dirs(true)
    .overwrite(false)
    .run()?;
```

## Why this crate should exist

Most Rust projects can use `std::fs` directly, and many should.

`steady-fs` is for the layer above that: tools where file operations are part of the user experience. In those tools, a missing parent directory, a partial write, an accidental overwrite, or an error without a path can waste time.

The crate should provide a small set of explicit helpers that make those workflows consistent without pulling the project into a large abstraction.
