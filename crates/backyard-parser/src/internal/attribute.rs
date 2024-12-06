use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ AttributeItemNode, AttributeNode, Location, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

use super::call::CallParser;

#[derive(Debug, Clone)]
pub struct AttributeParser;

impl AttributeParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Attribute])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let items = parser.get_children(
        &mut LoopArgument::new(
          "attribute",
          &[TokenType::Comma],
          &[TokenType::RightSquareBracket],
          &[(AttributeItemParser::test, AttributeItemParser::parse)]
        )
      )?;
      let expr = parser.get_statement(
        &mut LoopArgument::new("attribute", args.separators, args.breakers, args.parsers)
      )?;
      if let Some(mut expr) = expr {
        expr.leadings.insert(0, AttributeNode::loc(items, parser.gen_loc(start_loc)));
        return Ok(expr);
      }
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct AttributeItemParser;

impl AttributeItemParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[
        Lookup::Equal(&[TokenType::Identifier, TokenType::Name, TokenType::Get, TokenType::Set]),
        Lookup::Optional(&[TokenType::LeftParenthesis]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [name, has_argument] = matched.as_slice() {
      let name = if let LookupResultWrapper::Equal(name) = &name.wrapper {
        name.value.to_owned()
      } else {
        return Err(ParserError::Internal);
      };
      let mut arguments = vec![];
      if !has_argument.is_empty() {
        arguments = CallParser::get_arguments(parser)?;
      }
      return Ok(AttributeItemNode::loc(name, arguments, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
