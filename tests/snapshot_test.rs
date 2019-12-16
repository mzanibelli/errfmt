extern crate errfmt;

mod common;

#[test]
fn test_passthrough_snapshot() {
  common::run_snapshot("passthrough", errfmt::PASSTHROUGH_ERRFMT);
}

#[test]
fn test_php_error() {
  common::run_snapshot("php-error", errfmt::PHP_ERRFMT);
}

#[test]
fn test_php_warning() {
  common::run_snapshot("php-warning", errfmt::PHP_ERRFMT);
}

#[test]
fn test_rust_padding() {
  common::run_snapshot("rust-padding", errfmt::RUST_ERRFMT);
}

#[test]
fn test_rust_no_padding() {
  common::run_snapshot("rust-no-padding", errfmt::RUST_ERRFMT);
}

#[test]
fn test_rust_backquotes() {
  common::run_snapshot("rust-backquotes", errfmt::RUST_ERRFMT);
}

#[test]
fn test_rust_error_with_code() {
  common::run_snapshot("rust-error-with-code", errfmt::RUST_ERRFMT);
}

#[test]
fn test_eslint_error() {
  common::run_snapshot("eslint-error", errfmt::ESLINT_ERRFMT);
}

#[test]
fn test_eslint_warning() {
  common::run_snapshot("eslint-warning", errfmt::ESLINT_ERRFMT);
}
