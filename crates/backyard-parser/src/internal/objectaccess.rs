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
          let expr: Box<Node> = if
            let Some(m) = VariableParser::test(&parser.tokens[parser.position..].to_vec(), args)
          {
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
        _ => {}
      }
    }
    Err(ParserError::internal("ObjectAccess", args))
  }
}
