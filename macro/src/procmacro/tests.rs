use super::transform_test_with_dir;
use proc_macro2::TokenStream;
use quote::quote;

#[test]
fn no_args_unit_return() {
    let input = quote! {
        fn my_test(testdir: PathBuf) {
            assert!(testdir.is_dir());
        }
    };

    let output = transform_test_with_dir(input);

    let expected = quote! {
        #[test]
        fn my_test() {
            my_test_impl(
                {
                    let testdir =
                    ::target_test_dir::get_base_test_dir()
                        .join(format!("{}-{}", module_path!().replace("::", "-"), "my_test"));

                    if let Some(e) = std::fs::create_dir(&testdir).err() {
                        panic!("Could not create test dir {:?}: {}", testdir.display(), e);
                    };

                    testdir
                }
            )
        }

        fn my_test_impl(testdir: PathBuf) {
            assert!(testdir.is_dir());
        }
    };

    assert_tokens_eq(expected, output);
}

#[test]
fn no_args_result_return() {
    let input = quote! {
        fn my_test(testdir: PathBuf) -> std::io::Result<()> {
            assert!(testdir.is_dir());
            Ok(())
        }
    };

    let output = transform_test_with_dir(input);

    let expected = quote! {
        #[test]
        fn my_test() -> std::io::Result<()> {
            my_test_impl(
                {
                    let testdir =
                    ::target_test_dir::get_base_test_dir()
                        .join(format!("{}-{}", module_path!().replace("::", "-"), "my_test"));

                    if let Some(e) = std::fs::create_dir(&testdir).err() {
                        panic!("Could not create test dir {:?}: {}", testdir.display(), e);
                    };

                    testdir
                }
            )
        }

        fn my_test_impl(testdir: PathBuf) -> std::io::Result<()> {
            assert!(testdir.is_dir());
            Ok(())
        }
    };

    assert_tokens_eq(expected, output);
}

#[test]
fn extra_args_unit_return() {
    let input = quote! {
        fn my_test(s: &str, i: i64, testdir: PathBuf) -> std::io::Result<()> {
            assert!(testdir.is_dir());
            assert_eq!(i, i64::from_str(s).unwrap());
            Ok(())
        }
    };

    let output = transform_test_with_dir(input);

    let expected = quote! {
        #[test]
        fn my_test(s: &str, i: i64) -> std::io::Result<()> {
            my_test_impl(
                s,
                i,
                {
                    let testdir =
                    ::target_test_dir::get_base_test_dir()
                        .join(format!("{}-{}", module_path!().replace("::", "-"), "my_test"));

                    if let Some(e) = std::fs::create_dir(&testdir).err() {
                        panic!("Could not create test dir {:?}: {}", testdir.display(), e);
                    };

                    testdir
                }
            )
        }

        fn my_test_impl(s: &str, i: i64, testdir: PathBuf) -> std::io::Result<()> {
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
    prettyplease::unparse(&syn::parse2::<syn::File>(ts).unwrap())
}
