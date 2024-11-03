use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::{ block::BlockNode, interface::InterfaceNode },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, Lookup },
  },
};

use super::{ comment::CommentParser, identifier::IdentifierParser, method::MethodParser };

#[derive(Debug, Clone)]
pub struct InterfaceParser {}

impl Internal for InterfaceParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Interface]),
        Lookup::Equal(vec![TokenType::Identifier]),
        Lookup::Optional(vec![TokenType::Implements]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, name, _] = matched.as_slice() {
      let implements = parser.get_children(
        &mut LoopArgument::new(
          "interface_implements",
          &[TokenType::Comma],
          &[TokenType::LeftCurlyBracket],
          &[
            ParserInternal::Identifier(IdentifierParser {}),
            ParserInternal::Comment(CommentParser {}),
          ]
        )
      );
      let body = parser.get_children(
        &mut LoopArgument::new(
          "interface_body",
          &[TokenType::Semicolon],
          &[TokenType::RightCurlyBracket],
          &[ParserInternal::Method(MethodParser {}), ParserInternal::Comment(CommentParser {})]
        )
      );
      return Some(
        InterfaceNode::new(IdentifierParser::from_matched(name), implements, BlockNode::new(body))
      );
    }
    None
  }
}
