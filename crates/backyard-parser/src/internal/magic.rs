use backyard_lexer::token::TokenType;
use backyard_nodes::{ Location, MagicMethodName, MagicMethodNode, MagicName, MagicNode, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct MagicParser;

impl MagicParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, &[Lookup::Equal(&[TokenType::Magic, TokenType::MagicMethod])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [name] = matched.as_slice() {
      let name = name.as_equal(parser)?;
      if name.token_type == TokenType::MagicMethod {
        return Ok(
          MagicMethodNode::loc(
            MagicMethodName::try_from(&name.value).map_err(|_| ParserError::Internal)?,
            parser.gen_loc(start_loc)
          )
        );
      }
      return Ok(
        MagicNode::loc(
          MagicName::try_from(&name.value).map_err(|_| ParserError::Internal)?,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
