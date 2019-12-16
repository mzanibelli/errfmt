use std::fmt;

/// An Entry is a location (file, line) that is meant to be compatible
/// with kak(1)'s definition.
#[derive(Debug)]
pub struct Entry {
  pub file: String,
  pub line: u32,
  pub column: u32,
  pub kind: Kind,
  pub message: String,
}

impl Entry {
  /// Default values are for the most part meaningful and allows
  /// partially complete linters to step up their game for free.
  pub fn new() -> Self {
    Entry {
      file: String::new(),
      line: 1,
      column: 1,
      kind: Kind::Error,
      message: String::new(),
    }
  }
}

/// Must match kakoune's expected format. See lint.kak from standard rc
/// scripts. One day, this will maybe support other output formats...
impl fmt::Display for Entry {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}:{}:{}: {}: {}",
      self.file, self.line, self.column, self.kind, self.message
    )
  }
}

/// Simple representation of the error's log-level. The possible variants
/// are purposedly limited here: lint.kak script only supports these two.
#[derive(Debug)]
pub enum Kind {
  Warning,
  Error,
}

/// Explicitely add encountered notations here instead of blindly accept
/// any word and be forced to use an incorrect default value.
const WARNING: &str = "warning";
const ERROR: &str = "error";
const NOTE: &str = "note";

impl Kind {
  /// Must accept capitalized words to handle various linter
  /// formats.
  pub fn from(value: &str) -> Self {
    match value.to_lowercase().as_str() {
      WARNING | NOTE => Kind::Warning,
      ERROR => Kind::Error,
      value => panic!("unexpected kind: {}", value),
    }
  }
}

impl fmt::Display for Kind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Kind::Warning => write!(f, "{}", WARNING),
      Kind::Error => write!(f, "{}", ERROR),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_entry_values() {
    let expected = String::from(":1:1: error: ");
    let actual = Entry::new().to_string();
    assert_eq!(expected, actual)
  }

  #[test]
  fn test_arbitrary_entry_values() {
    let expected = String::from("/tmp/foo:2:3: warning: syntax error");
    let mut sut = Entry::new();
    sut.file = String::from("/tmp/foo");
    sut.line = 2;
    sut.column = 3;
    sut.kind = Kind::Warning;
    sut.message = String::from("syntax error");
    let actual = sut.to_string();
    assert_eq!(expected, actual)
  }

  #[test]
  fn test_error_kind() {
    let expected = Kind::Error.to_string();
    let actual = Kind::from("error").to_string();
    assert_eq!(expected, actual)
  }

  #[test]
  fn test_warning_kind() {
    let expected = Kind::Warning.to_string();
    let actual = Kind::from("warning").to_string();
    assert_eq!(expected, actual)
  }

  #[test]
  fn test_note_kind() {
    let expected = Kind::Warning.to_string();
    let actual = Kind::from("note").to_string();
    assert_eq!(expected, actual)
  }

  #[test]
  fn test_word_can_be_capitalized() {
    let expected = Kind::Error.to_string();
    let actual = Kind::from("Error").to_string();
    assert_eq!(expected, actual)
  }
}
