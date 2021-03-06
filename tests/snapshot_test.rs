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
fn test_rustfmt_padding() {
  common::run_snapshot("rustfmt-padding", errfmt::RUSTFMT_ERRFMT);
}

#[test]
fn test_rustfmt_no_padding() {
  common::run_snapshot("rustfmt-no-padding", errfmt::RUSTFMT_ERRFMT);
}

#[test]
fn test_rustfmt_backquotes() {
  common::run_snapshot("rustfmt-backquotes", errfmt::RUSTFMT_ERRFMT);
}

#[test]
fn test_rustfmt_error_with_code() {
  common::run_snapshot("rustfmt-error-with-code", errfmt::RUSTFMT_ERRFMT);
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
