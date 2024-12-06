use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ConstNode, ConstPropertyNode, Location, Node, Visibility };

use crate::{
  cast_lookup_result,
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{
    match_pattern,
    Lookup,
    LookupResult,
    LookupResultWrapper,
    ModifierLookup,
    ModifierResult,
  },
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
      let mut visibilities = vec![];
      if let LookupResultWrapper::Modifier(modifiers) = &modifiers.wrapper {
        if let [ModifierResult::Visibility(visibilities_modifier)] = modifiers.as_slice() {
          visibilities = visibilities_modifier
            .iter()
            .filter_map(|x| Visibility::try_parse(&x.value))
            .collect();
        }
      }
      let const_type = cast_lookup_result!(OptionalType, &const_type.wrapper);
      return Ok(
        ConstPropertyNode::loc(
          const_type.to_owned(),
          visibilities,
          ConstParser::get_consts(parser)?,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
