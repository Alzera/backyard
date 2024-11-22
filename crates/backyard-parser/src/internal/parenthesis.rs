use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ CastNode, Node, ParenthesisNode, TypeNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct ParenthesisParser {}

impl ParenthesisParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::LeftParenthesis])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
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
          let token = token.clone();
          if let Some(next_token) = parser.tokens.get(parser.position + 1) {
            if next_token.token_type == TokenType::RightParenthesis {
              parser.position += 2;
              let expression = guard!(
                parser.get_statement(
                  &mut LoopArgument::with_tokens("cast", &args.separators, &args.breakers)
                )?,
                {
                  return Err(ParserError::internal("Cast", args));
                }
              );
              return Ok(
                CastNode::new(TypeNode::new(false, vec![token.value.to_owned()]), expression)
              );
            }
          }
        }
      }
      let statement = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "parenthesis",
            &args.separators,
            &args.breakers.combine(&[TokenType::RightParenthesis])
          )
        )?,
        {
          return Err(ParserError::internal("Parenthesis", args));
        }
      );
      parser.position += 1;
      return Ok(ParenthesisNode::new(statement));
    }
    Err(ParserError::internal("Parenthesis", args))
  }
}
