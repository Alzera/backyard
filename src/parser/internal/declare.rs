use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ BodyType, DeclareArgumentNode, DeclareNode, Node },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, Lookup },
  },
};

use super::{ block::BlockParser, identifier::IdentifierParser };

#[derive(Debug, Clone)]
pub struct DeclareParser {}

impl Internal for DeclareParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Declare]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let arguments = parser.get_children(
        &mut LoopArgument::new(
          "declare",
          &[TokenType::Comma],
          &[TokenType::RightParenthesis],
          &[ParserInternal::DeclareArgument(DeclareArgumentParser {})]
        )
      );
      let body_type: BodyType = {
        let mut body_type = BodyType::Empty;
        if let Some(close) = parser.tokens.get(parser.position) {
          body_type = match close.token_type {
            TokenType::ShortFormStart => BodyType::Short,
            TokenType::LeftCurlyBracket => BodyType::Basic,
            _ => BodyType::Empty,
          };
        }
        body_type
      };
      let body = match body_type {
        BodyType::Empty => None,
        BodyType::Basic => Some(BlockParser::new(parser)),
        BodyType::Short => Some(BlockParser::new_short(parser, &[TokenType::EndDeclare])),
      };
      return Some(
        Box::new(DeclareNode {
          arguments,
          body,
          body_type,
        })
      );
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct DeclareArgumentParser {}

impl Internal for DeclareArgumentParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Identifier]),
        Lookup::Equal(vec![TokenType::Assignment]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [name, _] = matched.as_slice() {
      if
        let Some(value) = parser.get_statement(
          &mut LoopArgument::with_tokens(
            "declare_argument",
            &[TokenType::Comma],
            &[TokenType::RightParenthesis]
          )
        )
      {
        return Some(
          Box::new(DeclareArgumentNode {
            name: IdentifierParser::from_matched(name),
            value,
          })
        );
      }
    }
    None
  }
}
