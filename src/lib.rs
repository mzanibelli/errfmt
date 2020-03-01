//! # Crate `errfmt`
//!
//! ## Motivation
//!
//! Most editors support navigating a list of errors often resulting from
//! a failed compilation (or syntax check, or linting tool...). Every
//! editor expects a different list format and this program aims
//! to re-shape any error message and make it compatible with
//! [Kakoune](https://github.com/mawww/kakoune)'s `lint.kak` standard
//! rc script.
//!
//! ## Usage
//!
//! `errfmt(1)` works by parsing error messages that match a pre-defined
//! format. This format is specified with the `--errfmt` flag. It
//! must be a string composed with placeholders where actual data is
//! expected. Different helper placeholders can be used to skip parts
//! of the messages that are not useful.
//!
//! ### CLI examples
//!
//! - Format error messages from PHP syntax checking tool: `php -l myfile.php | errfmt -e '%k: %m in %f on line %l'`
//! - Make sure the file path is correct when input comes from STDIN: `cat myfile.php | php -l | errfmt -e '%k: %m in %f on line %l' -f myfile.php`
//!
//! ### Supported placeholders:
//! - `%f`: filename
//! - `%l`: line number
//! - `%c`: column number
//! - `%k`: error kind (warning or error)
//! - `%m`: error message
//! - `%.`: sequence of whitespace characters (including new lines)
//! - `%*`: anything
//! - ...every other sequence will be treated as literal.

#[macro_use]
extern crate lazy_static;

use regex::Captures;
use regex::Error;
use regex::Match;
use regex::Regex;
use std::convert::TryInto;

mod entry;
mod errfmt;
mod shape;
mod token;

use entry::Entry;
use entry::Kind;
use shape::Shape;
use token::Token;

pub use crate::errfmt::ESLINT_ERRFMT;
pub use crate::errfmt::GOLINT_ERRFMT;
pub use crate::errfmt::PASSTHROUGH_ERRFMT;
pub use crate::errfmt::PHP_ERRFMT;
pub use crate::errfmt::RUSTFMT_ERRFMT;
pub use crate::errfmt::SHELLCHECK_ERRFMT;

/// Entrypoint of the program: configure the errorformat string and
/// de-facto filename then filter input to re-shape it into the expected
/// format.
///
/// # Example: simple error message
///
/// ```
/// let messages = errfmt::run(
///   String::from("/tmp/myfile error on line 3 column 1: syntax error"),
///   String::from("%f %k on line %l column %c: %m"),
///   String::new() // this must be empty when not used
/// );
/// assert_eq!(String::from("/tmp/myfile:3:1: error: syntax error"), messages.unwrap()[0]);
/// ```
///
/// # Example: replace filenames with static value
///
/// ```
/// let messages = errfmt::run(
///   String::from("/tmp/myfile error on line 3 column 1: syntax error"),
///   String::from("%f %k on line %l column %c: %m"),
///   String::from("/tmp/anotherfile") // this will replace any filename in resulting output
/// );
/// assert_eq!(String::from("/tmp/anotherfile:3:1: error: syntax error"), messages.unwrap()[0]);
/// ```
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
        .map(Token::from)
        .fold(Shape::new(), |acc, t| acc.push(t)),
      file,
    }
  }

  /// Build the resulting pattern from the shape and gather the list of
  /// entries matching an error message.
  fn parse(&self, input: String) -> Result<Vec<Entry>, Error> {
    self.shape.clone().try_into().map(|r: Regex| {
      r.captures_iter(&input)
        .map(|matches| self.build_entry(&matches))
        .collect()
    })
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
    let input = String::from("/tmp/myfile: error on line 7: invalid syntax\n");
    let sut = Parser::new(String::from("%f: %k on line %l: %m"), String::new());
    let entries = sut.parse(input).unwrap();
    assert_eq!(
      "/tmp/myfile:7:1: error: invalid syntax",
      &entries[0].to_string()
    )
  }

  #[test]
  fn test_multiple_entries_with_single_line_mode() {
    let input = vec![
      String::from("/tmp/myfile: error on line 7: invalid syntax"),
      String::from("\n"),
      String::from("/tmp/anotherfile: error on line 7: invalid syntax"),
      String::from("\n"),
    ]
    .join("");
    let sut = Parser::new(String::from("%f: %k on line %l: %m%."), String::new());
    let entries = sut.parse(input).unwrap();
    assert_eq!(
      "/tmp/anotherfile:7:1: error: invalid syntax",
      &entries[1].to_string()
    )
  }

  #[test]
  fn test_multi_line_mode() {
    let input = vec![
      String::from("/tmp/myfile"),
      String::from("\n"),
      String::from("13:37"),
      String::from("\n"),
    ]
    .join("");
    let sut = Parser::new(String::from("%f%.%l:%c%."), String::new());
    let entries = sut.parse(input).unwrap();
    assert_eq!("/tmp/myfile:13:37: error: ", &entries[0].to_string())
  }

  #[test]
  fn test_multiples_entries_with_multi_line_mode() {
    let input = vec![
      String::from("/tmp/myfile"),
      String::from("\n"),
      String::from("13:37"),
      String::from("\n"),
      String::from("/tmp/anotherfile"),
      String::from("\n"),
      String::from("13:37"),
      String::from("\n"),
    ]
    .join("");
    let sut = Parser::new(String::from("%f%.%l:%c%."), String::new());
    let entries = sut.parse(input).unwrap();
    assert_eq!("/tmp/anotherfile:13:37: error: ", &entries[1].to_string())
  }

  #[test]
  fn test_filename_must_override_extracted_value() {
    let sut = Parser::new(String::from("%f"), String::from("/etc/shadow"));
    let entries = sut.parse(String::from("/tmp/myfile")).unwrap();
    assert_eq!("/etc/shadow:1:1: error: ", &entries[0].to_string())
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
