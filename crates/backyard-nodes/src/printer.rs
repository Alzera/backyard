use std::fmt::Debug;

use bstr::BString;

use crate::{
  AssignmentType,
  BinaryType,
  BodyType,
  CastType,
  Inheritance,
  MagicMethodName,
  MagicName,
  Modifier,
  Node,
  NodeWrapper,
  PostType,
  PreType,
  Quote,
  RangeLocation,
  UseItemModifier,
  Visibility,
};

#[derive(Debug, Clone)]
pub enum PrintType {
  Vec,
  Object,
  Inline,
}

#[derive(Clone)]
pub struct PrintBuilder {
  lines: Vec<String>,
  print_type: PrintType,
}

impl PrintBuilder {
  pub fn new(print_type: PrintType) -> Self {
    Self { lines: vec![], print_type }
  }

  pub fn new_line(&mut self) {
    self.lines.push(String::new());
  }

  pub fn push(&mut self, line: &str) {
    if let Some(last) = self.lines.last_mut() {
      last.push_str(line);
    }
  }

  pub fn shift(&mut self, text: &str) {
    if let Some(last) = self.lines.last_mut() {
      last.insert_str(0, text);
    }
  }

  pub fn shift_new_line(&mut self, text: &str) {
    self.lines.insert(0, text.to_string());
  }

  pub fn push_props(&mut self, check_last: bool, props: &mut [(&str, PrintBuilder)]) {
    let last_index = props.len().saturating_sub(1);
    for (i, (key, value)) in props.iter_mut().enumerate() {
      match &value.print_type {
        PrintType::Vec => {
          if value.lines.is_empty() {
            value.shift_new_line(format!("{}[]: ~", key).as_str());
          } else {
            value.shift_new_line(format!("{}[]", key).as_str());
          }
        }
        PrintType::Object => {
          value.indent(false, false);
          value.shift_new_line(key);
        }
        PrintType::Inline => {
          value.shift(format!("{}: ", key).as_str());
        }
      }
      value.indent(false, if check_last { i != last_index } else { true });
      self.extend(value.to_owned());
    }
  }

  pub fn extend(&mut self, builder: PrintBuilder) {
    self.lines.extend(builder.lines);
  }

  pub fn indent(&mut self, is_vec: bool, is_middle: bool) {
    let first_char = if is_vec {
      if is_middle { "╟-" } else { "╙-" }
    } else {
      if is_middle { "├-" } else { "└-" }
    };
    let middle_char = if is_middle {
      if is_vec { "║ " } else { "│ " }
    } else {
      "  "
    };
    for (i, line) in self.lines.iter_mut().enumerate() {
      if i == 0 {
        line.insert_str(0, first_char);
      } else {
        line.insert_str(0, middle_char);
      }
    }
  }
}

impl Debug for PrintBuilder {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for line in &self.lines {
      writeln!(f, "{}", line)?;
    }
    Ok(())
  }
}

pub struct PrintConfig {
  pub with_leading_trailing: bool,
  pub with_location: bool,
}

impl PrintConfig {
  pub fn new(with_leading_trailing: bool, with_location: bool) -> Self {
    Self {
      with_leading_trailing,
      with_location,
    }
  }
}

// ┴ ┬ ┤ - ┼ ├ └ │
pub trait Printable {
  fn print(&self, config: &PrintConfig) -> PrintBuilder;
}

impl<'arena> Printable for Node<'arena> {
  fn print(&self, config: &PrintConfig) -> PrintBuilder {
    let mut builder = match &self.wrapper {
      NodeWrapper::AnonymousClass(v) => v.print(config),
      NodeWrapper::AnonymousFunction(v) => v.print(config),
      NodeWrapper::CallArgument(v) => v.print(config),
      NodeWrapper::Array(v) => v.print(config),
      NodeWrapper::ArrayItem(v) => v.print(config),
      NodeWrapper::ArrayLookup(v) => v.print(config),
      NodeWrapper::ArrowFunction(v) => v.print(config),
      NodeWrapper::Assignment(v) => v.print(config),
      NodeWrapper::Attribute(v) => v.print(config),
      NodeWrapper::AttributeItem(v) => v.print(config),
      NodeWrapper::Bin(v) => v.print(config),
      NodeWrapper::Block(v) => v.print(config),
      NodeWrapper::Boolean(v) => v.print(config),
      NodeWrapper::Break(v) => v.print(config),
      NodeWrapper::Call(v) => v.print(config),
      NodeWrapper::Case(v) => v.print(config),
      NodeWrapper::Cast(v) => v.print(config),
      NodeWrapper::Catch(v) => v.print(config),
      NodeWrapper::Class(v) => v.print(config),
      NodeWrapper::ClassKeyword(v) => v.print(config),
      NodeWrapper::Clone(v) => v.print(config),
      NodeWrapper::CommentBlock(v) => v.print(config),
      NodeWrapper::CommentDoc(v) => v.print(config),
      NodeWrapper::CommentLine(v) => v.print(config),
      NodeWrapper::Const(v) => v.print(config),
      NodeWrapper::ConstProperty(v) => v.print(config),
      NodeWrapper::ConstructorParameter(v) => v.print(config),
      NodeWrapper::Continue(v) => v.print(config),
      NodeWrapper::Declare(v) => v.print(config),
      NodeWrapper::DeclareArgument(v) => v.print(config),
      NodeWrapper::DoWhile(v) => v.print(config),
      NodeWrapper::DoWhileCondition(v) => v.print(config),
      NodeWrapper::Echo(v) => v.print(config),
      NodeWrapper::Else(v) => v.print(config),
      NodeWrapper::Encapsed(v) => v.print(config),
      NodeWrapper::EncapsedPart(v) => v.print(config),
      NodeWrapper::Enum(v) => v.print(config),
      NodeWrapper::EnumItem(v) => v.print(config),
      NodeWrapper::Eval(v) => v.print(config),
      NodeWrapper::Exit(v) => v.print(config),
      NodeWrapper::Finally(v) => v.print(config),
      NodeWrapper::For(v) => v.print(config),
      NodeWrapper::Foreach(v) => v.print(config),
      NodeWrapper::Function(v) => v.print(config),
      NodeWrapper::Global(v) => v.print(config),
      NodeWrapper::Goto(v) => v.print(config),
      NodeWrapper::HereDoc(v) => v.print(config),
      NodeWrapper::Identifier(v) => v.print(config),
      NodeWrapper::If(v) => v.print(config),
      NodeWrapper::Include(v) => v.print(config),
      NodeWrapper::Inline(v) => v.print(config),
      NodeWrapper::Interface(v) => v.print(config),
      NodeWrapper::IntersectionType(v) => v.print(config),
      NodeWrapper::Label(v) => v.print(config),
      NodeWrapper::List(v) => v.print(config),
      NodeWrapper::Magic(v) => v.print(config),
      NodeWrapper::MagicMethod(v) => v.print(config),
      NodeWrapper::Match(v) => v.print(config),
      NodeWrapper::MatchArm(v) => v.print(config),
      NodeWrapper::Method(v) => v.print(config),
      NodeWrapper::Namespace(v) => v.print(config),
      NodeWrapper::Negate(v) => v.print(config),
      NodeWrapper::New(v) => v.print(config),
      NodeWrapper::NowDoc(v) => v.print(config),
      NodeWrapper::Null(v) => v.print(config),
      NodeWrapper::Number(v) => v.print(config),
      NodeWrapper::ObjectAccess(v) => v.print(config),
      NodeWrapper::Parameter(v) => v.print(config),
      NodeWrapper::Parent(v) => v.print(config),
      NodeWrapper::Parenthesis(v) => v.print(config),
      NodeWrapper::Post(v) => v.print(config),
      NodeWrapper::Pre(v) => v.print(config),
      NodeWrapper::Print(v) => v.print(config),
      NodeWrapper::Program(v) => v.print(config),
      NodeWrapper::Property(v) => v.print(config),
      NodeWrapper::PropertyHook(v) => v.print(config),
      NodeWrapper::PropertyItem(v) => v.print(config),
      NodeWrapper::Reference(v) => v.print(config),
      NodeWrapper::Return(v) => v.print(config),
      NodeWrapper::SelfKeyword(v) => v.print(config),
      NodeWrapper::Silent(v) => v.print(config),
      NodeWrapper::Static(v) => v.print(config),
      NodeWrapper::StaticKeyword(v) => v.print(config),
      NodeWrapper::StaticLookup(v) => v.print(config),
      NodeWrapper::String(v) => v.print(config),
      NodeWrapper::Switch(v) => v.print(config),
      NodeWrapper::Ternary(v) => v.print(config),
      NodeWrapper::This(v) => v.print(config),
      NodeWrapper::Trait(v) => v.print(config),
      NodeWrapper::TraitUse(v) => v.print(config),
      NodeWrapper::TraitUseAlias(v) => v.print(config),
      NodeWrapper::TraitUsePrecedence(v) => v.print(config),
      NodeWrapper::Throw(v) => v.print(config),
      NodeWrapper::Try(v) => v.print(config),
      NodeWrapper::Type(v) => v.print(config),
      NodeWrapper::UnionType(v) => v.print(config),
      NodeWrapper::Use(v) => v.print(config),
      NodeWrapper::UseItem(v) => v.print(config),
      NodeWrapper::Variable(v) => v.print(config),
      NodeWrapper::Variadic(v) => v.print(config),
      NodeWrapper::While(v) => v.print(config),
      NodeWrapper::Yield(v) => v.print(config),
      NodeWrapper::YieldFrom(v) => v.print(config),
    };
    if config.with_leading_trailing || config.with_location {
      let mut node_builder = PrintBuilder::new(PrintType::Object);
      let mut props = vec![];
      if config.with_leading_trailing {
        props.push(("leadings", self.leadings.print(config)));
        props.push(("trailings", self.trailings.print(config)));
      }
      if config.with_location {
        props.push(("location", self.loc.print(config)));
      }
      node_builder.push_props(true, props.as_mut_slice());
      builder.extend(node_builder);
    }
    builder
  }
}

impl Printable for RangeLocation {
  fn print(&self, _: &PrintConfig) -> PrintBuilder {
    let mut builder = PrintBuilder::new(PrintType::Object);
    builder.shift_new_line(
      format!(
        "end: line {:?}, column {:?}, offset {:?}",
        self.end.line,
        self.end.column,
        self.end.offset
      ).as_str()
    );
    builder.shift_new_line(
      format!(
        "start: line {:?}, column {:?}, offset {:?}",
        self.start.line,
        self.start.column,
        self.start.offset
      ).as_str()
    );
    builder
  }
}

impl<'arena> Printable for bumpalo::collections::Vec<'arena, Node<'arena>> {
  fn print(&self, config: &PrintConfig) -> PrintBuilder {
    let len = self.len();
    if len == 0 {
      return PrintBuilder::new(PrintType::Vec);
    }
    let mut builder = PrintBuilder::new(PrintType::Vec);
    let last_index = len.saturating_sub(1);
    for (i, node) in self.iter().enumerate() {
      let mut scoped_builder = node.print(config);
      if i == last_index {
        scoped_builder.indent(true, false);
      } else {
        scoped_builder.indent(true, true);
      }
      builder.extend(scoped_builder);
    }
    builder
  }
}

impl<T> Printable for Option<T> where T: Printable {
  fn print(&self, config: &PrintConfig) -> PrintBuilder {
    match self {
      Some(x) => x.print(config),
      None => {
        let mut builder = PrintBuilder::new(PrintType::Inline);
        builder.shift_new_line("-");
        builder
      }
    }
  }
}

impl<'arena> Printable for bumpalo::boxed::Box<'arena, Node<'arena>> {
  fn print(&self, config: &PrintConfig) -> PrintBuilder {
    self.as_ref().print(config)
  }
}

impl<T> Printable for Vec<T> where T: Printable {
  fn print(&self, config: &PrintConfig) -> PrintBuilder {
    let mut childs = Vec::new();
    for x in self {
      if let Some(last) = x.print(config).lines.last_mut() {
        childs.push(last.to_owned());
      }
    }
    let mut builder = PrintBuilder::new(PrintType::Inline);
    builder.shift_new_line(childs.join(" | ").as_str());
    return builder;
  }
}

macro_rules! impl_build_printableable {
  ($($t:ty),*) => {
      $(
          impl Printable for $t {
            fn print(&self, _: &PrintConfig) -> PrintBuilder {
              let mut builder = PrintBuilder::new(PrintType::Inline);
              builder.shift_new_line(format!("{:?}", self).as_str());
              builder
            }
          }
      )*
  };
}

macro_rules! impl_build_printableable_enum {
  ($($t:ty),*) => {
      $(
          impl Printable for $t {
            fn print(&self, _: &PrintConfig) -> PrintBuilder {
              let mut builder = PrintBuilder::new(PrintType::Inline);
              builder.shift_new_line(format!("{}::{:?}", stringify!($t), self).as_str());
              builder
            }
          }
      )*
  };
}

impl_build_printableable!(bool, BString);

impl_build_printableable_enum!(
  BodyType,
  AssignmentType,
  BinaryType,
  CastType,
  PostType,
  PreType,
  MagicName,
  MagicMethodName,
  UseItemModifier,
  Visibility,
  Inheritance,
  Quote,
  Modifier
);

#[cfg(test)]
mod tests {
  use bstr::BString;

  use crate::{
    builder::{ BlueprintBuildable, Builder },
    printer::{ PrintConfig, Printable },
    AssignmentType,
    CommentLineNode,
    Location,
    RangeLocation,
  };

  #[test]
  fn printer() {
    let arena = bumpalo::Bump::new();
    let b = Builder::new();
    let mut node = b
      .Program(
        &[b.Assignment(b.Variable(b.Identifier("a")), AssignmentType::Default, b.Number("21"))]
      )
      .build(&arena);
    let comment = CommentLineNode::loc(
      BString::new(b"Test comment".to_vec()),
      Some(RangeLocation {
        start: Location { line: 1, column: 0, offset: 0 },
        end: Location { line: 1, column: 0, offset: 0 },
      })
    );
    node.leadings = Some(bumpalo::vec![in &arena; comment]);
    println!("{:?}", node.print(&PrintConfig::new(false, true)));
  }
}
