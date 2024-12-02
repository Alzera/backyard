use backyard_lexer::token::{ Token, TokenType, TokenTypeArrayCombine };
use backyard_nodes::node::{
  BooleanNode,
  BreakNode,
  CloneNode,
  ContinueNode,
  GotoNode,
  InlineNode,
  NewNode,
  Location,
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
    start_loc: Location,
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
            TokenType::Parent => Ok(ParentNode::new(parser.gen_loc(start_loc))),
            TokenType::Static => Ok(StaticKeywordNode::new(parser.gen_loc(start_loc))),
            TokenType::This => Ok(ThisNode::new(parser.gen_loc(start_loc))),
            TokenType::SelfKeyword => Ok(SelfNode::new(parser.gen_loc(start_loc))),
            TokenType::True => Ok(BooleanNode::new(true, parser.gen_loc(start_loc))),
            TokenType::False => Ok(BooleanNode::new(false, parser.gen_loc(start_loc))),
            TokenType::Null => Ok(NullNode::new(parser.gen_loc(start_loc))),
            TokenType::Inline =>
              Ok(InlineNode::new(key.value.to_owned(), parser.gen_loc(start_loc))),
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
            return Ok(BreakNode::new(argument.to_owned(), parser.gen_loc(start_loc)));
          }
          TokenType::Continue => {
            return Ok(ContinueNode::new(argument.to_owned(), parser.gen_loc(start_loc)));
          }
          TokenType::Return => {
            return Ok(ReturnNode::new(argument.to_owned(), parser.gen_loc(start_loc)));
          }
          _ => {}
        }
        if argument.is_none() {
          return Err(ParserError::internal("Single: second group", args));
        }
        let argument = argument.unwrap();
        return match key.token_type {
          TokenType::New => Ok(NewNode::new(argument, parser.gen_loc(start_loc))),
          TokenType::Print => Ok(PrintNode::new(argument, parser.gen_loc(start_loc))),
          TokenType::Throw => Ok(ThrowNode::new(argument, parser.gen_loc(start_loc))),
          TokenType::Clone => Ok(CloneNode::new(argument, parser.gen_loc(start_loc))),
          TokenType::Goto => Ok(GotoNode::new(argument, parser.gen_loc(start_loc))),
          _ => Err(ParserError::internal("Single: third group", args)),
        };
      }
    }
    Err(ParserError::internal("Single", args))
  }
}
