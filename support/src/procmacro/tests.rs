use crate::transform_test_with_dir;
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
            let testdir =
            ::target_test_dir_support::get_base_test_dir()
                .join(format!("{}-{}", module_path!().replace("::", "-"), "my_test"));

            match std::fs::create_dir(&testdir) {
                Ok(()) => {}
                Err(e) => {
                    panic!("Could not create test dir {:?}: {}", testdir.display(), e);
                }
            }

            my_test_impl(testdir)
        }

        fn my_test_impl(testdir: PathBuf) {
            assert!(testdir.is_dir());
        }
    };

    assert_eq!(output.to_string(), expected.to_string());
}
