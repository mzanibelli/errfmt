#[macro_use]
extern crate lazy_static;

use regex::Captures;
use regex::Error;
use regex::Match;

mod entry;
mod errfmt;
mod token;

use entry::Entry;
use entry::Kind;
use token::Shape;
use token::Token;

/// Entrypoint of the program: configure the errorformat string and
/// de-facto filename then filter input to re-shape it into the expected
/// format.
pub fn run(input: String, errfmt: String, file: String) -> Result<Vec<String>, String> {
  Ok(
    Parser::new(errfmt, file)
      .parse(input)
      .map_err(|err| err.to_string())?
      .iter()
      .map(|entry| entry.to_string())
      .collect(),
  )
}

/// Parser is responsible for building a set of entries matching the
/// extracted error messages.
#[derive(Debug)]
struct Parser {
  shape: Shape,
  file: String,
}

impl Parser {
  /// Read the configuration (errorformat string) and compute the shape
  /// of an error message.
  fn new(errfmt: String, file: String) -> Self {
    Parser {
      shape: errfmt::tokenize(errfmt)
        .into_iter()
        .fold(Shape::new(), |acc, t| acc.push(Token::from(&t))),
      file,
    }
  }

  /// Build the resulting pattern from the shape and gather the list of
  /// entries matching an error message.
  fn parse(&self, input: String) -> Result<Vec<Entry>, Error> {
    Ok(
      self
        .shape
        .pattern()?
        .captures_iter(&input)
        .map(|matches| self.build_entry(&matches))
        .collect(),
    )
  }

  /// Add a new location to the result set by reading its data from
  /// capture groups.
  fn build_entry(&self, matches: &Captures) -> Entry {
    self
      .shape
      .iter()
      .enumerate()
      // Ignore the first match as it is the entire string.
      .map(|(n, token)| (matches.get(n + 1), token))
      .fold(Entry::new(), |entry, (group, token)| {
        self.mutate_entry(entry, token, group)
      })
  }

  /// Update a given entry according to the corresponding token.
  /// Given filename overrides any extracted data in case the linter
  /// cannot handle this. This function will easily panic in case there
  /// is no matching capture group or if the data could not be converted
  /// to an integer in the appropriate cases.
  fn mutate_entry(&self, mut entry: Entry, token: &Token, data: Option<Match>) -> Entry {
    let parse_str = || data.unwrap().as_str();
    let parse_u32 = || parse_str().parse::<u32>().unwrap();
    match token {
      Token::File => {
        entry.file = if String::is_empty(&self.file) {
          String::from(parse_str())
        } else {
          String::from(&self.file)
        }
      }
      Token::Column => entry.column = parse_u32(),
      Token::Kind => entry.kind = Kind::from(parse_str()),
      Token::Line => entry.line = parse_u32(),
      Token::Message => entry.message = String::from(parse_str()),
      Token::Whitespace | Token::Wildcard | Token::Literal(_) => (),
    };
    entry
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const PHP_ERRFMT: &str = r"%k: %m in %f on line %l";
  const RUST_ERRFMT: &str = r"%k%*: %m%.--> %f:%l:%c";

  #[test]
  fn test_parser_from_empty_errfmt() {
    let actual = Parser::new(String::new(), String::new()).shape.0.len();
    let expected = 0;
    assert_eq!(expected, actual)
  }

  #[test]
  fn test_parser_should_have_an_entry_if_it_matches() {
    let sut = Parser::new(String::from("Error: %f:%l:%c: %k: %m"), String::new());
    let entries = sut
      .parse(String::from("Error: /tmp/foo:42:42: warning: syntax error"))
      .unwrap();
    assert_eq!(1, entries.len())
  }

  #[test]
  fn test_single_line_mode() {
    let sut = Parser::new(String::from(PHP_ERRFMT), String::new());
    let entries = sut.parse(String::from("PHP Parse error:  syntax error, unexpected end of file, expecting ',' or ';' in /tmp/test.php on line 4")).unwrap();
    assert_eq!(
      "/tmp/test.php:4:1: error:  syntax error, unexpected end of file, expecting ',' or ';'",
      &entries[0].to_string()
    )
  }

  #[test]
  fn test_multiple_entries_with_single_line_mode() {
    let sut = Parser::new(String::from(PHP_ERRFMT), String::new());
    let entries = sut.parse(String::from(r"PHP Parse error:  syntax error, unexpected end of file, expecting ',' or ';' in /tmp/test.php on line 1
PHP Parse error:  syntax error, unexpected end of file, expecting ',' or ';' in /tmp/test.php on line 2")).unwrap();
    assert_eq!(
      "/tmp/test.php:2:1: error:  syntax error, unexpected end of file, expecting ',' or ';'",
      &entries[1].to_string()
    )
  }

  #[test]
  fn test_multi_line_mode() {
    let sut = Parser::new(String::from(RUST_ERRFMT), String::new());
    let entries = sut
      .parse(String::from(
        r"error: unexpected close delimiter: `}`
  --> /tmp/test.rs:85:1
   |
85 | }
   | ^ unexpected close delimiter",
      ))
      .unwrap();
    assert_eq!(
      "/tmp/test.rs:85:1: error: unexpected close delimiter: `}`",
      &entries[0].to_string()
    )
  }

  #[test]
  fn test_multiples_entries_with_multi_line_mode() {
    let sut = Parser::new(String::from(RUST_ERRFMT), String::new());
    let entries = sut
      .parse(String::from(
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
      ))
      .unwrap();
    assert_eq!(
      "/tmp/test.rs:2:1: error: unexpected close delimiter: `}`",
      &entries[1].to_string()
    )
  }

  #[test]
  fn test_filename_must_override_extracted_value() {
    let sut = Parser::new(String::from(PHP_ERRFMT), String::from("/etc/shadow"));
    let entries = sut.parse(String::from("PHP Parse error:  syntax error, unexpected end of file, expecting ',' or ';' in /tmp/test.php on line 4")).unwrap();
    assert_eq!(
      "/etc/shadow:4:1: error:  syntax error, unexpected end of file, expecting ',' or ';'",
      &entries[0].to_string()
    )
  }

  #[test]
  fn test_wildcard_before_placeholders_must_consume_any_single_line_message() {
    let sut = Parser::new(String::from("%k%*: %m"), String::new());
    let entries = sut
      .parse(String::from("error[zzz]:  syntax error"))
      .unwrap();
    assert_eq!(":1:1: error:  syntax error", &entries[0].to_string())
  }

  #[test]
  fn test_wildcard_before_placeholders_must_not_be_greedy() {
    let sut = Parser::new(String::from("%k%*: %m"), String::new());
    let entries = sut.parse(String::from("error: syntax error: foo")).unwrap();
    assert_eq!("syntax error: foo", entries[0].message)
  }
}
