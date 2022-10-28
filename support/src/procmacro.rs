use proc_macro2::TokenStream;

pub fn transform_test_with_dir<TS>(input: TS) -> TS
where
    TokenStream: From<TS>,
    TS: From<TokenStream>,
{
    let input = TokenStream::from(input);

    TS::from(transform_test_with_dir_inner(input).unwrap_or_else(syn::Error::into_compile_error))
}

fn transform_test_with_dir_inner(input: TokenStream) -> Result<TokenStream, syn::parse::Error> {
    use syn::{parse2, ItemFn};

    let _implfn: ItemFn = parse2(input)?;
    todo!();
}
