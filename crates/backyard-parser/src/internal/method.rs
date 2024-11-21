use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, MethodNode };
use utils::guard;

use crate::{ error::ParserError, parser::{ LoopArgument, Parser }, utils::some_or_default };

use super::{ comment::CommentParser, function::FunctionParser };

#[derive(Debug, Clone)]
pub struct MethodParser {}

impl MethodParser {
  #[allow(unused_variables)]
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    let modifiers_rule = [
      [TokenType::Public, TokenType::Private, TokenType::Protected].to_vec(),
      [TokenType::Abstract, TokenType::Final].to_vec(),
      [TokenType::Static].to_vec(),
    ];
    let mut modifiers = vec![vec![], vec![], vec![]];
    let mut pos = 0;
    println!("Ale 1");
    loop {
      let token = tokens.get(pos);
      println!("Ale 2: {:?}", token);
      pos += 1;
      if pos > 4 || token.is_none() {
        println!("Ale 3");
        return None;
      }
      let token = token.unwrap();
      println!("Ale 4");
      if token.token_type == TokenType::Function {
        println!("Ale 5");
        modifiers.push(vec![token.to_owned()]);
        break;
      }
      println!("Ale 6");
      let mut assigned = false;
      for (i, modifier) in modifiers_rule.iter().enumerate() {
        println!("Ale 7");
        if modifiers[i].len() > 0 {
          println!("Ale 8");
          continue;
        }
        println!("Ale 9");
        if modifier.contains(&token.token_type) {
          println!("Ale 10");
          modifiers[i].push(token.clone());
          assigned = true;
          break;
        }
      }
      if !assigned {
        return None;
      }
    }
    println!("Ale 11");
    return Some(modifiers);
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
