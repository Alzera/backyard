use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ Location, Node, AssignmentNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct AssignmentParser;

impl AssignmentParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    args.last_expr.as_ref()?;
    match_pattern(
      tokens,
      &[
        Lookup::Equal(
          &[
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
            TokenType::ReferenceAssignment,
          ]
        ),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [operator] = matched.as_slice() {
      if let Some(operator) = operator.first() {
        let left = args.last_expr.to_owned().unwrap();
        args.last_expr = None;
        if
          let Some(right) = parser.get_statement(
            &mut LoopArgument::safe(
              "assignment",
              &[],
              &args.breakers
                .combine(args.separators)
                .combine(&[TokenType::Semicolon, TokenType::Comma]),
              &DEFAULT_PARSERS
            )
          )?
        {
          return Ok(
            AssignmentNode::new(left, operator.value.to_owned(), right, parser.gen_loc(start_loc))
          );
        }
      }
    }
    Err(ParserError::internal("Assignment", args))
  }
}
