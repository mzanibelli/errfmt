use regex::Error;
use regex::Regex;
use regex::RegexBuilder;
use std::ops::Deref;

use crate::Token;

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
      RegexBuilder::new(&Self::serialize(patterns).join(""))
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
  fn serialize(patterns: Vec<Regex>) -> Vec<String> {
    patterns.into_iter().map(|p| p.to_string()).collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
}
