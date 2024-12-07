use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ BodyType, Location, Node, DeclareArgumentNode, DeclareNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::{ block::BlockParser, comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct DeclareParser;

impl DeclareParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Declare]), Lookup::Equal(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, _] = matched.as_slice() {
      let arguments = parser.get_children(
        &mut LoopArgument::new(
          "declare",
          &[TokenType::Comma],
          &[TokenType::RightParenthesis],
          &[
            (DeclareArgumentParser::test, DeclareArgumentParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      let body_type: BodyType = {
        let mut body_type = BodyType::Empty;
        if let Some(close) = parser.tokens.get(parser.position) {
          body_type = match close.token_type {
            TokenType::Colon => BodyType::Short,
            TokenType::LeftCurlyBracket => BodyType::Basic,
            _ => BodyType::Empty,
          };
        }
        body_type
      };
      let body = match body_type {
        BodyType::Empty => None,
        BodyType::Basic => Some(BlockParser::new_block(parser)?),
        BodyType::Short => Some(BlockParser::new_short(parser, &[TokenType::EndDeclare])?),
      };
      return Ok(DeclareNode::loc(arguments, body, body_type, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct DeclareArgumentParser;

impl DeclareArgumentParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Identifier]), Lookup::Equal(&[TokenType::Assignment])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [name, _] = matched.as_slice() {
      let name = IdentifierParser::from_token(name.as_equal()?);
      if
        let Some(value) = parser.get_statement(
          &mut LoopArgument::with_tokens(
            "declare_argument",
            &[TokenType::Comma],
            &[TokenType::RightParenthesis]
          )
        )?
      {
        return Ok(DeclareArgumentNode::loc(name, value, parser.gen_loc(start_loc)));
      }
    }
    Err(ParserError::Internal)
  }
}
