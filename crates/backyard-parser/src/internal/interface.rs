use bumpalo::vec;
use backyard_lexer::token::TokenType;
use backyard_nodes::{ BlockNode, InterfaceNode, Location, Node, utils::IntoBoxedNode };

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

use super::{
  attribute::AttributeParser,
  comment::CommentParser,
  consts::ConstPropertyParser,
  identifier::IdentifierParser,
  method::MethodParser,
};

#[derive(Debug, Clone)]
pub struct InterfaceParser;

impl InterfaceParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[Lookup::Equal(&[TokenType::Interface]), Lookup::Equal(&[TokenType::Identifier])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, name] = matched.as_slice() {
      let name = IdentifierParser::from_token(name.as_equal(parser)?);
      let extends = {
        let mut parsed = None;
        if let Ok(t) = parser.get_token(parser.position) {
          if t.token_type == TokenType::Extends {
            parser.position += 1;
            parsed = Some(
              parser.get_children(
                &mut LoopArgument::new(
                  parser.arena,
                  "interface_extends",
                  &[TokenType::Comma],
                  &[TokenType::LeftCurlyBracket],
                  &[
                    (IdentifierParser::test, IdentifierParser::parse),
                    (CommentParser::test, CommentParser::parse),
                  ]
                )
              )?
            );
            parser.position -= 1;
          }
        }
        if let Some(parsed) = parsed {
          parsed
        } else {
          vec![in parser.arena]
        }
      };
      let block_loc = parser.get_token(parser.position)?.get_location().unwrap();
      parser.position += 1;
      let body = parser.get_children(
        &mut LoopArgument::new(
          parser.arena,
          "interface_body",
          &[TokenType::Semicolon],
          &[TokenType::RightCurlyBracket],
          &[
            (MethodParser::test, MethodParser::parse),
            (ConstPropertyParser::test, ConstPropertyParser::parse),
            (AttributeParser::test, AttributeParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      return Ok(
        InterfaceNode::loc(
          name.into_boxed(parser.arena),
          extends,
          BlockNode::loc(body, parser.gen_loc(block_loc)).into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
