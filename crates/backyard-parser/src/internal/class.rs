use bumpalo::vec;
use backyard_lexer::token::TokenType;
use backyard_nodes::{
  AnonymousClassNode,
  BlockNode,
  ClassNode,
  Inheritance,
  Location,
  Node,
  utils::{ IntoBoxedNode, IntoBoxedOptionNode },
};

use crate::{
  error::ParserError,
  guard,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, ModifierLookup },
};

use super::{
  attribute::AttributeParser,
  comment::CommentParser,
  consts::ConstPropertyParser,
  identifier::IdentifierParser,
  method::MethodParser,
  property::PropertyParser,
  traituse::TraitUseParser,
};

#[derive(Debug, Clone)]
pub struct ClassParser;

impl ClassParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    if
      let Some(m) = match_pattern(
        parser,
        &[
          Lookup::Modifiers(
            &[
              ModifierLookup::Custom(&[TokenType::Readonly]),
              ModifierLookup::Custom(&[TokenType::Abstract, TokenType::Final]),
            ]
          ),
          Lookup::Equal(&[TokenType::Class]),
          Lookup::Equal(&[TokenType::Identifier]),
          Lookup::Optional(&[TokenType::Extends]),
          Lookup::Optional(&[TokenType::Identifier, TokenType::Name]),
          Lookup::Optional(&[TokenType::Implements]),
        ]
      )
    {
      return Some(m);
    }
    match_pattern(
      parser,
      &[Lookup::Equal(&[TokenType::Class]), Lookup::Optional(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    match matched.len() {
      6 => Self::parse_basic(parser, matched, start_loc, args),
      2 => Self::parse_anonymous(parser, matched, start_loc, args),
      _ => { Err(ParserError::Internal) }
    }
  }

  fn parse_anonymous<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [_, has_parameter] = matched.as_slice() {
      let parameters = if !has_parameter.is_empty() {
        parser.get_children(
          &mut LoopArgument::with_tokens(
            parser.arena,
            "class_anonymous_parameter",
            &[TokenType::Comma],
            &[TokenType::RightParenthesis]
          )
        )?
      } else {
        vec![in parser.arena]
      };
      let mut extends = None;
      if let Some(t) = parser.tokens.get(parser.position) {
        if t.token_type == TokenType::Extends {
          parser.position += 1;
          extends = Some(IdentifierParser::from_token(guard!(parser.tokens.get(parser.position))));
          parser.position += 1;
        }
      }
      let implements = {
        let mut parsed = None;
        if let Some(t) = parser.tokens.get(parser.position) {
          if t.token_type == TokenType::Implements {
            parser.position += 1;
            parsed = Some(
              parser.get_children(
                &mut LoopArgument::new(
                  parser.arena,
                  "class_anonymous_implements",
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
      let body_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
      parser.position += 1;
      let body = parser.get_children(
        &mut LoopArgument::new(
          parser.arena,
          "class_anonymous_body",
          &[TokenType::Semicolon],
          &[TokenType::RightCurlyBracket],
          &[
            (TraitUseParser::test, TraitUseParser::parse),
            (MethodParser::test, MethodParser::parse),
            (ConstPropertyParser::test, ConstPropertyParser::parse),
            (PropertyParser::test, PropertyParser::parse),
            (AttributeParser::test, AttributeParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      return Ok(
        AnonymousClassNode::loc(
          parameters,
          extends.into_boxed(parser.arena),
          implements,
          BlockNode::loc(body, parser.gen_loc(body_loc)).into_boxed(parser.arena),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }

  fn parse_basic<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: Vec<LookupResult<'arena>>,
    start_loc: Location,
    _: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [modifiers, _, name, _, extends, has_implements] = matched.as_slice() {
      let name = IdentifierParser::from_token(name.as_equal(parser)?);
      let extends = extends.as_optional(parser).map(IdentifierParser::from_token);
      let implements = if !has_implements.is_empty() {
        let t = parser.get_children(
          &mut LoopArgument::new(
            parser.arena,
            "class_implements",
            &[TokenType::Comma],
            &[TokenType::LeftCurlyBracket],
            &[
              (IdentifierParser::test, IdentifierParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        )?;
        parser.position -= 1;
        t
      } else {
        vec![in parser.arena]
      };
      let body_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
      parser.position += 1;
      let body = parser.get_children(
        &mut LoopArgument::new(
          parser.arena,
          "class_body",
          &[TokenType::Semicolon],
          &[TokenType::RightCurlyBracket],
          &[
            (TraitUseParser::test, TraitUseParser::parse),
            (MethodParser::test, MethodParser::parse),
            (ConstPropertyParser::test, ConstPropertyParser::parse),
            (PropertyParser::test, PropertyParser::parse),
            (AttributeParser::test, AttributeParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      )?;
      let mut inheritance = None;
      let mut is_readonly = false;
      if let Some([m0, m1]) = modifiers.as_modifier() {
        is_readonly = m0.as_custom(parser, |x| Ok(x == "readonly")).unwrap_or(false);
        inheritance = m1.as_custom(parser, |x| Inheritance::try_from(x));
      }
      return Ok(
        ClassNode::loc(
          inheritance,
          Some(name.into_boxed(parser.arena)),
          extends.into_boxed(parser.arena),
          implements,
          BlockNode::loc(body, parser.gen_loc(body_loc)).into_boxed(parser.arena),
          is_readonly,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
