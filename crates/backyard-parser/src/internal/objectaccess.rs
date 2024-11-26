use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, ObjectAccessNode };
use utils::guard;

use crate::{
  error::ParserError,
  internal::{ identifier::IdentifierParser, variable::VariableParser },
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct ObjectAccessParser;

impl ObjectAccessParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::ObjectAccess, TokenType::NullsafeObjectAccess])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [access_type] = matched.as_slice() {
      let is_nullsafe =
        guard!(access_type.first(), {
          return Err(ParserError::internal("ObjectAccess", args));
        }).token_type == TokenType::NullsafeObjectAccess;
      let is_bracket = if let Some(next_token) = parser.tokens.get(parser.position) {
        next_token.token_type == TokenType::LeftCurlyBracket
      } else {
        false
      };
      let expr = if is_bracket {
        parser.position += 1;
        let t = guard!(
          parser.get_statement(
            &mut LoopArgument::with_tokens("objectaccess", &[], &[TokenType::RightCurlyBracket])
          )?,
          {
            return Err(ParserError::internal("ObjectAccess", args));
          }
        );
        parser.position += 1;
        t
      } else if let Some(m) = VariableParser::test(&parser.tokens[parser.position..], args) {
        parser.position += m
          .iter()
          .map(|x| x.len())
          .sum::<usize>();
        VariableParser::parse(parser, m, args)?
      } else if let Some(token) = parser.tokens.get(parser.position) {
        parser.position += 1;
        IdentifierParser::new(token.value.to_owned())
      } else {
        return Err(ParserError::internal("ObjectAccess", args));
      };
      return Ok(
        ObjectAccessNode::new(args.last_expr.to_owned().unwrap(), expr, is_bracket, is_nullsafe)
      );
    }
    Err(ParserError::internal("ObjectAccess", args))
  }
}
