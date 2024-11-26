use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ Node, IncludeNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct IncludeParser;

impl IncludeParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[
        Lookup::Equal(
          &[TokenType::Require, TokenType::RequireOnce, TokenType::Include, TokenType::IncludeOnce]
        ),
        Lookup::Optional(&[TokenType::LeftParenthesis]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [keyword, use_parenthesis] = matched.as_slice() {
      let mut is_require = false;
      let mut is_once = false;
      if let Some(t) = keyword.first() {
        is_require = t.token_type == TokenType::Require || t.token_type == TokenType::RequireOnce;
        is_once = t.token_type == TokenType::RequireOnce || t.token_type == TokenType::IncludeOnce;
      }
      let use_parenthesis = !use_parenthesis.is_empty();
      let argument = guard!(
        if use_parenthesis {
          let a = parser.get_statement(
            &mut LoopArgument::with_tokens("include", &[], &[TokenType::RightParenthesis])
          )?;
          parser.position += 1;
          a
        } else {
          parser.get_statement(
            &mut LoopArgument::with_tokens("include", &[], &args.breakers.combine(args.separators))
          )?
        },
        {
          return Err(ParserError::internal("Include", args));
        }
      );
      return Ok(IncludeNode::new(use_parenthesis, is_require, is_once, argument));
    }
    Err(ParserError::internal("Include", args))
  }
}
