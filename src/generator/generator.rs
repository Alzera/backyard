use crate::{
  guard,
  parser::{
    node::{ Node, NodeTraitCast, NodeType, Nodes },
    nodes::comment::{ CommentBlockNode, CommentLineNode },
  },
};

pub type InternalGenerator = fn(&mut Generator, &mut Builder, &Node);

pub const DEFAULT_GENERATORS: [(NodeType, InternalGenerator); 45] = [
  (NodeType::AnonymousFunction, super::internal::function::FunctionGenerator::generate_anonymous),
  // (NodeType::Argument, super::internal::call::CallGenerator::generate_argument),
  (NodeType::Array, super::internal::array::ArrayGenerator::generate),
  // (NodeType::ArrayItem, super::internal::array::ArrayGenerator::generate_item),
  (NodeType::ArrayLookup, super::internal::arraylookup::ArrayLookupGenerator::generate),
  (NodeType::ArrowFunction, super::internal::function::FunctionGenerator::generate_arrow),
  (NodeType::Assignment, super::internal::assignment::AssignmentGenerator::generate),
  (NodeType::Bin, super::internal::bin::BinGenerator::generate),
  // (NodeType::Block, super::internal::block::BlockGenerator::generate),
  (NodeType::Break, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Call, super::internal::call::CallGenerator::generate),
  // (NodeType::Case, CaseGenerator::generate),
  // (NodeType::Cast, super::internal::parenthesis::ParenthesisGenerator::generate_cast),
  // (NodeType::Catch, CatchGenerator::generate),
  (NodeType::Class, super::internal::class::ClassGenerator::generate),
  (NodeType::Clone, super::internal::singles::SinglesGenerator::generate),
  // (NodeType::CommentBlock, CommentBlockGenerator::generate),
  // (NodeType::CommentLine, CommentLineGenerator::generate),
  (NodeType::Const, super::internal::consts::ConstGenerator::generate),
  // (NodeType::ConstProperty, ConstPropertyGenerator::generate),
  (NodeType::Continue, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Declare, super::internal::declare::DeclareGenerator::generate),
  // (NodeType::DeclareArgument, DeclareArgumentGenerator::generate),
  (NodeType::DoWhile, super::internal::dowhile::DoWhileGenerator::generate),
  (NodeType::Echo, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Encapsed, super::internal::string::StringGenerator::generate_encapsed),
  // (NodeType::EncapsedPart, StringGenerator::generate_encapsed_part),
  (NodeType::Enum, super::internal::enums::EnumGenerator::generate),
  // (NodeType::EnumItem, EnumItemGenerator::generate),
  // (NodeType::Eval, EvalGenerator::generate),
  // (NodeType::Exit, ExitGenerator::generate),
  (NodeType::For, super::internal::fors::ForGenerator::generate),
  (NodeType::Foreach, super::internal::foreach::ForeachGenerator::generate),
  (NodeType::Function, super::internal::function::FunctionGenerator::generate),
  // (NodeType::Global, GlobalGenerator::generate),
  // (NodeType::Goto, GotoGenerator::generate),
  (NodeType::Identifier, super::internal::identifier::IdentifierGenerator::generate),
  (NodeType::If, super::internal::ifs::IfGenerator::generate),
  // (NodeType::Include, IncludeGenerator::generate),
  // (NodeType::InstanceOf, InstanceOfGenerator::generate),
  (NodeType::Interface, super::internal::interface::InterfaceGenerator::generate),
  // (NodeType::Label, LabelGenerator::generate),
  // (NodeType::List, ListGenerator::generate),
  // (NodeType::Magic, MagicGenerator::generate),
  (NodeType::Match, super::internal::matchs::MatchGenerator::generate),
  // (NodeType::MatchArm, MatchArmGenerator::generate),
  // (NodeType::Method, super::internal::method::MethodGenerator::generate),
  (NodeType::Namespace, super::internal::namespace::NamespaceGenerator::generate),
  (NodeType::New, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Number, super::internal::number::NumberGenerator::generate),
  // (NodeType::ObjectAccess, ObjectAccessGenerator::generate),
  // (NodeType::Parameter, ParameterGenerator::generate),
  // (NodeType::Parent, ParentGenerator::generate),
  // (NodeType::Parenthesis, super::internal::parenthesis::ParenthesisGenerator::generate),
  (NodeType::Post, super::internal::post::PostGenerator::generate),
  (NodeType::Pre, super::internal::pre::PreGenerator::generate),
  (NodeType::Print, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Program, super::internal::program::ProgramGenerator::generate),
  // (NodeType::Property, super::internal::property::PropertyGenerator::generate),
  // (NodeType::PropertyItem, PropertyItemGenerator::generate),
  (NodeType::Return, super::internal::singles::SinglesGenerator::generate),
  // (NodeType::Static, StaticGenerator::generate),
  (NodeType::StaticLookup, super::internal::staticlookup::StaticLookupGenerator::generate),
  (NodeType::String, super::internal::string::StringGenerator::generate),
  (NodeType::Switch, super::internal::switch::SwitchGenerator::generate),
  (NodeType::Ternary, super::internal::ternary::TernaryGenerator::generate),
  (NodeType::Trait, super::internal::traits::TraitGenerator::generate),
  // (NodeType::TraitUse, super::internal::traituse::TraitUseGenerator::generate),
  // (NodeType::TraitUseAlias, TraitUseAliasGenerator::generate),
  // (NodeType::TraitUsePrecedence, TraitUsePrecedenceGenerator::generate),
  (NodeType::Throw, super::internal::singles::SinglesGenerator::generate),
  (NodeType::Try, super::internal::tries::TryGenerator::generate),
  (NodeType::Type, super::internal::types::TypeGenerator::generate),
  (NodeType::Use, super::internal::uses::UseGenerator::generate),
  (NodeType::Variable, super::internal::variable::VariableGenerator::generate),
  (NodeType::While, super::internal::whiles::WhileGenerator::generate),
  (NodeType::Yield, super::internal::yields::YieldGenerator::generate),
  (NodeType::YieldFrom, super::internal::yields::YieldGenerator::generate_from),
];

#[derive(Debug, Clone)]
pub struct Line {
  pub line: String,
  pub indent: usize,
}

impl Line {
  pub fn new() -> Self {
    Self { line: String::new(), indent: 0 }
  }

  pub fn shift(&mut self, line: &str) {
    self.line.insert_str(0, line);
  }

  pub fn push(&mut self, line: &str) {
    self.line.push_str(line);
  }

  pub fn to_string(&self) -> String {
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

impl Builder {
  pub fn new() -> Self {
    Self { lines: vec![] }
  }

  pub fn new_line(&mut self) {
    self.lines.push(Line::new());
  }

  pub fn shift(&mut self, line: &str) {
    guard!(self.lines.last_mut(), {
      return;
    }).shift(line);
  }

  pub fn push(&mut self, line: &str) {
    guard!(self.lines.last_mut(), {
      return;
    }).push(line);
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

  pub fn extend_first_line(&mut self, builder: &Builder) {
    if builder.lines.is_empty() {
      return;
    }
    let mut lines = builder.lines.clone();
    let first = lines.remove(0);
    guard!(self.lines.last_mut(), {
      return;
    }).push(&first.line);
    self.lines.extend(lines);
  }

  pub fn extend(&mut self, builder: &Builder) {
    self.lines.extend(builder.lines.clone());
  }

  pub fn to_string(&self, separator: &str) -> String {
    let mut lines = self.lines.clone();
    if self.last_len() == 0 {
      lines.pop();
    }
    lines
      .iter()
      .map(|i| i.to_string())
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
        ].contains(node_type)
      {
        return Some(";");
      }
    } else if self.end == EndMode::CommaWithoutEnd {
      if !self.is_last {
        return Some(",");
      }
    }
    return None;
  }
}

pub struct Generator {
  pub max_length: usize,
  nodes: Nodes,
}

impl Generator {
  pub fn new(nodes: Nodes) -> Self {
    Self { nodes, max_length: 60 }
  }

  pub fn start(&mut self) -> String {
    self
      .generate_nodes_new(&self.nodes.clone(), &mut GeneratorArgument::for_block())
      .to_string("\n")
  }

  pub fn generate_nodes_new(&mut self, nodes: &Nodes, args: &mut GeneratorArgument) -> Builder {
    let mut builder = Builder::new();
    self.generate_nodes(&mut builder, nodes, args);
    builder
  }

  pub fn generate_nodes(
    &mut self,
    builder: &mut Builder,
    nodes: &Nodes,
    args: &mut GeneratorArgument
  ) {
    for (i, node) in nodes.iter().enumerate() {
      args.is_last = i == nodes.len() - 1;
      builder.new_line();
      self.generate_node(builder, node, args);
    }
  }

  pub fn generate_node_new(&mut self, node: &Node) -> Builder {
    let mut builder = Builder::new();
    builder.new_line();
    self.generate_node(&mut builder, node, &mut GeneratorArgument::default());
    builder
  }

  pub fn generate_node(
    &mut self,
    builder: &mut Builder,
    node: &Node,
    args: &mut GeneratorArgument
  ) {
    for (node_type, generator) in args.generators.iter() {
      if *node_type == node.get_type() {
        // println!("Generating node: {:?}", node_type);
        let leading_comments = node.get_leading_comments();
        let trailing_comments = node.get_trailing_comments();
        if leading_comments.len() > 0 || trailing_comments.len() > 0 {
          let mut scoped_builder = Builder::new();
          Self::handle_comments(&mut scoped_builder, leading_comments);
          if scoped_builder.total_len() == 0 {
            scoped_builder.new_line();
          }
          generator(self, &mut scoped_builder, node);
          if let Some(end) = args.get_end_statement(&node.get_type()) {
            scoped_builder.push(end);
          }
          Self::handle_comments(&mut scoped_builder, trailing_comments);
          if builder.last_len() == 0 {
            builder.extend_first_line(&scoped_builder);
          } else {
            scoped_builder.indent();
            builder.extend(&scoped_builder);
          }
        } else {
          generator(self, builder, node);
          if let Some(end) = args.get_end_statement(&node.get_type()) {
            builder.push(end);
          }
        }
        return;
      }
    }
    println!("No generator for node: {:?}", node.get_type());
  }

  fn handle_comments(builder: &mut Builder, nodes: &Nodes) {
    if nodes.len() > 0 {
      builder.new_line();
      builder.push(
        &nodes
          .iter()
          .filter_map(|i| {
            let comment = match i.get_type() {
              NodeType::CommentBlock => {
                let c = guard!(i.to_owned().cast::<CommentBlockNode>().ok()).comment;
                format!("/*{}*/", c)
              }
              NodeType::CommentLine => {
                let c = guard!(i.to_owned().cast::<CommentLineNode>().ok()).comment;
                format!("//{}", c)
              }
              _ => {
                return None;
              }
            };
            Some(comment)
          })
          .collect::<Vec<String>>()
          .join("\n")
      );
      builder.new_line();
    }
  }

  pub fn check_nodes_has_comments(nodes: &Nodes) -> bool {
    nodes
      .iter()
      .fold(
        false,
        |acc, i| (acc || i.get_leading_comments().len() > 0 || i.get_trailing_comments().len() > 0)
      )
  }
}
