use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, TernaryNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser, TokenTypeArrayCombine, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct TernaryParser;

impl TernaryParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    args.last_expr.as_ref()?;
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::QuestionMark])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let left = args.last_expr.to_owned().unwrap();
      args.last_expr = None;
      let valid = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("ternary_valid", &[], &[TokenType::Colon])
        )?,
        {
          return Err(ParserError::internal("Ternary", args));
        }
      );
      parser.position += 1;
      let invalid = guard!(
        parser.get_statement(
          &mut LoopArgument::safe(
            "ternary_invalid",
            &[],
            &args.breakers.combine(args.separators).combine(&[TokenType::Semicolon]),
            &DEFAULT_PARSERS
          )
        )?,
        {
          return Err(ParserError::internal("Ternary", args));
        }
      );
      return Ok(TernaryNode::new(left, valid, invalid, parser.gen_loc(start_loc)));
    }
    Err(ParserError::internal("Ternary", args))
  }
}
