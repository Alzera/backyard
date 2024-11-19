use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, ObjectAccessNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct ObjectAccessParser {}

impl ObjectAccessParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    if
      let Some(m) = match_pattern(
        tokens,
        [
          Lookup::Equal(vec![TokenType::ObjectAccess, TokenType::NullsafeObjectAccess]),
          Lookup::Equal(vec![TokenType::Identifier]),
        ].to_vec()
      )
    {
      return Some(m);
    }
    match_pattern(
      tokens,
      [
        Lookup::Equal(
          vec![TokenType::ObjectAccessBracketOpen, TokenType::NullsafeObjectAccessBracketOpen]
        ),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    match matched.len() {
      2 => {
        if let [_, prop] = matched.as_slice() {
          return Ok(
            ObjectAccessNode::new(
              args.last_expr.to_owned().unwrap(),
              IdentifierParser::from_matched(prop)
            )
          );
        }
      }
      1 => {
        let expr = guard!(
          parser.get_statement(
            &mut LoopArgument::with_tokens(
              "objectaccess",
              &[TokenType::ObjectAccessBracketClose],
              &[]
            )
          )?,
          {
            return Err(ParserError::internal("ObjectAccess", args));
          }
        );
        parser.position += 1;
        return Ok(ObjectAccessNode::new(args.last_expr.to_owned().unwrap(), expr));
      }
      _ => {
        return Err(ParserError::internal("ObjectAccess", args));
      }
    }
    Err(ParserError::internal("ObjectAccess", args))
  }
}
