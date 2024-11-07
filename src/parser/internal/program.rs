use crate::{
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::program::ProgramNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct ProgramParser {}

impl ProgramParser {
  pub fn test(tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [Lookup::Equal(vec![TokenType::OpenTag, TokenType::OpenTagShort])].to_vec()
    )
  }

  pub fn parse(parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_] = matched.as_slice() {
      let program = parser.get_children(&mut LoopArgument::default("main"));
      return Some(ProgramNode::new(program));
    }
    None
  }
}
