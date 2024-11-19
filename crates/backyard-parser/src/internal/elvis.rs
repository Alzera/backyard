use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ Node, ElvisNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct ElvisParser {}

impl ElvisParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Elvis])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      if
        let Some(right) = parser.get_statement(
          &mut LoopArgument::with_tokens(
            "elvis",
            &args.separators.combine(&[TokenType::Semicolon, TokenType::Comma]),
            &args.breakers
          )
        )?
      {
        return Ok(ElvisNode::new(args.last_expr.to_owned().unwrap(), right));
      }
    }
    Err(ParserError::internal("Elvis", args))
  }
}
