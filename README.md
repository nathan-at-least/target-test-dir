# Target Test Directories

The [target-test-dir](crate) crate provides a convenient proc-macro [macro@test_with_dir] for tests which need a test-specific directory.

## Example

Your tests need to depend on both the [target-test-dir](crate) and [target-test-dir-support](target_test_dir_support) crates, so `Cargo.toml` includes:

```ignore
[dev-dependencies]
target-test-dir = "0.1.0"
target-test-dir-support = "0.1.0"
```

Then in any test which needs a directory, use the `with_test_dir` proc-macro attribute:

```
use target_test_dir::test_with_dir;
use std::path::PathBuf;

#[test_with_dir]
fn write_and_read_hello_world(testdir: PathBuf) {
    let hwpath = testdir.join("hello_world.txt");
    std::fs::write(&hwpath, "Hello World!").unwrap();

    let bytes = std::fs::read(hwpath).unwrap();
    let output = String::from_utf8(bytes).unwrap();

    assert_eq!(&output, "Hello World!");
}
```

## Test Directory Invariants

 The test directories follow these invariants:

- Each test has a test specific directory in `target/test-data/${TEST_SPECIFIC_NAME}`.
- The `TEST_SPECIFIC_NAME` is the full module + test function rust item path name with `::` replaced with `-`.
- During a test process run, the first test requiring one of these directories removes all of `target/test-data` so that the contents of that directory should always be due to the most recent run of tests (and should not mix data from different test processes).
- The directories are otherwise not removed by this framework, so that developers can inspect the results for both passing and failing tests.
- They live under `target/` so they should be ignored by Cargo's revision control conventions and cleaned up with `cargo clean`.
