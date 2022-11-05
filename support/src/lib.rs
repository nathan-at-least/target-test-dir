//! A support crate for the `target-test-dir` crate
//!
//! This crate provides [get_base_test_dir] which can be used independently of the proc macro, as
//! well as [transform_test_with_dir] which implements the actual macro transformation.

mod basedir;
mod procmacro;

pub use self::basedir::get_base_test_dir;
pub use self::procmacro::transform_test_with_dir;
