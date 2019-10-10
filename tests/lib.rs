extern crate errfmt;

use std::fs;

#[test]
fn test_passthrough_snapshot() {
  run_snapshot("passthrough");
}

#[test]
fn test_php_snapshot() {
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
    read_file(format!("./snapshots/{}/input", name)),
    read_file(format!("./snapshots/{}/errfmt", name)),
    read_file(format!("./snapshots/{}/expected", name)),
  )
}

fn read_file(name: String) -> String {
  fs::read_to_string(name)
    .unwrap()
    .trim_end_matches('\n')
    .to_string()
}
