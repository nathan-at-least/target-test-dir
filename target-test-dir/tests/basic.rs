use std::path::PathBuf;
use target_test_dir::test_with_dir;

#[test_with_dir()]
fn ensure_test_dir_is_dir(testdir: PathBuf) {
    dbg!(&testdir);
    assert!(testdir.is_dir());
}

#[test_with_dir]
fn ensure_test_dir_exists(testdir: PathBuf) {
    dbg!(&testdir);
    assert!(testdir.exists());
}
