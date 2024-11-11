use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ Node, NodeType, Nodes },
    nodes::array::{ ArrayItemNode, ArrayNode },
    parser::{ LoopArgument, Parser, DEFAULT_PARSERS },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct ArrayParser {}

impl ArrayParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Ellipsis]),
        Lookup::Equal(vec![TokenType::LeftSquareBracket]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Node> {
    if let [is_ellipsis, _] = matched.as_slice() {
      let mut loop_parsers = DEFAULT_PARSERS.to_vec();
      loop_parsers.insert(0, (ArrayItemParser::test, ArrayItemParser::parse));
      let values = parser
        .get_children(
          &mut LoopArgument::new(
            "array",
            &[TokenType::Comma],
            &[TokenType::RightSquareBracket],
            &loop_parsers
          )
        )
        .iter()
        .map(|i| (
          if i.get_type() == NodeType::ArrayItem {
            i.to_owned()
          } else {
            ArrayItemNode::boxed(None, i.to_owned())
          }
        ))
        .collect::<Nodes>();
      return Some(ArrayNode::boxed(is_ellipsis.len() > 0, values));
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct ArrayItemParser {}

impl ArrayItemParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Arrow])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
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
      let key = args.last_expr.to_owned();
      return Some(ArrayItemNode::boxed(key, value.unwrap()));
    }
    None
  }
}
