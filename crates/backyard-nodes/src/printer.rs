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
    } else if is_middle {
      "├-"
    } else {
      "└-"
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

impl<'arena> Node<'arena> {
  pub fn print(&self, with_leading_trailing: bool, with_location: bool) -> PrintBuilder {
    let config = PrintConfig::new(with_leading_trailing, with_location);
    self.build_print(&config)
  }
}

// ┴ ┬ ┤ - ┼ ├ └ │
pub(crate) trait Printable {
  fn build_print(&self, config: &PrintConfig) -> PrintBuilder;
}

impl<'arena> Printable for Node<'arena> {
  fn build_print(&self, config: &PrintConfig) -> PrintBuilder {
    let mut builder = match &self.wrapper {
      NodeWrapper::AnonymousClass(v) => v.build_print(config),
      NodeWrapper::AnonymousFunction(v) => v.build_print(config),
      NodeWrapper::Argument(v) => v.build_print(config),
      NodeWrapper::Array(v) => v.build_print(config),
      NodeWrapper::ArrayItem(v) => v.build_print(config),
      NodeWrapper::ArrayLookup(v) => v.build_print(config),
      NodeWrapper::ArrowFunction(v) => v.build_print(config),
      NodeWrapper::Assignment(v) => v.build_print(config),
      NodeWrapper::Attribute(v) => v.build_print(config),
      NodeWrapper::AttributeItem(v) => v.build_print(config),
      NodeWrapper::Bin(v) => v.build_print(config),
      NodeWrapper::Block(v) => v.build_print(config),
      NodeWrapper::Boolean(v) => v.build_print(config),
      NodeWrapper::Break(v) => v.build_print(config),
      NodeWrapper::Call(v) => v.build_print(config),
      NodeWrapper::Case(v) => v.build_print(config),
      NodeWrapper::Cast(v) => v.build_print(config),
      NodeWrapper::Catch(v) => v.build_print(config),
      NodeWrapper::Class(v) => v.build_print(config),
      NodeWrapper::ClassKeyword(v) => v.build_print(config),
      NodeWrapper::Clone(v) => v.build_print(config),
      NodeWrapper::CommentBlock(v) => v.build_print(config),
      NodeWrapper::CommentDoc(v) => v.build_print(config),
      NodeWrapper::CommentLine(v) => v.build_print(config),
      NodeWrapper::Const(v) => v.build_print(config),
      NodeWrapper::ConstProperty(v) => v.build_print(config),
      NodeWrapper::ConstructorParameter(v) => v.build_print(config),
      NodeWrapper::Continue(v) => v.build_print(config),
      NodeWrapper::Declare(v) => v.build_print(config),
      NodeWrapper::DeclareArgument(v) => v.build_print(config),
      NodeWrapper::DoWhile(v) => v.build_print(config),
      NodeWrapper::DoWhileCondition(v) => v.build_print(config),
      NodeWrapper::Echo(v) => v.build_print(config),
      NodeWrapper::Else(v) => v.build_print(config),
      NodeWrapper::Encapsed(v) => v.build_print(config),
      NodeWrapper::EncapsedPart(v) => v.build_print(config),
      NodeWrapper::Enum(v) => v.build_print(config),
      NodeWrapper::EnumItem(v) => v.build_print(config),
      NodeWrapper::Eval(v) => v.build_print(config),
      NodeWrapper::Exit(v) => v.build_print(config),
      NodeWrapper::Finally(v) => v.build_print(config),
      NodeWrapper::For(v) => v.build_print(config),
      NodeWrapper::Foreach(v) => v.build_print(config),
      NodeWrapper::Function(v) => v.build_print(config),
      NodeWrapper::Global(v) => v.build_print(config),
      NodeWrapper::Goto(v) => v.build_print(config),
      NodeWrapper::HaltCompiler(v) => v.build_print(config),
      NodeWrapper::HereDoc(v) => v.build_print(config),
      NodeWrapper::Identifier(v) => v.build_print(config),
      NodeWrapper::If(v) => v.build_print(config),
      NodeWrapper::Include(v) => v.build_print(config),
      NodeWrapper::Inline(v) => v.build_print(config),
      NodeWrapper::Interface(v) => v.build_print(config),
      NodeWrapper::IntersectionType(v) => v.build_print(config),
      NodeWrapper::Label(v) => v.build_print(config),
      NodeWrapper::List(v) => v.build_print(config),
      NodeWrapper::Magic(v) => v.build_print(config),
      NodeWrapper::MagicMethod(v) => v.build_print(config),
      NodeWrapper::Match(v) => v.build_print(config),
      NodeWrapper::MatchArm(v) => v.build_print(config),
      NodeWrapper::Method(v) => v.build_print(config),
      NodeWrapper::Namespace(v) => v.build_print(config),
      NodeWrapper::Negate(v) => v.build_print(config),
      NodeWrapper::New(v) => v.build_print(config),
      NodeWrapper::NowDoc(v) => v.build_print(config),
      NodeWrapper::Null(v) => v.build_print(config),
      NodeWrapper::Number(v) => v.build_print(config),
      NodeWrapper::ObjectAccess(v) => v.build_print(config),
      NodeWrapper::Parameter(v) => v.build_print(config),
      NodeWrapper::Parent(v) => v.build_print(config),
      NodeWrapper::Parenthesis(v) => v.build_print(config),
      NodeWrapper::Post(v) => v.build_print(config),
      NodeWrapper::Pre(v) => v.build_print(config),
      NodeWrapper::Print(v) => v.build_print(config),
      NodeWrapper::Program(v) => v.build_print(config),
      NodeWrapper::Property(v) => v.build_print(config),
      NodeWrapper::PropertyHook(v) => v.build_print(config),
      NodeWrapper::PropertyItem(v) => v.build_print(config),
      NodeWrapper::Reference(v) => v.build_print(config),
      NodeWrapper::Return(v) => v.build_print(config),
      NodeWrapper::SelfKeyword(v) => v.build_print(config),
      NodeWrapper::Silent(v) => v.build_print(config),
      NodeWrapper::Static(v) => v.build_print(config),
      NodeWrapper::StaticKeyword(v) => v.build_print(config),
      NodeWrapper::StaticLookup(v) => v.build_print(config),
      NodeWrapper::String(v) => v.build_print(config),
      NodeWrapper::Switch(v) => v.build_print(config),
      NodeWrapper::Ternary(v) => v.build_print(config),
      NodeWrapper::This(v) => v.build_print(config),
      NodeWrapper::Trait(v) => v.build_print(config),
      NodeWrapper::TraitUse(v) => v.build_print(config),
      NodeWrapper::TraitUseAlias(v) => v.build_print(config),
      NodeWrapper::TraitUsePrecedence(v) => v.build_print(config),
      NodeWrapper::Throw(v) => v.build_print(config),
      NodeWrapper::Try(v) => v.build_print(config),
      NodeWrapper::Type(v) => v.build_print(config),
      NodeWrapper::UnionType(v) => v.build_print(config),
      NodeWrapper::Use(v) => v.build_print(config),
      NodeWrapper::UseItem(v) => v.build_print(config),
      NodeWrapper::Variable(v) => v.build_print(config),
      NodeWrapper::Variadic(v) => v.build_print(config),
      NodeWrapper::While(v) => v.build_print(config),
      NodeWrapper::Yield(v) => v.build_print(config),
      NodeWrapper::YieldFrom(v) => v.build_print(config),
    };
    if config.with_leading_trailing || config.with_location {
      let mut node_builder = PrintBuilder::new(PrintType::Object);
      let mut props = vec![];
      if config.with_leading_trailing {
        props.push(("leadings", self.leadings.build_print(config)));
        props.push(("trailings", self.trailings.build_print(config)));
      }
      if config.with_location {
        props.push(("location", self.loc.build_print(config)));
      }
      node_builder.push_props(true, props.as_mut_slice());
      builder.extend(node_builder);
    }
    builder
  }
}

impl Printable for RangeLocation {
  fn build_print(&self, _: &PrintConfig) -> PrintBuilder {
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
  fn build_print(&self, config: &PrintConfig) -> PrintBuilder {
    let len = self.len();
    if len == 0 {
      return PrintBuilder::new(PrintType::Vec);
    }
    let mut builder = PrintBuilder::new(PrintType::Vec);
    let last_index = len.saturating_sub(1);
    for (i, node) in self.iter().enumerate() {
      let mut scoped_builder = node.build_print(config);
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
  fn build_print(&self, config: &PrintConfig) -> PrintBuilder {
    match self {
      Some(x) => x.build_print(config),
      None => {
        let mut builder = PrintBuilder::new(PrintType::Inline);
        builder.shift_new_line("-");
        builder
      }
    }
  }
}

impl<'arena> Printable for bumpalo::boxed::Box<'arena, Node<'arena>> {
  fn build_print(&self, config: &PrintConfig) -> PrintBuilder {
    self.as_ref().build_print(config)
  }
}

impl<T> Printable for Vec<T> where T: Printable {
  fn build_print(&self, config: &PrintConfig) -> PrintBuilder {
    let mut childs = Vec::new();
    for x in self {
      if let Some(last) = x.build_print(config).lines.last_mut() {
        childs.push(last.to_owned());
      }
    }
    let mut builder = PrintBuilder::new(PrintType::Inline);
    builder.shift_new_line(childs.join(" | ").as_str());
    builder
  }
}

macro_rules! impl_build_printableable {
  ($($t:ty),*) => {
      $(
          impl Printable for $t {
            fn build_print(&self, _: &PrintConfig) -> PrintBuilder {
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
            fn build_print(&self, _: &PrintConfig) -> PrintBuilder {
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
    println!("{:?}", node.build_print(&PrintConfig::new(false, true)));
  }
}
