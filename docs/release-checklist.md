# Release checklist

## Before release

- Run `cargo fmt --check`
- Run `cargo clippy --all-targets --all-features -- -D warnings`
- Run `cargo test`
- Run `cargo test --doc`
- Run `cargo check --examples`
- Run `cargo doc --no-deps`
- Run `cargo package -p steady-fs --allow-dirty`
- Review `crates/steady-fs/README.md`
- Review package metadata in `crates/steady-fs/Cargo.toml`
- Confirm repository URL is correct
- Confirm license files exist
- Confirm changelog is updated
- Confirm examples compile

## Publish dry-run

- Run `cargo publish -p steady-fs --dry-run`

## Publish

- Run `cargo publish -p steady-fs`

## After publish

- Check the crate page on crates.io
- Check the generated documentation on docs.rs
- Create a Git tag, for example `steady-fs-v0.1.0`

## After release

- Confirm crates.io page is live.
- Confirm docs.rs build is successful.
- Push Git tag.
- Start next development version when needed.
