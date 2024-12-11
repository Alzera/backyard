use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{ node::{ Location, Node, RangeLocation, VariableNode }, utils::IntoBoxedNode };
use bumpalo::Bump;

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct VariableParser;

impl VariableParser {
  pub fn from_token<'arena>(arena: &'arena Bump, name: &Token) -> Node<'arena> {
    let id = IdentifierParser::from_token(name);
    let loc = id.loc.clone();
    VariableParser::new_bracked(arena, id, loc)
  }

  pub fn new_bracked<'arena>(
    arena: &'arena Bump,
    name: Node<'arena>,
    loc: Option<RangeLocation>
  ) -> Node<'arena> {
    VariableNode::loc(name.into_boxed(arena), loc)
  }
}

impl VariableParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      tokens,
      &[Lookup::Equal(&[TokenType::Variable, TokenType::VariableBracketOpen])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [name] = matched.as_slice() {
      let name = name.as_equal()?;
      if name.token_type == TokenType::VariableBracketOpen {
        let expr = parser.get_statement(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "variable",
            &[],
            &[TokenType::VariableBracketClose]
          )
        )?;
        parser.position += 1;
        if let Some(expr) = expr {
          let end_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
          return Ok(
            VariableParser::new_bracked(
              parser.arena,
              expr,
              Some(RangeLocation { start: start_loc, end: end_loc })
            )
          );
        }
      } else {
        return Ok(VariableParser::from_token(parser.arena, name));
      }
    }
    Err(ParserError::Internal)
  }
}
