# Atomic Write Design

`steady-fs` should provide an `atomic_write` helper for practical file writes in CLI tools and developer apps.

The crate should be careful about its language. It should say that it writes through a temporary file in the destination directory and persists it to the final path. It should avoid overpromising perfect crash-proof behavior on every filesystem.

## Intended algorithm

1. Take the destination path.
2. Find the parent directory.
3. Optionally create parent directories.
4. Create a temporary file in the same parent directory.
5. Write all bytes to the temporary file.
6. Flush and sync the temporary file.
7. Optionally backup the existing target file.
8. Persist or rename the temporary file to the final destination.
9. Return path-aware errors if anything fails.

## Same-directory temporary files

The temporary file must be created in the same directory as the destination file.

Rename is only atomic within the same filesystem or volume. Creating the temporary file beside the destination avoids crossing filesystem boundaries during the final persist step.

## Platform notes

Unix-like systems usually allow atomic replacement through rename.

Windows replacement behavior is trickier when the destination exists. The implementation should be tested carefully and documented honestly.

The crate should avoid claims like "always crash-proof" or "perfectly atomic on every platform." The useful promise is narrower: write through a temp file in the destination directory and persist it to the final path, while surfacing clear errors when the platform refuses an operation.

## Example

```rust
use steady_fs::prelude::*;

atomic_write("config.toml", data)
    .create_parent_dirs(true)
    .backup_existing(true)
    .write()?;
```

## Test plan

- Writes file successfully.
- Creates parent directories when enabled.
- Fails when parent directories are missing and parent creation is disabled.
- Overwrites existing file when `overwrite` is true.
- Refuses overwrite when `overwrite` is false.
- Creates backup when requested.
- Dry-run does not write.
- Error includes target path.
