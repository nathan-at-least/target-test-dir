//! # Target Test Directories
//!
//! This crate provides a convenient proc-macro [macro@with_test_dir] for tests which need a test-specific directory.
//!
//! The [macro@with_test_dir] proc-macro inserts a `get_test_dir` macro into the body of a
//! function. `get_test_dir` creates and returns a test directory based on the name of the test
//! function in a well known location: `target/test-data/<module-path-and-function-name>`
//!
//! These `test-data` directories persist between test runs and are removed and recreated when
//! beginning a new test run. This facilitates inspecting test data between test runs for both
//! passing and failing tests.
//!
//! ## Example
//!
//! Your tests need to depend on this crate in `[dev-dependencies]`:
//!
//! ```ignore
//! [dev-dependencies]
//! target-test-dir = "…"
//! ```
//!
//! Then in any test which needs a directory, use the [macro@with_test_dir] proc-macro attribute on a
//! test fn:
//!
//! ```
//! use target_test_dir::with_test_dir;
//! use std::path::PathBuf;
//!
//! #[test]
//! #[with_test_dir]
//! fn write_and_read_hello_world() -> std::io::Result<()> {
//!     let testdir = get_test_dir!();
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
//! - The `get_test_dir` macro requires `$CARGO_MANIFEST_DIR` to be set due to invoking [get_base_test_dir].
//! - Each test has a test specific directory in `target/test-data/${TEST_SPECIFIC_NAME}`.
//! - The `TEST_SPECIFIC_NAME` is the full module + test function rust item path name with `::` replaced with `-`.
//! - Test process & `test-data` consistency:
//!   - During a test process run, the first test requiring one of these directories will remove all of `target/test-data` if present.
//!   - Each test creates its own test-specific directory prior to executing the wrapped test function.
//!   - These two invariants are designed to guarantee that the contents of that directory should always be due to the most recent run of tests (and should not mix data from different test processes).
//! - The directories are otherwise not removed by this framework, so that developers can inspect the results for both passing and failing tests.
//! - They live under `target/` so they should be ignored by Cargo's revision control conventions and cleaned up with `cargo clean`.
//!
//! ## Non-test functions
//!
//! The macro can be used on non-`#[test]` functions, provided the expectations of
//! [get_base_test_dir] are met and those functions are only invoked
//! once per test run. Example:
//!
//! ```
//! use target_test_dir::with_test_dir;
//! use std::path::PathBuf;
//!
//! #[with_test_dir]
//! fn setup_test_data() -> PathBuf {
//!     let testdir = get_test_dir!();
//!
//!     populate_test_data(&testdir);
//!     testdir
//! }
//!
//! #[test]
//! fn test_validator() {
//!     let testdir = setup_test_data();
//!     run_validator(testdir);
//! }
//!
//! # fn populate_test_data(_: &std::path::Path) {
//! #     unimplemented!("stub to demonstrate example");
//! # }
//! # fn run_validator(_: PathBuf) {
//! #     unimplemented!("stub to demonstrate example");
//! # }
//! ```
//!
//! ## Edge cases
//!
//! The `get_test_dir` macro panics if the directory already exists, since the design assumes tests
//! should always execute against a new empty directory for repeatability.
//!
//! This could occur, for example, when re-using a function multiple
//!
//! This may occur any time a given `#[with_test_dir]` function is called
//! multiple times in one test process. One case this occurs is when composing with the
//! [`test_case`](https://docs.rs/test-case/latest/test_case/) macro which calls the same test
//! function for each case within a single test process.

mod basedir;
mod testdir;

pub use self::basedir::get_base_test_dir;
pub use self::testdir::{get_test_dir, get_test_dir_name, try_get_test_dir};
/// The `named` proc-macro is re-exported from [function_name] because it is a dependency of [macro@with_test_dir]
pub use function_name::named;
pub use target_test_dir_macro::with_test_dir;
