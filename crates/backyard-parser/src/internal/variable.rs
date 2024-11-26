use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, VariableNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct VariableParser;

impl VariableParser {
  pub fn new(name: String) -> Box<Node> {
    VariableParser::new_bracked(IdentifierParser::new(name))
  }

  pub fn new_bracked(name: Box<Node>) -> Box<Node> {
    VariableNode::new(name)
  }
}

impl VariableParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, &[Lookup::Equal(&[TokenType::Variable, TokenType::VariableBracketOpen])])
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [name] = matched.as_slice() {
      if let Some(name) = name.first() {
        if name.token_type == TokenType::VariableBracketOpen {
          let expr = parser.get_statement(
            &mut LoopArgument::with_tokens("variable", &[], &[TokenType::VariableBracketClose])
          )?;
          parser.position += 1;
          if let Some(expr) = expr {
            return Ok(VariableParser::new_bracked(expr));
          }
        } else {
          return Ok(VariableParser::new(name.value.to_owned()));
        }
      }
    }
    Err(ParserError::internal("Variable", args))
  }
}
