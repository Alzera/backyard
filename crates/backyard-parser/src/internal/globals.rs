use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ GlobalNode, Location, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{ comment::CommentParser, property::PropertyItemParser };

#[derive(Debug, Clone)]
pub struct GlobalParser;

impl GlobalParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Global])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let items = parser.get_children(
        &mut LoopArgument::new(
          "global",
          &[TokenType::Comma],
          &[TokenType::Semicolon],
          &[
            (CommentParser::test, CommentParser::parse),
            (PropertyItemParser::test, PropertyItemParser::parse),
          ]
        )
      )?;
      return Ok(GlobalNode::new(items, parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("StaticLookup", args))
  }
}
