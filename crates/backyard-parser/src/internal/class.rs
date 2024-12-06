use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ AnonymousClassNode, BlockNode, ClassNode, Inheritance, Location, Node };

use crate::{
  error::ParserError,
  guard,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{
    match_pattern,
    Lookup,
    LookupResult,
    LookupResultWrapper,
    ModifierLookup,
    ModifierResult,
  },
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
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    if
      let Some(m) = match_pattern(
        tokens,
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
      tokens,
      &[Lookup::Equal(&[TokenType::Class]), Lookup::Optional(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    match matched.len() {
      6 => Self::parse_basic(parser, matched, start_loc, args),
      2 => Self::parse_anonymous(parser, matched, start_loc, args),
      _ => { Err(ParserError::Internal) }
    }
  }

  fn parse_anonymous(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, has_parameter] = matched.as_slice() {
      let parameters = if let LookupResultWrapper::Optional(Some(_)) = has_parameter.wrapper {
        parser.get_children(
          &mut LoopArgument::with_tokens(
            "class_anonymous_parameter",
            &[TokenType::Comma],
            &[TokenType::RightParenthesis]
          )
        )?
      } else {
        vec![]
      };
      let mut extends = None;
      if let Some(t) = parser.tokens.get(parser.position) {
        if t.token_type == TokenType::Extends {
          parser.position += 1;
          extends = Some(IdentifierParser::from_token(guard!(parser.tokens.get(parser.position))));
          parser.position += 1;
        }
      }
      let mut implements = vec![];
      if let Some(t) = parser.tokens.get(parser.position) {
        if t.token_type == TokenType::Implements {
          parser.position += 1;
          implements = parser.get_children(
            &mut LoopArgument::new(
              "class_anonymous_implements",
              &[TokenType::Comma],
              &[TokenType::LeftCurlyBracket],
              &[
                (IdentifierParser::test, IdentifierParser::parse),
                (CommentParser::test, CommentParser::parse),
              ]
            )
          )?;
          parser.position -= 1;
        }
      }
      let body_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
      parser.position += 1;
      let body = parser.get_children(
        &mut LoopArgument::new(
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
          extends,
          implements,
          BlockNode::loc(body, parser.gen_loc(body_loc)),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }

  fn parse_basic(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [modifiers, _, name, _, extends, has_implements] = matched.as_slice() {
      let name = if let LookupResultWrapper::Equal(name) = &name.wrapper {
        Some(IdentifierParser::from_token(name))
      } else {
        None
      };
      let extends = if let LookupResultWrapper::Optional(Some(extends)) = &extends.wrapper {
        Some(IdentifierParser::from_token(extends))
      } else {
        None
      };
      let implements = if let LookupResultWrapper::Optional(Some(_)) = has_implements.wrapper {
        let t = parser.get_children(
          &mut LoopArgument::new(
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
        vec![]
      };
      let body_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
      parser.position += 1;
      let body = parser.get_children(
        &mut LoopArgument::new(
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
      if let LookupResultWrapper::Modifier(modifiers) = &modifiers.wrapper {
        if
          let [
            ModifierResult::Custom(readonly_modifier),
            ModifierResult::Custom(inheritance_modifier),
          ] = modifiers.as_slice()
        {
          is_readonly = readonly_modifier.is_some();
          inheritance = Inheritance::try_parse(
            &inheritance_modifier
              .as_ref()
              .map(|i| i.value.to_owned())
              .unwrap_or_default()
          );
        }
      }
      return Ok(
        ClassNode::loc(
          inheritance,
          name,
          extends,
          implements,
          BlockNode::loc(body, parser.gen_loc(body_loc)),
          is_readonly,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
