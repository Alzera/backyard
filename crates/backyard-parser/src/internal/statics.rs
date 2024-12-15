use backyard_lexer::token::TokenType;
use backyard_nodes::{ Location, Node, NodeType, StaticNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::{ comment::CommentParser, property::PropertyItemParser };

#[derive(Debug, Clone)]
pub struct StaticsParser;

impl StaticsParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    args: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    if let Some(last_expr) = &args.last_expr {
      if last_expr.node_type == NodeType::StaticKeyword {
        return match_pattern(
          parser,
          &[Lookup::Equal(&[TokenType::Variable, TokenType::VariableBracketOpen])]
        );
      }
    }
    None
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      parser.position -= 1;
      let items = parser.get_children(
        &mut LoopArgument::new(
          parser.arena,
          "static",
          &[TokenType::Comma],
          &[TokenType::Semicolon],
          &[
            (CommentParser::test, CommentParser::parse),
            (PropertyItemParser::test, PropertyItemParser::parse),
          ]
        )
      )?;
      return Ok(StaticNode::loc(items, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
