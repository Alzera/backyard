use crate::{
  lexer::token::{ Token, TokenType, TokenTypeArrayCombine },
  parser::{
    node::Node,
    nodes::singles::{
      BreakNode,
      CloneNode,
      ContinueNode,
      EchoNode,
      GlobalNode,
      GotoNode,
      NewNode,
      ParentNode,
      PrintNode,
      ReturnNode,
      StaticNode,
      ThrowNode,
    },
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

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
            TokenType::Global
          ]
        ),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Option<Node> {
    if let [key] = matched.as_slice() {
      if let Some(key) = key.first() {
        if [TokenType::Parent, TokenType::Static].contains(&key.token_type) {
          return match key.token_type {
            TokenType::Parent => Some(ParentNode::boxed(key.value.to_owned())),
            TokenType::Static => Some(StaticNode::boxed(key.value.to_owned())),
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
        let node: Option<Node> = match key.token_type {
          TokenType::Break => Some(BreakNode::boxed(argument.to_owned())),
          TokenType::Continue => Some(ContinueNode::boxed(argument.to_owned())),
          TokenType::Return => Some(ReturnNode::boxed(argument.to_owned())),
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
          TokenType::Echo => Some(EchoNode::boxed(argument)),
          TokenType::New => Some(NewNode::boxed(argument)),
          TokenType::Print => Some(PrintNode::boxed(argument)),
          TokenType::Throw => Some(ThrowNode::boxed(argument)),
          TokenType::Clone => Some(CloneNode::boxed(argument)),
          TokenType::Global => Some(GlobalNode::boxed(argument)),
          TokenType::Goto => Some(GotoNode::boxed(argument)),
          _ => None,
        };
      }
    }
    None
  }
}
