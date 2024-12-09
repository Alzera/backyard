mod internal;
mod parser;
mod utils;
mod macros;
pub mod error;

use backyard_lexer::{ arena_lex, arena_lex_eval, error::LexError, token::Token };
use backyard_nodes::node::{ Location, Node, ProgramNode, RangeLocation };
use bumpalo::Bump;
use error::ParserError;
use parser::{ LocationHelper, LoopArgument, Parser };

pub fn parse(input: &str) -> Result<Node, ParserError> {
  let arena = Bump::new();
  let tokens = arena_lex(&arena, input);
  parse_base(&tokens)
}

pub fn parse_eval(input: &str) -> Result<Node, ParserError> {
  let arena = Bump::new();
  let tokens = arena_lex_eval(&arena, input);
  parse_base(&tokens)
}

pub fn parse_base(
  tokens: &Result<bumpalo::collections::Vec<Token>, LexError>
) -> Result<Node, ParserError> {
  match tokens {
    Ok(tokens) => {
      if tokens.is_empty() {
        return Err(ParserError::Eof);
      }
      let mut parser = Parser::new(&tokens);
      let parsed = parser.get_children(&mut LoopArgument::default("main"))?;
      let wrapped = ProgramNode::loc(
        parsed,
        Some(RangeLocation {
          start: Location { line: 1, column: 0, offset: 0 },
          end: tokens.last().unwrap().get_location().unwrap(),
        })
      );
      Ok(*wrapped)
    }
    Err(err) => Err(ParserError::LexError(err.to_owned())),
  }
}
