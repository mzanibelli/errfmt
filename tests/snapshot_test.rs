extern crate errfmt;

mod common;

#[test]
fn test_passthrough_snapshot() {
  common::run_snapshot("passthrough");
}

#[test]
fn test_php_snapshot() {
  common::run_snapshot("php-error");
  common::run_snapshot("php-warning");
}

#[test]
fn test_rust_snapshot() {
  common::run_snapshot("rust-alpha");
  common::run_snapshot("rust-beta");
}
