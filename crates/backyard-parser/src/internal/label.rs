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
      let name = guard!(args.last_expr.to_owned(), {
        return Err(ParserError::internal("Label", args));
      });
      return Ok(LabelNode::new(name, parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("Label", args))
  }
}
