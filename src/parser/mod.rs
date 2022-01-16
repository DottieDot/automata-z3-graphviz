use peg;

mod smt_type;
pub use smt_type::*;
mod expression;
pub use expression::*;

pub type Parameter = (String, SmtType);

#[derive(Debug)]
pub struct Function {
  pub name:      String,
  pub params:    Vec<Parameter>,
  pub type_name: SmtType,
  pub body:      Box<Expression>
}

pub type Model = Vec<Function>;

peg::parser! {
  pub grammar smt_parser() for str {
    pub rule model() -> Model
      = "sat" _ "(" _ functions:((i:function() _ {i})*) _ ")" _ {
        functions
      }

    rule expression() -> Expression
      = integer()
      / boolean()
      / string()
      / eq()
      / and()
      / ite()
      / cast()
      / parentheses()
      / identifier_expression()
      / function_call()

    rule function() -> Function
      = "(" _ "define-fun" _ name:identifier() _ "(" _ params:((i:parameter() _ {i})*) _ ")" _ type_name:type_name() _ body:expression() _ ")" {
        Function { name, params, type_name, body: Box::new(body) }
      }

    rule function_call() -> Expression
      = "(" _ name:identifier() _ expressions:((i:expression() _ {i})+) _ ")" {
        Expression::FunctionCall(name, expressions)
      }

    rule parameter() -> Parameter
      = "(" _ name:(i:identifier()) _ type_name:(i:type_name()) _ ")" {
        (name, type_name)
      }

    rule eq() -> Expression
      = "(" _ "=" _ expressions:((i:expression() _ {i})+) _ ")" {
        Expression::Eq(expressions)
      }

    rule and() -> Expression
      = "(" _ "and" _ expressions:((i:expression() _ {i})+) _ ")" {
        Expression::And(expressions)
      }

    rule ite() -> Expression
      = "(" _ "ite" _ cond:expression() _ a:expression() _ b:expression() _ ")" {
        Expression::Ite(Box::new(cond), Box::new(a), Box::new(b))
      }

    rule cast() -> Expression
      = "(" _ "_" _ type_name:type_name() _ value:raw_integer() _ ")" {
        Expression::Cast(type_name, value)
      }

    rule parentheses() -> Expression
      = "(" _ e:expression() _ ")" {
        Expression::Parentheses(Box::new(e))
      }

    rule integer() -> Expression
      = i:raw_integer() { Expression::Integer(i) }

    rule boolean() -> Expression
      = i:raw_boolean() { Expression::Boolean(i) }

    rule string() -> Expression
      = s:string_literal() { Expression::String(s) }

    rule identifier_expression() -> Expression
      = i:identifier() { Expression::Identifier(i) }

    rule type_name() -> SmtType
      = n:$("Bool"/"Int"/"String"/"Char") {
        ? SmtType::try_from(n).or(Err("SmtType"))
      }

    rule raw_integer() -> i32
      = n:$(['-']?['0'..='9']+) {
        ? n.parse().or(Err("i32"))
      }

    rule raw_boolean() -> bool
      = n:$("true" / "false") {
        ? n.parse().or(Err("bool"))
      }

    rule string_literal() -> String
      = quiet!{"\"" n:$([^'"']*) "\"" {
        n.to_owned()
      }}
      / expected!("string literal")

    rule identifier() -> String
      = quiet!{ n:$(['a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '!' | '.']*) {
        n.to_owned()
      }}
      / expected!("identifier")

    rule _() =  quiet!{[' ' | '\t' | '\r' | '\n']*}
  }
}
