# Roadmap

## v0.1 - Filesystem MVP

- workspace setup
- `steady-fs` crate
- path-aware error type
- `ensure_dir`
- `atomic_write`
- `clean_dir`
- `copy_file`
- `move_file`
- backup-before-overwrite
- dry-run mode
- README examples
- integration tests

## v0.2 - Lock files and safer moves

- lock file helper
- cross-device move fallback
- better Windows behavior
- more tests on Windows, macOS, and Linux

## v0.3 - Path helpers

- path normalization helpers
- safe display helpers
- maybe optional `camino` support

## Later

- umbrella `steady` crate
- `cargo-steady` helper CLI
- templates and examples for CLI apps
