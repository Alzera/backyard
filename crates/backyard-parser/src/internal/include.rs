use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, IncludeNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
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
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [keyword, _] = matched.as_slice() {
      let mut is_require = false;
      let mut is_once = false;
      if let Some(t) = keyword.get(0) {
        is_require = t.token_type == TokenType::Require || t.token_type == TokenType::RequireOnce;
        is_once = t.token_type == TokenType::RequireOnce || t.token_type == TokenType::IncludeOnce;
      }
      let argument = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("include", &[], &[TokenType::RightParenthesis])
        )?,
        {
          return Err(ParserError::internal("Include", args));
        }
      );
      parser.position += 1;
      return Ok(IncludeNode::new(is_require, is_once, argument));
    }
    Err(ParserError::internal("Include", args))
  }
}
