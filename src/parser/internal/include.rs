use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::include::IncludeNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct IncludeParser {}

impl IncludeParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(
          vec![
            TokenType::Require,
            TokenType::RequireOnce,
            TokenType::Include,
            TokenType::IncludeOnce
          ]
        ),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    _: &mut LoopArgument
  ) -> Option<Node> {
    if let [keyword, _] = matched.as_slice() {
      let mut is_require = false;
      let mut is_once = false;
      if let Some(t) = keyword.get(0) {
        is_require = t.token_type == TokenType::Require || t.token_type == TokenType::RequireOnce;
        is_once = t.token_type == TokenType::RequireOnce || t.token_type == TokenType::IncludeOnce;
      }
      let argument = parser.get_statement(
        &mut LoopArgument::with_tokens("include", &[], &[TokenType::RightParenthesis])
      );
      if argument.is_none() {
        return None;
      }
      parser.position += 1;
      return Some(IncludeNode::boxed(is_require, is_once, argument.unwrap()));
    }
    None
  }
}
