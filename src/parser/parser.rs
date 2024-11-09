use std::fmt::Debug;

use crate::lexer::token::{ Token, TokenType };
use super::{
  internal::{
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
    global::GlobalParser,
    goto::GotoParser,
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
  },
  node::{ Node, NodeType, Nodes },
};

type InternalParserTest = fn(&Vec<Token>, &mut LoopArgument) -> Option<Vec<Vec<Token>>>;
type InternalParserParse = fn(&mut Parser, Vec<Vec<Token>>, &mut LoopArgument) -> Option<Node>;
type InternalParser = (InternalParserTest, InternalParserParse);
pub static DEFAULT_PARSERS: [InternalParser; 46] = [
  (CommentParser::test, CommentParser::parse),
  (GotoParser::test, GotoParser::parse),
  (LabelParser::test, LabelParser::parse),
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
  (GlobalParser::test, GlobalParser::parse),
  (CallParser::test, CallParser::parse),
  (ClassParser::test, ClassParser::parse),
  (InterfaceParser::test, InterfaceParser::parse),
  (TraitParser::test, TraitParser::parse),
  (EnumParser::test, EnumParser::parse),
  (ConstParser::test, ConstParser::parse),
  (EvalParser::test, EvalParser::parse),
  (ExitParser::test, ExitParser::parse),
  (IdentifierParser::test, IdentifierParser::parse),
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
  (TypesParser::test, TypesParser::parse),
  (WhileParser::test, WhileParser::parse),
];

#[derive(Debug)]
pub struct LoopArgument<'a> {
  #[allow(dead_code)]
  context: &'a str,
  pub parsers: &'a [InternalParser],
  pub separators: &'a [TokenType],
  pub breakers: &'a [TokenType],
  pub last_expr: Option<Node>,
  pub statements: Nodes,
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
}

pub struct Parser {
  pub tokens: Vec<Token>,
  // pub length: usize,
  pub position: usize,
  // parsers: Vec<ParserInternal>,
}

impl Parser {
  pub fn new(tokens: &Vec<Token>) -> Self {
    Parser {
      tokens: tokens.to_vec(),
      // length: tokens.len(),
      position: 0,
      // parsers: ParserInternal::parsers(),
    }
  }

  pub fn get_children(&mut self, args: &mut LoopArgument) -> Nodes {
    while let Some(token) = self.tokens.get(self.position) {
      if args.breakers.contains(&token.token_type) {
        self.position += 1;
        break;
      }
      if args.separators.contains(&token.token_type) {
        self.position += 1;
        continue;
      }
      if let Some(n) = self.get_statement(args) {
        args.statements.push(n);
      } else {
        if let Some(t) = self.tokens.get(self.position) {
          if !(args.separators.contains(&t.token_type) || args.breakers.contains(&t.token_type)) {
            println!("Fail to parse children: {:?}, {:?}", t, args);
          }
        }
        break;
      }
    }
    args.statements.clone()
  }

  pub fn get_statement(&mut self, args: &mut LoopArgument) -> Option<Node> {
    while let Some(token) = self.tokens.get(self.position) {
      // println!("get_statement while: {:?}", token.token_type);
      if args.separators.contains(&token.token_type) || args.breakers.contains(&token.token_type) {
        break;
      }
      if let Some(n) = self.find_match(args) {
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
        ].contains(&n.get_type());
        // println!(
        //   "get_statement force_end_statement: {:?}, {:?}",
        //   force_end_statement,
        //   n.get_type()
        // );
        args.last_expr = Some(n);
        if force_end_statement {
          break;
        }
      } else {
        if let Some(t) = self.tokens.get(self.position) {
          if !(args.separators.contains(&t.token_type) || args.breakers.contains(&t.token_type)) {
            println!("Fail to parse statement: {:?}, {:?}", t, args);
          }
        }
        break;
      }
    }

    let last_expr = args.last_expr.to_owned();
    args.last_expr = None;
    last_expr
  }

  pub fn find_match(&mut self, args: &mut LoopArgument) -> Option<Node> {
    let tokens = self.tokens[self.position..].to_vec();

    for (test, parse) in args.parsers {
      // println!("parse_statement: {:?}", parser);
      if let Some(matched) = test(&tokens, args) {
        self.position += matched
          .iter()
          .map(|x| x.len())
          .sum::<usize>();
        // println!("parse_statement matched: {:?}", parser);
        return parse(self, matched, args);
      }
    }
    None
  }
}
