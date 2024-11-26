use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ Node, BinNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct BinParser;

impl BinParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    args.last_expr.as_ref()?;
    match_pattern(
      tokens,
      [
        Lookup::Equal(
          vec![
            TokenType::Addition,
            TokenType::Subtraction,
            TokenType::Multiplication,
            TokenType::Division,
            TokenType::Modulus,
            TokenType::Exponentiation,
            TokenType::BitwiseAnd,
            TokenType::BitwiseOr,
            TokenType::BitwiseXor,
            TokenType::BitwiseShiftLeft,
            TokenType::BitwiseShiftRight,
            TokenType::IsEqual,
            TokenType::IsIdentical,
            TokenType::IsNotEqual,
            TokenType::IsNotIdentical,
            TokenType::IsLesser,
            TokenType::IsGreater,
            TokenType::IsLesserOrEqual,
            TokenType::IsGreaterOrEqual,
            TokenType::Spaceship,
            TokenType::Concatenation,
            TokenType::Coalesce,
            TokenType::BooleanAnd,
            TokenType::BooleanOr,
            TokenType::And,
            TokenType::Or,
            TokenType::Xor,
            TokenType::Elvis,
            TokenType::InstanceOf
          ]
        ),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [operator] = matched.as_slice() {
      if let Some(operator) = operator.first() {
        let left = args.last_expr.to_owned().unwrap();
        args.last_expr = None;
        if
          let Some(right) = parser.get_statement(
            &mut LoopArgument::safe(
              "bin",
              &[],
              &args.breakers.combine(args.separators),
              &DEFAULT_PARSERS
            )
          )?
        {
          return Ok(BinNode::new(left, operator.value.to_owned(), right));
        }
      }
    }
    Err(ParserError::internal("Bin", args))
  }
}
