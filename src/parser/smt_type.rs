#[derive(Debug)]
pub enum SmtType {
  String,
  Int,
  Bool,
  Char
}

impl TryFrom<&str> for SmtType {
  type Error = ();

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "String" => Ok(SmtType::String),
      "Int" => Ok(SmtType::Int),
      "Bool" => Ok(SmtType::Bool),
      "Char" => Ok(SmtType::Char),
      _ => Err(())
    }
  }
}
