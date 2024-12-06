use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ BlockNode, Location, NamespaceNode, Node };

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

#[derive(Debug, Clone)]
pub struct NamespaceParser;

impl NamespaceParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
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
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, name] = matched.as_slice() {
      let name = if let LookupResultWrapper::Equal(name) = &name.wrapper {
        name.value.to_owned()
      } else {
        return Err(ParserError::Internal);
      };
      let is_bracket = if let Some(t) = parser.tokens.get(parser.position) {
        t.token_type == TokenType::LeftCurlyBracket
      } else {
        return Err(ParserError::Internal);
      };
      let block_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
      if is_bracket {
        parser.position += 1;
      }
      let body = BlockNode::loc(
        parser.get_children(&mut LoopArgument::default("block_parser"))?,
        parser.gen_loc(block_loc)
      );
      return Ok(NamespaceNode::loc(name, body, is_bracket, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
