use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{ node::{ LabelNode, Location, Node, NodeType }, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct LabelParser;

impl LabelParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    args: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    if let Some(last) = &args.last_expr {
      if last.node_type != NodeType::Identifier {
        return None;
      }
    }
    match_pattern(parser, tokens, &[Lookup::Equal(&[TokenType::Colon])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      let name = args.last_expr.take().unwrap();
      let name_loc = if let Some(loc) = &name.loc.as_ref().map(|x| x.start.clone()) {
        loc.to_owned()
      } else {
        start_loc
      };
      return Ok(LabelNode::loc(name.into_boxed(&parser.arena), parser.gen_loc(name_loc)));
    }
    Err(ParserError::Internal)
  }
}
