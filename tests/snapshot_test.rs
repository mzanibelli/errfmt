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
  common::run_snapshot("rust-padding", errfmt::RUSTFMT_ERRFMT);
}

#[test]
fn test_rust_no_padding() {
  common::run_snapshot("rust-no-padding", errfmt::RUSTFMT_ERRFMT);
}

#[test]
fn test_rust_backquotes() {
  common::run_snapshot("rust-backquotes", errfmt::RUSTFMT_ERRFMT);
}

#[test]
fn test_rust_error_with_code() {
  common::run_snapshot("rust-error-with-code", errfmt::RUSTFMT_ERRFMT);
}

#[test]
fn test_eslint_error() {
  common::run_snapshot("eslint-error", errfmt::ESLINT_ERRFMT);
}

#[test]
fn test_eslint_warning() {
  common::run_snapshot("eslint-warning", errfmt::ESLINT_ERRFMT);
}

#[test]
fn test_golint_error() {
  common::run_snapshot("golint-error", errfmt::GOLINT_ERRFMT);
}

#[test]
fn test_shellcheck_error() {
  common::run_snapshot("shellcheck-error", errfmt::SHELLCHECK_ERRFMT);
}

#[test]
fn test_shellcheck_note() {
  common::run_snapshot("shellcheck-note", errfmt::SHELLCHECK_ERRFMT);
}

