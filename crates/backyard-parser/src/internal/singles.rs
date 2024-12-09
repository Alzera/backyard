use backyard_lexer::token::{ Token, TokenType };
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
  parser::{ LoopArgument, Parser, TokenTypeArrayCombine },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct SinglesParser;

impl SinglesParser {
  pub fn test(tokens: &[Token], _: &mut LoopArgument) -> Option<Vec<LookupResult>> {
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
    matched: Vec<LookupResult>,
    start_loc: Location,
    args: &mut LoopArgument
  ) -> Result<Box<Node>, ParserError> {
    if let [key] = matched.as_slice() {
      let key = key.as_equal()?;
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
          TokenType::Parent => Ok(ParentNode::loc(parser.gen_loc(start_loc))),
          TokenType::Static => Ok(StaticKeywordNode::loc(parser.gen_loc(start_loc))),
          TokenType::This => Ok(ThisNode::loc(parser.gen_loc(start_loc))),
          TokenType::SelfKeyword => Ok(SelfNode::loc(parser.gen_loc(start_loc))),
          TokenType::True => Ok(BooleanNode::loc(true, parser.gen_loc(start_loc))),
          TokenType::False => Ok(BooleanNode::loc(false, parser.gen_loc(start_loc))),
          TokenType::Null => Ok(NullNode::loc(parser.gen_loc(start_loc))),
          TokenType::Inline => Ok(InlineNode::loc(key.value.to_owned(), parser.gen_loc(start_loc))),
          _ => Err(ParserError::Internal),
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
          return Ok(BreakNode::loc(argument, parser.gen_loc(start_loc)));
        }
        TokenType::Continue => {
          return Ok(ContinueNode::loc(argument, parser.gen_loc(start_loc)));
        }
        TokenType::Return => {
          return Ok(ReturnNode::loc(argument, parser.gen_loc(start_loc)));
        }
        _ => {}
      }
      if argument.is_none() {
        return Err(ParserError::Internal);
      }
      let argument = argument.unwrap();
      return match key.token_type {
        TokenType::New => Ok(NewNode::loc(argument, parser.gen_loc(start_loc))),
        TokenType::Print => Ok(PrintNode::loc(argument, parser.gen_loc(start_loc))),
        TokenType::Throw => Ok(ThrowNode::loc(argument, parser.gen_loc(start_loc))),
        TokenType::Clone => Ok(CloneNode::loc(argument, parser.gen_loc(start_loc))),
        TokenType::Goto => Ok(GotoNode::loc(argument, parser.gen_loc(start_loc))),
        _ => Err(ParserError::Internal),
      };
    }
    Err(ParserError::Internal)
  }
}
