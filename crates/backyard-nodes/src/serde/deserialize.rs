use bumpalo::Bump;
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

use super::{ SerializableNode, SerializableNodeWrapper };

pub(crate) trait Deserializable<'arena> {
  type Return: ?Sized;
  fn to_deserialize(&self, arena: &'arena Bump) -> Self::Return;
}

impl<'arena> Deserializable<'arena> for SerializableNode {
  type Return = Node<'arena>;

  fn to_deserialize(&self, arena: &'arena Bump) -> Self::Return {
    let wrapper = match &self.wrapper {
      SerializableNodeWrapper::AnonymousClass(v) =>
        NodeWrapper::AnonymousClass(v.to_deserialize(arena)),
      SerializableNodeWrapper::AnonymousFunction(v) =>
        NodeWrapper::AnonymousFunction(v.to_deserialize(arena)),
      SerializableNodeWrapper::Argument(v) => NodeWrapper::Argument(v.to_deserialize(arena)),
      SerializableNodeWrapper::Array(v) => NodeWrapper::Array(v.to_deserialize(arena)),
      SerializableNodeWrapper::ArrayItem(v) => NodeWrapper::ArrayItem(v.to_deserialize(arena)),
      SerializableNodeWrapper::ArrayLookup(v) => NodeWrapper::ArrayLookup(v.to_deserialize(arena)),
      SerializableNodeWrapper::ArrowFunction(v) =>
        NodeWrapper::ArrowFunction(v.to_deserialize(arena)),
      SerializableNodeWrapper::Assignment(v) => NodeWrapper::Assignment(v.to_deserialize(arena)),
      SerializableNodeWrapper::Attribute(v) => NodeWrapper::Attribute(v.to_deserialize(arena)),
      SerializableNodeWrapper::AttributeItem(v) =>
        NodeWrapper::AttributeItem(v.to_deserialize(arena)),
      SerializableNodeWrapper::Bin(v) => NodeWrapper::Bin(v.to_deserialize(arena)),
      SerializableNodeWrapper::Block(v) => NodeWrapper::Block(v.to_deserialize(arena)),
      SerializableNodeWrapper::Boolean(v) => NodeWrapper::Boolean(v.to_deserialize(arena)),
      SerializableNodeWrapper::Break(v) => NodeWrapper::Break(v.to_deserialize(arena)),
      SerializableNodeWrapper::Call(v) => NodeWrapper::Call(v.to_deserialize(arena)),
      SerializableNodeWrapper::Case(v) => NodeWrapper::Case(v.to_deserialize(arena)),
      SerializableNodeWrapper::Cast(v) => NodeWrapper::Cast(v.to_deserialize(arena)),
      SerializableNodeWrapper::Catch(v) => NodeWrapper::Catch(v.to_deserialize(arena)),
      SerializableNodeWrapper::Class(v) => NodeWrapper::Class(v.to_deserialize(arena)),
      SerializableNodeWrapper::ClassKeyword(v) =>
        NodeWrapper::ClassKeyword(v.to_deserialize(arena)),
      SerializableNodeWrapper::Clone(v) => NodeWrapper::Clone(v.to_deserialize(arena)),
      SerializableNodeWrapper::CommentBlock(v) =>
        NodeWrapper::CommentBlock(v.to_deserialize(arena)),
      SerializableNodeWrapper::CommentDoc(v) => NodeWrapper::CommentDoc(v.to_deserialize(arena)),
      SerializableNodeWrapper::CommentLine(v) => NodeWrapper::CommentLine(v.to_deserialize(arena)),
      SerializableNodeWrapper::Const(v) => NodeWrapper::Const(v.to_deserialize(arena)),
      SerializableNodeWrapper::ConstProperty(v) =>
        NodeWrapper::ConstProperty(v.to_deserialize(arena)),
      SerializableNodeWrapper::ConstructorParameter(v) =>
        NodeWrapper::ConstructorParameter(v.to_deserialize(arena)),
      SerializableNodeWrapper::Continue(v) => NodeWrapper::Continue(v.to_deserialize(arena)),
      SerializableNodeWrapper::Declare(v) => NodeWrapper::Declare(v.to_deserialize(arena)),
      SerializableNodeWrapper::DeclareArgument(v) =>
        NodeWrapper::DeclareArgument(v.to_deserialize(arena)),
      SerializableNodeWrapper::DoWhile(v) => NodeWrapper::DoWhile(v.to_deserialize(arena)),
      SerializableNodeWrapper::DoWhileCondition(v) =>
        NodeWrapper::DoWhileCondition(v.to_deserialize(arena)),
      SerializableNodeWrapper::Echo(v) => NodeWrapper::Echo(v.to_deserialize(arena)),
      SerializableNodeWrapper::Else(v) => NodeWrapper::Else(v.to_deserialize(arena)),
      SerializableNodeWrapper::Encapsed(v) => NodeWrapper::Encapsed(v.to_deserialize(arena)),
      SerializableNodeWrapper::EncapsedPart(v) =>
        NodeWrapper::EncapsedPart(v.to_deserialize(arena)),
      SerializableNodeWrapper::Enum(v) => NodeWrapper::Enum(v.to_deserialize(arena)),
      SerializableNodeWrapper::EnumItem(v) => NodeWrapper::EnumItem(v.to_deserialize(arena)),
      SerializableNodeWrapper::Eval(v) => NodeWrapper::Eval(v.to_deserialize(arena)),
      SerializableNodeWrapper::Exit(v) => NodeWrapper::Exit(v.to_deserialize(arena)),
      SerializableNodeWrapper::Finally(v) => NodeWrapper::Finally(v.to_deserialize(arena)),
      SerializableNodeWrapper::For(v) => NodeWrapper::For(v.to_deserialize(arena)),
      SerializableNodeWrapper::Foreach(v) => NodeWrapper::Foreach(v.to_deserialize(arena)),
      SerializableNodeWrapper::Function(v) => NodeWrapper::Function(v.to_deserialize(arena)),
      SerializableNodeWrapper::Global(v) => NodeWrapper::Global(v.to_deserialize(arena)),
      SerializableNodeWrapper::Goto(v) => NodeWrapper::Goto(v.to_deserialize(arena)),
      SerializableNodeWrapper::HaltCompiler(v) =>
        NodeWrapper::HaltCompiler(v.to_deserialize(arena)),
      SerializableNodeWrapper::HereDoc(v) => NodeWrapper::HereDoc(v.to_deserialize(arena)),
      SerializableNodeWrapper::Identifier(v) => NodeWrapper::Identifier(v.to_deserialize(arena)),
      SerializableNodeWrapper::If(v) => NodeWrapper::If(v.to_deserialize(arena)),
      SerializableNodeWrapper::Include(v) => NodeWrapper::Include(v.to_deserialize(arena)),
      SerializableNodeWrapper::Inline(v) => NodeWrapper::Inline(v.to_deserialize(arena)),
      SerializableNodeWrapper::Interface(v) => NodeWrapper::Interface(v.to_deserialize(arena)),
      SerializableNodeWrapper::IntersectionType(v) =>
        NodeWrapper::IntersectionType(v.to_deserialize(arena)),
      SerializableNodeWrapper::Label(v) => NodeWrapper::Label(v.to_deserialize(arena)),
      SerializableNodeWrapper::List(v) => NodeWrapper::List(v.to_deserialize(arena)),
      SerializableNodeWrapper::Magic(v) => NodeWrapper::Magic(v.to_deserialize(arena)),
      SerializableNodeWrapper::MagicMethod(v) => NodeWrapper::MagicMethod(v.to_deserialize(arena)),
      SerializableNodeWrapper::Match(v) => NodeWrapper::Match(v.to_deserialize(arena)),
      SerializableNodeWrapper::MatchArm(v) => NodeWrapper::MatchArm(v.to_deserialize(arena)),
      SerializableNodeWrapper::Method(v) => NodeWrapper::Method(v.to_deserialize(arena)),
      SerializableNodeWrapper::Namespace(v) => NodeWrapper::Namespace(v.to_deserialize(arena)),
      SerializableNodeWrapper::Negate(v) => NodeWrapper::Negate(v.to_deserialize(arena)),
      SerializableNodeWrapper::New(v) => NodeWrapper::New(v.to_deserialize(arena)),
      SerializableNodeWrapper::NowDoc(v) => NodeWrapper::NowDoc(v.to_deserialize(arena)),
      SerializableNodeWrapper::Null(v) => NodeWrapper::Null(v.to_deserialize(arena)),
      SerializableNodeWrapper::Number(v) => NodeWrapper::Number(v.to_deserialize(arena)),
      SerializableNodeWrapper::ObjectAccess(v) =>
        NodeWrapper::ObjectAccess(v.to_deserialize(arena)),
      SerializableNodeWrapper::Parameter(v) => NodeWrapper::Parameter(v.to_deserialize(arena)),
      SerializableNodeWrapper::Parent(v) => NodeWrapper::Parent(v.to_deserialize(arena)),
      SerializableNodeWrapper::Parenthesis(v) => NodeWrapper::Parenthesis(v.to_deserialize(arena)),
      SerializableNodeWrapper::Post(v) => NodeWrapper::Post(v.to_deserialize(arena)),
      SerializableNodeWrapper::Pre(v) => NodeWrapper::Pre(v.to_deserialize(arena)),
      SerializableNodeWrapper::Print(v) => NodeWrapper::Print(v.to_deserialize(arena)),
      SerializableNodeWrapper::Program(v) => NodeWrapper::Program(v.to_deserialize(arena)),
      SerializableNodeWrapper::Property(v) => NodeWrapper::Property(v.to_deserialize(arena)),
      SerializableNodeWrapper::PropertyHook(v) =>
        NodeWrapper::PropertyHook(v.to_deserialize(arena)),
      SerializableNodeWrapper::PropertyItem(v) =>
        NodeWrapper::PropertyItem(v.to_deserialize(arena)),
      SerializableNodeWrapper::Reference(v) => NodeWrapper::Reference(v.to_deserialize(arena)),
      SerializableNodeWrapper::Return(v) => NodeWrapper::Return(v.to_deserialize(arena)),
      SerializableNodeWrapper::SelfKeyword(v) => NodeWrapper::SelfKeyword(v.to_deserialize(arena)),
      SerializableNodeWrapper::Silent(v) => NodeWrapper::Silent(v.to_deserialize(arena)),
      SerializableNodeWrapper::Static(v) => NodeWrapper::Static(v.to_deserialize(arena)),
      SerializableNodeWrapper::StaticKeyword(v) =>
        NodeWrapper::StaticKeyword(v.to_deserialize(arena)),
      SerializableNodeWrapper::StaticLookup(v) =>
        NodeWrapper::StaticLookup(v.to_deserialize(arena)),
      SerializableNodeWrapper::String(v) => NodeWrapper::String(v.to_deserialize(arena)),
      SerializableNodeWrapper::Switch(v) => NodeWrapper::Switch(v.to_deserialize(arena)),
      SerializableNodeWrapper::Ternary(v) => NodeWrapper::Ternary(v.to_deserialize(arena)),
      SerializableNodeWrapper::This(v) => NodeWrapper::This(v.to_deserialize(arena)),
      SerializableNodeWrapper::Trait(v) => NodeWrapper::Trait(v.to_deserialize(arena)),
      SerializableNodeWrapper::TraitUse(v) => NodeWrapper::TraitUse(v.to_deserialize(arena)),
      SerializableNodeWrapper::TraitUseAlias(v) =>
        NodeWrapper::TraitUseAlias(v.to_deserialize(arena)),
      SerializableNodeWrapper::TraitUsePrecedence(v) =>
        NodeWrapper::TraitUsePrecedence(v.to_deserialize(arena)),
      SerializableNodeWrapper::Throw(v) => NodeWrapper::Throw(v.to_deserialize(arena)),
      SerializableNodeWrapper::Try(v) => NodeWrapper::Try(v.to_deserialize(arena)),
      SerializableNodeWrapper::Type(v) => NodeWrapper::Type(v.to_deserialize(arena)),
      SerializableNodeWrapper::UnionType(v) => NodeWrapper::UnionType(v.to_deserialize(arena)),
      SerializableNodeWrapper::Use(v) => NodeWrapper::Use(v.to_deserialize(arena)),
      SerializableNodeWrapper::UseItem(v) => NodeWrapper::UseItem(v.to_deserialize(arena)),
      SerializableNodeWrapper::Variable(v) => NodeWrapper::Variable(v.to_deserialize(arena)),
      SerializableNodeWrapper::Variadic(v) => NodeWrapper::Variadic(v.to_deserialize(arena)),
      SerializableNodeWrapper::While(v) => NodeWrapper::While(v.to_deserialize(arena)),
      SerializableNodeWrapper::Yield(v) => NodeWrapper::Yield(v.to_deserialize(arena)),
      SerializableNodeWrapper::YieldFrom(v) => NodeWrapper::YieldFrom(v.to_deserialize(arena)),
    };
    Node {
      node_type: self.node_type.clone(),
      wrapper,
      loc: self.loc.clone(),
      leadings: self.leadings.to_deserialize(arena),
      trailings: self.trailings.to_deserialize(arena),
    }
  }
}

impl<'arena, T, R> Deserializable<'arena> for Option<T> where T: Deserializable<'arena, Return = R> {
  type Return = Option<R>;

  fn to_deserialize(&self, arena: &'arena Bump) -> Self::Return {
    self.as_ref().map(|x| x.to_deserialize(arena))
  }
}

impl<'arena> Deserializable<'arena> for Vec<SerializableNode> {
  type Return = bumpalo::collections::Vec<'arena, Node<'arena>>;

  fn to_deserialize(&self, arena: &'arena Bump) -> Self::Return {
    bumpalo::collections::Vec::from_iter_in(
      self.iter().map(|x| x.to_deserialize(arena)),
      arena
    )
  }
}

impl<'arena> Deserializable<'arena> for Box<SerializableNode> {
  type Return = bumpalo::boxed::Box<'arena, Node<'arena>>;

  fn to_deserialize(&self, arena: &'arena Bump) -> Self::Return {
    bumpalo::boxed::Box::new_in(self.as_ref().to_deserialize(arena), arena)
  }
}

macro_rules! impl_serializable {
  ($($t:ty),*) => {
        $(
            impl<'arena> Deserializable<'arena> for $t {
                type Return = Self;
                fn to_deserialize(&self, _: &'arena Bump) -> Self::Return {
                    self.clone()
                }
            }
        )*
  };
}

impl_serializable!(
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
