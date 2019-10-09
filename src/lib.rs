use regex::Regex;
use std::fmt;

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

fn string_match(matches: &regex::Captures, i: usize) -> String {
  matches.get(i).unwrap().as_str().to_string()
}

fn u32_match(matches: &regex::Captures, i: usize) -> u32 {
  string_match(matches, i).parse::<u32>().unwrap()
}

fn should_split(acc: &[String], c: char) -> bool {
  acc.len() == 0 || c == '%' || Token::is_known(acc.last().unwrap())
}

#[derive(Debug)]
struct Entry {
  file: String,
  line: u32,
  column: u32,
  kind: Kind,
  message: String,
}

impl Entry {
  fn new() -> Self {
    Entry {
      file: String::new(),
      line: 1,
      column: 1,
      kind: Kind::Error,
      message: String::new(),
    }
  }
}

impl fmt::Display for Entry {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}:{}:{}: {}: {}",
      self.file, self.line, self.column, self.kind, self.message
    )
  }
}

#[derive(Debug)]
enum Kind {
  Warning,
  Error,
}

impl Kind {
  fn from(value: &str) -> Self {
    match value {
      "warning" => Kind::Warning,
      "error" => Kind::Error,
      value => panic!(format!("unexpected kind: {}", value)),
    }
  }
}

impl fmt::Display for Kind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Kind::Warning => write!(f, "warning"),
      Kind::Error => write!(f, "error"),
    }
  }
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
    assert_eq!(Kind::Warning.to_string(), entries[0].kind.to_string())
  }

  #[test]
  fn test_parser_entry_message() {
    let sut = Parser::new(String::from("Error: %f:%l:%c: %k: %m"));
    let entries = sut.parse(String::from("Error: /tmp/foo:42:42: warning: syntax error"));
    assert_eq!("syntax error", &entries[0].message)
  }

  #[test]
  fn test_parser_should_keep_matching_groups_only() {
    let sut = Parser::new(String::from("PHP Parse error: %m in %f on line %l"));
    let entries = sut.parse(String::from("PHP Parse error:  syntax error, unexpected end of file, expecting ',' or ';' in /tmp/test.php on line 4"));
    assert_eq!(1, entries.len())
  }
}
