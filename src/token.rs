#[derive(Debug, Clone)]
pub enum Token {
  File,
  Line,
  Column,
  Kind,
  Message,
  Literal(String),
}

impl Token {
  pub fn pattern(&self) -> String {
    match &self {
      Self::File => String::from(r"(.+)"),
      Self::Line => String::from(r"(\d+)"),
      Self::Column => String::from(r"(\d+)"),
      Self::Kind => String::from(r"\b(warning|error)\b"),
      Self::Message => String::from(r"(.+)"),
      Self::Literal(value) => String::from(value),
    }
  }

  pub fn from(value: &str) -> Self {
    match value {
      "%f" => Self::File,
      "%m" => Self::Message,
      "%l" => Self::Line,
      "%c" => Self::Column,
      "%k" => Self::Kind,
      value => Self::Literal(String::from(value)),
    }
  }
}

