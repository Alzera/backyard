use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ GlobalNode, Location, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::{ comment::CommentParser, variable::VariableParser };

#[derive(Debug, Clone)]
pub struct GlobalParser;

impl GlobalParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Global])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let items = parser.get_children(
        &mut LoopArgument::new(
          "global",
          &[TokenType::Comma],
          &[TokenType::Semicolon],
          &[
            (CommentParser::test, CommentParser::parse),
            (VariableParser::test, VariableParser::parse),
          ]
        )
      )?;
      return Ok(GlobalNode::loc(items, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
