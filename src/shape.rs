use regex::Error;
use regex::Regex;
use regex::RegexBuilder;
use std::convert::TryInto;
use std::ops::Deref;

use crate::Token;

/// Once the errorformat string is read and understood, this structure
/// represents a sequence of tokens: the shape of an error message.
#[derive(Debug, Clone)]
pub struct Shape(pub Vec<Token>);

/// Make sure we can access iterator methods quickly and concisely.
impl Deref for Shape {
  type Target = Vec<Token>;
  fn deref(&self) -> &Vec<Token> {
    &self.0
  }
}

/// Final pattern is made multi-line. The pattern ultimately comes
/// from user input, it is necessary to limit its size.
impl TryInto<Regex> for Shape {
  type Error = Error;
  fn try_into(self) -> Result<Regex, Error> {
    TryInto::<String>::try_into(self).and_then(|p| {
      RegexBuilder::new(&p)
        .size_limit(Self::REGEX_MAX_SIZE)
        .multi_line(true)
        .build()
    })
  }
}

/// Convert to an array of regexes before concatenating to string.
impl TryInto<String> for Shape {
  type Error = Error;
  fn try_into(self) -> Result<String, Error> {
    TryInto::<Vec<Regex>>::try_into(self).map(|p| {
      p.into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("")
    })
  }
}

/// Iteratively apply faillible conversion.
impl TryInto<Vec<Regex>> for Shape {
  type Error = Error;
  fn try_into(self) -> Result<Vec<Regex>, Error> {
    self.0.into_iter().map(TryInto::<Regex>::try_into).collect()
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
    let actual: Regex = sut.try_into().unwrap();
    let expected = r"(\[Linter\]: )([^\x00]+?)(\d+)(\d+)( )(\b[a-zA-Z]+\b)( )(\s+)(.*?)([^\n]+)";
    assert_eq!(expected, actual.to_string())
  }
}
