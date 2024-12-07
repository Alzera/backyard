use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ BlockNode, InterfaceNode, Location, Node };

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
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Interface]), Lookup::Equal(&[TokenType::Identifier])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    _: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, name] = matched.as_slice() {
      let name = IdentifierParser::from_token(name.as_equal()?);
      let extends = if let Some(t) = parser.tokens.get(parser.position) {
        if t.token_type == TokenType::Extends {
          parser.position += 1;
          let t = parser.get_children(
            &mut LoopArgument::new(
              "interface_extends",
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
        }
      } else {
        vec![]
      };
      let block_loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
      parser.position += 1;
      let body = parser.get_children(
        &mut LoopArgument::new(
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
          name,
          extends,
          BlockNode::loc(body, parser.gen_loc(block_loc)),
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
