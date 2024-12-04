use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, YieldFromNode, YieldNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser, TokenTypeArrayCombine },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct YieldParser;

impl YieldParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::Yield]), Lookup::Optional(&[TokenType::From])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_, has_from] = matched.as_slice() {
      if !has_from.is_empty() {
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
        return Ok(YieldFromNode::new(expr, parser.gen_loc(start_loc)));
      }
      let mut value = parser.get_statement(
        &mut LoopArgument::with_tokens(
          "yield",
          &[],
          &args.breakers.combine(args.separators).combine(&[TokenType::Arrow])
        )
      )?;
      if value.is_none() {
        return Ok(YieldNode::new(None, None, parser.gen_loc(start_loc)));
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
      return Ok(YieldNode::new(key, value, parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("Yield", args))
  }
}
