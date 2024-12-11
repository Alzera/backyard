use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, MagicNode, Node };

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct MagicParser;

impl MagicParser {
  pub fn from_token<'arena>(id: &Token) -> Node<'arena> {
    let loc = id.get_range_location();
    MagicNode::loc(id.value.to_owned(), loc)
  }

  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, tokens, &[Lookup::Equal(&[TokenType::Magic, TokenType::MagicMethod])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [text] = matched.as_slice() {
      return Ok(MagicNode::loc(text.as_equal()?.value.to_owned(), parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
