extern crate errfmt;

mod common;

#[test]
fn test_passthrough_snapshot() {
  common::run_snapshot("passthrough");
}

#[test]
fn test_php_snapshot() {
  common::run_snapshot("php");
}
