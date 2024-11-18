use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ CallNode, CastNode, Node, NodeType, NodeWrapper, ParenthesisNode };
use utils::guard_none;

use crate::{ parser::{ LoopArgument, Parser }, utils::{ match_pattern, Lookup } };

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
  ) -> Option<Box<Node>> {
    if let [_] = matched.as_slice() {
      if let Some(le) = args.last_expr.clone() {
        if le.node_type == NodeType::Parenthesis {
          if let NodeWrapper::Parenthesis(le) = le.node {
            if
              [NodeType::AnonymousFunction, NodeType::ArrowFunction].contains(
                &le.statement.node_type
              )
            {
              return Some(
                CallNode::new(args.last_expr.to_owned().unwrap(), CallParser::get_arguments(parser))
              );
            }
          } else {
            return None;
          }
        } else if [NodeType::StaticLookup, NodeType::ObjectAccess].contains(&le.node_type) {
          return Some(
            CallNode::new(args.last_expr.to_owned().unwrap(), CallParser::get_arguments(parser))
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
      if statement.node_type != NodeType::Type {
        return Some(ParenthesisNode::new(statement));
      }
      let expression = guard_none!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("cast", &args.separators, &args.breakers)
        )
      );

      return Some(CastNode::new(statement, expression));
    }
    None
  }
}
