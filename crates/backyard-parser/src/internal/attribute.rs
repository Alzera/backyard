use bumpalo::vec;
use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ AttributeItemNode, AttributeNode, Location, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::call::CallParser;

#[derive(Debug, Clone)]
pub struct AttributeParser;

impl AttributeParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(parser, tokens, &[Lookup::Equal(&[TokenType::Attribute])])
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_] = matched.as_slice() {
      let items = parser.get_children(
        &mut LoopArgument::new(
          &parser.arena,
          "attribute",
          &[TokenType::Comma],
          &[TokenType::RightSquareBracket],
          &[(AttributeItemParser::test, AttributeItemParser::parse)]
        )
      )?;
      let expr = parser.get_statement(
        &mut LoopArgument::new(
          parser.arena,
          "attribute",
          args.separators,
          args.breakers,
          args.parsers
        )
      )?;
      if let Some(mut expr) = expr {
        expr.leadings_shift(&parser.arena, AttributeNode::loc(items, parser.gen_loc(start_loc)));
        return Ok(expr);
      }
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct AttributeItemParser;

impl AttributeItemParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    tokens: &[Token],
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      tokens,
      &[
        Lookup::Equal(&[TokenType::Identifier, TokenType::Name, TokenType::Get, TokenType::Set]),
        Lookup::Optional(&[TokenType::LeftParenthesis]),
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [name, has_argument] = matched.as_slice() {
      let name = name.as_equal()?.value.to_owned();
      let arguments = if !has_argument.is_empty() {
        CallParser::get_arguments(parser)?
      } else {
        vec![in parser.arena]
      };
      return Ok(AttributeItemNode::loc(name, arguments, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
