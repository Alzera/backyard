use crate::{
  lexer::token::{ Token, TokenType, TokenTypeArrayCombine },
  parser::{
    node::Node,
    nodes::bin::BinNode,
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct BinParser {}

impl Internal for BinParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
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

  fn parse(
    &self,
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &LoopArgument
  ) -> Option<Node> {
    if let [operator] = matched.as_slice() {
      if let Some(operator) = operator.get(0) {
        if
          let Some(right) = parser.get_statement(
            &mut LoopArgument::with_tokens(
              "bin",
              &args.separators.combine(&[TokenType::Semicolon]),
              &args.breakers
            )
          )
        {
          return Some(
            BinNode::new(args.last_expr.to_owned().unwrap(), operator.value.to_owned(), right)
          );
        }
      }
    }
    None
  }
}
