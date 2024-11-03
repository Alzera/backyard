pub mod node;
mod internal;
mod parser;
mod utils;
mod nodes;

use node::Nodes;
use parser::LoopArgument;

use crate::lexer::lex;
use crate::parser::parser::Parser;

pub fn parse(input: String) -> Nodes {
  let lexer = lex(input);
  let mut parser = Parser::new(&lexer);
  let groups = parser.get_children(&mut LoopArgument::default("main"));

  groups
}
