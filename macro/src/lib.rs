//! The macro implementation for `target-test-dir`
//!
//! Use the `target-test-dir` crate rather than this, which has runtime support as well as complete docs.

mod procmacro;

use proc_macro::TokenStream;

/// Provide a `get_test_dir` macro within the body of the annotated function
///
/// This macro must be used via the `target-test-dir` crate, rather than directly from
/// [target-test-dir-macro]. The usage documentation lives in `target-test-dir`.
#[proc_macro_attribute]
pub fn with_test_dir(_args: TokenStream, input: TokenStream) -> TokenStream {
    // TODO: parse _args.

    self::procmacro::transform_with_test_dir(input)
}
