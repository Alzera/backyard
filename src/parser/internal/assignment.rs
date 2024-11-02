use crate::{
  lexer::token::{ Token, TokenType, TokenTypeArrayCombine },
  parser::{
    node::{ AssignmentNode, Node },
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct AssignmentParser {}

impl Internal for AssignmentParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
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
            TokenType::AdditionAssignment
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
              "assignment",
              &args.separators.combine(&[TokenType::Semicolon, TokenType::Comma]),
              &args.breakers
            )
          )
        {
          return Some(
            Box::new(AssignmentNode {
              left: args.last_expr.to_owned().unwrap(),
              operator: operator.value.to_owned(),
              right: right,
            })
          );
        }
      }
    }
    None
  }
}
