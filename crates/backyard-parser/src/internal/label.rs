use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ LabelNode, Location, Node, NodeType };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct LabelParser;

impl LabelParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    if let Some(last) = &args.last_expr {
      if last.node_type != NodeType::Identifier {
        return None;
      }
    }
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Colon])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let name = guard!(args.last_expr.to_owned());
      let name_loc = if let Some(loc) = &name.loc.as_ref().map(|x| x.start.clone()) {
        loc.to_owned()
      } else {
        start_loc
      };
      return Ok(LabelNode::loc(name, parser.gen_loc(name_loc)));
    }
    Err(ParserError::Internal)
  }
}
