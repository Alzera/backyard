use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ BlockNode, InterfaceNode, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{
  attribute::AttributeParser,
  comment::CommentParser,
  consts::ConstPropertyParser,
  identifier::IdentifierParser,
  method::MethodParser,
};

#[derive(Debug, Clone)]
pub struct InterfaceParser {}

impl InterfaceParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Interface]),
        Lookup::Equal(vec![TokenType::Identifier]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, name] = matched.as_slice() {
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
        InterfaceNode::new(IdentifierParser::from_matched(name), extends, BlockNode::new(body))
      );
    }
    Err(ParserError::internal("Interface", args))
  }
}
