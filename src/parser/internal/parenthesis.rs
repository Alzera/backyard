use crate::{
  guard_none,
  lexer::token::{ Token, TokenType, TokenTypeArrayCombine },
  parser::{
    node::{ Node, NodeTraitCast, NodeType },
    nodes::{ call::CallNode, parenthesis::{ CastNode, ParenthesisNode } },
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::call::CallParser;

#[derive(Debug, Clone)]
pub struct ParenthesisParser {}

impl ParenthesisParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
        // Lookup::Optional(vec![TokenType::Function, TokenType::Fn]),
      ].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Option<Node> {
    if let [_] = matched.as_slice() {
      if let Some(le) = args.last_expr.clone() {
        if le.get_type() == NodeType::Parenthesis {
          let le = guard_none!(le.cast::<ParenthesisNode>());
          if
            [NodeType::AnonymousFunction, NodeType::ArrowFunction].contains(
              &le.statement.get_type()
            )
          {
            return Some(
              CallNode::boxed(args.last_expr.to_owned().unwrap(), CallParser::get_arguments(parser))
            );
          }
        } else if [NodeType::StaticLookup, NodeType::ObjectAccess].contains(&le.get_type()) {
          return Some(
            CallNode::boxed(args.last_expr.to_owned().unwrap(), CallParser::get_arguments(parser))
          );
        }
      }
      let statement = guard_none!(
        parser.get_statement(
          &mut LoopArgument::with_tokens(
            "parenthesis",
            &args.separators,
            &args.breakers.combine(&[TokenType::RightParenthesis])
          )
        )
      );
      parser.position += 1;
      if statement.get_type() != NodeType::Type {
        return Some(ParenthesisNode::boxed(statement));
      }
      let expression = guard_none!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("cast", &args.separators, &args.breakers)
        )
      );

      return Some(CastNode::boxed(statement, expression));
    }
    None
  }
}
