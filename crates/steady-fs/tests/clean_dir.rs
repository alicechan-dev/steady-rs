use assert_fs::prelude::*;
use steady_fs::{clean_dir, Error};

#[test]
fn clean_dir_removes_files_and_subdirectories() {
    let temp = assert_fs::TempDir::new().unwrap();
    let dir = temp.child("target/tmp");
    let file = dir.child("file.txt");
    let nested_file = dir.child("nested/file.txt");
    dir.create_dir_all().unwrap();
    file.write_str("hello").unwrap();
    nested_file.write_str("nested").unwrap();

    clean_dir(dir.path()).run().unwrap();

    dir.assert(predicates::path::is_dir());
    file.assert(predicates::path::missing());
    nested_file.assert(predicates::path::missing());
}

#[test]
fn clean_dir_succeeds_when_directory_is_empty() {
    let temp = assert_fs::TempDir::new().unwrap();
    let dir = temp.child("target/tmp");
    dir.create_dir_all().unwrap();

    clean_dir(dir.path()).run().unwrap();

    dir.assert(predicates::path::is_dir());
}

#[test]
fn clean_dir_dry_run_does_not_remove_contents() {
    let temp = assert_fs::TempDir::new().unwrap();
    let dir = temp.child("target/tmp");
    let file = dir.child("file.txt");
    let nested_file = dir.child("nested/file.txt");
    dir.create_dir_all().unwrap();
    file.write_str("hello").unwrap();
    nested_file.write_str("nested").unwrap();

    clean_dir(dir.path()).dry_run(true).run().unwrap();

    dir.assert(predicates::path::is_dir());
    file.assert("hello");
    nested_file.assert("nested");
}

#[test]
fn clean_dir_returns_path_aware_error_for_missing_directory() {
    let temp = assert_fs::TempDir::new().unwrap();
    let dir = temp.child("missing");

    let err = clean_dir(dir.path()).run().unwrap_err();
    let msg = err.to_string();

    match err {
        Error::CleanDir { path, .. } => {
            assert!(path.ends_with("missing"));
        }
        other => panic!("expected CleanDir, got {other:?}"),
    }

    assert!(msg.contains("failed to clean directory"));
    assert!(msg.contains("missing"));
}

#[test]
fn clean_dir_refuses_current_directory() {
    let err = clean_dir(".").run().unwrap_err();
    let msg = err.to_string();

    match err {
        Error::DangerousCleanDir { path } => {
            assert!(path.ends_with("."));
        }
        other => panic!("expected DangerousCleanDir, got {other:?}"),
    }

    assert!(msg.contains("refusing to clean dangerous directory"));
}

#[test]
fn clean_dir_refuses_parent_directory_paths() {
    let err = clean_dir("../tmp").run().unwrap_err();
    let msg = err.to_string();

    match err {
        Error::DangerousCleanDir { path } => {
            assert!(path.ends_with("../tmp"));
        }
        other => panic!("expected DangerousCleanDir, got {other:?}"),
    }

    assert!(msg.contains("refusing to clean dangerous directory"));
    assert!(msg.contains(".."));
}

#[test]
fn clean_dir_refuses_root_directory() {
    let root = std::path::Path::new(std::path::MAIN_SEPARATOR_STR);
    let err = clean_dir(root).run().unwrap_err();
    let msg = err.to_string();

    match err {
        Error::DangerousCleanDir { path } => {
            assert!(path.has_root());
        }
        other => panic!("expected DangerousCleanDir, got {other:?}"),
    }

    assert!(msg.contains("refusing to clean dangerous directory"));
}
