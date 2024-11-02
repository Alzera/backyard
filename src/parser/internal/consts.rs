use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ ConstNode, ConstPropertyNode, Node, Nodes },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, some_or_default, Lookup },
  },
};

use super::{ assignment::AssignmentParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct ConstParser {}

impl ConstParser {
  pub fn get_consts(parser: &mut Parser) -> Nodes {
    let consts = parser.get_children(
      &mut LoopArgument::new(
        "const",
        &[TokenType::Comma],
        &[TokenType::Semicolon],
        &[
          ParserInternal::Identifier(IdentifierParser {}),
          ParserInternal::Assignment(AssignmentParser {}),
        ]
      )
    );
    parser.position -= 1;
    consts
  }
}

impl Internal for ConstParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Const])].to_vec())
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_] = matched.as_slice() {
      return Some(
        Box::new(ConstNode {
          consts: ConstParser::get_consts(parser),
        })
      );
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct ConstPropertyParser {}

impl Internal for ConstPropertyParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Public, TokenType::Private, TokenType::Protected]),
        Lookup::Equal(vec![TokenType::Const]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [visibility, _] = matched.as_slice() {
      return Some(
        Box::new(ConstPropertyNode {
          visibility: some_or_default(visibility.get(0), String::from(""), |i| i.value.to_owned()),
          consts: ConstParser::get_consts(parser),
        })
      );
    }
    None
  }
}
