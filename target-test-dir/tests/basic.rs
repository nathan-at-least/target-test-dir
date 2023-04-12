use target_test_dir::with_test_dir;

#[test]
#[with_test_dir()]
fn ensure_test_dir_is_dir() {
    let testdir = get_test_dir!();
    dbg!(&testdir);
    assert!(testdir.is_dir());
}

#[test]
#[with_test_dir]
fn ensure_test_dir_exists() {
    let testdir = get_test_dir!();
    dbg!(&testdir);
    assert!(testdir.exists());
}
