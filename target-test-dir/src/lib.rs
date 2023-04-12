//! # Target Test Directories
//!
//! This [target-test-dir](crate) crate provides a convenient proc-macro [macro@test_with_dir] for tests which need a test-specific directory.
//!
//! ## Example
//!
//! Your tests need to depend on this crate in `[dev-dependencies]`:
//!
//! ```ignore
//! [dev-dependencies]
//! target-test-dir = "0.2.0"
//! ```
//!
//! Then in any test which needs a directory, use the [macro@test_with_dir] proc-macro attribute on a
//! test-like fn which takes a single [std::path::PathBuf] argument to the test-specific directory:
//!
//! ```
//! use target_test_dir::test_with_dir;
//! use std::path::PathBuf;
//!
//! #[test_with_dir]
//! fn write_and_read_hello_world(testdir: PathBuf) -> std::io::Result<()> {
//!     let hwpath = testdir.join("hello_world.txt");
//!     std::fs::write(&hwpath, "Hello World!")?;
//!
//!     let bytes = std::fs::read(hwpath)?;
//!     let output = String::from_utf8(bytes).unwrap();
//!
//!     assert_eq!(&output, "Hello World!");
//!     Ok(())
//! }
//! ```
//!
//! ## Test Directory Invariants
//!
//! The test directories follow these invariants:
//!
//! - The directory passed to a test is guaranteed to exist and be empty.
//! - Each test has a test specific directory in `target/test-data/${TEST_SPECIFIC_NAME}`.
//! - The `TEST_SPECIFIC_NAME` is the full module + test function rust item path name with `::` replaced with `-`.
//! - Test process & `test-data` consistency:
//!   - During a test process run, the first test requiring one of these directories will remove all of `target/test-data` if present.
//!   - Each test creates its own test-specific directory prior to executing the wrapped test function.
//!   - These two invariants are designed to guarantee that the contents of that directory should always be due to the most recent run of tests (and should not mix data from different test processes).
//! - The directories are otherwise not removed by this framework, so that developers can inspect the results for both passing and failing tests.
//! - They live under `target/` so they should be ignored by Cargo's revision control conventions and cleaned up with `cargo clean`.

mod basedir;
mod testdir;

pub use self::basedir::get_base_test_dir;
pub use self::testdir::{get_test_dir, get_test_dir_name, try_get_test_dir};
pub use function_name::named;
pub use target_test_dir_macro::test_with_dir;
