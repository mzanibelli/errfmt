use regex::Error;
use regex::Regex;
use regex::RegexBuilder;
use std::ops::Deref;

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

impl Token {
  /// Regexes that will be involved in extracting text data from the input
  /// stream. POSIX allows any character except null bytes in filename.
  pub fn pattern(&self) -> Result<Regex, Error> {
    match &self {
      Self::Column => Regex::new(r"(\d+)"),
      Self::File => Regex::new(r"([^\x00]+?)"),
      Self::Kind => Regex::new(r"(\b[a-zA-Z]+\b)"),
      Self::Line => Regex::new(r"(\d+)"),
      Self::Message => Regex::new(r"([^\n]+)"),
      Self::Whitespace => Regex::new(r"(\s+)"),
      Self::Wildcard => Regex::new(r"(.*?)"),
      Self::Literal(value) => Regex::new(&escape_metacharacters(value)),
    }
  }

  /// Human-readable way of representing an expected sequence of
  /// tokens. Acts as a DSL for defining the errorformat string.
  pub fn from(value: &str) -> Self {
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

/// The percent sign is used as a placeholder prefix. As a result,
/// it is necessary to escape it.
fn dedupe_percent_signs(value: &str) -> String {
  if value == "%%" {
    String::from("%")
  } else {
    String::from(value)
  }
}

/// Make any given literal string interpreted as non-special character
/// by the regex. Wrap the result in a capture group.
fn escape_metacharacters(value: &str) -> String {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"([\\.+*?()|\[\]{}^$])").unwrap();
  }
  format!("({})", RE.replace_all(value, r"\$1"))
}

/// Once the errorformat string is read and understood, this structure
/// represents a sequence of tokens: the shape of an error message.
#[derive(Debug)]
pub struct Shape(pub Vec<Token>);

/// Make sure we can access iterator methods quickly and concisely.
impl Deref for Shape {
  type Target = Vec<Token>;
  fn deref(&self) -> &Vec<Token> {
    &self.0
  }
}

impl Shape {
  /// Keep in mind this is an approximate size. Also, from my
  /// understanding, this represents the amount of memory needed
  /// by a regex *once compiled*.
  const REGEX_MAX_SIZE: usize = 1024 * 128;

  /// Initialize a new shape, empty by default. This must match nothing.
  pub fn new() -> Self {
    Self(Vec::new())
  }

  /// Add a token to the parser shape.
  pub fn push(self, token: Token) -> Self {
    Self([self.to_vec(), vec![token]].concat())
  }

  /// Final pattern is made multi-line. The pattern ultimately comes
  /// from user input, it is necessary to limit its size.
  pub fn pattern(&self) -> Result<Regex, Error> {
    self.build().and_then(|patterns| {
      RegexBuilder::new(&self.serialize(patterns).join(""))
        .size_limit(Self::REGEX_MAX_SIZE)
        .multi_line(true)
        .build()
    })
  }

  /// Try to compile the shape's patterns.
  fn build(&self) -> Result<Vec<Regex>, Error> {
    self.iter().map(|t| t.pattern()).collect()
  }

  /// Prepare the patterns for display.
  fn serialize(&self, patterns: Vec<Regex>) -> Vec<String> {
    patterns.into_iter().map(|p| p.to_string()).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn token_matches(token: Token, value: &str) -> bool {
    token.pattern().unwrap().is_match(value)
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
  fn test_full_featured_regex_as_string() {
    let sut = Shape::new()
      .push(Token::Literal(String::from("[Linter]: ")))
      .push(Token::File)
      .push(Token::Line)
      .push(Token::Column)
      .push(Token::Literal(String::from(" ")))
      .push(Token::Kind)
      .push(Token::Literal(String::from(" ")))
      .push(Token::Whitespace)
      .push(Token::Wildcard)
      .push(Token::Message);
    let actual = sut.pattern().unwrap().to_string();
    let expected = r"(\[Linter\]: )([^\x00]+?)(\d+)(\d+)( )(\b[a-zA-Z]+\b)( )(\s+)(.*?)([^\n]+)";
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
