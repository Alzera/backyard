use backyard_lexer::token::TokenType;
use backyard_nodes::{ utils::IntoBoxedNode, BinNode, BinaryType, Location, Node };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser, TokenTypeArrayCombine, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct BinParser;

impl BinParser {
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
            TokenType::InstanceOf,
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
      let operator = BinaryType::try_from(&operator.as_equal(parser)?.value).map_err(
        |_| ParserError::Internal
      )?;
      let left = args.last_expr.take().unwrap();
      if
        let Some(right) = parser.get_statement(
          &mut LoopArgument::safe(
            parser.arena,
            "bin",
            &[],
            &args.breakers.combine(args.separators),
            &DEFAULT_PARSERS
          )
        )?
      {
        return Ok(
          BinNode::loc(
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
