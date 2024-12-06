use std::{ collections::HashSet, fmt::Debug };

use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::node::{ Location, Node, NodeType, RangeLocation };
use crate::{
  error::ParserError,
  internal::{
    attribute::AttributeParser,
    block::BlockParser,
    echo::EchoParser,
    globals::GlobalParser,
    statics::StaticsParser,
  },
  utils::LookupResult,
};

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
  singles::SinglesParser,
  staticlookup::StaticLookupParser,
  string::StringParser,
  switch::SwitchParser,
  ternary::TernaryParser,
  traits::TraitParser,
  tries::TryParser,
  uses::UseParser,
  variable::VariableParser,
  whiles::WhileParser,
  yields::YieldParser,
};

type InternalParserTest = fn(&[Token], &mut LoopArgument) -> Option<Vec<LookupResult>>;
type InternalParserParse = fn(
  &mut Parser,
  Vec<LookupResult>,
  Location,
  &mut LoopArgument
) -> Result<Box<Node>, ParserError>;
type InternalParser = (InternalParserTest, InternalParserParse);
pub static DEFAULT_PARSERS: [InternalParser; 46] = [
  (CommentParser::test, CommentParser::parse),
  (ListParser::test, ListParser::parse),
  (ArrayLookupParser::test, ArrayLookupParser::parse),
  (ArrayParser::test, ArrayParser::parse),
  (AssignmentParser::test, AssignmentParser::parse),
  (BinParser::test, BinParser::parse),
  (ObjectAccessParser::test, ObjectAccessParser::parse),
  (DeclareParser::test, DeclareParser::parse),
  (DoWhileParser::test, DoWhileParser::parse),
  (NamespaceParser::test, NamespaceParser::parse),
  (IncludeParser::test, IncludeParser::parse),
  (TryParser::test, TryParser::parse),
  (IfParser::test, IfParser::parse),
  (MatchParser::test, MatchParser::parse),
  (UseParser::test, UseParser::parse),
  (ForParser::test, ForParser::parse),
  (ForeachParser::test, ForeachParser::parse),
  (FunctionParser::test, FunctionParser::parse),
  (CallParser::test, CallParser::parse),
  (ParenthesisParser::test, ParenthesisParser::parse),
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
  (StaticLookupParser::test, StaticLookupParser::parse),
  (YieldParser::test, YieldParser::parse),
  (StringParser::test, StringParser::parse),
  (SwitchParser::test, SwitchParser::parse),
  (StaticsParser::test, StaticsParser::parse),
  (GlobalParser::test, GlobalParser::parse),
  (VariableParser::test, VariableParser::parse),
  (TernaryParser::test, TernaryParser::parse),
  (WhileParser::test, WhileParser::parse),
  (LabelParser::test, LabelParser::parse),
  (IdentifierParser::test, IdentifierParser::parse),
  (SinglesParser::test, SinglesParser::parse),
  (EchoParser::test, EchoParser::parse),
  (AttributeParser::test, AttributeParser::parse),
  (BlockParser::test, BlockParser::parse),
];

#[derive(Debug, Clone)]
pub struct LoopArgument<'a> {
  #[allow(dead_code)]
  pub context: &'a str,
  pub parsers: &'a [InternalParser],
  pub separators: &'a [TokenType],
  pub breakers: &'a [TokenType],
  pub last_expr: Option<Box<Node>>,
  pub statements: Vec<Box<Node>>,
  pub should_fail: bool,
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
      should_fail: true,
    }
  }

  pub fn safe(
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
      should_fail: false,
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
      should_fail: true,
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
      should_fail: true,
    }
  }
}

pub struct Parser<'a> {
  pub tokens: &'a [Token],
  pub position: usize,
}

impl<'a> Parser<'a> {
  pub fn new(tokens: &'a [Token]) -> Self {
    Parser {
      tokens,
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
      // println!("");
      // println!("context: {:?}", args.context);
      // println!("token: {:?}", token);
      // println!("last_expr: {:?}", args.last_expr);

      if
        args.separators.contains(&token.token_type) ||
        args.breakers.contains(&token.token_type) ||
        (token.token_type == TokenType::Inline && args.last_expr.is_some())
      {
        break;
      }
      match self.find_match(args) {
        Ok(n) => {
          if n.is_none() {
            break;
          }
          let n = n.unwrap();
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
            NodeType::Catch,
            NodeType::Finally,
            NodeType::CommentLine,
            NodeType::CommentBlock,
            NodeType::Inline,
            NodeType::Program,
            NodeType::PropertyHook,
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

  pub fn find_match(&mut self, args: &mut LoopArgument) -> Result<Option<Box<Node>>, ParserError> {
    let tokens = &self.tokens[self.position..];

    for (test, parse) in args.parsers {
      if let Some(matched) = test(tokens, args) {
        if let Some(start_loc) = tokens.first().map(|x| x.get_location().unwrap()) {
          self.position += matched
            .iter()
            .map(|x| x.size)
            .sum::<usize>();
          if let Ok(parsed) = parse(self, matched, start_loc, args) {
            return Ok(Some(parsed));
          } else {
            break;
          }
        } else {
          return Err(ParserError::Eof);
        }
      }
    }
    if args.should_fail {
      if let Some(token) = self.tokens.get(self.position) {
        Err(ParserError::UnexpectedToken(token.to_owned()))
      } else {
        Err(ParserError::Eof)
      }
    } else {
      Ok(None)
    }
  }

  pub fn gen_loc(&self, start: Location) -> Option<RangeLocation> {
    let end = self.tokens.get(self.position - 1);
    end.and_then(|x| x.get_location()).map(|end| RangeLocation { start, end })
  }

  pub fn gen_loc_helper<T>(&self, start: T) -> Option<RangeLocation> where T: LocationHelper {
    let start = start.get_location();
    if let Some(start) = start {
      self.gen_loc(start)
    } else {
      None
    }
  }
}

pub trait LocationHelper {
  fn get_location(&self) -> Option<Location>;
  fn get_range_location(&self) -> Option<RangeLocation>;
}

impl LocationHelper for &Node {
  fn get_location(&self) -> Option<Location> {
    self.loc.as_ref().map(|loc| loc.start.clone())
  }

  fn get_range_location(&self) -> Option<RangeLocation> {
    self.loc.clone()
  }
}

impl LocationHelper for &Token {
  fn get_location(&self) -> Option<Location> {
    Some(Location { line: self.line, column: self.column, offset: self.offset })
  }

  fn get_range_location(&self) -> Option<RangeLocation> {
    if let Some(start_loc) = self.get_location() {
      let len = self.value.len();
      let end_loc = Location {
        line: start_loc.line,
        column: start_loc.column + len,
        offset: start_loc.offset + len,
      };
      Some(RangeLocation {
        start: start_loc,
        end: end_loc,
      })
    } else {
      None
    }
  }
}

pub trait TokenTypeArrayCombine {
  fn combine(self, tokens: &[TokenType]) -> Vec<TokenType>;
}

impl TokenTypeArrayCombine for &[TokenType] {
  fn combine(self, tokens: &[TokenType]) -> Vec<TokenType> {
    let combined: Vec<TokenType> = [self, tokens].concat();
    let unique: HashSet<_> = combined.into_iter().collect();
    unique.into_iter().collect()
  }
}
