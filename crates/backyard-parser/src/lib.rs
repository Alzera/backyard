mod internal;
mod parser;
mod utils;

use backyard_lexer::lex;
use backyard_nodes::node::Node;
use parser::{ LoopArgument, Parser };

pub fn parse(input: &str) -> Vec<Box<Node>> {
  let lexer = lex(input);
  let mut parser = Parser::new(&lexer);
  let groups = parser.get_children(&mut LoopArgument::default("main"));

  groups
}
