use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ Node, TernaryNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct TernaryParser {}

impl TernaryParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::QuestionMark])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      if args.last_expr.is_none() {
        return Err(ParserError::internal("Ternary", args));
      }
      let valid = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("ternary_valid", &[], &[TokenType::Colon])
        )?,
        {
          return Err(ParserError::internal("Ternary", args));
        }
      );
      parser.position += 1;
      let invalid = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "ternary_invalid",
            &args.separators.combine(&[TokenType::Semicolon]),
            &args.breakers
          )
        )?,
        {
          return Err(ParserError::internal("Ternary", args));
        }
      );
      return Ok(TernaryNode::new(args.last_expr.to_owned().unwrap(), valid, invalid));
    }
    Err(ParserError::internal("Ternary", args))
  }
}
