extern crate errfmt;

mod common;

#[test]
fn test_passthrough_snapshot() {
  common::run_snapshot("passthrough");
}

#[test]
fn test_php_error() {
  common::run_snapshot("php-error");
}

#[test]
fn test_php_warning() {
  common::run_snapshot("php-warning");
}

#[test]
fn test_rust_padding() {
  common::run_snapshot("rust-padding");
}

#[test]
fn test_rust_no_padding() {
  common::run_snapshot("rust-no-padding");
}

#[test]
fn test_rust_backquotes() {
  common::run_snapshot("rust-backquotes");
}

#[test]
fn test_rust_error_with_code() {
  common::run_snapshot("rust-error-with-code");
}

#[test]
fn test_eslint_error() {
  common::run_snapshot("eslint-error");
}

#[test]
fn test_eslint_warning() {
  common::run_snapshot("eslint-warning");
}
