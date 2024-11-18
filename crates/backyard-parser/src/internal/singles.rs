use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{
  BreakNode,
  CloneNode,
  ContinueNode,
  EchoNode,
  GlobalNode,
  GotoNode,
  NewNode,
  Node,
  ParentNode,
  PrintNode,
  ReturnNode,
  StaticNode,
  ThisNode,
  ThrowNode,
};

use crate::{ parser::{ LoopArgument, Parser }, utils::{ match_pattern, Lookup } };

#[derive(Debug, Clone)]
pub struct SinglesParser {}

impl SinglesParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(
          vec![
            TokenType::Break,
            TokenType::Continue,
            TokenType::Echo,
            TokenType::Goto,
            TokenType::New,
            TokenType::Print,
            TokenType::Return,
            TokenType::Throw,
            TokenType::Parent,
            TokenType::Static,
            TokenType::Clone,
            TokenType::Global,
            TokenType::This
          ]
        ),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Option<Box<Node>> {
    if let [key] = matched.as_slice() {
      if let Some(key) = key.first() {
        if [TokenType::Parent, TokenType::Static, TokenType::This].contains(&key.token_type) {
          return match key.token_type {
            TokenType::Parent => Some(ParentNode::new(key.value.to_owned())),
            TokenType::Static => Some(StaticNode::new(key.value.to_owned())),
            TokenType::This => Some(ThisNode::new(key.value.to_owned())),
            _ => None,
          };
        }
        let argument = parser.get_statement(
          &mut LoopArgument::with_tokens(
            "singles",
            &args.separators.combine(&[TokenType::Semicolon]),
            &args.breakers.combine(&[TokenType::RightCurlyBracket])
          )
        );
        let node: Option<Box<Node>> = match key.token_type {
          TokenType::Break => Some(BreakNode::new(argument.to_owned())),
          TokenType::Continue => Some(ContinueNode::new(argument.to_owned())),
          TokenType::Return => Some(ReturnNode::new(argument.to_owned())),
          _ => None,
        };
        if node.is_some() {
          return node;
        }
        if argument.is_none() {
          return None;
        }
        let argument = argument.unwrap();
        return match key.token_type {
          TokenType::Echo => Some(EchoNode::new(argument)),
          TokenType::New => Some(NewNode::new(argument)),
          TokenType::Print => Some(PrintNode::new(argument)),
          TokenType::Throw => Some(ThrowNode::new(argument)),
          TokenType::Clone => Some(CloneNode::new(argument)),
          TokenType::Global => Some(GlobalNode::new(argument)),
          TokenType::Goto => Some(GotoNode::new(argument)),
          _ => None,
        };
      }
    }
    None
  }
}
