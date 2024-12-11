use bumpalo::{ Bump, boxed::Box, collections::Vec };
use compact_str::CompactString;
use crate::node::{
  BodyType,
  Inheritance,
  Modifier,
  Node,
  NodeWrapper,
  Quote,
  UseItemModifier,
  Visibility,
};

pub trait IntoBoxedNode<'arena> {
  fn into_boxed(self, arena: &'arena Bump) -> Box<'arena, Node<'arena>>;
}

impl<'arena> IntoBoxedNode<'arena> for Node<'arena> {
  #[inline]
  fn into_boxed(self, arena: &'arena Bump) -> Box<'arena, Node<'arena>> {
    Box::new_in(self, arena)
  }
}

pub trait IntoBoxedOptionNode<'arena> {
  fn into_boxed(self, arena: &'arena Bump) -> Option<Box<'arena, Node<'arena>>>;
}

impl<'arena> IntoBoxedOptionNode<'arena> for Option<Node<'arena>> {
  #[inline]
  fn into_boxed(self, arena: &'arena Bump) -> Option<Box<'arena, Node<'arena>>> {
    self.map(|x| Box::new_in(x, arena))
  }
}

pub trait CloneIn<'arena>: Sized {
  type Cloned: ?Sized;

  fn clone_in(&self, arena: &'arena Bump) -> Self::Cloned;
}

impl<'arena, T, C> CloneIn<'arena> for Option<T> where T: CloneIn<'arena, Cloned = C> {
  type Cloned = Option<C>;

  fn clone_in(&self, arena: &'arena Bump) -> Self::Cloned {
    self.as_ref().map(|it| it.clone_in(arena))
  }
}

impl<'arena> CloneIn<'arena> for Node<'_> {
  type Cloned = Node<'arena>;

  #[inline]
  fn clone_in(&self, arena: &'arena Bump) -> Self::Cloned {
    Node::new(self.node_type.clone(), self.wrapper.clone_in(arena), self.loc.clone())
  }
}

impl<'arena> CloneIn<'arena> for NodeWrapper<'_> {
  type Cloned = NodeWrapper<'arena>;

  #[inline]
  fn clone_in(&self, arena: &'arena Bump) -> Self::Cloned {
    match self {
      NodeWrapper::AnonymousClass(v) => NodeWrapper::AnonymousClass(v.clone_in(arena)),
      NodeWrapper::AnonymousFunction(v) => NodeWrapper::AnonymousFunction(v.clone_in(arena)),
      NodeWrapper::CallArgument(v) => NodeWrapper::CallArgument(v.clone_in(arena)),
      NodeWrapper::Array(v) => NodeWrapper::Array(v.clone_in(arena)),
      NodeWrapper::ArrayItem(v) => NodeWrapper::ArrayItem(v.clone_in(arena)),
      NodeWrapper::ArrayLookup(v) => NodeWrapper::ArrayLookup(v.clone_in(arena)),
      NodeWrapper::ArrowFunction(v) => NodeWrapper::ArrowFunction(v.clone_in(arena)),
      NodeWrapper::Assignment(v) => NodeWrapper::Assignment(v.clone_in(arena)),
      NodeWrapper::Attribute(v) => NodeWrapper::Attribute(v.clone_in(arena)),
      NodeWrapper::AttributeItem(v) => NodeWrapper::AttributeItem(v.clone_in(arena)),
      NodeWrapper::Bin(v) => NodeWrapper::Bin(v.clone_in(arena)),
      NodeWrapper::Block(v) => NodeWrapper::Block(v.clone_in(arena)),
      NodeWrapper::Boolean(v) => NodeWrapper::Boolean(v.clone_in(arena)),
      NodeWrapper::Break(v) => NodeWrapper::Break(v.clone_in(arena)),
      NodeWrapper::Call(v) => NodeWrapper::Call(v.clone_in(arena)),
      NodeWrapper::Case(v) => NodeWrapper::Case(v.clone_in(arena)),
      NodeWrapper::Cast(v) => NodeWrapper::Cast(v.clone_in(arena)),
      NodeWrapper::Catch(v) => NodeWrapper::Catch(v.clone_in(arena)),
      NodeWrapper::Class(v) => NodeWrapper::Class(v.clone_in(arena)),
      NodeWrapper::ClassKeyword(v) => NodeWrapper::ClassKeyword(v.clone_in(arena)),
      NodeWrapper::Clone(v) => NodeWrapper::Clone(v.clone_in(arena)),
      NodeWrapper::CommentBlock(v) => NodeWrapper::CommentBlock(v.clone_in(arena)),
      NodeWrapper::CommentDoc(v) => NodeWrapper::CommentDoc(v.clone_in(arena)),
      NodeWrapper::CommentLine(v) => NodeWrapper::CommentLine(v.clone_in(arena)),
      NodeWrapper::Const(v) => NodeWrapper::Const(v.clone_in(arena)),
      NodeWrapper::ConstProperty(v) => NodeWrapper::ConstProperty(v.clone_in(arena)),
      NodeWrapper::ConstructorParameter(v) => NodeWrapper::ConstructorParameter(v.clone_in(arena)),
      NodeWrapper::Continue(v) => NodeWrapper::Continue(v.clone_in(arena)),
      NodeWrapper::Declare(v) => NodeWrapper::Declare(v.clone_in(arena)),
      NodeWrapper::DeclareArgument(v) => NodeWrapper::DeclareArgument(v.clone_in(arena)),
      NodeWrapper::DoWhile(v) => NodeWrapper::DoWhile(v.clone_in(arena)),
      NodeWrapper::DoWhileCondition(v) => NodeWrapper::DoWhileCondition(v.clone_in(arena)),
      NodeWrapper::Echo(v) => NodeWrapper::Echo(v.clone_in(arena)),
      NodeWrapper::Else(v) => NodeWrapper::Else(v.clone_in(arena)),
      NodeWrapper::Encapsed(v) => NodeWrapper::Encapsed(v.clone_in(arena)),
      NodeWrapper::EncapsedPart(v) => NodeWrapper::EncapsedPart(v.clone_in(arena)),
      NodeWrapper::Enum(v) => NodeWrapper::Enum(v.clone_in(arena)),
      NodeWrapper::EnumItem(v) => NodeWrapper::EnumItem(v.clone_in(arena)),
      NodeWrapper::Eval(v) => NodeWrapper::Eval(v.clone_in(arena)),
      NodeWrapper::Exit(v) => NodeWrapper::Exit(v.clone_in(arena)),
      NodeWrapper::Finally(v) => NodeWrapper::Finally(v.clone_in(arena)),
      NodeWrapper::For(v) => NodeWrapper::For(v.clone_in(arena)),
      NodeWrapper::Foreach(v) => NodeWrapper::Foreach(v.clone_in(arena)),
      NodeWrapper::Function(v) => NodeWrapper::Function(v.clone_in(arena)),
      NodeWrapper::Global(v) => NodeWrapper::Global(v.clone_in(arena)),
      NodeWrapper::Goto(v) => NodeWrapper::Goto(v.clone_in(arena)),
      NodeWrapper::HereDoc(v) => NodeWrapper::HereDoc(v.clone_in(arena)),
      NodeWrapper::Identifier(v) => NodeWrapper::Identifier(v.clone_in(arena)),
      NodeWrapper::If(v) => NodeWrapper::If(v.clone_in(arena)),
      NodeWrapper::Include(v) => NodeWrapper::Include(v.clone_in(arena)),
      NodeWrapper::Inline(v) => NodeWrapper::Inline(v.clone_in(arena)),
      NodeWrapper::Interface(v) => NodeWrapper::Interface(v.clone_in(arena)),
      NodeWrapper::IntersectionType(v) => NodeWrapper::IntersectionType(v.clone_in(arena)),
      NodeWrapper::Label(v) => NodeWrapper::Label(v.clone_in(arena)),
      NodeWrapper::List(v) => NodeWrapper::List(v.clone_in(arena)),
      NodeWrapper::Magic(v) => NodeWrapper::Magic(v.clone_in(arena)),
      NodeWrapper::Match(v) => NodeWrapper::Match(v.clone_in(arena)),
      NodeWrapper::MatchArm(v) => NodeWrapper::MatchArm(v.clone_in(arena)),
      NodeWrapper::Method(v) => NodeWrapper::Method(v.clone_in(arena)),
      NodeWrapper::Namespace(v) => NodeWrapper::Namespace(v.clone_in(arena)),
      NodeWrapper::Negate(v) => NodeWrapper::Negate(v.clone_in(arena)),
      NodeWrapper::New(v) => NodeWrapper::New(v.clone_in(arena)),
      NodeWrapper::NowDoc(v) => NodeWrapper::NowDoc(v.clone_in(arena)),
      NodeWrapper::Null(v) => NodeWrapper::Null(v.clone_in(arena)),
      NodeWrapper::Number(v) => NodeWrapper::Number(v.clone_in(arena)),
      NodeWrapper::ObjectAccess(v) => NodeWrapper::ObjectAccess(v.clone_in(arena)),
      NodeWrapper::Parameter(v) => NodeWrapper::Parameter(v.clone_in(arena)),
      NodeWrapper::Parent(v) => NodeWrapper::Parent(v.clone_in(arena)),
      NodeWrapper::Parenthesis(v) => NodeWrapper::Parenthesis(v.clone_in(arena)),
      NodeWrapper::Post(v) => NodeWrapper::Post(v.clone_in(arena)),
      NodeWrapper::Pre(v) => NodeWrapper::Pre(v.clone_in(arena)),
      NodeWrapper::Print(v) => NodeWrapper::Print(v.clone_in(arena)),
      NodeWrapper::Program(v) => NodeWrapper::Program(v.clone_in(arena)),
      NodeWrapper::Property(v) => NodeWrapper::Property(v.clone_in(arena)),
      NodeWrapper::PropertyHook(v) => NodeWrapper::PropertyHook(v.clone_in(arena)),
      NodeWrapper::PropertyItem(v) => NodeWrapper::PropertyItem(v.clone_in(arena)),
      NodeWrapper::Reference(v) => NodeWrapper::Reference(v.clone_in(arena)),
      NodeWrapper::Return(v) => NodeWrapper::Return(v.clone_in(arena)),
      NodeWrapper::SelfKeyword(v) => NodeWrapper::SelfKeyword(v.clone_in(arena)),
      NodeWrapper::Silent(v) => NodeWrapper::Silent(v.clone_in(arena)),
      NodeWrapper::Static(v) => NodeWrapper::Static(v.clone_in(arena)),
      NodeWrapper::StaticKeyword(v) => NodeWrapper::StaticKeyword(v.clone_in(arena)),
      NodeWrapper::StaticLookup(v) => NodeWrapper::StaticLookup(v.clone_in(arena)),
      NodeWrapper::String(v) => NodeWrapper::String(v.clone_in(arena)),
      NodeWrapper::Switch(v) => NodeWrapper::Switch(v.clone_in(arena)),
      NodeWrapper::Ternary(v) => NodeWrapper::Ternary(v.clone_in(arena)),
      NodeWrapper::This(v) => NodeWrapper::This(v.clone_in(arena)),
      NodeWrapper::Trait(v) => NodeWrapper::Trait(v.clone_in(arena)),
      NodeWrapper::TraitUse(v) => NodeWrapper::TraitUse(v.clone_in(arena)),
      NodeWrapper::TraitUseAlias(v) => NodeWrapper::TraitUseAlias(v.clone_in(arena)),
      NodeWrapper::TraitUsePrecedence(v) => NodeWrapper::TraitUsePrecedence(v.clone_in(arena)),
      NodeWrapper::Throw(v) => NodeWrapper::Throw(v.clone_in(arena)),
      NodeWrapper::Try(v) => NodeWrapper::Try(v.clone_in(arena)),
      NodeWrapper::Type(v) => NodeWrapper::Type(v.clone_in(arena)),
      NodeWrapper::UnionType(v) => NodeWrapper::UnionType(v.clone_in(arena)),
      NodeWrapper::Use(v) => NodeWrapper::Use(v.clone_in(arena)),
      NodeWrapper::UseItem(v) => NodeWrapper::UseItem(v.clone_in(arena)),
      NodeWrapper::Variable(v) => NodeWrapper::Variable(v.clone_in(arena)),
      NodeWrapper::Variadic(v) => NodeWrapper::Variadic(v.clone_in(arena)),
      NodeWrapper::While(v) => NodeWrapper::While(v.clone_in(arena)),
      NodeWrapper::Yield(v) => NodeWrapper::Yield(v.clone_in(arena)),
      NodeWrapper::YieldFrom(v) => NodeWrapper::YieldFrom(v.clone_in(arena)),
    }
  }
}

impl<'arena, T, C: 'arena> CloneIn<'arena> for Box<'_, T> where T: CloneIn<'arena, Cloned = C> {
  type Cloned = Box<'arena, C>;

  #[inline]
  fn clone_in(&self, arena: &'arena Bump) -> Self::Cloned {
    Box::new_in(self.as_ref().clone_in(arena), arena)
  }
}

impl<'arena, T, C: 'arena> CloneIn<'arena> for Vec<'_, T> where T: CloneIn<'arena, Cloned = C> {
  type Cloned = Vec<'arena, C>;

  #[inline]
  fn clone_in(&self, arena: &'arena Bump) -> Self::Cloned {
    Vec::from_iter_in(
      self.iter().map(|it| it.clone_in(arena)),
      arena
    )
  }
}

macro_rules! impl_clone_in_clone {
  ($($t:ty),*) => {
      $(
          impl<'arena> CloneIn<'arena> for $t {
              type Cloned = Self;
              
              #[inline(always)]
              fn clone_in(&self, _: &'arena Bump) -> Self {
                  self.clone()
              }
          }
      )*
  };
}

impl_clone_in_clone!(
  UseItemModifier,
  Modifier,
  Quote,
  Inheritance,
  Visibility,
  BodyType,
  CompactString,
  std::vec::Vec<Visibility>
);

macro_rules! impl_clone_in {
  ($($t:ty),*) => {
      $(
          impl<'arena> CloneIn<'arena> for $t {
              type Cloned = Self;
              
              #[inline(always)]
              fn clone_in(&self, _: &'arena Bump) -> Self {
                  *self
              }
          }
      )*
  };
}

impl_clone_in!(
  usize,
  u8,
  u16,
  u32,
  u64,
  u128,
  isize,
  i8,
  i16,
  i32,
  i64,
  i128,
  f32,
  f64,
  bool,
  char
);
