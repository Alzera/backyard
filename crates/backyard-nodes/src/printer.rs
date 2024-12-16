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
          value.shift_new_line(format!("{}[]", key).as_str());
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
      if is_middle { "┼-" } else { "┴-" }
    } else {
      if is_middle { "├-" } else { "└-" }
    };
    let middle_char = if is_middle { "│ " } else { "  " };
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

// ┴ ┬ ┤ - ┼ ├ └ │
pub trait Printable {
  fn print(&self) -> PrintBuilder;
}

impl<'arena> Printable for Node<'arena> {
  fn print(&self) -> PrintBuilder {
    let mut builder = match &self.wrapper {
      NodeWrapper::AnonymousClass(v) => v.print(),
      NodeWrapper::AnonymousFunction(v) => v.print(),
      NodeWrapper::CallArgument(v) => v.print(),
      NodeWrapper::Array(v) => v.print(),
      NodeWrapper::ArrayItem(v) => v.print(),
      NodeWrapper::ArrayLookup(v) => v.print(),
      NodeWrapper::ArrowFunction(v) => v.print(),
      NodeWrapper::Assignment(v) => v.print(),
      NodeWrapper::Attribute(v) => v.print(),
      NodeWrapper::AttributeItem(v) => v.print(),
      NodeWrapper::Bin(v) => v.print(),
      NodeWrapper::Block(v) => v.print(),
      NodeWrapper::Boolean(v) => v.print(),
      NodeWrapper::Break(v) => v.print(),
      NodeWrapper::Call(v) => v.print(),
      NodeWrapper::Case(v) => v.print(),
      NodeWrapper::Cast(v) => v.print(),
      NodeWrapper::Catch(v) => v.print(),
      NodeWrapper::Class(v) => v.print(),
      NodeWrapper::ClassKeyword(v) => v.print(),
      NodeWrapper::Clone(v) => v.print(),
      NodeWrapper::CommentBlock(v) => v.print(),
      NodeWrapper::CommentDoc(v) => v.print(),
      NodeWrapper::CommentLine(v) => v.print(),
      NodeWrapper::Const(v) => v.print(),
      NodeWrapper::ConstProperty(v) => v.print(),
      NodeWrapper::ConstructorParameter(v) => v.print(),
      NodeWrapper::Continue(v) => v.print(),
      NodeWrapper::Declare(v) => v.print(),
      NodeWrapper::DeclareArgument(v) => v.print(),
      NodeWrapper::DoWhile(v) => v.print(),
      NodeWrapper::DoWhileCondition(v) => v.print(),
      NodeWrapper::Echo(v) => v.print(),
      NodeWrapper::Else(v) => v.print(),
      NodeWrapper::Encapsed(v) => v.print(),
      NodeWrapper::EncapsedPart(v) => v.print(),
      NodeWrapper::Enum(v) => v.print(),
      NodeWrapper::EnumItem(v) => v.print(),
      NodeWrapper::Eval(v) => v.print(),
      NodeWrapper::Exit(v) => v.print(),
      NodeWrapper::Finally(v) => v.print(),
      NodeWrapper::For(v) => v.print(),
      NodeWrapper::Foreach(v) => v.print(),
      NodeWrapper::Function(v) => v.print(),
      NodeWrapper::Global(v) => v.print(),
      NodeWrapper::Goto(v) => v.print(),
      NodeWrapper::HereDoc(v) => v.print(),
      NodeWrapper::Identifier(v) => v.print(),
      NodeWrapper::If(v) => v.print(),
      NodeWrapper::Include(v) => v.print(),
      NodeWrapper::Inline(v) => v.print(),
      NodeWrapper::Interface(v) => v.print(),
      NodeWrapper::IntersectionType(v) => v.print(),
      NodeWrapper::Label(v) => v.print(),
      NodeWrapper::List(v) => v.print(),
      NodeWrapper::Magic(v) => v.print(),
      NodeWrapper::MagicMethod(v) => v.print(),
      NodeWrapper::Match(v) => v.print(),
      NodeWrapper::MatchArm(v) => v.print(),
      NodeWrapper::Method(v) => v.print(),
      NodeWrapper::Namespace(v) => v.print(),
      NodeWrapper::Negate(v) => v.print(),
      NodeWrapper::New(v) => v.print(),
      NodeWrapper::NowDoc(v) => v.print(),
      NodeWrapper::Null(v) => v.print(),
      NodeWrapper::Number(v) => v.print(),
      NodeWrapper::ObjectAccess(v) => v.print(),
      NodeWrapper::Parameter(v) => v.print(),
      NodeWrapper::Parent(v) => v.print(),
      NodeWrapper::Parenthesis(v) => v.print(),
      NodeWrapper::Post(v) => v.print(),
      NodeWrapper::Pre(v) => v.print(),
      NodeWrapper::Print(v) => v.print(),
      NodeWrapper::Program(v) => v.print(),
      NodeWrapper::Property(v) => v.print(),
      NodeWrapper::PropertyHook(v) => v.print(),
      NodeWrapper::PropertyItem(v) => v.print(),
      NodeWrapper::Reference(v) => v.print(),
      NodeWrapper::Return(v) => v.print(),
      NodeWrapper::SelfKeyword(v) => v.print(),
      NodeWrapper::Silent(v) => v.print(),
      NodeWrapper::Static(v) => v.print(),
      NodeWrapper::StaticKeyword(v) => v.print(),
      NodeWrapper::StaticLookup(v) => v.print(),
      NodeWrapper::String(v) => v.print(),
      NodeWrapper::Switch(v) => v.print(),
      NodeWrapper::Ternary(v) => v.print(),
      NodeWrapper::This(v) => v.print(),
      NodeWrapper::Trait(v) => v.print(),
      NodeWrapper::TraitUse(v) => v.print(),
      NodeWrapper::TraitUseAlias(v) => v.print(),
      NodeWrapper::TraitUsePrecedence(v) => v.print(),
      NodeWrapper::Throw(v) => v.print(),
      NodeWrapper::Try(v) => v.print(),
      NodeWrapper::Type(v) => v.print(),
      NodeWrapper::UnionType(v) => v.print(),
      NodeWrapper::Use(v) => v.print(),
      NodeWrapper::UseItem(v) => v.print(),
      NodeWrapper::Variable(v) => v.print(),
      NodeWrapper::Variadic(v) => v.print(),
      NodeWrapper::While(v) => v.print(),
      NodeWrapper::Yield(v) => v.print(),
      NodeWrapper::YieldFrom(v) => v.print(),
    };
    let mut node_builder = PrintBuilder::new(PrintType::Object);
    node_builder.push_props(
      true,
      &mut [
        ("leadings", self.leadings.print()),
        ("trailings", self.trailings.print()),
        ("location", self.loc.print()),
      ]
    );
    builder.extend(node_builder);
    builder
  }
}

impl Printable for RangeLocation {
  fn print(&self) -> PrintBuilder {
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
  fn print(&self) -> PrintBuilder {
    let len = self.len();
    if len == 0 {
      let mut builder = PrintBuilder::new(PrintType::Inline);
      builder.shift_new_line("~");
      return builder;
    }
    let mut builder = PrintBuilder::new(PrintType::Vec);
    let last_index = len.saturating_sub(1);
    for (i, node) in self.iter().enumerate() {
      let mut scoped_builder = node.print();
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
  fn print(&self) -> PrintBuilder {
    match self {
      Some(x) => x.print(),
      None => {
        let mut builder = PrintBuilder::new(PrintType::Inline);
        builder.shift_new_line("-");
        builder
      }
    }
  }
}

impl<'arena> Printable for bumpalo::boxed::Box<'arena, Node<'arena>> {
  fn print(&self) -> PrintBuilder {
    self.as_ref().print()
  }
}

impl<T> Printable for Vec<T> where T: Printable {
  fn print(&self) -> PrintBuilder {
    let mut childs = Vec::new();
    for x in self {
      if let Some(last) = x.print().lines.last_mut() {
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
            fn print(&self) -> PrintBuilder {
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
            fn print(&self) -> PrintBuilder {
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
    printer::Printable,
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
    println!("{:?}", node.print());
  }
}
