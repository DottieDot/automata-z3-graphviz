use super::SmtType;

#[derive(Debug)]
pub enum Expression {
  Integer(i32),
  Boolean(bool),
  String(String),
  Parentheses(Vec<Expression>),
  Eq(Vec<Expression>),
  And(Vec<Expression>),
  Ite(Box<Expression>, Box<Expression>, Box<Expression>),
  FunctionCall(String, Vec<Expression>),
  Cast(SmtType, i32),
  Identifier(String),
  Not(Box<Expression>),
  Or(Vec<Expression>),
  Add(Vec<Expression>),
  Subtract(Vec<Expression>),
  GreaterOrEqual(Vec<Expression>),
  LowerOrEqual(Vec<Expression>),
}
