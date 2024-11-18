mod internal;
mod parser;
mod utils;
pub mod error;

use backyard_lexer::lex;
use error::{ ParserError, ParserResult };
use parser::{ LoopArgument, Parser };

pub fn parse(input: &str) -> ParserResult {
  match lex(input) {
    Ok(lexer) => {
      let mut parser = Parser::new(&lexer);
      let groups = parser.get_children(&mut LoopArgument::default("main"));
      Ok(groups)
    }
    Err(err) => Err(ParserError::LexError(err)),
  }
}
