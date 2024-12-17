use std::{ collections::VecDeque, iter::{ Chain, Rev }, slice::Iter };

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

pub struct Explorer<'arena, 'a> {
  ancestors: Vec<&'a Node<'arena>>,
  prev_siblings: Vec<&'a Node<'arena>>,
  next_siblings: Vec<&'a Node<'arena>>,
}

impl<'arena, 'a> Explorer<'arena, 'a> {
  pub fn ancestors(&self) -> Iter<'_, &'a Node<'arena>> {
    self.ancestors.iter()
  }

  pub fn prev_siblings(&self) -> Iter<'_, &'a Node<'arena>> {
    self.prev_siblings.iter()
  }

  pub fn next_siblings(&self) -> Iter<'_, &'a Node<'arena>> {
    self.next_siblings.iter()
  }

  pub fn siblings(&self) -> Chain<Rev<Iter<'_, &'a Node<'arena>>>, Iter<'_, &'a Node<'arena>>> {
    self.prev_siblings.iter().rev().chain(self.next_siblings.iter())
  }
}

#[derive(Debug, Clone)]
pub(crate) struct WalkerItem<'arena, 'a> {
  pub node: &'a Node<'arena>,
  pub is_vec: bool,
  pub level: u16,
}

pub struct Walker<'arena, 'a> {
  ancestors: Vec<WalkerItem<'arena, 'a>>,
  stack: VecDeque<WalkerItem<'arena, 'a>>,
}

impl<'arena, 'a> Walker<'arena, 'a> {
  pub(crate) fn new(root: &'a Node<'arena>) -> Self {
    let mut stack = VecDeque::new();
    stack.push_back(WalkerItem { node: root, is_vec: false, level: 0 });
    Self { ancestors: vec![], stack }
  }

  fn get_ancestors(&self, level: u16) -> Vec<&'a Node<'arena>> {
    let mut level = level.saturating_sub(1);
    let mut ancestors = vec![];
    for i in self.ancestors.iter() {
      if i.level == level {
        ancestors.push(i.node);
        level = level.saturating_sub(1);
      }
    }
    ancestors
  }

  fn get_siblings(&self, level: u16) -> (Vec<&'a Node<'arena>>, Vec<&'a Node<'arena>>) {
    let mut prevs = vec![];
    let end_level = level.saturating_sub(1);
    for i in self.ancestors.iter().rev() {
      if i.level == end_level {
        break;
      } else if i.level == level {
        prevs.push(i.node);
      }
    }
    let mut nexts = vec![];
    for i in self.stack.iter() {
      if i.level == level {
        nexts.push(i.node);
      }
    }
    (prevs, nexts)
  }
}

impl<'arena> Node<'arena> {
  pub fn walk(&self) -> Walker<'arena, '_> {
    Walker::new(self)
  }
}

pub(crate) trait Walkable<'arena> {
  fn populate_walks<'a>(&'a self, stack: &mut VecDeque<WalkerItem<'arena, 'a>>, level: u16);
}

impl<'arena, 'a> Iterator for Walker<'arena, 'a> {
  type Item = (Explorer<'arena, 'a>, &'a Node<'arena>);

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(item) = self.stack.pop_back() {
      match &item.node.wrapper {
        NodeWrapper::AnonymousClass(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::AnonymousFunction(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Argument(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Array(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::ArrayItem(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::ArrayLookup(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::ArrowFunction(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Assignment(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Attribute(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::AttributeItem(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Bin(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Block(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Boolean(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Break(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Call(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Case(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Cast(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Catch(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Class(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::ClassKeyword(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Clone(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::CommentBlock(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::CommentDoc(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::CommentLine(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Const(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::ConstProperty(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::ConstructorParameter(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Continue(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Declare(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::DeclareArgument(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::DoWhile(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::DoWhileCondition(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Echo(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Else(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Encapsed(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::EncapsedPart(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Enum(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::EnumItem(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Eval(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Exit(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Finally(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::For(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Foreach(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Function(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Global(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Goto(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::HaltCompiler(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::HereDoc(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Identifier(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::If(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Include(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Inline(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Interface(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::IntersectionType(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Label(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::List(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Magic(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::MagicMethod(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Match(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::MatchArm(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Method(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Namespace(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Negate(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::New(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::NowDoc(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Null(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Number(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::ObjectAccess(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Parameter(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Parent(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Parenthesis(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Post(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Pre(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Print(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Program(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Property(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::PropertyHook(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::PropertyItem(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Reference(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Return(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::SelfKeyword(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Silent(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Static(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::StaticKeyword(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::StaticLookup(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::String(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Switch(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Ternary(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::This(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Trait(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::TraitUse(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::TraitUseAlias(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::TraitUsePrecedence(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Throw(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Try(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Type(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::UnionType(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Use(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::UseItem(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Variable(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Variadic(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::While(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::Yield(v) => v.populate_walks(&mut self.stack, item.level),
        NodeWrapper::YieldFrom(v) => v.populate_walks(&mut self.stack, item.level),
      }
      let node = item.node;
      let level = item.level;
      let (prev_siblings, next_siblings) = if item.is_vec {
        self.get_siblings(level)
      } else {
        (vec![], vec![])
      };
      self.ancestors.push(item);
      Some((Explorer { ancestors: self.get_ancestors(level), prev_siblings, next_siblings }, node))
    } else {
      None
    }
  }
}

impl<'arena> Walkable<'arena> for bumpalo::collections::Vec<'arena, Node<'arena>> {
  fn populate_walks<'a>(&'a self, stack: &mut VecDeque<WalkerItem<'arena, 'a>>, level: u16) {
    self.iter().for_each(|x| stack.push_back(WalkerItem { node: x, is_vec: true, level }));
  }
}

impl<'arena, T> Walkable<'arena> for Option<T> where T: Walkable<'arena> {
  fn populate_walks<'a>(&'a self, stack: &mut VecDeque<WalkerItem<'arena, 'a>>, level: u16) {
    if let Some(x) = self {
      x.populate_walks(stack, level);
    }
  }
}

impl<'arena> Walkable<'arena> for bumpalo::boxed::Box<'arena, Node<'arena>> {
  fn populate_walks<'a>(&'a self, stack: &mut VecDeque<WalkerItem<'arena, 'a>>, level: u16) {
    stack.push_back(WalkerItem { node: self, is_vec: false, level });
  }
}

macro_rules! impl_map_into_walker_stack {
  ($($t:ty),*) => {
      $(
          impl<'arena> Walkable<'arena> for $t {
            fn populate_walks<'a>(&'a self, _: &mut VecDeque<WalkerItem<'arena, 'a>>, _: u16) {}
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
  use crate::{ builder::{ BlueprintBuildable, Builder }, AssignmentType, NodeType };

  #[test]
  fn walker() {
    let arena = bumpalo::Bump::new();
    let b = Builder::new();
    let node = b
      .Program(
        &[b.Assignment(b.Variable(b.Identifier("a")), AssignmentType::Default, b.Number("21"))]
      )
      .build(&arena);
    let mut walker = node.walk();

    assert_eq!(NodeType::Program, walker.next().unwrap().1.node_type);
    assert_eq!(NodeType::Assignment, walker.next().unwrap().1.node_type);
    assert_eq!(NodeType::Variable, walker.next().unwrap().1.node_type);
    assert_eq!(NodeType::Identifier, walker.next().unwrap().1.node_type);
    assert_eq!(NodeType::Number, walker.next().unwrap().1.node_type);
    assert!(walker.next().is_none());
  }
}
