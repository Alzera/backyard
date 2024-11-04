use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::enums::{ EnumItemNode, EnumNode },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, Lookup },
  },
};

use super::{ comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct EnumParser {}

impl Internal for EnumParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Enum]),
        Lookup::Equal(vec![TokenType::Identifier]),
        Lookup::Equal(vec![TokenType::LeftCurlyBracket]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, name, _] = matched.as_slice() {
      let items = parser.get_children(
        &mut LoopArgument::new(
          "enum",
          &[TokenType::Semicolon],
          &[TokenType::RightCurlyBracket],
          &[ParserInternal::EnumItem(EnumItemParser {}), ParserInternal::Comment(CommentParser {})]
        )
      );
      return Some(EnumNode::new(IdentifierParser::from_matched(name), items));
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct EnumItemParser {}

impl Internal for EnumItemParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Case])].to_vec())
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_] = matched.as_slice() {
      if
        let Some(value) = parser.get_statement(
          &mut LoopArgument::with_tokens(
            "enum_item",
            &[],
            &[TokenType::Semicolon, TokenType::RightSquareBracket]
          )
        )
      {
        return Some(EnumItemNode::new(value));
      }
    }
    None
  }
}
