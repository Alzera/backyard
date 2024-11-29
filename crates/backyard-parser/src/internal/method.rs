use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Inheritance, Location, MethodNode, Node, Visibility };

use crate::{ error::ParserError, guard, parser::{ LoopArgument, Parser } };

use super::{ comment::CommentParser, function::FunctionParser };

#[derive(Debug, Clone)]
pub struct MethodParser;

impl MethodParser {
  #[allow(unused_variables)]
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    let modifiers_rule = [
      [TokenType::Public, TokenType::Private, TokenType::Protected].to_vec(),
      [TokenType::Abstract, TokenType::Final].to_vec(),
      [TokenType::Static].to_vec(),
    ];
    let mut modifiers = vec![vec![], vec![], vec![]];
    let mut pos = 0;
    loop {
      let token = tokens.get(pos);
      pos += 1;
      if pos > 4 || token.is_none() {
        return None;
      }
      let token = token.unwrap();
      if token.token_type == TokenType::Function {
        modifiers.push(vec![token.to_owned()]);
        break;
      }
      let mut assigned = false;
      for (i, modifier) in modifiers_rule.iter().enumerate() {
        if !modifiers[i].is_empty() {
          continue;
        }
        if modifier.contains(&token.token_type) {
          modifiers[i].push(token.clone());
          assigned = true;
          break;
        }
      }
      if !assigned {
        return None;
      }
    }
    Some(modifiers)
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    start_loc: Location,
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
              (FunctionParser::class_test, FunctionParser::parse),
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
          Visibility::from_str(
            &visibility
              .first()
              .map(|i| i.value.to_owned())
              .unwrap_or_default()
          ),
          Inheritance::from_str(
            &modifier
              .first()
              .map(|i| i.value.to_owned())
              .unwrap_or_default()
          ),
          !is_static.is_empty(),
          function,
          parser.gen_loc(start_loc)
        )
      );
    }
    Err(ParserError::internal("Method", args))
  }
}
