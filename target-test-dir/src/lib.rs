#![doc = include_str!("../../README.md")]

use proc_macro::TokenStream;
use target_test_dir_support::transform_test_with_dir;

/// Annotate a test function which takes a single [std::path::PathBuf] argument which will be a
/// freshly created directory
///
/// The annotated function must behave like a standard `#[test]` fn with the addition of a single
/// [std::path::PathBuf] argument. Any return type is propagated.
///
/// See the [crate] docs for an example.
#[proc_macro_attribute]
pub fn test_with_dir(_args: TokenStream, input: TokenStream) -> TokenStream {
    // TODO: parse _args.

    transform_test_with_dir(input)
}
