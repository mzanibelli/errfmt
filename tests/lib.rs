extern crate errfmt;

use std::fs;

#[test]
fn test_passthrough() {
  let input = fs::read_to_string("./snapshots/passthrough/input").unwrap();
  let errfmt = fs::read_to_string("./snapshots/passthrough/errfmt").unwrap();
  let expected = fs::read_to_string("./snapshots/passthrough/expected").unwrap();
  match errfmt::run(input, errfmt) {
    Ok(actual) => assert_eq!(expected, actual),
    Err(_) => panic!("errfmt failed"),
  }
}
