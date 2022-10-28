use proc_macro::TokenStream;
use target_test_dir_support::transform_test_with_dir;

#[proc_macro_attribute]
pub fn test_with_dir(_args: TokenStream, input: TokenStream) -> TokenStream {
    // TODO: parse _args.

    transform_test_with_dir(input)
}
