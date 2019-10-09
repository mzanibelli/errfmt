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

const KIND_WARNING: &str = "warning";
const KIND_ERROR: &str = "error";

impl Kind {
  pub fn from(value: &str) -> Self {
    match value {
      KIND_WARNING => Kind::Warning,
      KIND_ERROR => Kind::Error,
      value => panic!("unexpected kind: {}", value),
    }
  }
}

impl fmt::Display for Kind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Kind::Warning => write!(f, "{}", KIND_WARNING),
      Kind::Error => write!(f, "{}", KIND_ERROR),
    }
  }
}
