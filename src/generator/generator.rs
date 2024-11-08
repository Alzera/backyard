use crate::{ guard, parser::node::{ Node, NodeType, Nodes } };

// #[napi(object)]
// #[derive(Debug, Clone)]
// pub struct GenPart {
//   pub groups: GenLine,
//   pub values: Vec<String>,
//   pub use_group: bool,
//   pub priority: u32,
// }
// impl GenPart {
//   pub fn new_values(values: Vec<String>, priority: u32) -> Self {
//     Self { groups: vec![], values, use_group: false, priority }
//   }

//   pub fn new_group(groups: GenLine, priority: u32) -> Self {
//     Self { groups, values: vec![], use_group: true, priority }
//   }

//   pub fn total_length(&self) -> usize {
//     if self.use_group {
//       self.groups
//         .iter()
//         .map(|i| i.total_length())
//         .sum()
//     } else {
//       self.values
//         .iter()
//         .map(|v| v.len())
//         .sum()
//     }
//   }

//   pub fn shift_values(&mut self, value: String) {
//     self.values.insert(0, value);
//   }

//   pub fn push_values(&mut self, value: String) {
//     self.values.push(value);
//   }
// }
// pub type GenLine = Vec<GenPart>;
// pub trait GenLineTrait {
//   fn max_priority(&self) -> u32;
// }
// impl GenLineTrait for GenLine {
//   fn max_priority(&self) -> u32 {
//     self
//       .iter()
//       .map(|i| i.priority)
//       .max()
//       .unwrap()
//   }
// }

// pub type GenLines = Vec<GenLine>;
type InternalGenerator = fn(&mut Generator, &mut Builder, &Node, &mut GeneratorArgument);

const DEFAULT_GENERATORS: [(NodeType, InternalGenerator); 10] = [
  (NodeType::AnonymousFunction, super::internal::function::FunctionGenerator::generate_anonymous),
  // (NodeType::Argument, super::internal::call::CallGenerator::generate_argument),
  // (NodeType::Array, super::internal::array::ArrayGenerator::generate),
  // (NodeType::ArrayItem, super::internal::array::ArrayGenerator::generate_item),
  // (NodeType::ArrayLookup, super::internal::arraylookup::ArrayLookupGenerator::generate),
  (NodeType::ArrowFunction, super::internal::function::FunctionGenerator::generate_arrow),
  (NodeType::Assignment, super::internal::assignment::AssignmentGenerator::generate),
  (NodeType::Bin, super::internal::bin::BinGenerator::generate),
  (NodeType::Block, super::internal::block::BlockGenerator::generate),
  // (NodeType::Break, BreakGenerator::generate),
  // (NodeType::Call, super::internal::call::CallGenerator::generate),
  // (NodeType::Case, CaseGenerator::generate),
  // (NodeType::Cast, super::internal::parenthesis::ParenthesisGenerator::generate_cast),
  // (NodeType::Catch, CatchGenerator::generate),
  // (NodeType::Class, ClassGenerator::generate),
  // (NodeType::Clone, CloneGenerator::generate),
  // (NodeType::CommentBlock, CommentBlockGenerator::generate),
  // (NodeType::CommentLine, CommentLineGenerator::generate),
  // (NodeType::Const, ConstGenerator::generate),
  // (NodeType::ConstProperty, ConstPropertyGenerator::generate),
  // (NodeType::Continue, ContinueGenerator::generate),
  // (NodeType::Declare, DeclareGenerator::generate),
  // (NodeType::DeclareArgument, DeclareArgumentGenerator::generate),
  // (NodeType::DoWhile, DoWhileGenerator::generate),
  // (NodeType::Echo, EchoGenerator::generate),
  // (NodeType::Encapsed, StringGenerator::generate_encapsed),
  // (NodeType::EncapsedPart, StringGenerator::generate_encapsed_part),
  // (NodeType::Enum, EnumGenerator::generate),
  // (NodeType::EnumItem, EnumItemGenerator::generate),
  // (NodeType::Eval, EvalGenerator::generate),
  // (NodeType::Exit, ExitGenerator::generate),
  // (NodeType::For, ForGenerator::generate),
  // (NodeType::Foreach, ForeachGenerator::generate),
  (NodeType::Function, super::internal::function::FunctionGenerator::generate),
  // (NodeType::Global, GlobalGenerator::generate),
  // (NodeType::Goto, GotoGenerator::generate),
  (NodeType::Identifier, super::internal::identifier::IdentifierGenerator::generate),
  // (NodeType::If, IfGenerator::generate),
  // (NodeType::Include, IncludeGenerator::generate),
  // (NodeType::InstanceOf, InstanceOfGenerator::generate),
  // (NodeType::Interface, InterfaceGenerator::generate),
  // (NodeType::Label, LabelGenerator::generate),
  // (NodeType::List, ListGenerator::generate),
  // (NodeType::Magic, MagicGenerator::generate),
  // (NodeType::Match, MatchGenerator::generate),
  // (NodeType::MatchArm, MatchArmGenerator::generate),
  // (NodeType::Method, MethodGenerator::generate),
  // (NodeType::Namespace, NamespaceGenerator::generate),
  // (NodeType::New, NewGenerator::generate),
  (NodeType::Number, super::internal::number::NumberGenerator::generate),
  // (NodeType::ObjectAccess, ObjectAccessGenerator::generate),
  // (NodeType::Parameter, ParameterGenerator::generate),
  // (NodeType::Parent, ParentGenerator::generate),
  // (NodeType::Parenthesis, super::internal::parenthesis::ParenthesisGenerator::generate),
  // (NodeType::Post, PostGenerator::generate),
  // (NodeType::Pre, PreGenerator::generate),
  // (NodeType::Print, PrintGenerator::generate),
  // (NodeType::Program, ProgramGenerator::generate),
  // (NodeType::Property, PropertyGenerator::generate),
  // (NodeType::PropertyItem, PropertyItemGenerator::generate),
  // (NodeType::Return, ReturnGenerator::generate),
  // (NodeType::Static, StaticGenerator::generate),
  // (NodeType::StaticLookup, StaticLookupGenerator::generate),
  // (NodeType::String, StringGenerator::generate),
  // (NodeType::Switch, SwitchGenerator::generate),
  // (NodeType::Ternary, TernaryGenerator::generate),
  // (NodeType::Trait, TraitGenerator::generate),
  // (NodeType::TraitUse, TraitUseGenerator::generate),
  // (NodeType::TraitUseAlias, TraitUseAliasGenerator::generate),
  // (NodeType::TraitUsePrecedence, TraitUsePrecedenceGenerator::generate),
  // (NodeType::Throw, ThrowGenerator::generate),
  // (NodeType::Try, TryGenerator::generate),
  (NodeType::Type, super::internal::types::TypeGenerator::generate),
  // (NodeType::Use, UseGenerator::generate),
  (NodeType::Variable, super::internal::variable::VariableGenerator::generate),
  // (NodeType::While, WhileGenerator::generate),
  // (NodeType::Yield, YieldGenerator::generate),
  // (NodeType::YieldFrom, YieldFromGenerator::generate),
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

  // pub fn shift(&mut self, line: &str) {
  //   self.line.insert_str(0, line);
  // }

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
  lines: Vec<Line>,
}

impl Builder {
  pub fn new() -> Self {
    Self { lines: vec![] }
  }

  pub fn new_line(&mut self) {
    self.lines.push(Line::new());
  }

  // pub fn shift(&mut self, line: &str) {
  //   guard!(self.lines.last_mut(), {
  //     return;
  //   }).shift(line);
  // }

  pub fn push(&mut self, line: &str) {
    guard!(self.lines.last_mut(), {
      return;
    }).push(line);
  }

  pub fn push_all_lines(&mut self, line: &str) {
    self.lines.iter_mut().for_each(|i| {
      i.push(line);
    });
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

  pub fn indent_at(&mut self, index: usize) {
    self.lines
      .iter_mut()
      .enumerate()
      .for_each(|(i, line)| {
        if i == index {
          line.indent += 1;
        }
      });
  }

  pub fn indent_last(&mut self) {
    self.indent_at(self.lines.len() - 1);
  }

  pub fn extend_first_line(&mut self, builder: &Builder) {
    if builder.lines.is_empty() {
      return;
    }
    let mut lines = builder.lines.clone();
    let first = lines.remove(0);
    self.lines.last_mut().unwrap().push(&first.line);
    self.lines.extend(lines);
  }

  pub fn extend(&mut self, builder: &Builder) {
    self.lines.extend(builder.lines.clone());
  }

  pub fn to_string(&self, separator: &str) -> String {
    self.lines
      .iter()
      .map(|i| i.to_string())
      .collect::<Vec<String>>()
      .join(separator)
  }
}

pub struct Generator {
  nodes: Nodes,
}

#[derive(Debug, Clone)]
pub struct GeneratorArgument<'a> {
  // separator: &'a str,
  // closure: &'a str,
  generators: &'a [(NodeType, InternalGenerator)],
  pub max_length: usize,
}

impl<'a> GeneratorArgument<'a> {
  pub fn default() -> Self {
    Self { generators: &DEFAULT_GENERATORS, max_length: 60 }
  }

  pub fn new(generators: &'a [(NodeType, InternalGenerator)], max_length: usize) -> Self {
    Self { generators, max_length }
  }

  // pub fn clone_with_indent(&self, indent: usize) -> Self {
  //   Self { separator: self.separator, closure: self.closure, generators: self.generators, indent }
  // }
}

impl Generator {
  pub fn new(nodes: Nodes) -> Self {
    Self { nodes }
  }

  pub fn start(&mut self) -> String {
    self.generate_nodes_new(&self.nodes.clone(), &mut GeneratorArgument::default()).to_string("\n")
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
    for node in nodes.iter() {
      builder.new_line();
      self.generate_node(builder, node, args);
    }
  }

  pub fn generate_node_new(&mut self, node: &Node, args: &mut GeneratorArgument) -> Builder {
    let mut builder = Builder::new();
    builder.new_line();
    self.generate_node(&mut builder, node, args);
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
        println!("Generating node: {:?}", node_type);
        generator(self, builder, node, args);
        return;
      }
    }
    println!("No generator for node: {:?}", node.get_type());
  }
}

// pub struct Formatter {}

// impl Formatter {
//   pub fn get_indent(i: usize) -> String {
//     let mut indent = String::new();
//     for _ in 0..i {
//       indent.push_str("  ");
//     }
//     indent
//   }

//   pub fn format(input: String, i: usize) -> String {
//     let indent = Self::get_indent(i);
//     input
//       .split("\n")
//       .map(|i| format!("{indent}{i}"))
//       .collect::<Vec<String>>()
//       .join("\n")
//   }
// }
