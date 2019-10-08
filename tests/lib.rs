extern crate errfmt;

use std::fs;

#[test]
fn test_snapshots() {
  run_snapshot("passthrough");
  run_snapshot("php");
}

fn run_snapshot(name: &str) -> () {
  check_snapshot(read_snapshot(name));
}

fn check_snapshot((input, errfmt, expected): (String, String, String)) -> () {
  assert_eq!(expected, errfmt::run(input, errfmt).unwrap());
}

fn read_snapshot(name: &str) -> (String, String, String) {
  (
    fs::read_to_string(format!("./snapshots/{}/input", name)).unwrap(),
    fs::read_to_string(format!("./snapshots/{}/errfmt", name)).unwrap(),
    fs::read_to_string(format!("./snapshots/{}/expected", name)).unwrap(),
  )
}
