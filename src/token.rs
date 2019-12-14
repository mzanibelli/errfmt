#[derive(Debug, Clone)]
pub enum Token {
  File,
  Line,
  Column,
  Kind,
  NewLine,
  Message,
  Literal(String),
}

/// A Token is a section of input data. It can be referred to using
/// pre-defined placeholders that compose an errorformat string.
impl Token {
  /// Regexes that will be involved in extracting text data from
  /// the input stream.
  pub fn pattern(&self) -> String {
    match &self {
      Self::File => String::from(r"([^\n]+)"),
      Self::Line => String::from(r"(\d+)"),
      Self::Column => String::from(r"(\d+)"),
      Self::Kind => String::from(r"\b(warning|error)\b"),
      Self::NewLine => String::from(r"\n"),
      Self::Message => String::from(r"([^\n]+)"),
      Self::Literal(value) => String::from(value),
    }
  }

  /// Human-readable way of representing an expected sequence of
  /// tokens. Acts as a DSL for defining the errorformat string.
  pub fn from(value: &str) -> Self {
    match value {
      "%f" => Self::File,
      "%m" => Self::Message,
      "%l" => Self::Line,
      "%c" => Self::Column,
      "%k" => Self::Kind,
      "%." => Self::NewLine,
      value => Self::Literal(dedupe_percent_signs(value)),
    }
  }
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

/// Once the errorformat string is read and understood, this structure
/// represents a sequence of tokens: the shape of an error message.
#[derive(Debug)]
pub struct ErrFmt(pub Vec<Token>);

impl ErrFmt {
  pub fn new() -> Self {
    Self(Vec::new())
  }

  pub fn push(self, token: Token) -> Self {
    Self([self.0.to_vec(), vec![token]].concat())
  }

  /// Final pattern is made multi-line and wrapped in a capture
  /// group.
  pub fn pattern(&self) -> String {
    format!("(?m:^{}$)", self.serialize().join(""))
  }

  fn serialize(&self) -> Vec<String> {
    self.0.iter().map(|t| t.pattern()).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use regex::Regex;

  fn token_matches(token: Token, value: &str) -> bool {
    Regex::new(&token.pattern()).unwrap().is_match(value)
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
  fn test_line_number_pattern_match() {
    assert!(token_matches(Token::Line, r"42"))
  }

  #[test]
  fn test_column_number_pattern_match() {
    assert!(token_matches(Token::Column, r"42"))
  }

  #[test]
  fn test_warning_kind_pattern_match() {
    assert!(token_matches(Token::Kind, r"warning"))
  }

  #[test]
  fn test_error_kind_pattern_match() {
    assert!(token_matches(Token::Kind, r"error"))
  }

  #[test]
  fn test_warning_kind_pattern_mismatch() {
    assert!(!token_matches(Token::Kind, r"globalwarning"))
  }

  #[test]
  fn test_error_kind_pattern_mismatch() {
    assert!(!token_matches(Token::Kind, r"errorized"))
  }

  #[test]
  fn test_newline_kind_pattern_match() {
    assert!(token_matches(Token::NewLine, "\n"))
  }

  #[test]
  fn test_message_pattern_match() {
    assert!(token_matches(
      Token::Message,
      r"This! is? an error message <core>"
    ))
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
  fn test_errfmt_pattern() {
    let sut = ErrFmt::new()
      .push(Token::Literal(String::from("Error: ")))
      .push(Token::File)
      .push(Token::Line)
      .push(Token::Column)
      .push(Token::Literal(String::from(" ")))
      .push(Token::Kind)
      .push(Token::Literal(String::from(" ")))
      .push(Token::NewLine)
      .push(Token::Message);
    let actual = sut.pattern();
    let expected = r"(?m:^Error: ([^\n]+)(\d+)(\d+) \b(warning|error)\b \n([^\n]+)$)";
    assert_eq!(expected, actual)
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
