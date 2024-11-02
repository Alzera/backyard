use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ CatchNode, Node, TryNode },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, Lookup },
  },
};

use super::{ block::BlockParser, identifier::IdentifierParser, variable::VariableParser };

#[derive(Debug, Clone)]
pub struct TryParser {}

impl Internal for TryParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Try]),
        Lookup::Equal(vec![TokenType::LeftCurlyBracket]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let body = BlockParser::new(parser);
      let mut catches: Vec<Node> = vec![];
      let mut finally = None;
      loop {
        let is_finally = match guard!(parser.tokens.get(parser.position)).token_type {
          TokenType::Finally => true,
          TokenType::Catch => false,
          _ => {
            break;
          }
        };
        parser.position += 1;
        if is_finally {
          finally = Some(BlockParser::new(parser));
          break;
        }
        parser.position += 1;
        let types = parser.get_children(
          &mut LoopArgument::new(
            "catch_types",
            &[TokenType::BitwiseOr],
            &[TokenType::Variable, TokenType::VariableBracketOpen],
            &[ParserInternal::Identifier(IdentifierParser {})]
          )
        );
        parser.position -= 1;
        let variable = guard!(
          parser.get_statement(
            &mut LoopArgument::new(
              "catch_variable",
              &[],
              &[TokenType::RightParenthesis],
              &[ParserInternal::Variable(VariableParser {})]
            )
          )
        );
        parser.position += 1;
        catches.push(Box::new(CatchNode { types, variable, body: BlockParser::new(parser) }));
      }
      return Some(
        Box::new(TryNode {
          body,
          catches,
          finally,
        })
      );
    }
    None
  }
}
