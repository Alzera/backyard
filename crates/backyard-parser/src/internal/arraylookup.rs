use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ArrayLookupNode, Location, Node, NodeType };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct ArrayLookupParser;

impl ArrayLookupParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    if let Some(last_expr) = &args.last_expr {
      if
        [
          NodeType::This,
          NodeType::Variable,
          NodeType::StaticLookup,
          NodeType::ArrayLookup,
          NodeType::Call,
          NodeType::Match,
          NodeType::Array,
          NodeType::ObjectAccess,
          NodeType::Parenthesis,
        ].contains(&last_expr.node_type)
      {
        return match_pattern(tokens, &[Lookup::Equal(&[TokenType::LeftSquareBracket])]);
      }
    }
    None
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let on = guard!(args.last_expr.to_owned());
      let target = parser.get_statement(
        &mut LoopArgument::with_tokens("arraylookup", &[], &[TokenType::RightSquareBracket])
      )?;
      parser.position += 1;
      return Ok(ArrayLookupNode::loc(on, target, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
