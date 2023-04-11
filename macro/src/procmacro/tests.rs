use super::transform_test_with_dir;
use quote::quote;

#[test]
fn unit_return() {
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

    assert_eq!(output.to_string(), expected.to_string());
}

#[test]
fn result_return() {
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

    assert_eq!(output.to_string(), expected.to_string());
}
