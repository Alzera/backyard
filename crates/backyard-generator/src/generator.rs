use backyard_nodes::node::{ Node, NodeType };

use crate::internal::{ attribute::AttributeGenerator, comment::CommentGenerator };

pub type InternalGenerator = fn(&mut Generator, &mut Builder, &Box<Node>);

pub const DEFAULT_GENERATORS: [(NodeType, InternalGenerator); 79] = [
  (NodeType::AnonymousClass, super::internal::class::ClassGenerator::generate_anonymous),
  (NodeType::AnonymousFunction, super::internal::function::FunctionGenerator::generate_anonymous),
  // (NodeType::Argument, super::internal::call::CallGenerator::generate_argument),
  (NodeType::Array, super::internal::array::ArrayGenerator::generate),
  // (NodeType::ArrayItem, super::internal::array::ArrayGenerator::generate_item),
  (NodeType::ArrayLookup, super::internal::arraylookup::ArrayLookupGenerator::generate),
  (NodeType::ArrowFunction, super::internal::function::FunctionGenerator::generate_arrow),
  (NodeType::Assignment, super::internal::assignment::AssignmentGenerator::generate),
  (NodeType::Attribute, super::internal::attribute::AttributeGenerator::generate),
  (NodeType::Bin, super::internal::bin::BinGenerator::generate),
  (NodeType::Block, super::internal::block::BlockGenerator::generate_single),
  (NodeType::Boolean, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Break, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Call, super::internal::call::CallGenerator::generate),
  // (NodeType::Case, CaseGenerator::generate),
  (NodeType::Cast, super::internal::parenthesis::ParenthesisGenerator::generate_cast),
  // (NodeType::Catch, CatchGenerator::generate),
  (NodeType::Class, super::internal::class::ClassGenerator::generate),
  (NodeType::Clone, super::internal::singles::SinglesGenerator::generate),
  (NodeType::CommentBlock, super::internal::comment::CommentGenerator::generate_block),
  (NodeType::CommentDoc, super::internal::comment::CommentGenerator::generate_doc),
  (NodeType::CommentLine, super::internal::comment::CommentGenerator::generate),
  (NodeType::Const, super::internal::consts::ConstGenerator::generate),
  // (NodeType::ConstProperty, ConstPropertyGenerator::generate),
  (NodeType::Continue, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Declare, super::internal::declare::DeclareGenerator::generate),
  // (NodeType::DeclareArgument, DeclareArgumentGenerator::generate),
  (NodeType::DoWhile, super::internal::dowhile::DoWhileGenerator::generate),
  (NodeType::DoWhileCondition, super::internal::dowhile::DoWhileGenerator::generate_condition),
  (NodeType::Echo, super::internal::echo::EchoGenerator::generate),
  (NodeType::Else, super::internal::ifs::IfGenerator::generate_else),
  (NodeType::Encapsed, super::internal::string::StringGenerator::generate_encapsed),
  // (NodeType::EncapsedPart, StringGenerator::generate_encapsed_part),
  (NodeType::Enum, super::internal::enums::EnumGenerator::generate),
  // (NodeType::EnumItem, EnumItemGenerator::generate),
  (NodeType::Eval, super::internal::eval::EvalGenerator::generate),
  (NodeType::Exit, super::internal::exit::ExitGenerator::generate),
  (NodeType::For, super::internal::fors::ForGenerator::generate),
  (NodeType::Foreach, super::internal::foreach::ForeachGenerator::generate),
  (NodeType::Function, super::internal::function::FunctionGenerator::generate),
  (NodeType::Global, super::internal::globals::GlobalGenerator::generate),
  (NodeType::Goto, super::internal::singles::SinglesGenerator::generate),
  (NodeType::HereDoc, super::internal::string::StringGenerator::generate_heredoc),
  (NodeType::Identifier, super::internal::identifier::IdentifierGenerator::generate),
  (NodeType::If, super::internal::ifs::IfGenerator::generate),
  (NodeType::Include, super::internal::include::IncludeGenerator::generate),
  (NodeType::Inline, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Interface, super::internal::interface::InterfaceGenerator::generate),
  (NodeType::IntersectionType, super::internal::types::TypeGenerator::generate),
  (NodeType::Label, super::internal::label::LabelGenerator::generate),
  (NodeType::List, super::internal::list::ListGenerator::generate),
  (NodeType::Magic, super::internal::magic::MagicGenerator::generate),
  (NodeType::Match, super::internal::matchs::MatchGenerator::generate),
  // (NodeType::MatchArm, MatchArmGenerator::generate),
  // (NodeType::Method, super::internal::method::MethodGenerator::generate),
  (NodeType::Namespace, super::internal::namespace::NamespaceGenerator::generate),
  (NodeType::Negate, super::internal::pre::PreGenerator::generate),
  (NodeType::New, super::internal::singles::SinglesGenerator::generate),
  (NodeType::NowDoc, super::internal::string::StringGenerator::generate_nowdoc),
  (NodeType::Null, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Number, super::internal::number::NumberGenerator::generate),
  (NodeType::ObjectAccess, super::internal::objectaccess::ObjectAccessGenerator::generate),
  // (NodeType::Parameter, ParameterGenerator::generate),
  (NodeType::Parent, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Parenthesis, super::internal::parenthesis::ParenthesisGenerator::generate),
  (NodeType::Post, super::internal::post::PostGenerator::generate),
  (NodeType::Pre, super::internal::pre::PreGenerator::generate),
  (NodeType::Print, super::internal::singles::SinglesGenerator::generate),
  // (NodeType::Property, super::internal::property::PropertyGenerator::generate),
  // (NodeType::PropertyItem, PropertyItemGenerator::generate),
  (NodeType::Reference, super::internal::pre::PreGenerator::generate),
  (NodeType::Return, super::internal::singles::SinglesGenerator::generate),
  (NodeType::SelfKeyword, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Silent, super::internal::pre::PreGenerator::generate),
  (NodeType::Static, super::internal::statics::StaticGenerator::generate),
  (NodeType::StaticKeyword, super::internal::singles::SinglesGenerator::generate),
  (NodeType::StaticLookup, super::internal::staticlookup::StaticLookupGenerator::generate),
  (NodeType::String, super::internal::string::StringGenerator::generate),
  (NodeType::Switch, super::internal::switch::SwitchGenerator::generate),
  (NodeType::Ternary, super::internal::ternary::TernaryGenerator::generate),
  (NodeType::This, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Trait, super::internal::traits::TraitGenerator::generate),
  // (NodeType::TraitUse, super::internal::traituse::TraitUseGenerator::generate),
  // (NodeType::TraitUseAlias, TraitUseAliasGenerator::generate),
  // (NodeType::TraitUsePrecedence, TraitUsePrecedenceGenerator::generate),
  (NodeType::Throw, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Try, super::internal::tries::TryGenerator::generate),
  (NodeType::Type, super::internal::types::TypeGenerator::generate),
  (NodeType::UnionType, super::internal::types::TypeGenerator::generate),
  (NodeType::Use, super::internal::uses::UseGenerator::generate),
  (NodeType::Variable, super::internal::variable::VariableGenerator::generate),
  (NodeType::Variadic, super::internal::pre::PreGenerator::generate),
  (NodeType::While, super::internal::whiles::WhileGenerator::generate),
  (NodeType::Yield, super::internal::yields::YieldGenerator::generate),
  (NodeType::YieldFrom, super::internal::yields::YieldGenerator::generate_from),
];

#[derive(Debug, Clone)]
pub struct Line {
  pub line: String,
  pub indent: usize,
}

impl Default for Line {
  fn default() -> Self {
    Self::new()
  }
}

impl Line {
  pub fn new() -> Self {
    Self { line: String::new(), indent: 0 }
  }

  pub fn push(&mut self, line: &str) {
    self.line.push_str(line);
  }

  pub fn print(&self) -> String {
    let indent = Self::get_indent(self.indent);
    format!("{indent}{}", self.line)
  }

  fn get_indent(i: usize) -> String {
    let mut indent = String::new();
    for _ in 0..i {
      indent.push_str("  ");
    }
    indent
  }
}

#[derive(Debug, Clone)]
pub struct Builder {
  pub lines: Vec<Line>,
}

impl Default for Builder {
  fn default() -> Self {
    Self::new()
  }
}

impl Builder {
  pub fn new() -> Self {
    Self { lines: vec![] }
  }

  pub fn new_line(&mut self) {
    self.lines.push(Line::new());
  }

  pub fn push(&mut self, line: &str) {
    if let Some(last) = self.lines.last_mut() {
      last.push(line);
    }
  }

  pub fn total_len(&self) -> usize {
    self.lines
      .iter()
      .map(|i| i.line.len())
      .sum()
  }

  pub fn total_len_with_separator(&self, separator: &str) -> usize {
    let total_len = self.total_len();
    let total_separator_len = separator.len() * self.lines.len();
    total_len + total_separator_len
  }

  pub fn first_len(&self) -> usize {
    if let Some(line) = self.lines.first() { line.line.len() } else { 0 }
  }

  pub fn last_len(&self) -> usize {
    if let Some(line) = self.lines.last() { line.line.len() } else { 0 }
  }

  pub fn indent(&mut self) {
    self.lines.iter_mut().for_each(|i| {
      i.indent += 1;
    });
  }

  pub fn extend_first_line(&mut self, mut builder: Builder) {
    if builder.lines.is_empty() {
      return;
    }
    let first = builder.lines.remove(0);
    if let Some(last) = self.lines.last_mut() {
      last.push(&first.line);
    }
    self.lines.extend(builder.lines);
  }

  pub fn extend(&mut self, builder: Builder) {
    self.lines.extend(builder.lines);
  }

  pub fn print(&self, separator: &str) -> String {
    self.lines
      .iter()
      .filter_map(|x| {
        if x.line.is_empty() { None } else { Some(x.print()) }
      })
      .collect::<Vec<String>>()
      .join(separator)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EndMode {
  CommaWithoutEnd,
  SemicolonDynamic,
  None,
}

#[derive(Debug, Clone)]
pub struct GeneratorArgument<'a> {
  generators: &'a [(NodeType, InternalGenerator)],
  is_last: bool,
  end: EndMode,
}

impl<'a> GeneratorArgument<'a> {
  pub fn default() -> Self {
    Self { generators: &DEFAULT_GENERATORS, is_last: false, end: EndMode::None }
  }

  pub fn new(end: EndMode, generators: &'a [(NodeType, InternalGenerator)]) -> Self {
    Self { generators, is_last: false, end }
  }

  pub fn generator(generators: &'a [(NodeType, InternalGenerator)]) -> Self {
    Self { generators, is_last: false, end: EndMode::None }
  }

  pub fn for_parameter(generators: &'a [(NodeType, InternalGenerator)]) -> Self {
    Self { generators, is_last: false, end: EndMode::CommaWithoutEnd }
  }

  pub fn for_block() -> Self {
    Self {
      generators: &DEFAULT_GENERATORS,
      is_last: false,
      end: EndMode::SemicolonDynamic,
    }
  }

  fn get_end_statement(&mut self, node_type: &NodeType) -> Option<&str> {
    if self.end == EndMode::SemicolonDynamic {
      if
        ![
          NodeType::Function,
          NodeType::Program,
          NodeType::Class,
          NodeType::Interface,
          NodeType::Trait,
          NodeType::Enum,
          NodeType::Method,
          NodeType::TraitUse,
          NodeType::Declare,
          NodeType::Namespace,
          NodeType::If,
          NodeType::Switch,
          NodeType::Case,
          NodeType::Foreach,
          NodeType::For,
          NodeType::While,
          NodeType::DoWhile,
          NodeType::Try,
          NodeType::Label,
          NodeType::CommentBlock,
          NodeType::CommentDoc,
          NodeType::CommentLine,
          NodeType::Inline,
          NodeType::PropertyHook,
        ].contains(node_type)
      {
        return Some(";");
      }
    } else if self.end == EndMode::CommaWithoutEnd && !self.is_last {
      return Some(",");
    }
    None
  }
}

pub struct Generator<'a> {
  pub max_length: usize,
  nodes: &'a [Box<Node>],
}

impl<'a> Generator<'a> {
  pub fn new(nodes: &'a [Box<Node>]) -> Self {
    Self { nodes, max_length: 100 }
  }

  pub fn start(&mut self) -> String {
    let mut result = self
      .generate_nodes_new(self.nodes, &mut GeneratorArgument::for_block())
      .print("\n");
    if result.ends_with("<?php ") {
      result = result[..result.len() - 6].to_string();
    }
    if result.starts_with(" ?>") {
      result = result.replacen(" ?>", "", 1);
    }
    result
  }

  pub fn generate_nodes_new(
    &mut self,
    nodes: &[Box<Node>],
    args: &mut GeneratorArgument
  ) -> Builder {
    let mut builder = Builder::new();
    self.generate_nodes(&mut builder, nodes, args);
    builder
  }

  pub fn generate_nodes(
    &mut self,
    builder: &mut Builder,
    nodes: &[Box<Node>],
    args: &mut GeneratorArgument
  ) {
    for (i, node) in nodes.iter().enumerate() {
      args.is_last = i == nodes.len() - 1;
      if
        ![NodeType::Inline, NodeType::Program].contains(&node.node_type) ||
        builder.lines.is_empty()
      {
        builder.new_line();
      }
      self.generate_node(builder, node, args);
    }
  }

  pub fn generate_node_new(&mut self, node: &Box<Node>) -> Builder {
    let mut builder = Builder::new();
    builder.new_line();
    self.generate_node(&mut builder, node, &mut GeneratorArgument::default());
    builder
  }

  pub fn generate_node(
    &mut self,
    builder: &mut Builder,
    node: &Box<Node>,
    args: &mut GeneratorArgument
  ) {
    for (node_type, generator) in args.generators.iter() {
      if *node_type == node.node_type {
        let leadings = &node.leadings;
        let trailings = &node.trailings;
        if !leadings.is_empty() || !trailings.is_empty() {
          let mut scoped_builder = Builder::new();
          self.handle_comments(&mut scoped_builder, leadings);
          if scoped_builder.total_len() == 0 {
            scoped_builder.new_line();
          }
          generator(self, &mut scoped_builder, node);
          if let Some(end) = args.get_end_statement(&node.node_type) {
            scoped_builder.push(end);
          }
          self.handle_comments(&mut scoped_builder, trailings);
          if builder.last_len() == 0 {
            builder.extend_first_line(scoped_builder);
          } else {
            if ![NodeType::DoWhileCondition, NodeType::Else].contains(&node.node_type) {
              scoped_builder.indent();
            }
            builder.extend(scoped_builder);
          }
        } else {
          generator(self, builder, node);
          if node.node_type != NodeType::Inline {
            if let Some(end) = args.get_end_statement(&node.node_type) {
              builder.push(end);
            }
          }
        }
        return;
      }
    }
    println!("No generator for node: {:?}, {:?}", node.node_type, args.generators);
  }

  pub fn handle_comments(&mut self, builder: &mut Builder, nodes: &[Box<Node>]) {
    if !nodes.is_empty() {
      for node in nodes.iter() {
        match &node.node_type {
          NodeType::CommentBlock => {
            builder.new_line();
            CommentGenerator::generate_block(self, builder, node);
          }
          NodeType::CommentDoc => {
            builder.new_line();
            CommentGenerator::generate_doc(self, builder, node);
          }
          NodeType::CommentLine => {
            builder.new_line();
            CommentGenerator::generate(self, builder, node);
          }
          NodeType::Attribute => {
            builder.new_line();
            AttributeGenerator::generate(self, builder, node);
          }
          _ => {
            continue;
          }
        };
      }
      builder.new_line();
    }
  }

  pub fn check_nodes_has_comments(nodes: &[Box<Node>]) -> bool {
    nodes.iter().fold(false, |acc, i| (acc || !i.leadings.is_empty() || !i.trailings.is_empty()))
  }
}
