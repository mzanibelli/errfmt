use regex::Regex;
use std::fmt;

pub fn run(input: String, errfmt: String) -> Result<String, ()> {
  let entries = Parser::new(errfmt).parse(input);
  Ok(format!("{:?}", entries))
}

#[derive(Debug, Clone)]
enum Token {
  File,
  Line,
  Column,
  Kind,
  Message,
  Literal(String),
}

impl Token {
  fn pattern(&self) -> String {
    match &self {
      Token::File => String::from(r"(.+)"),
      Token::Line => String::from(r"(\d+)"),
      Token::Column => String::from(r"(\d+)"),
      Token::Kind => String::from(r"\b(warning|error)\b"),
      Token::Message => String::from(r"(.+)"),
      Token::Literal(value) => String::from(value),
    }
  }

  fn from(value: &str) -> Self {
    match value {
      "%f" => Token::File,
      "%m" => Token::Message,
      "%l" => Token::Line,
      "%c" => Token::Column,
      "%k" => Token::Kind,
      value => Token::Literal(String::from(value)),
    }
  }

  fn is_known(val: &str) -> bool {
    Regex::new(r"^%[flckm]$").unwrap().is_match(val)
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      Token::File => write!(f, "%f"),
      Token::Line => write!(f, "%l"),
      Token::Column => write!(f, "%c"),
      Token::Kind => write!(f, "%k"),
      Token::Message => write!(f, "%m"),
      Token::Literal(value) => write!(f, "{}", value),
    }
  }
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

#[derive(Debug)]
struct Parser {
  errfmt: ErrFmt,
}

impl Parser {
  fn new(errfmt: String) -> Self {
    Parser {
      errfmt: tokenize_errfmt(errfmt)
        .into_iter()
        .fold(ErrFmt::new(), |acc, t| acc.push(Token::from(&t))),
    }
  }

  fn parse(&self, input: String) -> Vec<Entry> {
    let pattern = Regex::new(&self.errfmt.pattern()).unwrap();
    input.lines().fold(Vec::new(), |mut acc, line| {
      if let Some(matches) = pattern.captures(&line) {
        acc.push(Entry {
          file: string_match(&matches, 1),
          line: u32_match(&matches, 2),
          column: u32_match(&matches, 3),
          kind: string_match(&matches, 4),
          message: string_match(&matches, 5),
        })
      }
      acc
    })
  }
}

fn string_match(matches: &regex::Captures, i: usize) -> String {
  matches.get(i).unwrap().as_str().to_string()
}

fn u32_match(matches: &regex::Captures, i: usize) -> u32 {
  string_match(matches, i).parse::<u32>().unwrap()
}

fn tokenize_errfmt(errfmt: String) -> Vec<String> {
  errfmt.chars().fold(Vec::new(), |mut acc, c| {
    if should_split(&acc, c) {
      let mut new = String::new();
      new.push(c);
      acc.push(new);
    } else {
      acc.last_mut().unwrap().push(c);
    }
    acc
  })
}

fn should_split(acc: &[String], c: char) -> bool {
  acc.len() == 0 || c == '%' || Token::is_known(acc.last().unwrap())
}

#[derive(Debug)]
struct Entry {
  file: String,
  line: u32,
  column: u32,
  kind: String,
  message: String,
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_tokenize_errfmt_should_split_at_placeholders() {
    let input = String::from("foo: %fhello %m");
    let expected = vec![
      String::from("foo: "),
      String::from("%f"),
      String::from("hello "),
      String::from("%m"),
    ];
    let actual = tokenize_errfmt(input);
    assert_eq!(expected, actual);
  }

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
  fn test_parser_entry_filename() {
    let sut = Parser::new(String::from("Error: %f:%l:%c: %k: %m"));
    let entries = sut.parse(String::from("Error: /tmp/foo:42:42: warning: syntax error"));
    assert_eq!("/tmp/foo", &entries[0].file)
  }

  #[test]
  fn test_parser_entry_line() {
    let sut = Parser::new(String::from("Error: %f:%l:%c: %k: %m"));
    let entries = sut.parse(String::from("Error: /tmp/foo:42:42: warning: syntax error"));
    assert_eq!(42, entries[0].line)
  }

  #[test]
  fn test_parser_entry_column() {
    let sut = Parser::new(String::from("Error: %f:%l:%c: %k: %m"));
    let entries = sut.parse(String::from("Error: /tmp/foo:42:42: warning: syntax error"));
    assert_eq!(42, entries[0].column)
  }

  #[test]
  fn test_parser_entry_kind() {
    let sut = Parser::new(String::from("Error: %f:%l:%c: %k: %m"));
    let entries = sut.parse(String::from("Error: /tmp/foo:42:42: warning: syntax error"));
    assert_eq!("warning", &entries[0].kind)
  }

  #[test]
  fn test_parser_entry_message() {
    let sut = Parser::new(String::from("Error: %f:%l:%c: %k: %m"));
    let entries = sut.parse(String::from("Error: /tmp/foo:42:42: warning: syntax error"));
    assert_eq!("syntax error", &entries[0].message)
  }
}
