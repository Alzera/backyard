use crate::{
  guard_none,
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ Node, NodeType },
    nodes::arraylookup::ArrayLookupNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct ArrayLookupParser {}

impl ArrayLookupParser {
  pub fn test(tokens: &Vec<Token>, args: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    let last_expr = guard_none!(&args.last_expr);
    if
      ![
        NodeType::Variable,
        NodeType::StaticLookup,
        NodeType::ArrayLookup,
        NodeType::Call,
        NodeType::Match,
        NodeType::Array,
        NodeType::ObjectAccess,
        // NodeType::Parenthesis
      ].contains(&last_expr.get_type())
    {
      return None;
    }
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::LeftSquareBracket])].to_vec())
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Option<Node> {
    if let [_] = matched.as_slice() {
      let on = guard_none!(args.last_expr.to_owned());
      let target = guard_none!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("arraylookup", &[], &[TokenType::RightSquareBracket])
        )
      );
      parser.position += 1;
      return Some(ArrayLookupNode::boxed(on, target));
    }
    None
  }
}
