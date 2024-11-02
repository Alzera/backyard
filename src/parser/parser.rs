use std::fmt::Debug;

use crate::lexer::token::{ Token, TokenType };
use super::{
  internal::{
    array::ArrayParser,
    arraylookup::ArrayLookupParser,
    assignment::AssignmentParser,
    bin::BinParser,
    call::{ ArgumentParser, CallParser },
    class::ClassParser,
    consts::{ ConstParser, ConstPropertyParser },
    declare::{ DeclareArgumentParser, DeclareParser },
    dowhile::DoWhileParser,
    eval::EvalParser,
    exit::ExitParser,
    foreach::ForeachParser,
    fors::ForParser,
    function::{ FunctionParser, ParameterParser },
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
    matchs::{ MatchArmParser, MatchParser },
    method::MethodParser,
    namespace::NamespaceParser,
    number::NumberParser,
    objectaccess::ObjectAccessParser,
    parenthesis::ParenthesisParser,
    post::PostParser,
    pre::PreParser,
    property::{ PropertyItemParser, PropertyParser },
    singles::SinglesParser,
    staticlookup::StaticLookupParser,
    string::StringParser,
    switch::{ CaseParser, SwitchParser },
    ternary::TernaryParser,
    traits::TraitParser,
    traituse::{ TraitUseAliasParser, TraitUseParser, TraitUsePrecedenceParser },
    tries::TryParser,
    types::TypesParser,
    uses::UsesParser,
    variable::VariableParser,
    whiles::WhileParser,
    yields::YieldParser,
  },
  node::{ Node, NodeType, Nodes },
};

pub trait Internal: Debug {
  fn test(&self, tokens: &Vec<Token>, args: &LoopArgument) -> Option<Vec<Vec<Token>>>;
  fn parse(
    &self,
    parser: &mut Parser,
    matched: Vec<Vec<Token>>,
    args: &LoopArgument
  ) -> Option<Node>;
}

#[derive(Clone, Debug)]
pub enum ParserInternal {
  Array(ArrayParser),
  ArrayLookup(ArrayLookupParser),
  Argument(ArgumentParser),
  Assignment(AssignmentParser),
  Bin(BinParser),
  Call(CallParser),
  Case(CaseParser),
  Class(ClassParser),
  Const(ConstParser),
  ConstProperty(ConstPropertyParser),
  Declare(DeclareParser),
  DeclareArgument(DeclareArgumentParser),
  DoWhile(DoWhileParser),
  Eval(EvalParser),
  Exit(ExitParser),
  For(ForParser),
  Foreach(ForeachParser),
  Function(FunctionParser),
  Global(GlobalParser),
  Goto(GotoParser),
  Identifier(IdentifierParser),
  If(IfParser),
  Include(IncludeParser),
  InstanceOf(InstanceOfParser),
  Interface(InterfaceParser),
  Label(LabelParser),
  List(ListParser),
  Magic(MagicParser),
  Match(MatchParser),
  MatchArm(MatchArmParser),
  Method(MethodParser),
  Namespace(NamespaceParser),
  Number(NumberParser),
  ObjectAccess(ObjectAccessParser),
  Parameter(ParameterParser),
  Parenthesis(ParenthesisParser),
  Post(PostParser),
  Pre(PreParser),
  Property(PropertyParser),
  PropertyItem(PropertyItemParser),
  Singles(SinglesParser),
  StaticLookup(StaticLookupParser),
  String(StringParser),
  Switch(SwitchParser),
  Ternary(TernaryParser),
  Trait(TraitParser),
  TraitUse(TraitUseParser),
  TraitUseAlias(TraitUseAliasParser),
  TraitUsePrecedence(TraitUsePrecedenceParser),
  Try(TryParser),
  Type(TypesParser),
  Use(UsesParser),
  Variable(VariableParser),
  While(WhileParser),
  Yield(YieldParser),
}

impl ParserInternal {
  fn as_internal(&self) -> Box<&dyn Internal> {
    match self {
      ParserInternal::Array(x) => Box::new(x),
      ParserInternal::ArrayLookup(x) => Box::new(x),
      ParserInternal::Argument(x) => Box::new(x),
      ParserInternal::Assignment(x) => Box::new(x),
      ParserInternal::Bin(x) => Box::new(x),
      ParserInternal::Call(x) => Box::new(x),
      ParserInternal::Case(x) => Box::new(x),
      ParserInternal::Class(x) => Box::new(x),
      ParserInternal::Const(x) => Box::new(x),
      ParserInternal::ConstProperty(x) => Box::new(x),
      ParserInternal::Declare(x) => Box::new(x),
      ParserInternal::DeclareArgument(x) => Box::new(x),
      ParserInternal::DoWhile(x) => Box::new(x),
      ParserInternal::Eval(x) => Box::new(x),
      ParserInternal::Exit(x) => Box::new(x),
      ParserInternal::For(x) => Box::new(x),
      ParserInternal::Foreach(x) => Box::new(x),
      ParserInternal::Function(x) => Box::new(x),
      ParserInternal::Global(x) => Box::new(x),
      ParserInternal::Goto(x) => Box::new(x),
      ParserInternal::Identifier(x) => Box::new(x),
      ParserInternal::If(x) => Box::new(x),
      ParserInternal::Include(x) => Box::new(x),
      ParserInternal::InstanceOf(x) => Box::new(x),
      ParserInternal::Interface(x) => Box::new(x),
      ParserInternal::Label(x) => Box::new(x),
      ParserInternal::List(x) => Box::new(x),
      ParserInternal::Magic(x) => Box::new(x),
      ParserInternal::Match(x) => Box::new(x),
      ParserInternal::MatchArm(x) => Box::new(x),
      ParserInternal::Method(x) => Box::new(x),
      ParserInternal::Namespace(x) => Box::new(x),
      ParserInternal::Number(x) => Box::new(x),
      ParserInternal::ObjectAccess(x) => Box::new(x),
      ParserInternal::Parameter(x) => Box::new(x),
      ParserInternal::Parenthesis(x) => Box::new(x),
      ParserInternal::Post(x) => Box::new(x),
      ParserInternal::Pre(x) => Box::new(x),
      ParserInternal::Property(x) => Box::new(x),
      ParserInternal::PropertyItem(x) => Box::new(x),
      ParserInternal::Singles(x) => Box::new(x),
      ParserInternal::StaticLookup(x) => Box::new(x),
      ParserInternal::String(x) => Box::new(x),
      ParserInternal::Switch(x) => Box::new(x),
      ParserInternal::Ternary(x) => Box::new(x),
      ParserInternal::Trait(x) => Box::new(x),
      ParserInternal::TraitUse(x) => Box::new(x),
      ParserInternal::TraitUseAlias(x) => Box::new(x),
      ParserInternal::TraitUsePrecedence(x) => Box::new(x),
      ParserInternal::Try(x) => Box::new(x),
      ParserInternal::Type(x) => Box::new(x),
      ParserInternal::Use(x) => Box::new(x),
      ParserInternal::Variable(x) => Box::new(x),
      ParserInternal::While(x) => Box::new(x),
      ParserInternal::Yield(x) => Box::new(x),
    }
  }
}

pub static DEFAULT_PARSERS: [ParserInternal; 43] = [
  ParserInternal::Goto(GotoParser {}),
  ParserInternal::Label(LabelParser {}),
  ParserInternal::List(ListParser {}),
  ParserInternal::Parenthesis(ParenthesisParser {}),
  ParserInternal::ArrayLookup(ArrayLookupParser {}),
  ParserInternal::Array(ArrayParser {}),
  ParserInternal::Assignment(AssignmentParser {}),
  ParserInternal::Bin(BinParser {}),
  ParserInternal::ObjectAccess(ObjectAccessParser {}),
  ParserInternal::InstanceOf(InstanceOfParser {}),
  ParserInternal::Declare(DeclareParser {}),
  ParserInternal::DoWhile(DoWhileParser {}),
  ParserInternal::Namespace(NamespaceParser {}),
  ParserInternal::Include(IncludeParser {}),
  ParserInternal::Try(TryParser {}),
  ParserInternal::If(IfParser {}),
  ParserInternal::Match(MatchParser {}),
  ParserInternal::Use(UsesParser {}),
  ParserInternal::For(ForParser {}),
  ParserInternal::Foreach(ForeachParser {}),
  ParserInternal::Function(FunctionParser {}),
  ParserInternal::Global(GlobalParser {}),
  ParserInternal::Call(CallParser {}),
  ParserInternal::Class(ClassParser {}),
  ParserInternal::Interface(InterfaceParser {}),
  ParserInternal::Trait(TraitParser {}),
  ParserInternal::Const(ConstParser {}),
  ParserInternal::Eval(EvalParser {}),
  ParserInternal::Exit(ExitParser {}),
  ParserInternal::Identifier(IdentifierParser {}),
  ParserInternal::Magic(MagicParser {}),
  ParserInternal::Number(NumberParser {}),
  ParserInternal::Post(PostParser {}),
  ParserInternal::Pre(PreParser {}),
  ParserInternal::Singles(SinglesParser {}),
  ParserInternal::StaticLookup(StaticLookupParser {}),
  ParserInternal::Yield(YieldParser {}),
  ParserInternal::String(StringParser {}),
  ParserInternal::Switch(SwitchParser {}),
  ParserInternal::Variable(VariableParser {}),
  ParserInternal::Ternary(TernaryParser {}),
  ParserInternal::Type(TypesParser {}),
  ParserInternal::While(WhileParser {}),
];

#[derive(Debug)]
pub struct LoopArgument<'a> {
  #[allow(dead_code)]
  context: &'a str,
  pub parsers: &'a [ParserInternal],
  pub separators: &'a [TokenType],
  pub breakers: &'a [TokenType],
  pub last_expr: Option<Node>,
}

impl<'a> LoopArgument<'a> {
  pub fn new(
    context: &'a str,
    separators: &'a [TokenType],
    breakers: &'a [TokenType],
    parsers: &'a [ParserInternal]
  ) -> Self {
    LoopArgument {
      context,
      parsers,
      separators,
      breakers,
      last_expr: None,
    }
  }

  pub fn default(context: &'a str) -> Self {
    LoopArgument {
      context,
      parsers: &DEFAULT_PARSERS,
      separators: &[TokenType::Semicolon],
      breakers: &[TokenType::RightCurlyBracket],
      last_expr: None,
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
    let mut statements = vec![];

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
        statements.push(n);
      } else {
        if let Some(t) = self.tokens.get(self.position) {
          if !(args.separators.contains(&t.token_type) || args.breakers.contains(&t.token_type)) {
            println!("Fail to parse children: {:?}, {:?}", t, args);
          }
        }
        break;
      }
    }
    statements
  }

  pub fn get_statement(&mut self, args: &mut LoopArgument) -> Option<Node> {
    while let Some(token) = self.tokens.get(self.position) {
      // println!("get_statement while: {:?}", token.token_type);
      if args.separators.contains(&token.token_type) || args.breakers.contains(&token.token_type) {
        break;
      }
      if let Some(n) = self.find_match(&args) {
        let force_end_statement = [
          NodeType::Declare,
          NodeType::Namespace,
          NodeType::Function,
          NodeType::AnonymousFunction,
          NodeType::ArrowFunction,
          NodeType::Class,
          NodeType::Interface,
          NodeType::Trait,
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

  pub fn find_match(&mut self, args: &LoopArgument) -> Option<Node> {
    let tokens = self.tokens[self.position..].to_vec();

    for parser in args.parsers {
      let parser = parser.as_internal();
      // println!("parse_statement: {:?}", parser);
      if let Some(matched) = parser.test(&tokens, args) {
        self.position += matched
          .iter()
          .map(|x| x.len())
          .sum::<usize>();
        println!("parse_statement matched: {:?}", parser);
        return parser.parse(self, matched, args);
      }
    }
    None
  }
}
