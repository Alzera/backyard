use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, NodeType, StaticNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::{ comment::CommentParser, property::PropertyItemParser };

#[derive(Debug, Clone)]
pub struct StaticsParser;

impl StaticsParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    if let Some(last_expr) = &args.last_expr {
      if last_expr.node_type == NodeType::StaticKeyword {
        return match_pattern(
          tokens,
          &[Lookup::Equal(&[TokenType::Variable, TokenType::VariableBracketOpen])]
        );
      }
    }
    None
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      parser.position -= 1;
      let items = parser.get_children(
        &mut LoopArgument::new(
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
