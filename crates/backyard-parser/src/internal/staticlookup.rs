use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ClassKeywordNode, Node, StaticLookupNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct StaticLookupParser {}

impl StaticLookupParser {
  pub fn test(tokens: &Vec<Token>, args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if args.last_expr.is_none() {
      return None;
    }
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::DoubleColon])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let left = args.last_expr.to_owned().unwrap();
      args.last_expr = None;
      if let Some(t) = parser.tokens.get(parser.position) {
        parser.position += 1;
        let expr = if t.token_type == TokenType::Class {
          ClassKeywordNode::new()
        } else {
          IdentifierParser::new(t.value.to_owned())
        };
        return Ok(StaticLookupNode::new(left, expr));
      };
    }
    Err(ParserError::internal("StaticLookup", args))
  }
}
