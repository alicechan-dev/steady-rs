# steady-rs

`steady-rs` is a Rust workspace for small, practical helper crates aimed at CLI tools and developer apps.

The first crate is `steady-fs`: boring, reliable filesystem helpers for common file operations that are easy to get almost right and annoying to debug when they fail.

This project is currently in the design and documentation phase. The public API shown here is a proposal, not a finished implementation.

## Philosophy

- Keep the API small.
- Prefer sane defaults over configuration soup.
- Work well with `std::fs` and existing Rust crates.
- Make filesystem errors readable and path-aware.
- Keep behavior cross-platform where practical.
- Avoid magic behavior.
- Be boring in the good way.

## First crate: steady-fs

`steady-fs` does not try to replace `std::fs`. It wraps common error-prone filesystem workflows into clearer helpers:

- atomic-style file writes through a temp file
- optional parent directory creation
- backup-before-overwrite helpers
- safe copy and move helpers
- directory ensure and clean helpers
- dry-run support for CLI tools
- path-aware errors

## Before and after

Before:

```rust
std::fs::write("output/report.txt", data)?;
```

This is fine for simple cases, but in CLI tools it often leaves policy decisions implicit: should parent directories be created, should existing files be overwritten, and what path should be shown when something fails?

After:

```rust
use steady_fs::prelude::*;

atomic_write("output/report.txt", data)
    .create_parent_dirs(true)
    .backup_existing(true)
    .write()?;
```

The goal is not to hide filesystem behavior. The goal is to make common intent explicit.

## Intended workspace structure

```txt
steady-rs/
  Cargo.toml
  README.md
  docs/
    steady-fs.md
    roadmap.md
    api-design.md
    error-design.md
    atomic-write.md
  crates/
    steady-fs/
      Cargo.toml
      README.md
      src/
        lib.rs
```

## Planned crates

- `steady-fs`: filesystem helpers for CLI tools and developer apps
- `steady`: possible umbrella crate for selected helpers
- `cargo-steady`: possible helper CLI later

## Status

Design and documentation first. Implementation is intentionally deferred until the API shape, error model, and platform promises are boring enough to trust.
