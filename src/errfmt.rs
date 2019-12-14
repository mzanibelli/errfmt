use regex::Regex;

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

/// A "known" placeholder is a percent-sequence like: %s, %d, %%... etc.
fn is_known_placeholder(val: &str) -> bool {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^%[%a-z.]$").unwrap();
  }
  RE.is_match(val)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_new_tokens_at_placeholders() {
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
