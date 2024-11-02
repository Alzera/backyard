use crate::{
  guard,
  lexer::token::{ Token, TokenType },
  parser::{
    node::{ BlockNode, CaseNode, Node, SwitchNode },
    parser::{ Internal, LoopArgument, Parser, ParserInternal },
    utils::{ match_pattern, Lookup },
  },
};

#[derive(Debug, Clone)]
pub struct SwitchParser {}

impl Internal for SwitchParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      [
        Lookup::Equal(vec![TokenType::Switch]),
        Lookup::Equal(vec![TokenType::LeftParenthesis]),
      ].to_vec()
    )
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [_, _] = matched.as_slice() {
      let condition = guard!(
        parser.get_statement(
          &mut LoopArgument::with_tokens("switch", &[], &[TokenType::RightParenthesis])
        )
      );
      parser.position += 1;
      let is_short =
        guard!(parser.tokens.get(parser.position)).token_type == TokenType::ShortFormStart;
      parser.position += 1;
      let statements = parser.get_children(
        &mut LoopArgument::new(
          "switch_body",
          &[],
          &[TokenType::RightCurlyBracket, TokenType::EndSwitch],
          &[ParserInternal::Case(CaseParser {})]
        )
      );
      return Some(
        Box::new(SwitchNode {
          condition,
          body: Box::new(BlockNode { statements }),
          is_short,
        })
      );
    }
    None
  }
}

#[derive(Debug, Clone)]
pub struct CaseParser {}

impl Internal for CaseParser {
  fn test(&self, tokens: &Vec<Token>, _: &LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(tokens, [Lookup::Equal(vec![TokenType::Case, TokenType::Default])].to_vec())
  }

  fn parse(&self, parser: &mut Parser, matched: Vec<Vec<Token>>, _: &LoopArgument) -> Option<Node> {
    if let [is_default] = matched.as_slice() {
      let condition = match guard!(is_default.get(0)).token_type {
        TokenType::Default => None,
        _ => {
          parser.get_statement(
            &mut LoopArgument::with_tokens("switch_case_condition", &[], &[TokenType::ShortForm])
          )
        }
      };
      parser.position += 1;
      let statements = parser.get_children(
        &mut LoopArgument::with_tokens(
          "switch_case_body",
          &[TokenType::Semicolon],
          &[TokenType::Case, TokenType::Default, TokenType::RightCurlyBracket, TokenType::EndSwitch]
        )
      );
      parser.position -= 1;
      return Some(
        Box::new(CaseNode {
          condition,
          body: Box::new(BlockNode { statements }),
        })
      );
    }
    None
  }
}
