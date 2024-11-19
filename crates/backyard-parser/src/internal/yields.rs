use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ Node, YieldFromNode, YieldNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct YieldParser {}

impl YieldParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [Lookup::Equal(vec![TokenType::Yield]), Lookup::Optional(vec![TokenType::From])].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, has_from] = matched.as_slice() {
      let mut value = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "singles",
            &args.separators.combine(&[]),
            &args.breakers.combine(&[TokenType::Arrow, TokenType::Semicolon])
          )
        )?,
        {
          return Err(ParserError::internal("Yield", args));
        }
      );
      if has_from.len() > 0 {
        return Ok(YieldFromNode::new(value));
      }
      let mut key = None;
      if
        guard!(parser.tokens.get(parser.position), {
          return Err(ParserError::internal("Yield", args));
        }).token_type == TokenType::Arrow
      {
        key = Some(value);
        parser.position += 1;
        value = guard!(
          parser.get_statement(
            &mut LoopArgument::with_tokens(
              "singles",
              &args.separators.combine(&[]),
              &args.breakers.combine(&[TokenType::Semicolon])
            )
          )?,
          {
            return Err(ParserError::internal("Yield", args));
          }
        );
      }
      return Ok(YieldNode::new(key, value));
    }
    Err(ParserError::internal("Yield", args))
  }
}
