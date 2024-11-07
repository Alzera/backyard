use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ Node, Nodes },
    nodes::{
      function::{ AnonymousFunctionNode, ArrowFunctionNode, FunctionNode, ParameterNode },
      property::PropertyNode,
      singles::StaticNode,
    },
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, some_or_default, Lookup },
  },
};

use super::{
  block::BlockParser,
  comment::CommentParser,
  identifier::IdentifierParser,
  property::{ PropertyItemParser, PropertyParser },
  types::TypesParser,
};

#[derive(Debug, Clone)]
pub struct FunctionParser {}

impl FunctionParser {
  pub fn test(tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    if
      let Some(m) = match_pattern(
        tokens,
        [
          Lookup::Equal(vec![TokenType::Function]),
          Lookup::Optional(vec![TokenType::BitwiseAnd]),
          Lookup::Equal(vec![TokenType::Identifier, TokenType::Clone]),
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

  pub fn parse(parser: &mut Parser, matched: Vec<Vec<Token>>, args: &LoopArgument) -> Option<Node> {
    match matched.len() {
      4 => FunctionParser::parse_basic(matched, parser),
      3 => {
        if let Some(f) = matched.get(0) {
          if let Some(f) = f.get(0) {
            if f.token_type == TokenType::Fn {
              return FunctionParser::parse_arrow(matched, parser, args);
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
  pub fn parse_arrow(
    matched: Vec<Vec<Token>>,
    parser: &mut Parser,
    args: &LoopArgument
  ) -> Option<Node> {
    if let [_, is_ref, _] = matched.as_slice() {
      let arguments = FunctionParser::get_parameters(parser);
      let return_type = FunctionParser::get_return_type(parser);
      parser.position += 1;
      let body = guard!(
        parser.get_statement(&mut LoopArgument::with_tokens("function_arrow", &[], args.breakers))
      );
      return Some(ArrowFunctionNode::new(is_ref.len() > 0, arguments, return_type, body));
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
      return Some(AnonymousFunctionNode::new(is_ref.len() > 0, arguments, uses, return_type, body));
    }
    None
  }

  pub fn parse_basic(matched: Vec<Vec<Token>>, parser: &mut Parser) -> Option<Node> {
    if let [_, is_ref, name, _] = matched.as_slice() {
      let name = some_or_default(name.get(0), String::from(""), |i| i.value.to_owned());
      let arguments = if name == "__construct" {
        parser.get_children(
          &mut LoopArgument::new(
            "function_construct_parameters",
            &[TokenType::Comma],
            &[TokenType::RightParenthesis],
            &[
              (TypesParser::test, TypesParser::parse),
              (ConstructorParameterParser::test, ConstructorParameterParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        )
      } else {
        FunctionParser::get_parameters(parser)
      };
      let return_type = FunctionParser::get_return_type(parser);
      let body = if guard!(parser.tokens.get(parser.position)).token_type == TokenType::Semicolon {
        None
      } else {
        Some(BlockParser::new(parser))
      };
      return Some(
        FunctionNode::new(
          is_ref.len() > 0,
          IdentifierParser::new(name),
          arguments,
          return_type,
          body
        )
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
        &[
          (TypesParser::test, TypesParser::parse),
          (ParameterParser::test, ParameterParser::parse),
          (CommentParser::test, CommentParser::parse),
        ]
      )
    )
  }

  pub fn get_return_type(parser: &mut Parser) -> Option<Node> {
    if let Some(next_token) = parser.tokens.get(parser.position) {
      if next_token.token_type == TokenType::Colon {
        parser.position += 1;
        if let Some(next_token) = parser.tokens.get(parser.position) {
          if next_token.token_type == TokenType::Static {
            parser.position += 1;
            return Some(StaticNode::new(String::from("static")));
          }
        }
        return parser.get_statement(
          &mut LoopArgument::new(
            "function_return_type",
            &[TokenType::LeftCurlyBracket, TokenType::Arrow],
            &[],
            &[
              (TypesParser::test, TypesParser::parse),
              (CommentParser::test, CommentParser::parse),
            ]
          )
        );
      }
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct ConstructorParameterParser {}

impl ConstructorParameterParser {
  pub fn test(tokens: &Vec<Token>, args: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    PropertyParser::test(tokens, args)
  }

  pub fn parse(parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [visibility, modifier] = matched.as_slice() {
      let item = guard!(
        parser.get_statement(
          &mut LoopArgument::new(
            "constructor_parameter",
            &[],
            &[TokenType::Comma, TokenType::RightParenthesis],
            &[
              (CommentParser::test, CommentParser::parse),
              (TypesParser::test, TypesParser::parse),
              (PropertyItemParser::test, PropertyItemParser::parse),
            ]
          )
        )
      );
      return Some(
        PropertyNode::new(
          some_or_default(visibility.get(0), String::from(""), |i| i.value.to_owned()),
          some_or_default(modifier.get(0), String::from(""), |i| i.value.to_owned()),
          vec![item]
        )
      );
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct ParameterParser {}

impl ParameterParser {
  pub fn test(tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Optional(vec![TokenType::Reference]),
        Lookup::Optional(vec![TokenType::Ellipsis]),
        Lookup::Equal(vec![TokenType::Variable]),
        Lookup::Optional(vec![TokenType::Assignment]),
      ].to_vec()
    )
  }

  pub fn parse(parser: &mut Parser, matched: Vec<Vec<Token>>, args: &LoopArgument) -> Option<Node> {
    if let [is_ref, is_ellipsis, name, has_value] = matched.as_slice() {
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
        ParameterNode::new(
          args.last_expr.to_owned(),
          is_ref.len() > 0,
          is_ellipsis.len() > 0,
          IdentifierParser::from_matched(name),
          value
        )
      );
    }
    None
  }
}
