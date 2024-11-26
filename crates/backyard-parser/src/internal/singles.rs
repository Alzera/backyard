use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{
  BooleanNode,
  BreakNode,
  CloneNode,
  ContinueNode,
  GlobalNode,
  GotoNode,
  InlineNode,
  NewNode,
  Node,
  NullNode,
  ParentNode,
  PrintNode,
  ReturnNode,
  SelfNode,
  StaticKeywordNode,
  ThisNode,
  ThrowNode,
};

use crate::{
  error::ParserError,
  parser::{ LoopArgument, Parser },
  utils::{ match_pattern, Lookup },
};

#[derive(Debug, Clone)]
pub struct SinglesParser;

impl SinglesParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<Vec<Token>>> {
    match_pattern(
      tokens,
      &[
        Lookup::Equal(
          &[
            TokenType::Break,
            TokenType::Continue,
            TokenType::Goto,
            TokenType::New,
            TokenType::Print,
            TokenType::Return,
            TokenType::Throw,
            TokenType::Parent,
            TokenType::Static,
            TokenType::Clone,
            TokenType::Global,
            TokenType::This,
            TokenType::True,
            TokenType::False,
            TokenType::Null,
            TokenType::SelfKeyword,
            TokenType::Inline,
          ]
        ),
      ]
    )
  }

  pub fn parse(
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [key] = matched.as_slice() {
      if let Some(key) = key.first() {
        if
          [
            TokenType::Parent,
            TokenType::Static,
            TokenType::This,
            TokenType::SelfKeyword,
            TokenType::True,
            TokenType::False,
            TokenType::Null,
            TokenType::Inline,
          ].contains(&key.token_type)
        {
          return match key.token_type {
            TokenType::Parent => Ok(ParentNode::new()),
            TokenType::Static => Ok(StaticKeywordNode::new()),
            TokenType::This => Ok(ThisNode::new()),
            TokenType::SelfKeyword => Ok(SelfNode::new()),
            TokenType::True => Ok(BooleanNode::new(true)),
            TokenType::False => Ok(BooleanNode::new(false)),
            TokenType::Null => Ok(NullNode::new()),
            TokenType::Inline => Ok(InlineNode::new(key.value.to_owned())),
            _ => Err(ParserError::internal("Single: first group", args)),
          };
        }
        let argument = parser.get_statement(
          &mut LoopArgument::with_tokens(
            "singles",
            &args.separators.combine(&[TokenType::Semicolon]),
            &args.breakers.combine(&[TokenType::RightCurlyBracket])
          )
        )?;
        match key.token_type {
          TokenType::Break => {
            return Ok(BreakNode::new(argument.to_owned()));
          }
          TokenType::Continue => {
            return Ok(ContinueNode::new(argument.to_owned()));
          }
          TokenType::Return => {
            return Ok(ReturnNode::new(argument.to_owned()));
          }
          _ => {}
        }
        if argument.is_none() {
          return Err(ParserError::internal("Single: second group", args));
        }
        let argument = argument.unwrap();
        return match key.token_type {
          TokenType::New => Ok(NewNode::new(argument)),
          TokenType::Print => Ok(PrintNode::new(argument)),
          TokenType::Throw => Ok(ThrowNode::new(argument)),
          TokenType::Clone => Ok(CloneNode::new(argument)),
          TokenType::Global => Ok(GlobalNode::new(argument)),
          TokenType::Goto => Ok(GotoNode::new(argument)),
          _ => Err(ParserError::internal("Single: third group", args)),
        };
      }
    }
    Err(ParserError::internal("Single", args))
  }
}
