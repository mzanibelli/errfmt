use regex::Regex;

/// Supported placeholders:
/// %f: filename
/// %l: line number
/// %c: column number
/// %k: error kind (warning or error)
/// %m: error message
/// %.: sequence of whitespace characters (including new lines)
/// %*: anything
/// ...every other sequence will be treated as literal.

/// Documentation of what works and has been tested so far...
pub const ESLINT_ERRFMT: &str = r"%.%f%.%l:%c  %k  %m";
pub const GOLINT_ERRFMT: &str = r"%f:%l:%c: %m";
pub const PASSTHROUGH_ERRFMT: &str = r"%f:%l:%c: %k: %m";
pub const PHP_ERRFMT: &str = r"%k: %m in %f on line %l";
pub const RUSTFMT_ERRFMT: &str = r"%k%*: %m%.--> %f:%l:%c";
pub const SHELLCHECK_ERRFMT: &str = r"%f:%l:%c: %k: %m";

/// Stream characters of the errorformat string and build logical sections
/// (tokens) from them.
pub fn tokenize(errfmt: String) -> Vec<String> {
  errfmt.chars().fold(Vec::new(), |mut acc, c| {
    if token_start(&acc, c) {
      let mut new = String::new();
      new.push(c);
      acc.push(new);
    } else {
      acc.last_mut().unwrap().push(c);
    }
    acc
  })
}

/// Guess if a given character must be added to the previous ongoing
/// token, or if it should be the first character of a new token.
fn token_start(acc: &[String], c: char) -> bool {
  match (acc.len(), c, acc.last()) {
    (0, _, _) => true,
    (_, '%', Some(last)) => last != "%",
    (_, _, Some(last)) => is_known_placeholder(last),
    _ => false,
  }
}

/// A "known" placeholder is a percent-sequence like: %f, %m, %%... etc.
fn is_known_placeholder(val: &str) -> bool {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^%[%flckm.*]$").unwrap();
  }
  RE.is_match(val)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_single_character() {
    let input = String::from("a");
    let expected = vec![String::from("a")];
    let actual = tokenize(input);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_single_literal_expression() {
    let input = String::from("hello world");
    let expected = vec![String::from("hello world")];
    let actual = tokenize(input);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_new_tokens_at_placeholders() {
    let input = String::from("foo: %fhello %m");
    let expected = vec![
      String::from("foo: "),
      String::from("%f"),
      String::from("hello "),
      String::from("%m"),
    ];
    let actual = tokenize(input);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_handling_of_literal_percent_sign() {
    let input = String::from("foo: %%bar");
    let expected = vec![
      String::from("foo: "),
      String::from("%%"),
      String::from("bar"),
    ];
    let actual = tokenize(input);
    assert_eq!(expected, actual);
  }
}
