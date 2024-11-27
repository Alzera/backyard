mod internal;
mod parser;
mod utils;
mod guards;
pub mod error;

use backyard_lexer::{ lex, lex_eval };
use backyard_nodes::node::Node;
use error::ParserError;
use parser::{ LoopArgument, Parser };

pub fn parse(input: &str) -> Result<Vec<Box<Node>>, ParserError> {
  match lex(input) {
    Ok(tokens) => {
      let mut parser = Parser::new(&tokens);
      parser.get_children(&mut LoopArgument::default("main"))
    }
    Err(err) => Err(ParserError::LexError(err)),
  }
}

pub fn parse_eval(input: &str) -> Result<Vec<Box<Node>>, ParserError> {
  match lex_eval(input) {
    Ok(tokens) => {
      let mut parser = Parser::new(&tokens);
      parser.get_children(&mut LoopArgument::default("main"))
    }
    Err(err) => Err(ParserError::LexError(err)),
  }
}
