use bumpalo::collections::Vec;
use backyard_lexer::token::TokenType;
use backyard_nodes::{ ConstNode, ConstPropertyNode, Location, Node, utils::IntoBoxedOptionNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, ModifierLookup },
};

use super::{ assignment::AssignmentParser, comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct ConstParser;

impl ConstParser {
  pub fn get_consts<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>
  ) -> Result<Vec<'arena, Node<'arena>>, ParserError> {
    let consts = parser.get_children(
      &mut LoopArgument::new(
        parser.arena,
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
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, &[Lookup::Equal(&[TokenType::Const])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      return Ok(ConstNode::loc(ConstParser::get_consts(parser)?, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct ConstPropertyParser;

impl ConstPropertyParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[
        Lookup::Modifiers(&[ModifierLookup::Visibility]),
        Lookup::Equal(&[TokenType::Const]),
        Lookup::OptionalType,
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    mut matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [modifiers, _, const_type] = matched.as_mut_slice() {
      let visibilities = if let Some([m0]) = modifiers.as_modifier() {
        m0.as_visibilities(parser)
      } else {
        vec![]
      };
      return Ok(
        ConstPropertyNode::loc(
          const_type.as_optional_type().into_boxed(parser.arena),
          visibilities,
          ConstParser::get_consts(parser)?,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
