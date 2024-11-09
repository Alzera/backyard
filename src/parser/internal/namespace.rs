use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::namespace::NamespaceNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::{ block::BlockParser, comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct NamespaceParser {}

impl NamespaceParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Namespace])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Node> {
    if let [_] = matched.as_slice() {
      let name = parser.get_children(
        &mut LoopArgument::new(
          "namespace",
          &[TokenType::BackSlash],
          &[TokenType::Semicolon, TokenType::LeftCurlyBracket],
          &[
            (IdentifierParser::test, IdentifierParser::parse),
            (CommentParser::test, CommentParser::parse),
          ]
        )
      );
      let is_bracket = if let Some(t) = parser.tokens.get(parser.position - 1) {
        let is_bracket = t.token_type == TokenType::LeftCurlyBracket;
        parser.position -= 1;
        is_bracket
      } else {
        false
      };
      let body = BlockParser::new(parser);
      return Some(NamespaceNode::new(name, body, is_bracket));
    }
    None
  }
}
