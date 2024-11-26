use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, NodeType, ArrayLookupNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct ArrayLookupParser;

impl ArrayLookupParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    let last_expr = guard!(&args.last_expr, {
      return None;
    });
    if
      ![
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
      return None;
    }
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::LeftSquareBracket])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let on = guard!(args.last_expr.to_owned(), {
        return Err(ParserError::internal("ArrayLookup", args));
      });
      let target = parser.get_statement(
        &mut LoopArgument::with_tokens("arraylookup", &[], &[TokenType::RightSquareBracket])
      )?;
      parser.position += 1;
      return Ok(ArrayLookupNode::new(on, target));
    }
    Err(ParserError::internal("ArrayLookup", args))
  }
}
