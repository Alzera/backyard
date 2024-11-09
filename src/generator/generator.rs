use crate::{
  guard,
  parser::{
    node::{ Node, NodeTraitCast, NodeType, Nodes },
    nodes::comment::{ CommentBlockNode, CommentLineNode },
  },
};

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

const DEFAULT_GENERATORS: [(NodeType, InternalGenerator); 13] = [
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
  (NodeType::Encapsed, super::internal::string::StringGenerator::generate_encapsed),
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
  (NodeType::Program, super::internal::program::ProgramGenerator::generate),
  // (NodeType::Property, PropertyGenerator::generate),
  // (NodeType::PropertyItem, PropertyItemGenerator::generate),
  // (NodeType::Return, ReturnGenerator::generate),
  // (NodeType::Static, StaticGenerator::generate),
  // (NodeType::StaticLookup, StaticLookupGenerator::generate),
  (NodeType::String, super::internal::string::StringGenerator::generate),
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
  pub lines: Vec<Line>,
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

  pub fn pop(&mut self) -> Option<char> {
    guard!(self.lines.last_mut()).line.pop()
  }

  // pub fn push_all_lines(&mut self, line: &str) {
  //   self.lines.iter_mut().for_each(|i| {
  //     i.push(line);
  //   });
  // }

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

  pub fn block_end_callback(node: &Node) -> Option<&str> {
    if [NodeType::Function].contains(&node.get_type()) { None } else { Some(";") }
  }
}

pub struct Generator {
  nodes: Nodes,
}

#[derive(Debug, Clone)]
pub struct GeneratorArgument<'a> {
  generators: &'a [(NodeType, InternalGenerator)],
  pub max_length: usize,
}

impl<'a> GeneratorArgument<'a> {
  pub fn default() -> Self {
    Self { generators: &DEFAULT_GENERATORS, max_length: 60 }
  }

  pub fn generator(generators: &'a [(NodeType, InternalGenerator)]) -> Self {
    Self { generators, max_length: 60 }
  }
}

impl Generator {
  pub fn new(nodes: Nodes) -> Self {
    Self { nodes }
  }

  pub fn start(&mut self) -> String {
    self
      .generate_nodes_new(
        &self.nodes.clone(),
        Builder::block_end_callback,
        &mut GeneratorArgument::default()
      )
      .to_string("\n")
  }

  pub fn generate_nodes_new<T>(
    &mut self,
    nodes: &Nodes,
    end_callback: T,
    args: &mut GeneratorArgument
  ) -> Builder
    where T: Fn(&Node) -> Option<&str>
  {
    let mut builder = Builder::new();
    self.generate_nodes(&mut builder, nodes, end_callback, args);
    builder
  }

  pub fn generate_nodes<T>(
    &mut self,
    builder: &mut Builder,
    nodes: &Nodes,
    end_callback: T,
    args: &mut GeneratorArgument
  )
    where T: Fn(&Node) -> Option<&str>
  {
    for node in nodes.iter() {
      builder.new_line();
      self.generate_node(builder, node, &end_callback, args);
    }
  }

  pub fn generate_node_new<T>(
    &mut self,
    node: &Node,
    end_callback: T,
    args: &mut GeneratorArgument
  ) -> Builder
    where T: Fn(&Node) -> Option<&str>
  {
    let mut builder = Builder::new();
    builder.new_line();
    self.generate_node(&mut builder, node, end_callback, args);
    builder
  }

  pub fn generate_node<T>(
    &mut self,
    builder: &mut Builder,
    node: &Node,
    end_callback: T,
    args: &mut GeneratorArgument
  )
    where T: Fn(&Node) -> Option<&str>
  {
    for (node_type, generator) in args.generators.iter() {
      if *node_type == node.get_type() {
        println!("Generating node: {:?}", node_type);
        let leading_comments = node.get_leading_comments();
        let trailing_comments = node.get_trailing_comments();
        if leading_comments.len() > 0 || trailing_comments.len() > 0 {
          let mut scoped_builder = Builder::new();
          Self::handle_comments(&mut scoped_builder, leading_comments);
          if scoped_builder.total_len() == 0 {
            scoped_builder.new_line();
          }
          generator(self, &mut scoped_builder, node, args);
          if let Some(end) = end_callback(node) {
            scoped_builder.push(&end);
          }
          Self::handle_comments(&mut scoped_builder, trailing_comments);
          if builder.last_len() == 0 {
            builder.extend_first_line(&scoped_builder);
          } else {
            scoped_builder.indent();
            builder.extend(&scoped_builder);
          }
        } else {
          generator(self, builder, node, args);
          if let Some(end) = end_callback(node) {
            builder.push(&end);
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
