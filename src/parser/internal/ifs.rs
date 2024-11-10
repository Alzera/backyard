use crate::{
  guard_none,
  lexer::token::{ Token, TokenType },
  parser::{
    node::Node,
    nodes::ifs::IfNode,
    parser::{ LoopArgument, Parser },
    utils::{ match_pattern, Lookup },
  },
};

use super::block::BlockParser;

#[derive(Debug, Clone)]
pub struct IfParser {}

impl IfParser {
  pub fn test(tokens: &Vec<Token>, _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [Lookup::Equal(vec![TokenType::If]), Lookup::Equal(vec![TokenType::LeftParenthesis])].to_vec()
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let condition = guard_none!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("if", &[], &[TokenType::RightParenthesis])
        )
      );
      parser.position += 1;

      let next_token = guard_none!(parser.tokens.get(parser.position));
      let mut is_short = false;
      let valid = match next_token.token_type {
        TokenType::Colon => {
          is_short = true;
          let parsed = BlockParser::new_short(
            parser,
            &[TokenType::ElseIf, TokenType::Else, TokenType::EndIf]
          );
          parser.position -= 1;
          parsed
        }
        TokenType::LeftCurlyBracket => BlockParser::new(parser),
        _ => {
          let parsed = guard_none!(
            parser.get_statement(
              &mut LoopArgument::with_tokens("if_body", &[], &[TokenType::Semicolon])
            )
          );
          parser.position += 1;
          parsed
        }
      };
      let mut invalid = None;
      if let Some(next_token) = parser.tokens.get(parser.position) {
        match next_token.token_type {
          TokenType::Else => {
            parser.position += 1;
            if let Some(next_token) = parser.tokens.get(parser.position) {
              match next_token.token_type {
                TokenType::LeftCurlyBracket | TokenType::Colon => {
                  if is_short {
                    invalid = Some(BlockParser::new_short(parser, &[TokenType::EndIf]));
                  } else {
                    invalid = Some(BlockParser::new(parser));
                  }
                }
                TokenType::If => {
                  invalid = parser.get_statement(&mut LoopArgument::default("if_invalid"));
                }
                _ => {
                  invalid = parser.get_statement(
                    &mut LoopArgument::with_tokens(
                      "if_else",
                      &[],
                      &[TokenType::Semicolon, TokenType::Else, TokenType::EndIf]
                    )
                  );
                }
              };
            }
          }
          TokenType::ElseIf => {
            parser.position += 2;
            invalid = IfParser::parse(parser, vec![vec![], vec![]], args);
          }
          _ => {}
        }
      }
      return Some(IfNode::new(condition, valid, invalid, is_short));
    }
    None
  }
}
