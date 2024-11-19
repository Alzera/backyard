use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, VariableNode };

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::identifier::IdentifierParser;

#[derive(Debug, Clone)]
pub struct VariableParser {}

impl VariableParser {
  pub fn new(name: String, is_ref: bool) -> Box<Node> {
    VariableParser::new_bracked(IdentifierParser::new(name), is_ref)
  }

  pub fn new_bracked(name: Box<Node>, is_ref: bool) -> Box<Node> {
    VariableNode::new(is_ref, name)
  }
}

impl VariableParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Reference]),
        Lookup::Equal(vec![TokenType::Variable, TokenType::VariableBracketOpen]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if matched.len() == 2 {
      if let [is_ref, name] = matched.as_slice() {
        if let Some(name) = name.get(0) {
          if name.token_type == TokenType::VariableBracketOpen {
            let expr = parser.get_statement(
              &mut LoopArgument::with_tokens("variable", &[TokenType::VariableBracketClose], &[])
            )?;
            parser.position += 1;
            if expr.is_some() {
              return Ok(VariableParser::new_bracked(expr.unwrap(), is_ref.len() > 0));
            }
          } else {
            return Ok(VariableParser::new(name.value.to_owned(), is_ref.len() > 0));
          }
        }
      }
    }
    Err(ParserError::internal("Variable", args))
  }
}
