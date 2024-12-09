use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{
  NegateNode,
  Location,
  Node,
  PreNode,
  ReferenceNode,
  SilentNode,
  VariadicNode,
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
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
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

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [operator] = matched.as_slice() {
      let operator = operator.as_equal()?;
      let argument = parser.get_statement(
        &mut LoopArgument::safe("pre", args.separators, args.breakers, &DEFAULT_PARSERS)
      )?;
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
