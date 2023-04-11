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

    // We want to propagate all args except the last, which is the testdir:
    let mut wrapper_inputs = implfn.sig.inputs.clone();
    wrapper_inputs.pop(); // Remove the last arg from propagation.

    // Now we want to propagate the arg bindings in the call, with comma termination:
    let arg_call_names = {
        use syn::{
            punctuated::Punctuated, spanned::Spanned, token::Comma, Error, Expr, FnArg::Typed, Pat,
            PatIdent, PatType,
        };

        let mut pct = Punctuated::<Expr, Comma>::new();
        for fnarg in &wrapper_inputs {
            if let Typed(PatType { pat, .. }) = fnarg {
                if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                    pct.push(syn::parse2(quote! { #ident })?);
                } else {
                    return Err(Error::new(
                        fnarg.span(),
                        "unexpected pattern; expecting identifier",
                    ));
                }
            } else {
                return Err(Error::new(
                    fnarg.span(),
                    "unexpected received; expecting `<identifier>: <type>`",
                ));
            }
        }

        if !pct.is_empty() {
            pct.push_punct(Comma::default());
        }

        pct
    };

    // Propagate the return type:
    let output = implfn.sig.output.clone();

    Ok(quote! {
        #[test]
        fn #testname( #wrapper_inputs ) #output {
            #implname (
                #arg_call_names

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
