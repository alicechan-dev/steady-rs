# API Design

This document sketches the intended public API for `steady-fs`.

The API is a draft. It should stay small, readable, and ordinary.

## Proposed functions

```rust
atomic_write(path, data).write()?;
ensure_dir(path)?;
clean_dir(path).run()?;
copy_file(from, to).run()?;
move_file(from, to).run()?;
```

## Atomic write

```rust
use steady_fs::prelude::*;

atomic_write("out/report.txt", data)
    .create_parent_dirs(true)
    .backup_existing(true)
    .overwrite(true)
    .dry_run(false)
    .write()?;
```

`atomic_write` should write through a temporary file in the destination directory, then persist it to the final path.

The crate should avoid exaggerated guarantees. The intended promise is practical: write through a temp file in the destination directory and persist it to the final path, with path-aware errors when something fails.

## Ensure directory

```rust
use steady_fs::prelude::*;

ensure_dir("cache")?;
```

For the MVP, `ensure_dir` should be a plain function because the common operation has no obvious required options.

## Clean directory

```rust
use steady_fs::prelude::*;

clean_dir("target/tmp").run()?;
```

`clean_dir` should remove the contents of a directory while applying basic safety checks against dangerous paths.

## Copy file

```rust
use steady_fs::prelude::*;

copy_file("a.txt", "b.txt")
    .create_parent_dirs(true)
    .overwrite(false)
    .backup_existing(false)
    .dry_run(false)
    .run()?;
```

## Move file

```rust
use steady_fs::prelude::*;

move_file("a.txt", "archive/a.txt")
    .create_parent_dirs(true)
    .overwrite(false)
    .backup_existing(false)
    .dry_run(false)
    .run()?;
```

## Naming rules

- Top-level functions use clear verbs.
- Builders use plain boolean options.
- Final methods are `.write()` for writing and `.run()` for general operations.
- Avoid clever names.
- Avoid magic behavior.

## Builder options

Common builder options should be named plainly:

- `create_parent_dirs(bool)`
- `overwrite(bool)`
- `backup_existing(bool)`
- `dry_run(bool)`

Defaults should be conservative and documented on each builder before the API is stabilized.

## Prelude

The crate should provide:

```rust
use steady_fs::prelude::*;
```

The prelude should re-export the main operations and result types once they exist.

## Open design question

Should `ensure_dir("cache")` be a plain function returning `Result<()>`, or a builder like `ensure_dir("cache").run()`?

Recommendation for now:

- Use `ensure_dir(path)?` as a simple function.
- Use builders only where options are needed.
