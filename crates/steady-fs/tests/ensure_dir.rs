use assert_fs::prelude::*;
use steady_fs::ensure_dir;

#[test]
fn ensure_dir_creates_missing_directory() {
    let temp = assert_fs::TempDir::new().unwrap();
    let dir = temp.child("cache");

    ensure_dir(dir.path()).unwrap();

    dir.assert(predicates::path::is_dir());
}

#[test]
fn ensure_dir_creates_nested_directories() {
    let temp = assert_fs::TempDir::new().unwrap();
    let dir = temp.child("target/tmp/cache");

    ensure_dir(dir.path()).unwrap();

    dir.assert(predicates::path::is_dir());
}

#[test]
fn ensure_dir_succeeds_if_directory_already_exists() {
    let temp = assert_fs::TempDir::new().unwrap();
    let dir = temp.child("cache");
    dir.create_dir_all().unwrap();

    ensure_dir(dir.path()).unwrap();

    dir.assert(predicates::path::is_dir());
}

#[test]
fn ensure_dir_returns_path_aware_error_when_creation_fails() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file = temp.child("not_a_dir");
    file.write_str("hello").unwrap();

    let err = ensure_dir(file.path()).unwrap_err();
    let msg = err.to_string();

    assert!(msg.contains("failed to create directory"));
    assert!(msg.contains("not_a_dir"));
}
