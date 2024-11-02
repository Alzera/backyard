use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ AnonymousFunctionNode, ArrowFunctionNode, FunctionNode, Node, Nodes, ParameterNode },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, Lookup },
  },
};

use super::{ block::BlockParser, identifier::IdentifierParser, types::TypesParser };

#[derive(Debug, Clone)]
pub struct FunctionParser {}

impl Internal for FunctionParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    if
      let Some(m) = match_pattern(
        tokens,
        [
          Lookup::Equal(vec![TokenType::Function]),
          Lookup::Optional(vec![TokenType::BitwiseAnd]),
          Lookup::Equal(vec![TokenType::Identifier]),
          Lookup::Equal(vec![TokenType::LeftParenthesis]),
        ].to_vec()
      )
    {
      return Some(m);
    }
    if
      let Some(m) = match_pattern(
        tokens,
        [
          Lookup::Equal(vec![TokenType::Function]),
          Lookup::Optional(vec![TokenType::BitwiseAnd]),
          Lookup::Equal(vec![TokenType::LeftParenthesis]),
        ].to_vec()
      )
    {
      return Some(m);
    }
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Fn]),
        Lookup::Optional(vec![TokenType::BitwiseAnd]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    match matched.len() {
      4 => FunctionParser::parse_basic(matched, parser),
      3 => {
        if let Some(f) = matched.get(0) {
          if let Some(f) = f.get(0) {
            if f.token_type == TokenType::Fn {
              return FunctionParser::parse_arrow(matched, parser);
            } else if f.token_type == TokenType::Function {
              return FunctionParser::parse_anonymous(matched, parser);
            }
          }
        }
        None
      }
      _ => None,
    }
  }
}

impl FunctionParser {
  pub fn parse_arrow(matched: Vec<Vec<Token>>, parser: &mut Parser) -> Option<Node> {
    if let [_, is_ref, _] = matched.as_slice() {
      let arguments = FunctionParser::get_parameters(parser);
      let return_type = FunctionParser::get_return_type(parser);
      parser.position += 1;
      let body = guard!(parser.get_statement(&mut LoopArgument::default("function_arrow")));
      return Some(
        Box::new(ArrowFunctionNode {
          is_ref: is_ref.len() > 0,
          arguments,
          return_type,
          body,
        })
      );
    }
    None
  }

  pub fn parse_anonymous(matched: Vec<Vec<Token>>, parser: &mut Parser) -> Option<Node> {
    if let [_, is_ref, _] = matched.as_slice() {
      let arguments = FunctionParser::get_parameters(parser);
      let mut uses = vec![];
      if let Some(next_token) = parser.tokens.get(parser.position) {
        if next_token.token_type == TokenType::Use {
          parser.position += 2;
          uses = parser.get_children(
            &mut LoopArgument::with_tokens(
              "function_anonymous",
              &[TokenType::Comma],
              &[TokenType::RightParenthesis]
            )
          );
        }
      }
      let return_type = FunctionParser::get_return_type(parser);
      let body = BlockParser::new(parser);
      return Some(
        Box::new(AnonymousFunctionNode {
          is_ref: is_ref.len() > 0,
          arguments,
          uses,
          return_type,
          body,
        })
      );
    }
    None
  }

  pub fn parse_basic(matched: Vec<Vec<Token>>, parser: &mut Parser) -> Option<Node> {
    if let [_, is_ref, name, _] = matched.as_slice() {
      let arguments = FunctionParser::get_parameters(parser);
      let return_type = FunctionParser::get_return_type(parser);
      let body = if guard!(parser.tokens.get(parser.position)).token_type == TokenType::Semicolon {
        None
      } else {
        Some(BlockParser::new(parser))
      };
      return Some(
        Box::new(FunctionNode {
          is_ref: is_ref.len() > 0,
          name: IdentifierParser::from_matched(name),
          arguments,
          return_type,
          body,
        })
      );
    }
    None
  }

  pub fn get_parameters(parser: &mut Parser) -> Nodes {
    parser.get_children(
      &mut LoopArgument::new(
        "function_parameters",
        &[TokenType::Comma],
        &[TokenType::RightParenthesis],
        &[ParserInternal::Parameter(ParameterParser {})]
      )
    )
  }

  pub fn get_return_type(parser: &mut Parser) -> Option<Node> {
    if let Some(next_token) = parser.tokens.get(parser.position) {
      if next_token.token_type == TokenType::Colon {
        parser.position += 1;
        return parser.get_statement(
          &mut LoopArgument::new(
            "function_return_type",
            &[TokenType::LeftCurlyBracket, TokenType::Arrow],
            &[],
            &[ParserInternal::Type(TypesParser {})]
          )
        );
      }
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct ParameterParser {}

impl Internal for ParameterParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::QuestionMark]),
        Lookup::Optional(vec![TokenType::Type, TokenType::Identifier]),
        Lookup::Optional(vec![TokenType::Reference]),
        Lookup::Optional(vec![TokenType::Ellipsis]),
        Lookup::Equal(vec![TokenType::Variable]),
        Lookup::Optional(vec![TokenType::Assignment]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [is_nullable, type_name, is_ref, is_ellipsis, name, has_value] = matched.as_slice() {
      let variable_type = TypesParser::new(is_nullable, type_name);
      let value = if has_value.len() > 0 {
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "parameter",
            &[TokenType::Comma, TokenType::RightParenthesis],
            &[]
          )
        )
      } else {
        None
      };
      return Some(
        Box::new(ParameterNode {
          variable_type,
          is_ref: is_ref.len() > 0,
          is_ellipsis: is_ellipsis.len() > 0,
          name: IdentifierParser::from_matched(name),
          value,
        })
      );
    }
    None
  }
}
