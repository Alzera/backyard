use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, NamespaceNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct NamespaceParser;

impl NamespaceParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[
        Lookup::Equal(&[TokenType::Namespace]),
        Lookup::Equal(&[TokenType::Identifier, TokenType::Name]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, name] = matched.as_slice() {
      let name = guard!(name.first(), {
        return Err(ParserError::internal("Namespace", args));
      }).value.to_owned();
      let is_bracket = if let Some(t) = parser.tokens.get(parser.position) {
        // parser.position -= 1;
        t.token_type == TokenType::LeftCurlyBracket
      } else {
        false
      };
      let body = BlockParser::new(parser)?;
      return Ok(NamespaceNode::new(name, body, is_bracket, parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("Namespace", args))
  }
}
