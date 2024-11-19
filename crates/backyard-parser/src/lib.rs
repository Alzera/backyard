mod internal;
mod parser;
mod utils;
pub mod error;

use backyard_lexer::lex;
use backyard_nodes::node::Node;
use error::ParserError;
use parser::{ LoopArgument, Parser };

pub fn parse(input: &str) -> Result<Vec<Box<Node>>, ParserError> {
  match lex(input) {
    Ok(lexer) => {
      let mut parser = Parser::new(&lexer);
      parser.get_children(&mut LoopArgument::default("main"))
    }
    Err(err) => Err(ParserError::LexError(err)),
  }
}
