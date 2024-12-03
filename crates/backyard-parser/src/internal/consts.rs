use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ ConstNode, ConstPropertyNode, Location, Node, Visibility };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
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
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      return Ok(ConstNode::new(ConstParser::get_consts(parser)?, parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("Const", args))
  }
}

#[derive(Debug, Clone)]
pub struct ConstPropertyParser;

impl ConstPropertyParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[
        Lookup::Modifiers(&[&[TokenType::Public, TokenType::Private, TokenType::Protected]]),
        Lookup::Equal(&[TokenType::Const]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [modifiers, _] = matched.as_slice() {
      let mut visibility = None;
      if let LookupResultWrapper::Modifier(modifiers) = &modifiers.wrapper {
        if let [visibility_modifier] = modifiers.as_slice() {
          visibility = Visibility::try_parse(
            &visibility_modifier
              .as_ref()
              .map(|i| i.value.to_owned())
              .unwrap_or_default()
          );
        }
      }
      return Ok(
        ConstPropertyNode::new(
          visibility,
          ConstParser::get_consts(parser)?,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("ConstProperty", args))
  }
}
