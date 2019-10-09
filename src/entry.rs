use std::fmt;

#[derive(Debug)]
pub struct Entry {
  pub file: String,
  pub line: u32,
  pub column: u32,
  pub kind: Kind,
  pub message: String,
}

impl Entry {
  pub fn new() -> Self {
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
pub enum Kind {
  Warning,
  Error,
}

impl Kind {
  pub fn from(value: &str) -> Self {
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
