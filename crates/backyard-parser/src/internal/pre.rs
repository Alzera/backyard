use backyard_lexer::token::TokenType;
use backyard_nodes::{
  Location,
  NegateNode,
  Node,
  PreNode,
  ReferenceNode,
  SilentNode,
  VariadicNode,
  utils::IntoBoxedOptionNode,
};

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct PreParser;

impl PreParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[
        Lookup::Equal(
          &[
            TokenType::PreIncrement,
            TokenType::PreDecrement,
            TokenType::BooleanNegate,
            TokenType::AtSign,
            TokenType::Addition,
            TokenType::Subtraction,
            TokenType::Ellipsis,
            TokenType::BitwiseAnd,
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
      let argument = parser
        .get_statement(
          &mut LoopArgument::safe(
            parser.arena,
            "pre",
            args.separators,
            args.breakers,
            &DEFAULT_PARSERS
          )
        )?
        .into_boxed(parser.arena);
      let operator = operator.as_equal(parser)?;
      if operator.token_type == TokenType::Ellipsis {
        return Ok(VariadicNode::loc(argument, parser.gen_loc(start_loc)));
      }
      let argument = guard!(argument);
      return match operator.token_type {
        | TokenType::PreIncrement
        | TokenType::PreDecrement
        | TokenType::Addition
        | TokenType::Subtraction =>
          Ok(PreNode::loc(argument, operator.value.to_owned(), parser.gen_loc(start_loc))),
        TokenType::BooleanNegate => Ok(NegateNode::loc(argument, parser.gen_loc(start_loc))),
        TokenType::AtSign => Ok(SilentNode::loc(argument, parser.gen_loc(start_loc))),
        TokenType::BitwiseAnd => Ok(ReferenceNode::loc(argument, parser.gen_loc(start_loc))),
        _ => Err(ParserError::Internal),
      };
    }
    Err(ParserError::Internal)
  }
}
