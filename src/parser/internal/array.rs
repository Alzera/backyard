use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::array::{ ArrayItemNode, ArrayNode },
    parser::{ Internal, LoopArgument, Parser, ParserInternal, DEFAULT_PARSERS },
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
      let mut loop_parsers = DEFAULT_PARSERS.to_vec();
      loop_parsers.insert(0, ParserInternal::ArrayItem(ArrayItemParser {}));
      let values = parser.get_children(
        &mut LoopArgument::new(
          "array",
          &[TokenType::Comma],
          &[TokenType::RightSquareBracket],
          &loop_parsers
        )
      );
      return Some(ArrayNode::new(is_ellipsis.len() > 0, values));
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct ArrayItemParser {}

impl Internal for ArrayItemParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Arrow])].to_vec())
  }

  fn parse(
    &self,
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &LoopArgument
  ) -> Option<Node> {
    if let [_] = matched.as_slice() {
      let value = parser.get_statement(
        &mut LoopArgument::with_tokens(
          "array_item",
          &[],
          &[TokenType::Comma, TokenType::RightSquareBracket]
        )
      );
      if value.is_none() {
        return None;
      }
      let key = guard!(args.last_expr.to_owned());
      return Some(ArrayItemNode::new(key, value.unwrap()));
    }
    None
  }
}
