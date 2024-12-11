mod internal;
mod parser;
mod utils;
mod macros;
pub mod error;

use backyard_lexer::{ lex, lex_eval, error::LexError, token::Token };
use backyard_nodes::node::{ Location, Node, ProgramNode, RangeLocation };
use bumpalo::Bump;
use error::ParserError;
use parser::{ LocationHelper, LoopArgument, Parser };

pub fn parse<'arena>(arena: &'arena Bump, input: &str) -> Result<Node<'arena>, ParserError> {
  let tokens = lex(arena, input);
  parse_base(arena, &tokens)
}

pub fn parse_eval<'arena>(arena: &'arena Bump, input: &str) -> Result<Node<'arena>, ParserError> {
  let tokens = lex_eval(arena, input);
  parse_base(arena, &tokens)
}

pub fn parse_base<'arena>(
  arena: &'arena Bump,
  tokens: &Result<bumpalo::collections::Vec<'arena, Token>, LexError>
) -> Result<Node<'arena>, ParserError> {
  match tokens {
    Ok(tokens) => {
      if tokens.is_empty() {
        return Err(ParserError::Eof);
      }
      let mut parser = Parser::new(arena, tokens);
      Ok(
        ProgramNode::loc(
          parser.get_children(&mut LoopArgument::default(arena, "main"))?,
          Some(RangeLocation {
            start: Location { line: 1, column: 0, offset: 0 },
            end: tokens.last().unwrap().get_location().unwrap(),
          })
        )
      )
    }
    Err(err) => Err(ParserError::LexError(err.to_owned())),
  }
}
