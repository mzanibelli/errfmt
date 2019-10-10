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

impl Parser {
  fn new(errfmt: String) -> Self {
    Parser {
      errfmt: errfmt::tokenize(errfmt)
        .into_iter()
        .fold(ErrFmt::new(), |acc, t| acc.push(Token::from(&t))),
    }
  }

  fn parse(&self, input: String) -> Vec<Entry> {
    let pattern = Regex::new(&self.errfmt.pattern()).unwrap();
    input.lines().fold(Vec::new(), |mut acc, line| {
      if let Some(matches) = pattern.captures(&line) {
        acc.push(self.build_entry(&matches))
      }
      acc
    })
  }

  fn build_entry(&self, matches: &regex::Captures) -> Entry {
    let mut entry = Entry::new();
    let mut n = 1;
    for token in &self.errfmt.0 {
      n = mutate_entry(&mut entry, &token, &matches, n);
    }
    entry
  }
}

fn mutate_entry(entry: &mut Entry, token: &Token, matches: &Captures, n: usize) -> usize {
  let parse_str = || matches.get(n).unwrap().as_str();
  let parse_u32 = || parse_str().parse::<u32>().unwrap();
  match token {
    Token::File => entry.file = String::from(parse_str()),
    Token::Kind => entry.kind = Kind::from(parse_str()),
    Token::Message => entry.message = String::from(parse_str()),
    Token::Line => entry.line = parse_u32(),
    Token::Column => entry.column = parse_u32(),
    Token::Literal(_) => return n,
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
  fn test_parser_entry_shape() {
    let sut = Parser::new(String::from("PHP Parse error: %m in %f on line %l"));
    let entries = sut.parse(String::from("PHP Parse error:  syntax error, unexpected end of file, expecting ',' or ';' in /tmp/test.php on line 4"));
    assert_eq!(
      "/tmp/test.php:4:1: error:  syntax error, unexpected end of file, expecting ',' or ';'",
      &entries[0].to_string()
    )
  }
}
