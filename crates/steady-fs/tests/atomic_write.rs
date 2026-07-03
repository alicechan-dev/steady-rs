use assert_fs::prelude::*;
use steady_fs::{atomic_write, Error};

#[test]
fn atomic_write_writes_missing_file() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("report.txt");

    atomic_write(file.path(), "hello").write().unwrap();

    file.assert("hello");
}

#[test]
fn atomic_write_creates_parent_dirs_when_enabled() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("out/reports/report.txt");

    atomic_write(file.path(), "hello")
        .create_parent_dirs(true)
        .write()
        .unwrap();

    file.assert("hello");
}

#[test]
fn atomic_write_fails_when_parent_dirs_are_missing() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("out/report.txt");

    let err = atomic_write(file.path(), "hello").write().unwrap_err();
    let msg = err.to_string();

    assert!(msg.contains("failed to write temporary file"));
    assert!(msg.contains("report.txt"));
    file.assert(predicates::path::missing());
}

#[test]
fn atomic_write_overwrites_existing_file_when_enabled() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("report.txt");
    file.write_str("old").unwrap();

    atomic_write(file.path(), "new")
        .overwrite(true)
        .write()
        .unwrap();

    file.assert("new");
}

#[test]
fn atomic_write_refuses_to_overwrite_existing_file_when_disabled() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("report.txt");
    file.write_str("old").unwrap();

    let err = atomic_write(file.path(), "new")
        .overwrite(false)
        .write()
        .unwrap_err();

    match err {
        Error::AlreadyExists { path } => {
            assert!(path.ends_with("report.txt"));
        }
        other => panic!("expected AlreadyExists, got {other:?}"),
    }

    file.assert("old");
}

#[test]
fn atomic_write_error_mentions_target_path_when_overwrite_refused() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("report.txt");
    file.write_str("old").unwrap();

    let err = atomic_write(file.path(), "new")
        .overwrite(false)
        .write()
        .unwrap_err();
    let msg = err.to_string();

    assert!(msg.contains("refusing to overwrite existing file"));
    assert!(msg.contains("report.txt"));
}

#[test]
fn atomic_write_backs_up_existing_file_with_default_suffix() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("report.txt");
    let backup = temp.child("report.txt.bak");
    file.write_str("old").unwrap();

    atomic_write(file.path(), "new")
        .backup_existing(true)
        .write()
        .unwrap();

    file.assert("new");
    backup.assert("old");
}

#[test]
fn atomic_write_backs_up_existing_file_with_custom_suffix() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("report.txt");
    let backup = temp.child("report.txt.prev");
    file.write_str("old").unwrap();

    atomic_write(file.path(), "new")
        .backup_existing(true)
        .backup_suffix(".prev")
        .write()
        .unwrap();

    file.assert("new");
    backup.assert("old");
}

#[test]
fn atomic_write_dry_run_does_not_create_missing_file() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("report.txt");

    atomic_write(file.path(), "new")
        .dry_run(true)
        .write()
        .unwrap();

    file.assert(predicates::path::missing());
}

#[test]
fn atomic_write_dry_run_does_not_overwrite_existing_file_or_create_backup() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("report.txt");
    let backup = temp.child("report.txt.bak");
    file.write_str("old").unwrap();

    atomic_write(file.path(), "new")
        .backup_existing(true)
        .dry_run(true)
        .write()
        .unwrap();

    file.assert("old");
    backup.assert(predicates::path::missing());
}

#[test]
fn atomic_write_dry_run_does_not_create_parent_dirs() {
    let temp = assert_fs::TempDir::new().unwrap();
    let dir = temp.child("out");
    let file = dir.child("report.txt");

    atomic_write(file.path(), "new")
        .create_parent_dirs(true)
        .dry_run(true)
        .write()
        .unwrap();

    dir.assert(predicates::path::missing());
    file.assert(predicates::path::missing());
}

#[test]
fn atomic_write_returns_path_aware_backup_error() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("report.txt");
    let backup = temp.child("report.txt.bak");
    file.write_str("old").unwrap();
    backup.create_dir_all().unwrap();

    let err = atomic_write(file.path(), "new")
        .backup_existing(true)
        .write()
        .unwrap_err();
    let msg = err.to_string();

    match err {
        Error::Backup {
            path, backup_path, ..
        } => {
            assert!(path.ends_with("report.txt"));
            assert!(backup_path.ends_with("report.txt.bak"));
        }
        other => panic!("expected Backup, got {other:?}"),
    }

    assert!(msg.contains("failed to backup existing file"));
    assert!(msg.contains("report.txt"));
    assert!(msg.contains("report.txt.bak"));
    file.assert("old");
}
