use crate::{
  lexer::token::{ Token, TokenType, TokenTypeArrayCombine },
  parser::{
    node::Node,
    nodes::singles::{
      BreakNode,
      ContinueNode,
      EchoNode,
      NewNode,
      PrintNode,
      ReturnNode,
      ThrowNode,
    },
    parser::{ Internal, LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct SinglesParser {}

impl Internal for SinglesParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
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
            TokenType::Throw
          ]
        ),
      ].to_vec()
    )
  }

  fn parse(
    &self,
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &LoopArgument
  ) -> Option<Node> {
    if let [key] = matched.as_slice() {
      if let Some(key) = key.first() {
        let argument = parser.get_statement(
          &mut LoopArgument::with_tokens(
            "singles",
            &args.separators.combine(&[TokenType::Semicolon]),
            &args.breakers.combine(&[TokenType::RightCurlyBracket])
          )
        );
        let node: Option<Node> = match key.token_type {
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
          _ => None,
        };
      }
    }
    None
  }
}
