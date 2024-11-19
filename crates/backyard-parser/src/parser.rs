use std::fmt::Debug;

use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Node, NodeType };
use crate::{ error::ParserError, internal::elvis::ElvisParser };

use super::internal::{
  array::ArrayParser,
  arraylookup::ArrayLookupParser,
  assignment::AssignmentParser,
  bin::BinParser,
  call::CallParser,
  class::ClassParser,
  comment::CommentParser,
  consts::ConstParser,
  declare::DeclareParser,
  dowhile::DoWhileParser,
  enums::EnumParser,
  eval::EvalParser,
  exit::ExitParser,
  foreach::ForeachParser,
  fors::ForParser,
  function::FunctionParser,
  identifier::IdentifierParser,
  ifs::IfParser,
  include::IncludeParser,
  instanceof::InstanceOfParser,
  interface::InterfaceParser,
  label::LabelParser,
  list::ListParser,
  magic::MagicParser,
  matchs::MatchParser,
  namespace::NamespaceParser,
  number::NumberParser,
  objectaccess::ObjectAccessParser,
  parenthesis::ParenthesisParser,
  post::PostParser,
  pre::PreParser,
  program::ProgramParser,
  singles::SinglesParser,
  staticlookup::StaticLookupParser,
  string::StringParser,
  switch::SwitchParser,
  ternary::TernaryParser,
  traits::TraitParser,
  tries::TryParser,
  types::TypesParser,
  uses::UsesParser,
  variable::VariableParser,
  whiles::WhileParser,
  yields::YieldParser,
};

type InternalParserTest = fn(&Vec<Token>, &mut LoopArgument) -> Option<Vec<Vec<Token>>>;
type InternalParserParse = fn(
  &mut Parser,
  Vec<Vec<Token>>,
  &mut LoopArgument
) -> Result<Box<Node>, ParserError>;
type InternalParser = (InternalParserTest, InternalParserParse);
pub static DEFAULT_PARSERS: [InternalParser; 45] = [
  (CommentParser::test, CommentParser::parse),
  (ListParser::test, ListParser::parse),
  (ParenthesisParser::test, ParenthesisParser::parse),
  (ArrayLookupParser::test, ArrayLookupParser::parse),
  (ArrayParser::test, ArrayParser::parse),
  (AssignmentParser::test, AssignmentParser::parse),
  (BinParser::test, BinParser::parse),
  (ObjectAccessParser::test, ObjectAccessParser::parse),
  (InstanceOfParser::test, InstanceOfParser::parse),
  (DeclareParser::test, DeclareParser::parse),
  (DoWhileParser::test, DoWhileParser::parse),
  (NamespaceParser::test, NamespaceParser::parse),
  (IncludeParser::test, IncludeParser::parse),
  (TryParser::test, TryParser::parse),
  (IfParser::test, IfParser::parse),
  (MatchParser::test, MatchParser::parse),
  (UsesParser::test, UsesParser::parse),
  (ForParser::test, ForParser::parse),
  (ForeachParser::test, ForeachParser::parse),
  (FunctionParser::test, FunctionParser::parse),
  (CallParser::test, CallParser::parse),
  (ClassParser::test, ClassParser::parse),
  (InterfaceParser::test, InterfaceParser::parse),
  (TraitParser::test, TraitParser::parse),
  (EnumParser::test, EnumParser::parse),
  (ConstParser::test, ConstParser::parse),
  (EvalParser::test, EvalParser::parse),
  (ExitParser::test, ExitParser::parse),
  (MagicParser::test, MagicParser::parse),
  (NumberParser::test, NumberParser::parse),
  (PostParser::test, PostParser::parse),
  (PreParser::test, PreParser::parse),
  (ProgramParser::test, ProgramParser::parse),
  (SinglesParser::test, SinglesParser::parse),
  (StaticLookupParser::test, StaticLookupParser::parse),
  (YieldParser::test, YieldParser::parse),
  (StringParser::test, StringParser::parse),
  (SwitchParser::test, SwitchParser::parse),
  (VariableParser::test, VariableParser::parse),
  (TernaryParser::test, TernaryParser::parse),
  (WhileParser::test, WhileParser::parse),
  (LabelParser::test, LabelParser::parse),
  (IdentifierParser::test, IdentifierParser::parse),
  (TypesParser::test, TypesParser::parse),
  (ElvisParser::test, ElvisParser::parse),
];

#[derive(Debug)]
pub struct LoopArgument<'a> {
  #[allow(dead_code)]
  context: &'a str,
  pub parsers: &'a [InternalParser],
  pub separators: &'a [TokenType],
  pub breakers: &'a [TokenType],
  pub last_expr: Option<Box<Node>>,
  pub statements: Vec<Box<Node>>,
}

impl<'a> LoopArgument<'a> {
  pub fn new(
    context: &'a str,
    separators: &'a [TokenType],
    breakers: &'a [TokenType],
    parsers: &'a [InternalParser]
  ) -> Self {
    LoopArgument {
      context,
      parsers,
      separators,
      breakers,
      last_expr: None,
      statements: vec![],
    }
  }

  pub fn default(context: &'a str) -> Self {
    LoopArgument {
      context,
      parsers: &DEFAULT_PARSERS,
      separators: &[TokenType::Semicolon],
      breakers: &[TokenType::RightCurlyBracket],
      last_expr: None,
      statements: vec![],
    }
  }

  pub fn with_tokens(
    context: &'a str,
    separators: &'a [TokenType],
    breakers: &'a [TokenType]
  ) -> Self {
    LoopArgument {
      context,
      parsers: &DEFAULT_PARSERS,
      separators,
      breakers,
      last_expr: None,
      statements: vec![],
    }
  }

  pub fn to_string(&self) -> String {
    let last_statement = if let Some(last) = self.statements.last() {
      Some(&last.node_type)
    } else {
      None
    };
    let last_expr = if let Some(last) = &self.last_expr { Some(&last.node_type) } else { None };
    format!(
      "LoopArgument {{ context: {}, separators: {:?}, breakers: {:?}, last_expr: {:?}, last_statements: {:?} }}",
      self.context,
      self.separators,
      self.breakers,
      last_expr,
      last_statement
    )
  }
}

pub struct Parser {
  pub tokens: Vec<Token>,
  pub position: usize,
}

impl Parser {
  pub fn new(tokens: &Vec<Token>) -> Self {
    Parser {
      tokens: tokens.to_vec(),
      position: 0,
    }
  }

  pub fn get_children(&mut self, args: &mut LoopArgument) -> Result<Vec<Box<Node>>, ParserError> {
    while let Some(token) = self.tokens.get(self.position) {
      if args.breakers.contains(&token.token_type) {
        self.position += 1;
        break;
      }
      if args.separators.contains(&token.token_type) {
        self.position += 1;
        continue;
      }
      let statement = self.get_statement(args);
      if statement.is_err() {
        return Err(statement.err().unwrap());
      }
      if let Some(statement) = statement.unwrap() {
        args.statements.push(statement);
      } else {
        break;
      }
    }
    Ok(args.statements.to_owned())
  }

  pub fn get_statement(
    &mut self,
    args: &mut LoopArgument
  ) -> Result<Option<Box<Node>>, ParserError> {
    while let Some(token) = self.tokens.get(self.position) {
      if args.separators.contains(&token.token_type) || args.breakers.contains(&token.token_type) {
        break;
      }
      match self.find_match(args) {
        Ok(n) => {
          let force_end_statement = [
            NodeType::Declare,
            NodeType::Namespace,
            NodeType::Function,
            NodeType::AnonymousFunction,
            NodeType::ArrowFunction,
            NodeType::Class,
            NodeType::Interface,
            NodeType::Trait,
            NodeType::Enum,
            NodeType::Property,
            NodeType::Method,
            NodeType::TraitUse,
            NodeType::Foreach,
            NodeType::For,
            NodeType::While,
            NodeType::DoWhile,
            NodeType::Match,
            NodeType::MatchArm,
            NodeType::If,
            NodeType::Switch,
            NodeType::Case,
            NodeType::Label,
            NodeType::Try,
            NodeType::CommentLine,
            NodeType::CommentBlock,
          ].contains(&n.node_type);
          args.last_expr = Some(n);
          if force_end_statement {
            break;
          }
        }
        Err(e) => {
          args.last_expr = None;
          return Err(e);
        }
      }
    }

    let last_expr = args.last_expr.to_owned();
    args.last_expr = None;
    Ok(last_expr.to_owned())
  }

  pub fn find_match(&mut self, args: &mut LoopArgument) -> Result<Box<Node>, ParserError> {
    let tokens = self.tokens[self.position..].to_vec();

    for (test, parse) in args.parsers {
      if let Some(matched) = test(&tokens, args) {
        self.position += matched
          .iter()
          .map(|x| x.len())
          .sum::<usize>();
        return parse(self, matched, args);
      }
    }
    Err(
      ParserError::Failed(
        format!(
          "Failed to find match: {:?}, {:?}",
          args.to_string(),
          tokens.iter().take(3).collect::<Vec<&Token>>()
        )
      )
    )
  }
}
