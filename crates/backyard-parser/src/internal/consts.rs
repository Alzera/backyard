use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ConstNode, ConstPropertyNode, Location, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, ModifierLookup },
};

use super::{ assignment::AssignmentParser, comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct ConstParser;

impl ConstParser {
  pub fn get_consts(parser: &mut Parser) -> Result<Vec<Box<Node>>, ParserError> {
    let consts = parser.get_children(
      &mut LoopArgument::new(
        "const",
        &[TokenType::Comma],
        &[TokenType::Semicolon],
        &[
          (IdentifierParser::test, IdentifierParser::parse),
          (AssignmentParser::test, AssignmentParser::parse),
          (CommentParser::test, CommentParser::parse),
        ]
      )
    );
    parser.position -= 1;
    consts
  }
}

impl ConstParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Const])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      return Ok(ConstNode::loc(ConstParser::get_consts(parser)?, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct ConstPropertyParser;

impl ConstPropertyParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[
        Lookup::Modifiers(&[ModifierLookup::Visibility]),
        Lookup::Equal(&[TokenType::Const]),
        Lookup::OptionalType,
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [modifiers, _, const_type] = matched.as_slice() {
      let visibilities = if let Some([m0]) = modifiers.as_modifier() {
        m0.as_visibilities()
      } else {
        vec![]
      };
      return Ok(
        ConstPropertyNode::loc(
          const_type.as_optional_type(),
          visibilities,
          ConstParser::get_consts(parser)?,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
