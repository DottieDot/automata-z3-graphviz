use super::SmtType;

#[derive(Debug)]
pub enum Expression {
  Integer(i32),
  Boolean(bool),
  String(String),
  Parentheses(Box<Expression>),
  Eq(Vec<Expression>),
  And(Vec<Expression>),
  Ite(Box<Expression>, Box<Expression>, Box<Expression>),
  FunctionCall(String, Vec<Expression>),
  Cast(SmtType, i32),
  Identifier(String)
}
