use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, ObjectAccessNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{ call::CallParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct ObjectAccessParser {}

impl ObjectAccessParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(
          vec![
            TokenType::ObjectAccess,
            TokenType::NullsafeObjectAccess,
            TokenType::ObjectAccessBracketOpen,
            TokenType::NullsafeObjectAccessBracketOpen
          ]
        ),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [access_type] = matched.as_slice() {
      match
        guard!(access_type.get(0), {
          return Err(ParserError::internal("ObjectAccess", args));
        }).token_type
      {
        TokenType::ObjectAccess | TokenType::NullsafeObjectAccess => {
          let expr = guard!(
            parser.get_statement(
              &mut LoopArgument::safe(
                "objectaccess",
                &[],
                &[
                  TokenType::Semicolon,
                  TokenType::ObjectAccess,
                  TokenType::NullsafeObjectAccess,
                  TokenType::ObjectAccessBracketOpen,
                  TokenType::NullsafeObjectAccessBracketOpen,
                ],
                &[
                  (CallParser::test, CallParser::parse),
                  (IdentifierParser::test, IdentifierParser::parse),
                ]
              )
            )?,
            {
              return Err(ParserError::internal("ObjectAccess", args));
            }
          );
          if let Some(next_token) = parser.tokens.get(parser.position) {
            if
              [
                TokenType::Semicolon,
                TokenType::ObjectAccess,
                TokenType::NullsafeObjectAccess,
                TokenType::ObjectAccessBracketOpen,
                TokenType::NullsafeObjectAccessBracketOpen,
              ].contains(&next_token.token_type)
            {
              parser.position += 1;
            }
          }
          return Ok(ObjectAccessNode::new(args.last_expr.to_owned().unwrap(), expr));
        }
        TokenType::ObjectAccessBracketOpen | TokenType::NullsafeObjectAccessBracketOpen => {
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
    }
    Err(ParserError::internal("ObjectAccess", args))
  }
}
