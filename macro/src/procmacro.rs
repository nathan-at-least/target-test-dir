use proc_macro2::TokenStream;

/// Transform a token stream of a test function definition into a proper `#[test]` fn
///
/// This requires the input to be a fn item just like a standard `#[test]` function _except_ it
/// takes a single [std::path::PathBuf] argument to an empty test-specific directory. Any return
/// type is propagated, although any errors during setting up the test directory cause panics.
///
/// Almost every use of this function would be via the `test_with_dir` macro within the
/// `target-test-dir` crate; see that crate for examples.
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

    // Propagate the return type:
    let output = implfn.sig.output.clone();

    Ok(quote! {
        #[test]
        fn #testname() #output {
            #implname (
                // We initialize and pass the testdir in a local scope to avoid collutions in
                // #testnate scope:
                {
                    let testdir =
                    ::target_test_dir::get_base_test_dir()
                        .join(format!("{}-{}", module_path!().replace("::", "-"), #testnamestr));

                    if let Some(e) = std::fs::create_dir(&testdir).err() {
                        panic!("Could not create test dir {:?}: {}", testdir.display(), e);
                    };

                    testdir
                }
            )
        }

        #implfn
    })
}

#[cfg(test)]
mod tests;
