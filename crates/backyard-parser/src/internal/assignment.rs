use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{ node::{ AssignmentNode, Location, Node }, utils::IntoBoxedNode };

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
    tokens: &[Token],
    args: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    args.last_expr.as_ref()?;
    match_pattern(
      parser,
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

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [operator] = matched.as_slice() {
      let operator = operator.as_equal()?;
      let left = args.last_expr.take().unwrap();
      args.last_expr = None;
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
            operator.value.to_owned(),
            right.into_boxed(parser.arena),
            parser.gen_loc(start_loc)
          )
        );
      }
    }
    Err(ParserError::Internal)
  }
}
