use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, AssignmentNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser, TokenTypeArrayCombine, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

#[derive(Debug, Clone)]
pub struct AssignmentParser;

impl AssignmentParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<LookupResult>> {
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
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [operator] = matched.as_slice() {
      if let LookupResultWrapper::Equal(operator) = &operator.wrapper {
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
            AssignmentNode::loc(left, operator.value.to_owned(), right, parser.gen_loc(start_loc))
          );
        }
      }
    }
    Err(ParserError::Internal)
  }
}
