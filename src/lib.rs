use regex::Regex;

mod errfmt;
mod token;
mod entry;

use token::Token;
use entry::Entry;
use entry::Kind;

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
      match token {
        Token::File => entry.file = string_match(matches, n),
        Token::Kind => entry.kind = Kind::from(&string_match(matches, n)),
        Token::Message => entry.message = string_match(matches, n),
        Token::Line => entry.line = u32_match(matches, n),
        Token::Column => entry.column = u32_match(matches, n),
        Token::Literal(_) => n -= 1,
      };
      n += 1;
    }
    entry
  }
}

fn string_match(matches: &regex::Captures, i: usize) -> String {
  matches.get(i).unwrap().as_str().to_string()
}

fn u32_match(matches: &regex::Captures, i: usize) -> u32 {
  string_match(matches, i).parse::<u32>().unwrap()
}

#[derive(Debug)]
struct ErrFmt(Vec<Token>);

impl ErrFmt {
  fn new() -> Self {
    Self(Vec::new())
  }

  fn push(self, token: Token) -> Self {
    Self([self.0.to_vec(), vec![token]].concat())
  }

  fn serialize(&self) -> Vec<String> {
    self.0.iter().map(|t| t.pattern()).collect()
  }

  fn pattern(&self) -> String {
    format!("^{}$", self.serialize().join(""))
  }
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
  fn test_parser_should_build_regex() {
    let sut = Parser::new(String::from("Error: %f:%l:%c: %k: %m"));
    let actual = sut.errfmt.pattern();
    let expected = r"^Error: (.+):(\d+):(\d+): \b(warning|error)\b: (.+)$";
    assert_eq!(expected, actual)
  }

  #[test]
  fn test_parser_should_have_an_entry_if_it_matches() {
    let sut = Parser::new(String::from("Error: %f:%l:%c: %k: %m"));
    let entries = sut.parse(String::from("Error: /tmp/foo:42:42: warning: syntax error"));
    assert_eq!(1, entries.len())
  }

  #[test]
  fn test_parser_entry_shape() {
    let sut = Parser::new(String::from("Error: %f:%l:%c: %k: %m"));
    let entries = sut.parse(String::from("Error: /tmp/foo:42:42: warning: syntax error"));
    assert_eq!("/tmp/foo:42:42: warning: syntax error", &entries[0].to_string())
  }

  #[test]
  fn test_parser_should_keep_matching_groups_only() {
    let sut = Parser::new(String::from("PHP Parse error: %m in %f on line %l"));
    let entries = sut.parse(String::from("PHP Parse error:  syntax error, unexpected end of file, expecting ',' or ';' in /tmp/test.php on line 4"));
    assert_eq!(1, entries.len())
  }
}
