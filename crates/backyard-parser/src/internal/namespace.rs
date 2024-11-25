use std::vec;

use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, NamespaceNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct NamespaceParser {}

impl NamespaceParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Namespace]),
        Lookup::Equal(vec![TokenType::Identifier, TokenType::Name]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, name] = matched.as_slice() {
      let name = guard!(name.get(0), {
        return Err(ParserError::internal("Namespace", args));
      }).value.to_owned();
      let is_bracket = if let Some(t) = parser.tokens.get(parser.position) {
        let is_bracket = t.token_type == TokenType::LeftCurlyBracket;
        // parser.position -= 1;
        is_bracket
      } else {
        false
      };
      let body = BlockParser::new(parser)?;
      return Ok(NamespaceNode::new(name, body, is_bracket));
    }
    Err(ParserError::internal("Namespace", args))
  }
}
