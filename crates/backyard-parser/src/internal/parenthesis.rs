use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ CastNode, Location, Node, ParenthesisNode };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser, DEFAULT_PARSERS },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct ParenthesisParser;

impl ParenthesisParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::LeftParenthesis])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      if let Some(token) = parser.tokens.get(parser.position) {
        if
          [
            "int",
            "integer",
            "bool",
            "boolean",
            "float",
            "double",
            "real",
            "string",
            "binary",
            "array",
            "object",
            "unset",
          ].contains(&token.value.as_str())
        {
          if let Some(next_token) = parser.tokens.get(parser.position + 1) {
            if next_token.token_type == TokenType::RightParenthesis {
              parser.position += 2;
              let expression = guard!(
                parser.get_statement(
                  &mut LoopArgument::safe("cast", args.separators, args.breakers, &DEFAULT_PARSERS)
                )?
              );
              return Ok(
                CastNode::new(token.value.to_owned(), expression, parser.gen_loc(start_loc))
              );
            }
          }
        }
      }
      let statement = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("parenthesis", &[], &[TokenType::RightParenthesis])
        )?
      );
      parser.position += 1;
      return Ok(ParenthesisNode::new(statement, parser.gen_loc(start_loc)));
    }
    Err(ParserError::Internal)
  }
}
