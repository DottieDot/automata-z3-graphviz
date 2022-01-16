use petgraph::graph::{DiGraph, NodeIndex};
use std::{fmt, borrow::Borrow, collections::{HashMap, hash_map::Entry}};

use crate::parser::{Expression, Model, SmtType};

#[derive(Debug)]
pub struct Error {
  message: String
}

impl Error {
  pub fn new(message: &str) -> Self {
    Self {
      message: message.to_string()
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let message = &self.message;
    write!(f, "invalid input provided ({message})")
  }
}

fn get_node_from_eq(eq: &Vec<Expression>) -> Result<i32, Error> {
  if let [Expression::Identifier(_), Expression::Integer(node_id)] = &eq[..] {
    Ok(*node_id)
  } else {
    return Err(Error::new("malformed eq expression"))
  }
}

fn get_label_for_edge(eq: &Vec<Expression>) -> Result<char, Error> {
  if let [Expression::Identifier(_), Expression::FunctionCall(_, params)] = &eq[..] {
    if let [Expression::Cast(SmtType::Char, char_code)] = params[..] {
      let char = char::from(char_code as u8);
      Ok(char)
    } else {
      Err(Error::new("missing label for edge"))
    }
  } else {
    return Err(Error::new("malformed eq expression"))
  }
}

fn graph_from_ite_chain(ite_chain: &Expression) -> Result<DiGraph<i32, String>, Error> {
  let mut graph: DiGraph<i32, String> = Default::default();
  let mut nodes: HashMap<i32, NodeIndex<u32>> = Default::default();
  let mut next: &Expression = ite_chain;
  while let Expression::Ite(a, _, b) = next {
    if let Expression::And(expressions) = a.borrow() {
      if let [Expression::Eq(a), Expression::Eq(b), Expression::Eq(label)] = &expressions[..] {
        let node_a = get_node_from_eq(a)?;
        let node_b = get_node_from_eq(b)?;
        let edge_label = get_label_for_edge(label)?;

        let a_id: NodeIndex<u32> = match nodes.entry(node_a) {
          Entry::Occupied(o) => *o.into_mut(),
          Entry::Vacant(v) => *v.insert(graph.add_node(node_a))
        };
        let b_id: NodeIndex<u32> = match nodes.entry(node_b) {
          Entry::Occupied(o) => *o.into_mut(),
          Entry::Vacant(v) => *v.insert(graph.add_node(node_b))
        };

        graph.add_edge(a_id, b_id, edge_label.to_string());
      } else {
        return Err(Error::new("malformed and expression"))
      }
    } else {
      return Err(Error::new("malformed ite expression"))
    }
    next = b;
  }

  Ok(graph)
}

pub fn graph_from_model(model: Model) -> Result<DiGraph<i32, String>, Error> {
  if let Some(function) = model.iter().find(|function| function.name == "A") {
    graph_from_ite_chain(function.body.borrow())
  } else {
    Err(Error::new("Model is missing `A` function"))
  }
}
