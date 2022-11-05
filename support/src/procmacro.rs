use proc_macro2::TokenStream;

/// Transform a token stream of a test function definition into a proper `#[test]` fn
///
/// This requires the input to be a fn item just like a standard `#[test]` function _except_ it
/// takes a single [std::path::PathBuf] argument to an empty test-specific directory.
pub fn transform_test_with_dir<TS>(input: TS) -> TS
where
    TokenStream: From<TS>,
    TS: From<TokenStream>,
{
    let input = TokenStream::from(input);

    TS::from(transform_test_with_dir_inner(input).unwrap_or_else(syn::Error::into_compile_error))
}

fn transform_test_with_dir_inner(input: TokenStream) -> Result<TokenStream, syn::parse::Error> {
    use quote::quote;
    use syn::{parse2, Ident, ItemFn};

    let mut implfn: ItemFn = parse2(input)?;

    // Save the textual name for a generated wrapper function so that the actual #[test] has the
    // user-specified name:
    let testname = implfn.sig.ident;
    let testnamestr = testname.to_string();

    // Rename the user-provided implementation function to be wrapped:
    let implname = Ident::new(&format!("{}_impl", &testnamestr), testname.span());
    implfn.sig.ident = implname.clone();

    // TODO: propagate the user test return type.
    Ok(quote! {
        #[test]
        fn #testname() {
            let testdir =
            ::target_test_dir_support::get_base_test_dir()
                .join(format!("{}-{}", module_path!().replace("::", "-"), #testnamestr));

            match std::fs::create_dir(&testdir) {
                Ok(()) => {}
                Err(e) => {
                    panic!("Could not create test dir {:?}: {}", testdir.display(), e);
                }
            }

            #implname (testdir)
        }

        #implfn
    })
}

#[cfg(test)]
mod tests;
