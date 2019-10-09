use regex::Regex;

pub fn tokenize(errfmt: String) -> Vec<String> {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tokenize_should_split_at_placeholders() {
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
}

fn should_split(acc: &[String], c: char) -> bool {
  acc.len() == 0 || c == '%' || not_literal(acc.last().unwrap())
}

fn not_literal(val: &str) -> bool {
  Regex::new(r"^%[flckm]$").unwrap().is_match(val)
}
