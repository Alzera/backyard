use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ Node, BinNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct BinParser {}

impl BinParser {
  pub fn test(tokens: &Vec<Token>, args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if args.last_expr.is_none() {
      return None;
    }
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
            TokenType::Xor
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
      if let Some(operator) = operator.get(0) {
        if
          let Some(right) = parser.get_statement(
            &mut LoopArgument::with_tokens(
              "bin",
              &args.separators.combine(&[TokenType::Semicolon]),
              &args.breakers
            )
          )?
        {
          return Ok(
            BinNode::new(args.last_expr.to_owned().unwrap(), operator.value.to_owned(), right)
          );
        }
      }
    }
    Err(ParserError::internal("Bin", args))
  }
}
