use super::transform_with_test_dir;
use proc_macro2::TokenStream;
use quote::quote;

#[test]
fn no_args_unit_return() {
    let input = quote! {
        fn my_test() {
            let testdir = get_test_dir!();
            assert!(testdir.is_dir());
        }
    };

    let output = transform_with_test_dir(input);

    let expected = quote! {
        #[::target_test_dir::named]
        fn my_test() {
            macro_rules! get_test_dir {
                () => {
                    ::target_test_dir::get_test_dir(module_path!(), function_name!())
                }
            }

            let testdir = get_test_dir!();
            assert!(testdir.is_dir());
        }
    };

    assert_tokens_eq(expected, output);
}

#[test]
fn no_args_result_return() {
    let input = quote! {
        fn my_test() -> std::io::Result<()> {
            let testdir = get_test_dir!();
            assert!(testdir.is_dir());
            Ok(())
        }
    };

    let output = transform_with_test_dir(input);

    let expected = quote! {
        #[::target_test_dir::named]
        fn my_test() -> std::io::Result<()> {
            macro_rules! get_test_dir {
                () => {
                    ::target_test_dir::get_test_dir(module_path!(), function_name!())
                }
            }

            let testdir = get_test_dir!();
            assert!(testdir.is_dir());
            Ok(())
        }
    };

    assert_tokens_eq(expected, output);
}

#[test]
fn extra_args_unit_return() {
    let input = quote! {
        fn my_test(s: &str, i: i64) -> std::io::Result<()> {
            let testdir = get_test_dir!();
            assert!(testdir.is_dir());
            assert_eq!(i, i64::from_str(s).unwrap());
            Ok(())
        }
    };

    let output = transform_with_test_dir(input);

    let expected = quote! {
        #[::target_test_dir::named]
        fn my_test(s: &str, i: i64) -> std::io::Result<()> {
            macro_rules! get_test_dir {
                () => {
                    ::target_test_dir::get_test_dir(module_path!(), function_name!())
                }
            }

            let testdir = get_test_dir!();
            assert!(testdir.is_dir());
            assert_eq!(i, i64::from_str(s).unwrap());
            Ok(())
        }
    };

    assert_tokens_eq(expected, output);
}

fn assert_tokens_eq(expected: TokenStream, actual: TokenStream) {
    let expected = prettify(expected);
    let actual = prettify(actual);
    assert_eq!(
        &expected, &actual,
        "Not equal:\n\n=== expected ===\n{expected}\n\n=== actual ===\n{actual}\n\n",
    );
}

fn prettify(ts: TokenStream) -> String {
    let tsstr = ts.to_string();
    match syn::parse2::<syn::File>(ts) {
        Ok(f) => prettyplease::unparse(&f),
        Err(e) => panic!(
            "internal parse error:\ntokens: {:#?}\nsource: {:?}\ndetail: {}",
            tsstr,
            e.span().source_text(),
            e
        ),
    }
}
