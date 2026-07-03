use assert_fs::prelude::*;
use steady_fs::{copy_file, Error};

#[test]
fn copy_file_copies_missing_destination() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("config.toml");
    let to = temp.child("backup/config.toml");
    std::fs::create_dir_all(to.path().parent().unwrap()).unwrap();
    from.write_str("hello").unwrap();

    let bytes = copy_file(from.path(), to.path()).run().unwrap();

    assert_eq!(bytes, 5);
    from.assert("hello");
    to.assert("hello");
}

#[test]
fn copy_file_creates_parent_dirs_when_enabled() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("config.toml");
    let to = temp.child("backup/config.toml");
    from.write_str("hello").unwrap();

    copy_file(from.path(), to.path())
        .create_parent_dirs(true)
        .run()
        .unwrap();

    to.assert("hello");
}

#[test]
fn copy_file_fails_when_parent_dirs_are_missing() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("config.toml");
    let to = temp.child("backup/config.toml");
    from.write_str("hello").unwrap();

    let err = copy_file(from.path(), to.path()).run().unwrap_err();
    let msg = err.to_string();

    match err {
        Error::CopyFile { from, to, .. } => {
            assert!(from.ends_with("config.toml"));
            assert!(to.ends_with("backup/config.toml"));
        }
        other => panic!("expected CopyFile, got {other:?}"),
    }

    assert!(msg.contains("failed to copy file"));
    assert!(msg.contains("config.toml"));
    to.assert(predicates::path::missing());
}

#[test]
fn copy_file_overwrites_existing_file_when_enabled() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("config.toml");
    let to = temp.child("backup.toml");
    from.write_str("new").unwrap();
    to.write_str("old").unwrap();

    copy_file(from.path(), to.path())
        .overwrite(true)
        .run()
        .unwrap();

    to.assert("new");
}

#[test]
fn copy_file_refuses_to_overwrite_existing_file_when_disabled() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("config.toml");
    let to = temp.child("backup.toml");
    from.write_str("new").unwrap();
    to.write_str("old").unwrap();

    let err = copy_file(from.path(), to.path())
        .overwrite(false)
        .run()
        .unwrap_err();

    match err {
        Error::AlreadyExists { path } => {
            assert!(path.ends_with("backup.toml"));
        }
        other => panic!("expected AlreadyExists, got {other:?}"),
    }

    to.assert("old");
}

#[test]
fn copy_file_backs_up_existing_file_with_default_suffix() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("config.toml");
    let to = temp.child("backup.toml");
    let backup = temp.child("backup.toml.bak");
    from.write_str("new").unwrap();
    to.write_str("old").unwrap();

    copy_file(from.path(), to.path())
        .backup_existing(true)
        .run()
        .unwrap();

    to.assert("new");
    backup.assert("old");
}

#[test]
fn copy_file_backs_up_existing_file_with_custom_suffix() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("config.toml");
    let to = temp.child("backup.toml");
    let backup = temp.child("backup.toml.prev");
    from.write_str("new").unwrap();
    to.write_str("old").unwrap();

    copy_file(from.path(), to.path())
        .backup_existing(true)
        .backup_suffix(".prev")
        .run()
        .unwrap();

    to.assert("new");
    backup.assert("old");
}

#[test]
fn copy_file_dry_run_does_not_copy_create_parent_dirs_or_backup() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("config.toml");
    let to = temp.child("backup/config.toml");
    let backup = temp.child("backup/config.toml.bak");
    from.write_str("new").unwrap();

    let bytes = copy_file(from.path(), to.path())
        .create_parent_dirs(true)
        .backup_existing(true)
        .dry_run(true)
        .run()
        .unwrap();

    assert_eq!(bytes, 0);
    from.assert("new");
    to.assert(predicates::path::missing());
    backup.assert(predicates::path::missing());
}

#[test]
fn copy_file_dry_run_still_refuses_overwrite_when_disabled() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("config.toml");
    let to = temp.child("backup.toml");
    from.write_str("new").unwrap();
    to.write_str("old").unwrap();

    let err = copy_file(from.path(), to.path())
        .overwrite(false)
        .dry_run(true)
        .run()
        .unwrap_err();

    match err {
        Error::AlreadyExists { path } => {
            assert!(path.ends_with("backup.toml"));
        }
        other => panic!("expected AlreadyExists, got {other:?}"),
    }

    to.assert("old");
}

#[test]
fn copy_file_returns_path_aware_error_when_source_is_missing() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("missing.toml");
    let to = temp.child("backup.toml");

    let err = copy_file(from.path(), to.path()).run().unwrap_err();
    let msg = err.to_string();

    match err {
        Error::CopyFile { from, to, .. } => {
            assert!(from.ends_with("missing.toml"));
            assert!(to.ends_with("backup.toml"));
        }
        other => panic!("expected CopyFile, got {other:?}"),
    }

    assert!(msg.contains("failed to copy file"));
    assert!(msg.contains("missing.toml"));
    assert!(msg.contains("backup.toml"));
}

#[test]
fn copy_file_returns_path_aware_backup_error() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("config.toml");
    let to = temp.child("backup.toml");
    let backup = temp.child("backup.toml.bak");
    from.write_str("new").unwrap();
    to.write_str("old").unwrap();
    backup.create_dir_all().unwrap();

    let err = copy_file(from.path(), to.path())
        .backup_existing(true)
        .run()
        .unwrap_err();
    let msg = err.to_string();

    match err {
        Error::Backup {
            path, backup_path, ..
        } => {
            assert!(path.ends_with("backup.toml"));
            assert!(backup_path.ends_with("backup.toml.bak"));
        }
        other => panic!("expected Backup, got {other:?}"),
    }

    assert!(msg.contains("failed to backup existing file"));
    assert!(msg.contains("backup.toml"));
    assert!(msg.contains("backup.toml.bak"));
    to.assert("old");
}
