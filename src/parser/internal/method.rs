use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::method::MethodNode,
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, some_or_default, Lookup },
  },
};

use super::{ comment::CommentParser, function::FunctionParser };

#[derive(Debug, Clone)]
pub struct MethodParser {}

impl Internal for MethodParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    let basic_grammar = [
      Lookup::Optional(vec![TokenType::Public, TokenType::Private, TokenType::Protected]),
      Lookup::Optional(vec![TokenType::Abstract, TokenType::Final]),
      Lookup::Optional(vec![TokenType::Static]),
      Lookup::Equal(vec![TokenType::Function]),
    ].to_vec();
    match_pattern(tokens, basic_grammar)
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [visibility, modifier, is_static, _] = matched.as_slice() {
      parser.position -= 1;
      let function = parser.get_statement(
        &mut LoopArgument::new(
          "method",
          &[TokenType::RightCurlyBracket],
          &[],
          &[ParserInternal::Function(FunctionParser {}), ParserInternal::Comment(CommentParser {})]
        )
      );
      if function.is_none() {
        return None;
      }
      return Some(
        MethodNode::new(
          some_or_default(visibility.get(0), String::from(""), |i| i.value.to_owned()),
          some_or_default(modifier.get(0), String::from(""), |i| i.value.to_owned()),
          is_static.len() > 0,
          function.unwrap()
        )
      );
    }
    None
  }
}
