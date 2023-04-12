use std::path::PathBuf;
use target_test_dir::with_test_dir;

#[with_test_dir()]
fn ensure_test_dir_is_dir(testdir: PathBuf) {
    dbg!(&testdir);
    assert!(testdir.is_dir());
}

#[with_test_dir]
fn ensure_test_dir_exists(testdir: PathBuf) {
    dbg!(&testdir);
    assert!(testdir.exists());
}
