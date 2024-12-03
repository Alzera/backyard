use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Inheritance, Location, MethodNode, Node, Visibility };

use crate::{
  error::ParserError,
  guard,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult, LookupResultWrapper },
};

use super::{ comment::CommentParser, function::FunctionParser };

#[derive(Debug, Clone)]
pub struct MethodParser;

impl MethodParser {
  #[allow(unused_variables)]
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
    match_pattern(
      tokens,
      &[
        Lookup::Modifiers(
          &[
            &[TokenType::Public, TokenType::Private, TokenType::Protected],
            &[TokenType::Abstract, TokenType::Final],
            &[TokenType::Static],
          ]
        ),
        Lookup::Equal(&[TokenType::Function]),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [modifiers, _] = matched.as_slice() {
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
      let mut visibility = None;
      let mut inheritance = None;
      let mut is_static = false;
      if let LookupResultWrapper::Modifier(modifiers) = &modifiers.wrapper {
        if let [visibility_modifier, inheritance_modifier, static_modifier] = modifiers.as_slice() {
          visibility = Visibility::try_parse(
            &visibility_modifier
              .as_ref()
              .map(|i| i.value.to_owned())
              .unwrap_or_default()
          );
          inheritance = Inheritance::try_parse(
            &inheritance_modifier
              .as_ref()
              .map(|i| i.value.to_owned())
              .unwrap_or_default()
          );
          is_static = static_modifier.is_some();
        }
      }
      return Ok(
        MethodNode::new(visibility, inheritance, is_static, function, parser.gen_loc(start_loc))
      );
    }
    Err(ParserError::internal("Method", args))
  }
}
