use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ AnonymousClassNode, BlockNode, ClassNode, Location, Node };

use crate::{
  error::ParserError,
  guard,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, some_or_default, Lookup },
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
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    let modifiers_rule = [
      [TokenType::Readonly].to_vec(),
      [TokenType::Abstract, TokenType::Final].to_vec(),
    ];
    let mut modifiers = vec![vec![], vec![]];
    let mut pos = 0;
    loop {
      let token = tokens.get(pos);
      pos += 1;
      if pos > 2 || token.is_none() {
        break;
      }
      let token = token.unwrap();
      for (i, modifier) in modifiers_rule.iter().enumerate() {
        if !modifiers[i].is_empty() {
          continue;
        }
        if modifier.contains(&token.token_type) {
          modifiers[i].push(token.clone());
          break;
        }
      }
    }
    let modifier_count = modifiers
      .iter()
      .map(|i| i.len())
      .sum::<usize>();
    if
      let Some(next_modifiers) = match_pattern(
        &tokens[modifier_count..],
        &[
          Lookup::Equal(&[TokenType::Class]),
          Lookup::Equal(&[TokenType::Identifier]),
          Lookup::Optional(&[TokenType::Extends]),
          Lookup::Optional(&[TokenType::Identifier, TokenType::Name]),
        ]
      )
    {
      modifiers.extend(next_modifiers);
      return Some(modifiers);
    }
    // anonymous class
    match_pattern(
      &tokens[modifier_count..],
      &[Lookup::Equal(&[TokenType::Class]), Lookup::Optional(&[TokenType::LeftParenthesis])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    match matched.len() {
      6 => Self::parse_basic(parser, matched, start_loc, args),
      2 => Self::parse_anonymous(parser, matched, start_loc, args),
      _ => { Err(ParserError::internal("Class", args)) }
    }
  }

  fn parse_anonymous(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, has_parameter] = matched.as_slice() {
      let parameters = if !has_parameter.is_empty() {
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
          extends = Some(
            IdentifierParser::from_token(
              guard!(parser.tokens.get(parser.position), {
                return Err(ParserError::internal("Class: failed to parse", args));
              })
            )
          );
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
        AnonymousClassNode::new(
          parameters,
          extends,
          implements,
          BlockNode::new(body, parser.gen_loc(body_loc)),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Class: failed to parse", args))
  }

  fn parse_basic(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [readonly, modifier, _, name, _, extends] = matched.as_slice() {
      let extends = extends.first().map(|t| IdentifierParser::from_token(t));
      let mut implements = vec![];
      if let Some(t) = parser.tokens.get(parser.position) {
        if t.token_type == TokenType::Implements {
          parser.position += 1;
          implements = parser.get_children(
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
        }
      }
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
      let name = if !name.is_empty() { Some(IdentifierParser::from_matched(name)) } else { None };
      return Ok(
        ClassNode::new(
          some_or_default(modifier.first(), String::from(""), |i| i.value.to_owned()),
          name,
          extends,
          implements,
          BlockNode::new(body, parser.gen_loc(body_loc)),
          !readonly.is_empty(),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Class: failed to parse", args))
  }
}
