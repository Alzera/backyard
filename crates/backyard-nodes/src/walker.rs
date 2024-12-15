use std::collections::VecDeque;

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
  UseItemModifier,
  Visibility,
};

pub struct Walker<'arena, 'a> {
  stack: VecDeque<&'a Node<'arena>>,
}

impl<'arena, 'a> Walker<'arena, 'a> {
  pub fn new(root: &'a Node<'arena>) -> Self {
    let mut stack = VecDeque::new();
    stack.push_back(root);
    Self { stack }
  }
}

pub trait Walkable<'arena> {
  fn populate_walks<'a>(&'a self) -> VecDeque<&'a Node<'arena>>;
}

impl<'arena, 'a> Iterator for Walker<'arena, 'a> {
  type Item = &'a Node<'arena>;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(node) = self.stack.pop_back() {
      let _ = match &node.wrapper {
        NodeWrapper::AnonymousClass(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::AnonymousFunction(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::CallArgument(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Array(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::ArrayItem(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::ArrayLookup(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::ArrowFunction(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Assignment(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Attribute(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::AttributeItem(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Bin(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Block(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Boolean(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Break(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Call(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Case(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Cast(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Catch(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Class(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::ClassKeyword(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Clone(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::CommentBlock(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::CommentDoc(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::CommentLine(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Const(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::ConstProperty(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::ConstructorParameter(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Continue(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Declare(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::DeclareArgument(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::DoWhile(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::DoWhileCondition(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Echo(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Else(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Encapsed(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::EncapsedPart(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Enum(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::EnumItem(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Eval(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Exit(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Finally(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::For(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Foreach(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Function(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Global(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Goto(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::HereDoc(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Identifier(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::If(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Include(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Inline(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Interface(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::IntersectionType(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Label(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::List(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Magic(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::MagicMethod(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Match(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::MatchArm(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Method(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Namespace(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Negate(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::New(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::NowDoc(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Null(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Number(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::ObjectAccess(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Parameter(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Parent(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Parenthesis(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Post(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Pre(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Print(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Program(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Property(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::PropertyHook(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::PropertyItem(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Reference(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Return(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::SelfKeyword(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Silent(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Static(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::StaticKeyword(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::StaticLookup(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::String(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Switch(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Ternary(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::This(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Trait(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::TraitUse(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::TraitUseAlias(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::TraitUsePrecedence(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Throw(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Try(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Type(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::UnionType(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Use(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::UseItem(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Variable(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Variadic(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::While(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::Yield(v) => self.stack.append(&mut v.populate_walks()),
        NodeWrapper::YieldFrom(v) => self.stack.append(&mut v.populate_walks()),
      };
      Some(node)
    } else {
      None
    }
  }
}

pub trait MapIntoWalkerStack<'arena> {
  fn map_into_walker_stack<'a>(&'a self, stack: &mut std::collections::VecDeque<&'a Node<'arena>>);
}

impl<'arena> MapIntoWalkerStack<'arena> for bumpalo::collections::Vec<'arena, Node<'arena>> {
  fn map_into_walker_stack<'a>(&'a self, stack: &mut std::collections::VecDeque<&'a Node<'arena>>) {
    self.iter().for_each(|x| stack.push_back(x));
  }
}

impl<'arena, T> MapIntoWalkerStack<'arena> for Option<T> where T: MapIntoWalkerStack<'arena> {
  fn map_into_walker_stack<'a>(&'a self, stack: &mut std::collections::VecDeque<&'a Node<'arena>>) {
    if let Some(x) = self {
      x.map_into_walker_stack(stack);
    }
  }
}

impl<'arena> MapIntoWalkerStack<'arena> for bumpalo::boxed::Box<'arena, Node<'arena>> {
  fn map_into_walker_stack<'a>(&'a self, stack: &mut std::collections::VecDeque<&'a Node<'arena>>) {
    stack.push_back(self);
  }
}

macro_rules! impl_map_into_walker_stack {
  ($($t:ty),*) => {
      $(
          impl<'arena> MapIntoWalkerStack<'arena> for $t {
            fn map_into_walker_stack<'a>(
              &'a self,
              _: &mut std::collections::VecDeque<&'a Node<'arena>>
            ) {}
          }
      )*
  };
}

impl_map_into_walker_stack!(
  bool,
  BString,
  BodyType,
  std::vec::Vec<Visibility>,
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
  use crate::{ builder::{ BlueprintBuildable, Builder }, walker::Walker, AssignmentType, NodeType };

  #[test]
  fn walker() {
    let arena = bumpalo::Bump::new();
    let b = Builder::new();
    let node = b
      .Program(
        &[b.Assignment(b.Variable(b.Identifier("a")), AssignmentType::Default, b.Number("21"))]
      )
      .build(&arena);
    let mut walker = Walker::new(&*node).into_iter();

    assert_eq!(NodeType::Program, walker.next().unwrap().node_type);
    assert_eq!(NodeType::Assignment, walker.next().unwrap().node_type);
    assert_eq!(NodeType::Variable, walker.next().unwrap().node_type);
    assert_eq!(NodeType::Identifier, walker.next().unwrap().node_type);
    assert_eq!(NodeType::Number, walker.next().unwrap().node_type);
    assert!(walker.next().is_none());
  }
}
