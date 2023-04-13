use anyhow_std::PathAnyhow;
use std::str::FromStr;
use target_test_dir::with_test_dir;
use test_case::test_case;

#[with_test_dir]
#[test_case("42" => 42 ; "fortytwo")]
#[test_case("-3" => -3 ; "negative three")]
fn int_parse(repr: &str) -> i64 {
    let testdir = get_test_dir!();

    let valuepath = testdir.join(repr);
    valuepath.write_anyhow(repr).unwrap();
    let readrepr = valuepath.read_to_string_anyhow().unwrap();

    i64::from_str(&readrepr).unwrap()
}
