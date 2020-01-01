use regex::Error;
use regex::Regex;
use std::convert::From;
use std::convert::TryInto;

/// A Token is a section of input data. It can be referred to using
/// pre-defined placeholders that compose an errorformat string.
#[derive(Debug, Clone)]
pub enum Token {
  Column,
  File,
  Kind,
  Line,
  Message,
  Whitespace,
  Wildcard,
  Literal(String),
}

/// Human-readable way of representing an expected sequence of
/// tokens. Acts as a DSL for defining the errorformat string.
impl From<&str> for Token {
  fn from(value: &str) -> Self {
    match value {
      "%c" => Self::Column,
      "%f" => Self::File,
      "%k" => Self::Kind,
      "%l" => Self::Line,
      "%m" => Self::Message,
      "%." => Self::Whitespace,
      "%*" => Self::Wildcard,
      value => Self::Literal(dedupe_percent_signs(value)),
    }
  }
}

/// For convenience.
impl From<String> for Token {
  fn from(value: String) -> Self {
    Self::from(value.as_ref())
  }
}

/// Regexes that will be involved in extracting text data from the input
/// stream. POSIX allows any character except null bytes in filename.
impl TryInto<Regex> for Token {
  type Error = Error;
  fn try_into(self) -> Result<Regex, Error> {
    match &self {
      Self::Column => mkregex(r"\d+"),
      Self::File => mkregex(r"[^\x00]+?"),
      Self::Kind => mkregex(r"\b[a-zA-Z]+\b"),
      Self::Line => mkregex(r"\d+"),
      Self::Message => mkregex(r"[^\n]+"),
      Self::Whitespace => mkregex(r"\s+"),
      Self::Wildcard => mkregex(r".*?"),
      Self::Literal(value) => mkregex(&regex::escape(&value)),
    }
  }
}

/// Wrap given pattern in a capture group.
fn mkregex(s: &str) -> Result<Regex, Error> {
  Regex::new(&format!("({})", s))
}

/// The percent sign is used as a placeholder prefix. As a result,
/// it is necessary to escape it.
fn dedupe_percent_signs(value: &str) -> String {
  if value == "%%" {
    String::from("%")
  } else {
    String::from(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn token_matches(token: Token, value: &str) -> bool {
    let r: Regex = token.try_into().unwrap();
    r.is_match(value)
  }

  #[test]
  fn test_standard_filename_pattern_match() {
    assert!(token_matches(Token::File, r"/file/with/extension.foo"))
  }

  #[test]
  fn test_filename_with_space_pattern_match() {
    assert!(token_matches(
      Token::File,
      r"/file/with/space\ in\ name.foo"
    ))
  }

  #[test]
  fn test_filename_mismatch() {
    assert!(!token_matches(Token::File, "\0"))
  }

  #[test]
  fn test_line_number_pattern_match() {
    assert!(token_matches(Token::Line, r"42"))
  }

  #[test]
  fn test_column_number_pattern_match() {
    assert!(token_matches(Token::Column, r"42"))
  }

  #[test]
  fn test_line_number_pattern_mismatch() {
    assert!(!token_matches(Token::Line, r"foo"))
  }

  #[test]
  fn test_column_number_pattern_mismatch() {
    assert!(!token_matches(Token::Column, r"foo"))
  }

  #[test]
  fn test_kind_pattern_match() {
    assert!(token_matches(Token::Kind, r"anyWord"))
  }

  #[test]
  fn test_kind_pattern_mismatch() {
    assert!(!token_matches(Token::Kind, r"[notG00d]"))
  }

  #[test]
  fn test_whitespace_pattern_match() {
    assert!(token_matches(Token::Whitespace, "	 \n"))
  }

  #[test]
  fn test_whitespace_pattern_mismatch() {
    assert!(!token_matches(Token::Whitespace, "abcd"))
  }

  #[test]
  fn test_message_pattern_match() {
    assert!(token_matches(
      Token::Message,
      r"This! is? an error message <core>"
    ))
  }

  #[test]
  fn test_message_pattern_mismatch() {
    assert!(token_matches(
      Token::Message,
      r"Messages cannot be\nmulti-line..."
    ))
  }

  #[test]
  fn test_wildcard_pattern_match() {
    assert!(token_matches(Token::Wildcard, r"E00kdjksh1an"))
  }

  #[test]
  fn test_wildcard_pattern_mismatch() {
    assert!(token_matches(Token::Wildcard, "hello\nworld"))
  }

  #[test]
  fn test_literal_pattern_match() {
    assert!(token_matches(
      Token::Literal(String::from("foo bar")),
      r"foo bar"
    ))
  }

  #[test]
  fn test_literal_pattern_mismatch() {
    assert!(!token_matches(
      Token::Literal(String::from("foo baz")),
      r"foo bar"
    ))
  }

  #[test]
  fn test_literal_must_not_be_a_regex() {
    let tests = vec![
      vec![r".", r"a"],
      vec![r"\s", r"	"],
      vec![r"a+", r"a"],
      vec![r"a?", r"a"],
      vec![r"(a)", r"a"],
      vec![r"a|b", r"a"],
      vec![r"[a]", r"a"],
      vec![r"a{1}", r"a"],
      vec![r"^a", r"a"],
      vec![r"a$", r"a"],
    ];
    for test in tests {
      assert!(!token_matches(
        Token::Literal(String::from(test[0])),
        test[1]
      ));
    }
  }

  #[test]
  fn test_literal_must_work_with_metacharacters() {
    let tests = vec![
      vec![r".", r"."],
      vec![r"(a)", r"(a)"],
      vec![r"a|b", r"a|b"],
      vec![r"[a]", r"[a]"],
    ];
    for test in tests {
      assert!(token_matches(
        Token::Literal(String::from(test[0])),
        test[1]
      ));
    }
  }

  #[test]
  fn test_from_dedupes_percent_signs() {
    if let Token::Literal(actual) = Token::from("%%") {
      assert_eq!(String::from("%"), actual)
    } else {
      panic!()
    }
  }
}
