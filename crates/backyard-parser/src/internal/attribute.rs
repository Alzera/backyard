use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ AttributeItemNode, AttributeNode, Node };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct AttributeParser {}

impl AttributeParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Attribute])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      let items = parser.get_children(
        &mut LoopArgument::new(
          "attribute",
          &[TokenType::Comma],
          &[TokenType::RightSquareBracket],
          &[(AttributeItemParser::test, AttributeItemParser::parse)]
        )
      )?;
      let expr = parser.get_statement(
        &mut LoopArgument::new("attribute", args.separators, args.breakers, args.parsers)
      )?;
      if let Some(mut expr) = expr {
        expr.leadings.insert(0, AttributeNode::new(items));
        return Ok(expr);
      }
    }
    Err(ParserError::internal("Attribute", args))
  }
}

#[derive(Debug, Clone)]
pub struct AttributeItemParser {}

impl AttributeItemParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Identifier, TokenType::Name])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [name] = matched.as_slice() {
      let name = guard!(name.get(0), {
        return Err(ParserError::internal("ArrayItem", args));
      }).value.to_owned();
      let mut arguments = vec![];
      let token = guard!(parser.tokens.get(parser.position), {
        return Err(ParserError::internal("ArrayItem", args));
      });
      if [TokenType::LeftParenthesis].contains(&token.token_type) {
        parser.position += 1;
        arguments = parser.get_children(
          &mut LoopArgument::with_tokens(
            "attribute_item",
            &[TokenType::Comma],
            &[TokenType::RightParenthesis]
          )
        )?;
      }
      return Ok(AttributeItemNode::new(name, arguments));
    }
    Err(ParserError::internal("ArrayItem", args))
  }
}
