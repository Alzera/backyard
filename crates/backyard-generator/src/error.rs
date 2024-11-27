use std::fmt::{ Display, Formatter };

#[derive(Debug, Clone, PartialEq)]
pub enum GeneratorError {
  NotAProgram,
}

impl Display for GeneratorError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      GeneratorError::NotAProgram => write!(f, "Not a program"),
    }
  }
}
