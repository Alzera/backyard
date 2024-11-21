use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{ CallNode, CastNode, Node, NodeType, NodeWrapper, ParenthesisNode };
use utils::guard;

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

use super::{ call::CallParser, types::TypesParser };

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
  ) -> Result<Box<Node>, ParserError> {
    if let [_] = matched.as_slice() {
      if let Some(le) = args.last_expr.clone() {
        if le.node_type == NodeType::Parenthesis {
          if let NodeWrapper::Parenthesis(le) = le.node {
            if
              [NodeType::AnonymousFunction, NodeType::ArrowFunction].contains(
                &le.statement.node_type
              )
            {
              return Ok(
                CallNode::new(
                  args.last_expr.to_owned().unwrap(),
                  CallParser::get_arguments(parser)?
                )
              );
            }
          }
        } else if [NodeType::StaticLookup, NodeType::ObjectAccess].contains(&le.node_type) {
          return Ok(
            CallNode::new(args.last_expr.to_owned().unwrap(), CallParser::get_arguments(parser)?)
          );
        }
      }
      if let Some(m) = TypesParser::test(&parser.tokens[parser.position..].to_vec(), args) {
        parser.position += m
          .iter()
          .map(|x| x.len())
          .sum::<usize>();
        let statement = TypesParser::parse(parser, m, args)?;
        parser.position += 1;
        let expression = guard!(
          parser.get_statement(
            &mut LoopArgument::with_tokens("cast", &args.separators, &args.breakers)
          )?,
          {
            return Err(ParserError::internal("Parenthesis: fail to get expression", args));
          }
        );
        return Ok(CastNode::new(statement, expression));
      } else {
        let statement = guard!(
          parser.get_statement(
            &mut LoopArgument::with_tokens(
              "parenthesis",
              &args.separators,
              &args.breakers.combine(&[TokenType::RightParenthesis])
            )
          )?,
          {
            return Err(ParserError::internal("Parenthesis: fail to get statement", args));
          }
        );
        parser.position += 1;
        return Ok(ParenthesisNode::new(statement));
      }
    }
    Err(ParserError::internal("Parenthesis", args))
  }
}
