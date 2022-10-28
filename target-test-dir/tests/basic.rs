use std::path::PathBuf;
use target_test_dir::test_with_dir;

#[test_with_dir()]
fn ensure_test_dir_exists(testdir: PathBuf) {
    dbg!(&testdir);
    assert!(testdir.is_dir());
}
