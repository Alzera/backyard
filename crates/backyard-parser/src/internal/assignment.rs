use backyard_lexer::token::TokenType;
use backyard_nodes::{ utils::IntoBoxedNode, AssignmentNode, AssignmentType, Location, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser, TokenTypeArrayCombine, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct AssignmentParser;

impl AssignmentParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    args: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    args.last_expr.as_ref()?;
    match_pattern(
      parser,
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

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [operator] = matched.as_slice() {
      let operator = AssignmentType::try_from(&operator.as_equal(parser)?.value).map_err(
        |_| ParserError::Internal
      )?;
      let left = args.last_expr.take().unwrap();
      if
        let Some(right) = parser.get_statement(
          &mut LoopArgument::safe(
            parser.arena,
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
          AssignmentNode::loc(
            left.into_boxed(parser.arena),
            operator,
            right.into_boxed(parser.arena),
            parser.gen_loc(start_loc)
          )
        );
      }
    }
    Err(ParserError::Internal)
  }
}
