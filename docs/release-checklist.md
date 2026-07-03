# Release checklist

## Before release

- Run `cargo fmt --check`
- Run `cargo clippy --all-targets --all-features -- -D warnings`
- Run `cargo test`
- Run `cargo test --doc`
- Run `cargo check --examples`
- Run `cargo package -p steady-fs --allow-dirty`
- Review `crates/steady-fs/README.md`
- Review package metadata in `crates/steady-fs/Cargo.toml`
- Confirm repository URL is correct
- Confirm license files exist
- Confirm changelog is updated

## Publish

- Run `cargo publish -p steady-fs --dry-run`
- Run `cargo publish -p steady-fs`