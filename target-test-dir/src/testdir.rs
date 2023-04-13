use crate::get_base_test_dir;
use std::path::PathBuf;

/// Create and return the function-specific testdir path; panic on errors
///
/// This function panics if creating the testdir fails.
///
/// # Example
///
/// Callers should use [std::module_path] and `function_name` macros
/// for the arguments:
///
/// ```
/// use target_test_dir::{named, get_test_dir};
/// use std::path::PathBuf;
///
/// #[named]
/// fn my_func() -> PathBuf {
///     get_test_dir(module_path!(), function_name!())
/// }
///
/// let testdir = my_func();
/// assert!(testdir.is_dir());
/// assert_eq!(
///     testdir.file_name().unwrap().to_str().unwrap(),
///     // Note: for doc examples the crate name is `rust_out`:
///     "rust_out-my_func",
/// );
/// ```
pub fn get_test_dir(modpath: &str, funcname: &str) -> PathBuf {
    try_get_test_dir(modpath, funcname).unwrap()
}

/// Create and return the function-specific testdir path, or any IO error encountered
///
/// # Example
///
/// Callers should use [std::module_path] and `function_name` macros
/// for the arguments:
///
/// ```
/// use target_test_dir::{named, try_get_test_dir};
/// use std::path::PathBuf;
///
/// #[named]
/// fn my_func() -> PathBuf {
///     try_get_test_dir(module_path!(), function_name!()).unwrap()
/// }
///
/// let testdir = my_func();
/// assert!(testdir.is_dir());
/// assert_eq!(
///     testdir.file_name().unwrap().to_str().unwrap(),
///     // Note: for doc examples the crate name is `rust_out`:
///     "rust_out-my_func",
/// );
/// ```
pub fn try_get_test_dir(modpath: &str, funcname: &str) -> anyhow::Result<PathBuf> {
    use anyhow_std::PathAnyhow;

    let testdir = get_base_test_dir().join(get_test_dir_name(modpath, funcname));
    testdir.create_dir_anyhow()?;
    Ok(testdir)
}

/// Get a testdir name given a `modpath` and `funcname`
///
/// # Example
///
/// Callers should use [std::module_path] and `function_name` macros
/// for the arguments:
///
/// ```
/// use target_test_dir::{named, get_test_dir_name};
///
/// #[named]
/// fn my_func() -> String {
///     get_test_dir_name(module_path!(), function_name!())
/// }
///
/// assert_eq!(
///     my_func(),
///     // Note: for doc examples the crate name is `rust_out`:
///     "rust_out-my_func",
/// );
/// ```
pub fn get_test_dir_name(modpath: &str, funcname: &str) -> String {
    format!("{}-{}", modpath.replace("::", "-"), funcname)
}
