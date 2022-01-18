use clap::Parser;
use std::{process::{Command, Stdio}, io::{Write, Read}, fs::File};

mod graph;
mod parser;

use parser::smt_parser;
use petgraph::dot::Dot;
use graph::graph_from_model;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(long, help = "The smt file to run z3 on", conflicts_with = "z3", required_unless_present = "z3")]
  smt: Option<String>,

  #[clap(long, help = "The z3 output to use", conflicts_with = "smt", required_unless_present = "smt")]
  z3: Option<String>,

  #[clap(long, short, help = "The output file")]
  output: String,

  #[clap(long, short, help = "Wether or not to print the model", takes_value = false)]
  model: bool
}

fn main() {
  let args = Args::parse();

  let mut input: String = "".to_string();

  if let Some(smt_file) = args.smt {
    let raw_input = Command::new("z3")
      .arg(smt_file)
      .output()
      .expect("failed to start z3");
    input = String::from_utf8(raw_input.stdout)
      .expect("z3 output not a utf-8 string");
  } else if let Some(z3_file) = args.z3 {
    let mut file = File::open(z3_file).expect("Failed to open {z3_file}");
    file.read_to_string(&mut input).expect("Failed to read z3 file");
  } else {
    panic!("No input provided")
  }

  println!("Parsing \"{input}\":");
  
  let model = smt_parser::model(&input)
    .map_err(|e| panic!("{}", e))
    .unwrap();

  if args.model {
    println!("\n\nmodel");
    println!("{}", String::from_utf8(vec![b'='; 64]).unwrap());
    
    println!("{model:#?}");
  }

  println!("\n\ndot");
  println!("{}", String::from_utf8(vec![b'='; 64]).unwrap());
  
  let graph = graph_from_model(model)
    .map_err(|e| panic!("{}", e))
    .unwrap();
  let dot = Dot::new(&graph);
  let dot_string = format!("{dot:?}");
  println!("{dot_string}");

  let mut graphviz = Command::new("dot")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .arg("-Tpng")
    .arg("-o")
    .arg(&args.output)
    .spawn()
    .expect("Failed to start GraphViz");

  let stdin = graphviz.stdin.as_mut().unwrap();
  stdin.write(dot_string.as_bytes()).unwrap();
  drop(stdin);
  graphviz.wait_with_output()
    .map_err(|e| panic!("{e}"))
    .unwrap();
  
  opener::open(&args.output).expect("Failed to open output image");
}
