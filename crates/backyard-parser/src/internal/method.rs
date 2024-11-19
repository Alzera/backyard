use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, MethodNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, some_or_default, Lookup },
};

use super::{ comment::CommentParser, function::FunctionParser };

#[derive(Debug, Clone)]
pub struct MethodParser {}

impl MethodParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    let basic_grammar = [
      Lookup::Optional(vec![TokenType::Public, TokenType::Private, TokenType::Protected]),
      Lookup::Optional(vec![TokenType::Abstract, TokenType::Final]),
      Lookup::Optional(vec![TokenType::Static]),
      Lookup::Equal(vec![TokenType::Function]),
    ].to_vec();
    match_pattern(tokens, basic_grammar)
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [visibility, modifier, is_static, _] = matched.as_slice() {
      parser.position -= 1;
      let function = guard!(
        parser.get_statement(
          &mut LoopArgument::new(
            "method",
            &[TokenType::RightCurlyBracket],
            &[],
            &[
              (FunctionParser::test, FunctionParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        )?,
        {
          return Err(ParserError::internal("Method", args));
        }
      );
      return Ok(
        MethodNode::new(
          some_or_default(visibility.get(0), String::from(""), |i| i.value.to_owned()),
          some_or_default(modifier.get(0), String::from(""), |i| i.value.to_owned()),
          is_static.len() > 0,
          function
        )
      );
    }
    Err(ParserError::internal("Method", args))
  }
}
