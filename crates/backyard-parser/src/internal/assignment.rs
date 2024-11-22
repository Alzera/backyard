use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ Node, AssignmentNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct AssignmentParser {}

impl AssignmentParser {
  pub fn test(tokens: &Vec<Token>, args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if args.last_expr.is_none() {
      return None;
    }
    match_pattern(
      tokens,
      [
        Lookup::Equal(
          vec![
            TokenType::Assignment,
            TokenType::BitwiseAndAssignment,
            TokenType::CoalesceAssignment,
            TokenType::ModulusAssignment,
            TokenType::BitwiseXorAssignment,
            TokenType::ExponentiationAssignment,
            TokenType::MultiplicationAssignment,
            TokenType::DivisionAssignment,
            TokenType::ConcatenationAssignment,
            TokenType::BitwiseOrAssignment,
            TokenType::SubtractionAssignment,
            TokenType::BitwiseShiftRightAssignment,
            TokenType::BitwiseShiftLeftAssignment,
            TokenType::AdditionAssignment,
            TokenType::ReferenceAssignment
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
        let left = args.last_expr.to_owned().unwrap();
        args.last_expr = None;
        if
          let Some(right) = parser.get_statement(
            &mut LoopArgument::with_tokens(
              "assignment",
              &args.separators.combine(&[TokenType::Semicolon, TokenType::Comma]),
              &args.breakers
            )
          )?
        {
          return Ok(AssignmentNode::new(left, operator.value.to_owned(), right));
        }
      }
    }
    Err(ParserError::internal("Assignment", args))
  }
}
