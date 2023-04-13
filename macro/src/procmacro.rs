use proc_macro2::TokenStream;

/// Transform a token stream of a test function definition into a proper `#[test]` fn
///
/// This requires the input to be a fn item just like a standard `#[test]` function _except_ it
/// takes a single [std::path::PathBuf] argument to an empty test-specific directory. Any return
/// type is propagated, although any errors during setting up the test directory cause panics.
///
/// Almost every use of this function would be via the `with_test_dir` macro within the
/// `target-test-dir` crate; see that crate for examples.
pub fn transform_with_test_dir<TS>(input: TS) -> TS
where
    TokenStream: From<TS>,
    TS: From<TokenStream>,
{
    let input = TokenStream::from(input);

    TS::from(transform_with_test_dir_inner(input).unwrap_or_else(syn::Error::into_compile_error))
}

fn transform_with_test_dir_inner(input: TokenStream) -> Result<TokenStream, syn::parse::Error> {
    use quote::quote;
    use syn::{parse2, ItemFn};

    let ItemFn {
        attrs,
        vis,
        sig,
        mut block,
    }: ItemFn = parse2(input)?;

    // Insert the `get_test_dir` macro:
    block.stmts.insert(
        0,
        syn::parse2::<syn::Stmt>(quote! {
            macro_rules! get_test_dir {
                () => {
                    ::target_test_dir::get_test_dir(module_path!(), function_name!())
                }
            }
        })?,
    );

    Ok(quote! {
        #[::target_test_dir::named]
        #( #attrs )*
        #vis
        #sig
        #block
    })
}

#[cfg(test)]
mod tests;
