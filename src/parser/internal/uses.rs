use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::uses::UseNode,
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, some_or_default, Lookup },
  },
};

use super::{ comment::CommentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct UsesParser {}

impl Internal for UsesParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Use]),
        Lookup::Optional(vec![TokenType::Function, TokenType::Const]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, modifier] = matched.as_slice() {
      let modifier = some_or_default(modifier.get(0), String::from(""), |i| i.value.to_owned());
      let name = parser.get_children(
        &mut LoopArgument::new(
          "uses_name",
          &[TokenType::BackSlash],
          &[TokenType::Semicolon, TokenType::LeftCurlyBracket],
          &[
            ParserInternal::Identifier(IdentifierParser {}),
            ParserInternal::Comment(CommentParser {}),
          ]
        )
      );
      parser.position -= 1;
      let items = {
        let mut items = vec![];
        if let Some(t) = parser.tokens.get(parser.position) {
          if t.token_type == TokenType::LeftCurlyBracket {
            parser.position += 1;
            items = parser.get_children(
              &mut LoopArgument::new(
                "uses_items",
                &[TokenType::Comma],
                &[TokenType::RightCurlyBracket],
                &[
                  ParserInternal::Identifier(IdentifierParser {}),
                  ParserInternal::Comment(CommentParser {}),
                ]
              )
            );
          }
        }
        items
      };
      return Some(UseNode::new(modifier, name, items));
    }
    None
  }
}
