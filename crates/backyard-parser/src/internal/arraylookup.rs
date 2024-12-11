use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{
  node::{ ArrayLookupNode, Location, Node, NodeType },
  utils::{ IntoBoxedNode, IntoBoxedOptionNode },
};

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct ArrayLookupParser;

impl ArrayLookupParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    args: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    args.last_expr.as_ref()?;
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
        return match_pattern(parser, tokens, &[Lookup::Equal(&[TokenType::LeftSquareBracket])]);
      }
    }
    None
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      let on = args.last_expr.take().unwrap();
      let target = parser.get_statement(
        &mut LoopArgument::with_tokens(
          parser.arena,
          "arraylookup",
          &[],
          &[TokenType::RightSquareBracket]
        )
      )?;
      parser.position += 1;
      return Ok(
        ArrayLookupNode::loc(
          on.into_boxed(parser.arena),
          target.into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
