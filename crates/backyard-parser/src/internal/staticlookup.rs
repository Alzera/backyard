use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, StaticLookupNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct StaticLookupParser {}

impl StaticLookupParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::DoubleColon]),
        Lookup::Equal(vec![TokenType::Identifier, TokenType::Class]),
      ].to_vec()
    )
  }

  pub fn parse(
    _: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, prop] = matched.as_slice() {
      let on = guard!(args.last_expr.to_owned(), {
        return Err(ParserError::internal("StaticLookup", args));
      });
      return Ok(StaticLookupNode::new(on, IdentifierParser::from_matched(prop)));
    }
    Err(ParserError::internal("StaticLookup", args))
  }
}
