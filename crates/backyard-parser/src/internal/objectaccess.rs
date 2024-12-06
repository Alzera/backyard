use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, ObjectAccessNode };

use crate::{
  error::ParserError,
  guard,
  internal::{ identifier::IdentifierParser, variable::VariableParser },
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

#[derive(Debug, Clone)]
pub struct ObjectAccessParser;

impl ObjectAccessParser {
  pub fn test(tokens: &[Token], args: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    args.last_expr.as_ref()?;
    match_pattern(
      tokens,
      &[Lookup::Equal(&[TokenType::ObjectAccess, TokenType::NullsafeObjectAccess])]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [access_type] = matched.as_slice() {
      let is_nullsafe = if let LookupResultWrapper::Equal(access_type) = &access_type.wrapper {
        access_type.token_type == TokenType::NullsafeObjectAccess
      } else {
        return Err(ParserError::Internal);
      };
      let is_bracket = if let Some(next_token) = parser.tokens.get(parser.position) {
        next_token.token_type == TokenType::LeftCurlyBracket
      } else {
        return Err(ParserError::Internal);
      };
      let expr = if is_bracket {
        parser.position += 1;
        let t = guard!(
          parser.get_statement(
            &mut LoopArgument::with_tokens("objectaccess", &[], &[TokenType::RightCurlyBracket])
          )?
        );
        parser.position += 1;
        t
      } else if let Some(m) = VariableParser::test(&parser.tokens[parser.position..], args) {
        let loc = parser.tokens.get(parser.position).unwrap().get_location().unwrap();
        parser.position += m
          .iter()
          .map(|x| x.size)
          .sum::<usize>();
        VariableParser::parse(parser, m, loc, args)?
      } else if let Some(token) = parser.tokens.get(parser.position) {
        parser.position += 1;
        IdentifierParser::from_token(token)
      } else {
        return Err(ParserError::Internal);
      };
      return Ok(
        ObjectAccessNode::new(
          args.last_expr.to_owned().unwrap(),
          expr,
          is_bracket,
          is_nullsafe,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::Internal)
  }
}
