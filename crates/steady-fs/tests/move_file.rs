use assert_fs::prelude::*;
use steady_fs::{move_file, Error};

#[test]
fn move_file_moves_missing_destination() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("download.tmp");
    let to = temp.child("archive/download.txt");
    std::fs::create_dir_all(to.path().parent().unwrap()).unwrap();
    from.write_str("hello").unwrap();

    move_file(from.path(), to.path()).run().unwrap();

    from.assert(predicates::path::missing());
    to.assert("hello");
}

#[test]
fn move_file_creates_parent_dirs_when_enabled() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("download.tmp");
    let to = temp.child("archive/download.txt");
    from.write_str("hello").unwrap();

    move_file(from.path(), to.path())
        .create_parent_dirs(true)
        .run()
        .unwrap();

    from.assert(predicates::path::missing());
    to.assert("hello");
}

#[test]
fn move_file_fails_when_parent_dirs_are_missing() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("download.tmp");
    let to = temp.child("archive/download.txt");
    from.write_str("hello").unwrap();

    let err = move_file(from.path(), to.path()).run().unwrap_err();
    let msg = err.to_string();

    match err {
        Error::MoveFile { from, to, .. } => {
            assert!(from.ends_with("download.tmp"));
            assert!(to.ends_with("archive/download.txt"));
        }
        other => panic!("expected MoveFile, got {other:?}"),
    }

    assert!(msg.contains("failed to move file"));
    assert!(msg.contains("download.tmp"));
    assert!(msg.contains("archive/download.txt"));
    from.assert("hello");
    to.assert(predicates::path::missing());
}

#[test]
fn move_file_overwrites_existing_file_when_enabled() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("download.tmp");
    let to = temp.child("archive.txt");
    from.write_str("new").unwrap();
    to.write_str("old").unwrap();

    move_file(from.path(), to.path())
        .overwrite(true)
        .run()
        .unwrap();

    from.assert(predicates::path::missing());
    to.assert("new");
}

#[test]
fn move_file_refuses_to_overwrite_existing_file_when_disabled() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("download.tmp");
    let to = temp.child("archive.txt");
    from.write_str("new").unwrap();
    to.write_str("old").unwrap();

    let err = move_file(from.path(), to.path())
        .overwrite(false)
        .run()
        .unwrap_err();

    match err {
        Error::AlreadyExists { path } => {
            assert!(path.ends_with("archive.txt"));
        }
        other => panic!("expected AlreadyExists, got {other:?}"),
    }

    from.assert("new");
    to.assert("old");
}

#[test]
fn move_file_backs_up_existing_file_with_default_suffix() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("download.tmp");
    let to = temp.child("archive.txt");
    let backup = temp.child("archive.txt.bak");
    from.write_str("new").unwrap();
    to.write_str("old").unwrap();

    move_file(from.path(), to.path())
        .backup_existing(true)
        .run()
        .unwrap();

    from.assert(predicates::path::missing());
    to.assert("new");
    backup.assert("old");
}

#[test]
fn move_file_backs_up_existing_file_with_custom_suffix() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("download.tmp");
    let to = temp.child("archive.txt");
    let backup = temp.child("archive.txt.prev");
    from.write_str("new").unwrap();
    to.write_str("old").unwrap();

    move_file(from.path(), to.path())
        .backup_existing(true)
        .backup_suffix(".prev")
        .run()
        .unwrap();

    from.assert(predicates::path::missing());
    to.assert("new");
    backup.assert("old");
}

#[test]
fn move_file_dry_run_does_not_move_create_parent_dirs_or_backup() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("download.tmp");
    let to = temp.child("archive/download.txt");
    let backup = temp.child("archive/download.txt.bak");
    from.write_str("new").unwrap();

    move_file(from.path(), to.path())
        .create_parent_dirs(true)
        .backup_existing(true)
        .dry_run(true)
        .run()
        .unwrap();

    from.assert("new");
    to.assert(predicates::path::missing());
    backup.assert(predicates::path::missing());
}

#[test]
fn move_file_dry_run_still_refuses_overwrite_when_disabled() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("download.tmp");
    let to = temp.child("archive.txt");
    from.write_str("new").unwrap();
    to.write_str("old").unwrap();

    let err = move_file(from.path(), to.path())
        .overwrite(false)
        .dry_run(true)
        .run()
        .unwrap_err();

    match err {
        Error::AlreadyExists { path } => {
            assert!(path.ends_with("archive.txt"));
        }
        other => panic!("expected AlreadyExists, got {other:?}"),
    }

    from.assert("new");
    to.assert("old");
}

#[test]
fn move_file_returns_path_aware_error_when_source_is_missing() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("missing.tmp");
    let to = temp.child("archive.txt");

    let err = move_file(from.path(), to.path()).run().unwrap_err();
    let msg = err.to_string();

    match err {
        Error::MoveFile { from, to, .. } => {
            assert!(from.ends_with("missing.tmp"));
            assert!(to.ends_with("archive.txt"));
        }
        other => panic!("expected MoveFile, got {other:?}"),
    }

    assert!(msg.contains("failed to move file"));
    assert!(msg.contains("missing.tmp"));
    assert!(msg.contains("archive.txt"));
}

#[test]
fn move_file_returns_path_aware_backup_error() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("download.tmp");
    let to = temp.child("archive.txt");
    let backup = temp.child("archive.txt.bak");
    from.write_str("new").unwrap();
    to.write_str("old").unwrap();
    backup.create_dir_all().unwrap();

    let err = move_file(from.path(), to.path())
        .backup_existing(true)
        .run()
        .unwrap_err();
    let msg = err.to_string();

    match err {
        Error::Backup {
            path, backup_path, ..
        } => {
            assert!(path.ends_with("archive.txt"));
            assert!(backup_path.ends_with("archive.txt.bak"));
        }
        other => panic!("expected Backup, got {other:?}"),
    }

    assert!(msg.contains("failed to backup existing file"));
    assert!(msg.contains("archive.txt"));
    assert!(msg.contains("archive.txt.bak"));
    from.assert("new");
    to.assert("old");
}

#[test]
fn move_file_refuses_directory_sources() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("download-dir");
    let to = temp.child("archive.txt");
    from.create_dir_all().unwrap();

    let err = move_file(from.path(), to.path())
        .fallback_copy_delete(true)
        .run()
        .unwrap_err();
    let msg = err.to_string();

    match err {
        Error::MoveFile { from, to, .. } => {
            assert!(from.ends_with("download-dir"));
            assert!(to.ends_with("archive.txt"));
        }
        other => panic!("expected MoveFile, got {other:?}"),
    }

    assert!(msg.contains("failed to move file"));
    from.assert(predicates::path::is_dir());
    to.assert(predicates::path::missing());
}

#[test]
fn move_file_accepts_fallback_copy_delete_option_on_successful_rename() {
    let temp = assert_fs::TempDir::new().unwrap();
    let from = temp.child("download.tmp");
    let to = temp.child("archive.txt");
    from.write_str("hello").unwrap();

    move_file(from.path(), to.path())
        .fallback_copy_delete(true)
        .run()
        .unwrap();

    from.assert(predicates::path::missing());
    to.assert("hello");
}
