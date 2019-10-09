use regex::Regex;

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

fn token_start(acc: &[String], c: char) -> bool {
  match (acc.len(), c, acc.last()) {
    (0, _, _) => true,
    (_, '%', Some(last)) => last != "%",
    (_, _, Some(last)) => last == "%%" || not_literal(last),
    _ => false,
  }
}

fn not_literal(val: &str) -> bool {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^%[a-z]$").unwrap();
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
