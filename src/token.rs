#[derive(Debug, Clone)]
pub enum Token {
  File,
  Line,
  Column,
  Kind,
  Message,
  Literal(String),
}

impl Token {
  pub fn pattern(&self) -> String {
    match &self {
      Self::File => String::from(r"(.+)"),
      Self::Line => String::from(r"(\d+)"),
      Self::Column => String::from(r"(\d+)"),
      Self::Kind => String::from(r"\b(warning|error)\b"),
      Self::Message => String::from(r"(.+)"),
      Self::Literal(value) => String::from(value),
    }
  }

  pub fn from(value: &str) -> Self {
    match value {
      "%f" => Self::File,
      "%m" => Self::Message,
      "%l" => Self::Line,
      "%c" => Self::Column,
      "%k" => Self::Kind,
      value => Self::Literal(dedupe_percent_signs(value)),
    }
  }
}

fn dedupe_percent_signs(value: &str) -> String {
  if value == "%%" {
    String::from("%")
  } else {
    String::from(value)
  }
}

#[derive(Debug)]
pub struct ErrFmt(pub Vec<Token>);

impl ErrFmt {
  pub fn new() -> Self {
    Self(Vec::new())
  }

  pub fn push(self, token: Token) -> Self {
    Self([self.0.to_vec(), vec![token]].concat())
  }

  pub fn pattern(&self) -> String {
    format!("^{}$", self.serialize().join(""))
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
      .push(Token::Message);
    let actual = sut.pattern();
    let expected = r"^Error: (.+)(\d+)(\d+) \b(warning|error)\b (.+)$";
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
