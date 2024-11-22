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
      if has_from.len() > 0 {
        let expr = guard!(
          parser.get_statement(
            &mut LoopArgument::with_tokens(
              "yield_from",
              &[],
              &args.breakers.combine(args.separators)
            )
          )?,
          {
            return Err(ParserError::internal("Yield", args));
          }
        );
        return Ok(YieldFromNode::new(expr));
      }
      let mut value = parser.get_statement(
        &mut LoopArgument::with_tokens(
          "yield",
          &[],
          &args.breakers.combine(args.separators).combine(&[TokenType::Arrow])
        )
      )?;
      if value.is_none() {
        return Ok(YieldNode::new(None, None));
      }
      let mut key = None;
      if
        guard!(parser.tokens.get(parser.position), {
          return Err(ParserError::internal("Yield", args));
        }).token_type == TokenType::Arrow
      {
        key = Some(value.unwrap());
        parser.position += 1;
        value = Some(
          guard!(
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
          )
        );
      }
      return Ok(YieldNode::new(key, value));
    }
    Err(ParserError::internal("Yield", args))
  }
}
