#[macro_use]
extern crate lazy_static;

use regex::Captures;
use regex::Regex;

mod entry;
mod errfmt;
mod token;

use entry::Entry;
use entry::Kind;
use token::ErrFmt;
use token::Token;

pub fn run(input: String, errfmt: String) -> Result<String, String> {
  Ok(
    Parser::new(errfmt)
      .parse(input)
      .iter()
      .map(|entry| entry.to_string())
      .collect::<Vec<String>>()
      .join("\n"),
  )
}

#[derive(Debug)]
struct Parser {
  errfmt: ErrFmt,
}

/// Parser is responsible for building a set of entries matching the
/// extracted error messages.
impl Parser {
  /// Read the configuration (errorformat string) and compute the shape
  /// of an error message.
  fn new(errfmt: String) -> Self {
    Parser {
      errfmt: errfmt::tokenize(errfmt)
        .into_iter()
        .fold(ErrFmt::new(), |acc, t| acc.push(Token::from(&t))),
    }
  }

  /// Return a list of extracted locations.
  fn parse(&self, input: String) -> Vec<Entry> {
    let pattern = Regex::new(&self.errfmt.pattern()).unwrap();
    pattern
      .captures_iter(&input)
      .fold(Vec::new(), |mut acc, matches| {
        acc.push(self.build_entry(&matches));
        acc
      })
  }

  /// Add a new location to the result set by reading its data
  /// from capture groups.
  fn build_entry(&self, matches: &regex::Captures) -> Entry {
    let mut entry = Entry::new();
    let mut n = 1; // skip the first capture group as it is the entire string
    for token in &self.errfmt.0 {
      n = mutate_entry(&mut entry, &token, &matches, n);
    }
    entry
  }
}

/// Update a given entry according to the corresponding token.
fn mutate_entry(entry: &mut Entry, token: &Token, matches: &Captures, n: usize) -> usize {
  let parse_str = || matches.get(n).unwrap().as_str();
  let parse_u32 = || parse_str().parse::<u32>().unwrap();
  match token {
    Token::File => entry.file = String::from(parse_str()),
    Token::Kind => entry.kind = Kind::from(parse_str()),
    Token::Message => entry.message = String::from(parse_str()),
    Token::Line => entry.line = parse_u32(),
    Token::Column => entry.column = parse_u32(),
    Token::NewLine | Token::Literal(_) => return n, // do not consume next match
  };
  n + 1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parser_from_empty_errfmt() {
    let actual = Parser::new(String::new()).errfmt.0.len();
    let expected = 0;
    assert_eq!(expected, actual)
  }

  #[test]
  fn test_parser_should_have_an_entry_if_it_matches() {
    let sut = Parser::new(String::from("Error: %f:%l:%c: %k: %m"));
    let entries = sut.parse(String::from("Error: /tmp/foo:42:42: warning: syntax error"));
    assert_eq!(1, entries.len())
  }

  #[test]
  fn test_parser_should_keep_matching_groups_only() {
    let sut = Parser::new(String::from("PHP Parse error: %m in %f on line %l"));
    let entries = sut.parse(String::from("PHP Parse error:  syntax error, unexpected end of file, expecting ',' or ';' in /tmp/test.php on line 4"));
    assert_eq!(1, entries.len())
  }

  #[test]
  fn test_single_line_mode() {
    let sut = Parser::new(String::from("PHP Parse error: %m in %f on line %l"));
    let entries = sut.parse(String::from("PHP Parse error:  syntax error, unexpected end of file, expecting ',' or ';' in /tmp/test.php on line 4"));
    assert_eq!(
      "/tmp/test.php:4:1: error:  syntax error, unexpected end of file, expecting ',' or ';'",
      &entries[0].to_string()
    )
  }

  #[test]
  fn test_multiple_entries_with_single_line_mode() {
    let sut = Parser::new(String::from("PHP Parse error: %m in %f on line %l"));
    let entries = sut.parse(String::from(r"PHP Parse error:  syntax error, unexpected end of file, expecting ',' or ';' in /tmp/test.php on line 1
PHP Parse error:  syntax error, unexpected end of file, expecting ',' or ';' in /tmp/test.php on line 2"));
    assert_eq!(
      "/tmp/test.php:2:1: error:  syntax error, unexpected end of file, expecting ',' or ';'",
      &entries[1].to_string()
    )
  }

  #[test]
  fn test_multi_line_mode() {
    let sut = Parser::new(String::from("%k: %m%.  --> %f:%l:%c"));
    let entries = sut.parse(String::from(
      r"error: unexpected close delimiter: `}`
  --> /tmp/test.rs:85:1
   |
85 | }
   | ^ unexpected close delimiter",
    ));
    assert_eq!(
      "/tmp/test.rs:85:1: error: unexpected close delimiter: `}`",
      &entries[0].to_string()
    )
  }

  #[test]
  fn test_multiples_entries_with_multi_line_mode() {
    let sut = Parser::new(String::from("%k: %m%.  --> %f:%l:%c"));
    let entries = sut.parse(String::from(
      r"error: unexpected close delimiter: `}`
  --> /tmp/test.rs:1:1
   |
85 | }
   | ^ unexpected close delimiter
error: unexpected close delimiter: `}`
  --> /tmp/test.rs:2:1
   |
85 | }
   | ^ unexpected close delimiter",
    ));
    assert_eq!(
      "/tmp/test.rs:2:1: error: unexpected close delimiter: `}`",
      &entries[1].to_string()
    )
  }
}
