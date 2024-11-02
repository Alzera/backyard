use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ ArrayNode, Node },
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct ArrayParser {}

impl Internal for ArrayParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Ellipsis]),
        Lookup::Equal(vec![TokenType::LeftSquareBracket]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [is_ellipsis, _] = matched.as_slice() {
      let values = parser.get_children(
        &mut LoopArgument::with_tokens(
          "array",
          &[TokenType::Comma],
          &[TokenType::RightSquareBracket]
        )
      );
      return Some(
        Box::new(ArrayNode {
          is_ellipsis: is_ellipsis.len() > 0,
          values,
        })
      );
    }
    None
  }
}
