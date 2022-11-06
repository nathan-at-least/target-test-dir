//! The macro implementation for `target-test-dir`
//!
//! Use the `target-test-dir` crate rather than this, which has runtime support as well as complete docs.

mod procmacro;

use proc_macro::TokenStream;

/// Annotate a test function which takes a single [std::path::PathBuf] argument which will be a
/// freshly created directory
///
/// The annotated function must behave like a standard `#[test]` fn with the addition of a single
/// [std::path::PathBuf] argument. Any return type is propagated.
#[proc_macro_attribute]
pub fn test_with_dir(_args: TokenStream, input: TokenStream) -> TokenStream {
    // TODO: parse _args.

    self::procmacro::transform_test_with_dir(input)
}
