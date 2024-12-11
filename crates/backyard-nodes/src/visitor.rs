use std::collections::VecDeque;

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

pub trait Visitable<'arena> {
  fn populate_visits<'a>(&'a self) -> VecDeque<&'a Node<'arena>>;
}

impl<'arena, 'a> Iterator for Walker<'arena, 'a> {
  type Item = &'a Node<'arena>;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(node) = self.stack.pop_back() {
      let _ = match &node.wrapper {
        NodeWrapper::AnonymousClass(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::AnonymousFunction(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::CallArgument(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Array(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::ArrayItem(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::ArrayLookup(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::ArrowFunction(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Assignment(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Attribute(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::AttributeItem(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Bin(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Block(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Boolean(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Break(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Call(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Case(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Cast(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Catch(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Class(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::ClassKeyword(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Clone(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::CommentBlock(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::CommentDoc(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::CommentLine(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Const(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::ConstProperty(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::ConstructorParameter(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Continue(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Declare(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::DeclareArgument(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::DoWhile(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::DoWhileCondition(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Echo(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Else(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Encapsed(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::EncapsedPart(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Enum(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::EnumItem(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Eval(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Exit(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Finally(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::For(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Foreach(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Function(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Global(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Goto(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::HereDoc(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Identifier(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::If(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Include(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Inline(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Interface(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::IntersectionType(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Label(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::List(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Magic(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Match(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::MatchArm(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Method(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Namespace(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Negate(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::New(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::NowDoc(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Null(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Number(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::ObjectAccess(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Parameter(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Parent(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Parenthesis(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Post(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Pre(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Print(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Program(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Property(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::PropertyHook(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::PropertyItem(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Reference(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Return(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::SelfKeyword(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Silent(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Static(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::StaticKeyword(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::StaticLookup(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::String(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Switch(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Ternary(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::This(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Trait(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::TraitUse(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::TraitUseAlias(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::TraitUsePrecedence(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Throw(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Try(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Type(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::UnionType(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Use(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::UseItem(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Variable(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Variadic(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::While(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::Yield(v) => self.stack.append(&mut v.populate_visits()),
        NodeWrapper::YieldFrom(v) => self.stack.append(&mut v.populate_visits()),
      };
      Some(node)
    } else {
      None
    }
  }
}

pub trait MapIntoVisitorStack<'arena> {
  fn map_into_visitor_stack<'a>(&'a self, stack: &mut std::collections::VecDeque<&'a Node<'arena>>);
}

impl<'arena> MapIntoVisitorStack<'arena> for bumpalo::collections::Vec<'arena, Node<'arena>> {
  fn map_into_visitor_stack<'a>(
    &'a self,
    stack: &mut std::collections::VecDeque<&'a Node<'arena>>
  ) {
    self.iter().for_each(|x| stack.push_back(x));
  }
}

impl<'arena, T> MapIntoVisitorStack<'arena> for Option<T> where T: MapIntoVisitorStack<'arena> {
  fn map_into_visitor_stack<'a>(
    &'a self,
    stack: &mut std::collections::VecDeque<&'a Node<'arena>>
  ) {
    if let Some(x) = self {
      x.map_into_visitor_stack(stack);
    }
  }
}

impl<'arena> MapIntoVisitorStack<'arena> for bumpalo::boxed::Box<'arena, Node<'arena>> {
  fn map_into_visitor_stack<'a>(
    &'a self,
    stack: &mut std::collections::VecDeque<&'a Node<'arena>>
  ) {
    stack.push_back(self);
  }
}

macro_rules! impl_map_into_visitor_stack {
  ($($t:ty),*) => {
      $(
          impl<'arena> MapIntoVisitorStack<'arena> for $t {
            fn map_into_visitor_stack<'a>(
              &'a self,
              _: &mut std::collections::VecDeque<&'a Node<'arena>>
            ) {}
          }
      )*
  };
}

impl_map_into_visitor_stack!(
  bool,
  CompactString,
  BodyType,
  std::vec::Vec<Visibility>,
  UseItemModifier,
  Visibility,
  Inheritance,
  Quote,
  Modifier
);
