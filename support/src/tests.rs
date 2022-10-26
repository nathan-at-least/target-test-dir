#[test]
fn check_dir_exists() {
    assert!(crate::get_base_test_dir().is_dir());
}

#[test]
fn check_dir_exists_concurrently() {
    const NUMBER_OF_THREADS: usize = 1024;

    let mut handles = vec![];

    for _ in 0..NUMBER_OF_THREADS {
        handles.push(std::thread::spawn(check_dir_exists));
    }

    let mut success = true;
    for h in handles {
        success = success && h.join().is_ok();
    }
    assert!(success);
}
