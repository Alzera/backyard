use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::{ block::BlockNode, interface::InterfaceNode },
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::{ comment::CommentParser, identifier::IdentifierParser, method::MethodParser };

#[derive(Debug, Clone)]
pub struct InterfaceParser {}

impl InterfaceParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Interface]),
        Lookup::Equal(vec![TokenType::Identifier]),
        Lookup::Optional(vec![TokenType::Implements]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Node> {
    if let [_, name, _] = matched.as_slice() {
      let implements = parser.get_children(
        &mut LoopArgument::new(
          "interface_implements",
          &[TokenType::Comma],
          &[TokenType::LeftCurlyBracket],
          &[
            (IdentifierParser::test, IdentifierParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      );
      let body = parser.get_children(
        &mut LoopArgument::new(
          "interface_body",
          &[TokenType::Semicolon],
          &[TokenType::RightCurlyBracket],
          &[
            (MethodParser::test, MethodParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      );
      return Some(
        InterfaceNode::new(IdentifierParser::from_matched(name), implements, BlockNode::new(body))
      );
    }
    None
  }
}
