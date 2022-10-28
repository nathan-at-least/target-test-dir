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
    use quote::quote;
    use syn::{parse2, Ident, ItemFn};

    let mut implfn: ItemFn = parse2(input)?;

    // Save the textual name for a generated wrapper function so that the actual #[test] has the
    // user-specified name:
    let testname = implfn.sig.ident;

    // Rename the user-provided implementation function to be wrapped:
    let implname = Ident::new(&format!("{}_impl", &testname), testname.span());
    implfn.sig.ident = implname.clone();

    // TODO: propagate the user test return type.
    Ok(quote! {
        #[test]
        fn #testname() {
            let testdir =
            ::target_test_dir_support::get_base_test_dir()
                .join(format!("{}-{}". module_path!().replace("::", "-"), #testname));

            #implname (testdir)
        }

        #implfn
    })
}
