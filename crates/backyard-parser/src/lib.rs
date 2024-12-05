mod internal;
mod parser;
mod utils;
mod guards;
pub mod error;

use backyard_lexer::{ error::LexError, lex, lex_eval, token::Token };
use backyard_nodes::node::{ Location, Node, ProgramNode, RangeLocation };
use error::ParserError;
use parser::{ LocationHelper, LoopArgument, Parser };

pub fn parse(input: &str) -> Result<Box<Node>, ParserError> {
  parse_base(lex(input))
}

pub fn parse_eval(input: &str) -> Result<Box<Node>, ParserError> {
  parse_base(lex_eval(input))
}

fn parse_base(tokens: Result<Vec<Token>, LexError>) -> Result<Box<Node>, ParserError> {
  match tokens {
    Ok(tokens) => {
      if tokens.is_empty() {
        return Err(ParserError::Failed("Empty tokens".to_string()));
      }
      let mut parser = Parser::new(&tokens);
      let parsed = parser.get_children(&mut LoopArgument::default("main"))?;
      let wrapped = ProgramNode::new(
        parsed,
        Some(RangeLocation {
          start: Location { line: 1, column: 0, offset: 0 },
          end: tokens.last().unwrap().get_location().unwrap(),
        })
      );
      Ok(wrapped)
    }
    Err(err) => Err(ParserError::LexError(err)),
  }
}
