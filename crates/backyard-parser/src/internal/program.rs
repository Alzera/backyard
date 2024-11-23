use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, ProgramNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct ProgramParser {}

impl ProgramParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::OpenTag, TokenType::OpenTagShort, TokenType::OpenTagEcho]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [opentag] = matched.as_slice() {
      let opentag = guard!(opentag.get(0), {
        return Err(ParserError::internal("Program", args));
      });
      if opentag.token_type == TokenType::OpenTagEcho {
        let expr = guard!(
          parser.get_statement(
            &mut LoopArgument::with_tokens(
              "program",
              &[],
              &[TokenType::Semicolon, TokenType::CloseTag]
            )
          )?,
          {
            return Err(ParserError::internal("Program", args));
          }
        );
        if let Some(token) = parser.tokens.get(parser.position) {
          if token.token_type == TokenType::Semicolon {
            parser.position += 2;
          } else if token.token_type == TokenType::CloseTag {
            parser.position += 1;
          }
        }
        return Ok(ProgramNode::new(opentag.value.to_owned(), vec![expr]));
      } else {
        let closetag = if opentag.token_type == TokenType::OpenTag {
          TokenType::CloseTag
        } else {
          TokenType::CloseTagShort
        };
        let program = parser.get_children(
          &mut LoopArgument::with_tokens("program", &[TokenType::Semicolon], &[closetag])
        )?;
        return Ok(ProgramNode::new(opentag.value.to_owned(), program));
      }
    }
    Err(ParserError::internal("Program", args))
  }
}
