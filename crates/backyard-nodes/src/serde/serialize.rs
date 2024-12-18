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

use super::{ node::SerializableNodeWrapper, SerializableNode };

pub(crate) trait Serializable {
  type Return: ?Sized;
  fn to_serializable(&self) -> Self::Return;
}

impl<'arena> Serializable for Node<'arena> {
  type Return = SerializableNode;
  fn to_serializable(&self) -> Self::Return {
    let wrapper = match &self.wrapper {
      NodeWrapper::AnonymousClass(v) =>
        SerializableNodeWrapper::AnonymousClass(v.to_serializable()),
      NodeWrapper::AnonymousFunction(v) =>
        SerializableNodeWrapper::AnonymousFunction(v.to_serializable()),
      NodeWrapper::Argument(v) => SerializableNodeWrapper::Argument(v.to_serializable()),
      NodeWrapper::Array(v) => SerializableNodeWrapper::Array(v.to_serializable()),
      NodeWrapper::ArrayItem(v) => SerializableNodeWrapper::ArrayItem(v.to_serializable()),
      NodeWrapper::ArrayLookup(v) => SerializableNodeWrapper::ArrayLookup(v.to_serializable()),
      NodeWrapper::ArrowFunction(v) => SerializableNodeWrapper::ArrowFunction(v.to_serializable()),
      NodeWrapper::Assignment(v) => SerializableNodeWrapper::Assignment(v.to_serializable()),
      NodeWrapper::Attribute(v) => SerializableNodeWrapper::Attribute(v.to_serializable()),
      NodeWrapper::AttributeItem(v) => SerializableNodeWrapper::AttributeItem(v.to_serializable()),
      NodeWrapper::Bin(v) => SerializableNodeWrapper::Bin(v.to_serializable()),
      NodeWrapper::Block(v) => SerializableNodeWrapper::Block(v.to_serializable()),
      NodeWrapper::Boolean(v) => SerializableNodeWrapper::Boolean(v.to_serializable()),
      NodeWrapper::Break(v) => SerializableNodeWrapper::Break(v.to_serializable()),
      NodeWrapper::Call(v) => SerializableNodeWrapper::Call(v.to_serializable()),
      NodeWrapper::Case(v) => SerializableNodeWrapper::Case(v.to_serializable()),
      NodeWrapper::Cast(v) => SerializableNodeWrapper::Cast(v.to_serializable()),
      NodeWrapper::Catch(v) => SerializableNodeWrapper::Catch(v.to_serializable()),
      NodeWrapper::Class(v) => SerializableNodeWrapper::Class(v.to_serializable()),
      NodeWrapper::ClassKeyword(v) => SerializableNodeWrapper::ClassKeyword(v.to_serializable()),
      NodeWrapper::Clone(v) => SerializableNodeWrapper::Clone(v.to_serializable()),
      NodeWrapper::CommentBlock(v) => SerializableNodeWrapper::CommentBlock(v.to_serializable()),
      NodeWrapper::CommentDoc(v) => SerializableNodeWrapper::CommentDoc(v.to_serializable()),
      NodeWrapper::CommentLine(v) => SerializableNodeWrapper::CommentLine(v.to_serializable()),
      NodeWrapper::Const(v) => SerializableNodeWrapper::Const(v.to_serializable()),
      NodeWrapper::ConstProperty(v) => SerializableNodeWrapper::ConstProperty(v.to_serializable()),
      NodeWrapper::ConstructorParameter(v) =>
        SerializableNodeWrapper::ConstructorParameter(v.to_serializable()),
      NodeWrapper::Continue(v) => SerializableNodeWrapper::Continue(v.to_serializable()),
      NodeWrapper::Declare(v) => SerializableNodeWrapper::Declare(v.to_serializable()),
      NodeWrapper::DeclareArgument(v) =>
        SerializableNodeWrapper::DeclareArgument(v.to_serializable()),
      NodeWrapper::DoWhile(v) => SerializableNodeWrapper::DoWhile(v.to_serializable()),
      NodeWrapper::DoWhileCondition(v) =>
        SerializableNodeWrapper::DoWhileCondition(v.to_serializable()),
      NodeWrapper::Echo(v) => SerializableNodeWrapper::Echo(v.to_serializable()),
      NodeWrapper::Else(v) => SerializableNodeWrapper::Else(v.to_serializable()),
      NodeWrapper::Encapsed(v) => SerializableNodeWrapper::Encapsed(v.to_serializable()),
      NodeWrapper::EncapsedPart(v) => SerializableNodeWrapper::EncapsedPart(v.to_serializable()),
      NodeWrapper::Enum(v) => SerializableNodeWrapper::Enum(v.to_serializable()),
      NodeWrapper::EnumItem(v) => SerializableNodeWrapper::EnumItem(v.to_serializable()),
      NodeWrapper::Eval(v) => SerializableNodeWrapper::Eval(v.to_serializable()),
      NodeWrapper::Exit(v) => SerializableNodeWrapper::Exit(v.to_serializable()),
      NodeWrapper::Finally(v) => SerializableNodeWrapper::Finally(v.to_serializable()),
      NodeWrapper::For(v) => SerializableNodeWrapper::For(v.to_serializable()),
      NodeWrapper::Foreach(v) => SerializableNodeWrapper::Foreach(v.to_serializable()),
      NodeWrapper::Function(v) => SerializableNodeWrapper::Function(v.to_serializable()),
      NodeWrapper::Global(v) => SerializableNodeWrapper::Global(v.to_serializable()),
      NodeWrapper::Goto(v) => SerializableNodeWrapper::Goto(v.to_serializable()),
      NodeWrapper::HaltCompiler(v) => SerializableNodeWrapper::HaltCompiler(v.to_serializable()),
      NodeWrapper::HereDoc(v) => SerializableNodeWrapper::HereDoc(v.to_serializable()),
      NodeWrapper::Identifier(v) => SerializableNodeWrapper::Identifier(v.to_serializable()),
      NodeWrapper::If(v) => SerializableNodeWrapper::If(v.to_serializable()),
      NodeWrapper::Include(v) => SerializableNodeWrapper::Include(v.to_serializable()),
      NodeWrapper::Inline(v) => SerializableNodeWrapper::Inline(v.to_serializable()),
      NodeWrapper::Interface(v) => SerializableNodeWrapper::Interface(v.to_serializable()),
      NodeWrapper::IntersectionType(v) =>
        SerializableNodeWrapper::IntersectionType(v.to_serializable()),
      NodeWrapper::Label(v) => SerializableNodeWrapper::Label(v.to_serializable()),
      NodeWrapper::List(v) => SerializableNodeWrapper::List(v.to_serializable()),
      NodeWrapper::Magic(v) => SerializableNodeWrapper::Magic(v.to_serializable()),
      NodeWrapper::MagicMethod(v) => SerializableNodeWrapper::MagicMethod(v.to_serializable()),
      NodeWrapper::Match(v) => SerializableNodeWrapper::Match(v.to_serializable()),
      NodeWrapper::MatchArm(v) => SerializableNodeWrapper::MatchArm(v.to_serializable()),
      NodeWrapper::Method(v) => SerializableNodeWrapper::Method(v.to_serializable()),
      NodeWrapper::Namespace(v) => SerializableNodeWrapper::Namespace(v.to_serializable()),
      NodeWrapper::Negate(v) => SerializableNodeWrapper::Negate(v.to_serializable()),
      NodeWrapper::New(v) => SerializableNodeWrapper::New(v.to_serializable()),
      NodeWrapper::NowDoc(v) => SerializableNodeWrapper::NowDoc(v.to_serializable()),
      NodeWrapper::Null(v) => SerializableNodeWrapper::Null(v.to_serializable()),
      NodeWrapper::Number(v) => SerializableNodeWrapper::Number(v.to_serializable()),
      NodeWrapper::ObjectAccess(v) => SerializableNodeWrapper::ObjectAccess(v.to_serializable()),
      NodeWrapper::Parameter(v) => SerializableNodeWrapper::Parameter(v.to_serializable()),
      NodeWrapper::Parent(v) => SerializableNodeWrapper::Parent(v.to_serializable()),
      NodeWrapper::Parenthesis(v) => SerializableNodeWrapper::Parenthesis(v.to_serializable()),
      NodeWrapper::Post(v) => SerializableNodeWrapper::Post(v.to_serializable()),
      NodeWrapper::Pre(v) => SerializableNodeWrapper::Pre(v.to_serializable()),
      NodeWrapper::Print(v) => SerializableNodeWrapper::Print(v.to_serializable()),
      NodeWrapper::Program(v) => SerializableNodeWrapper::Program(v.to_serializable()),
      NodeWrapper::Property(v) => SerializableNodeWrapper::Property(v.to_serializable()),
      NodeWrapper::PropertyHook(v) => SerializableNodeWrapper::PropertyHook(v.to_serializable()),
      NodeWrapper::PropertyItem(v) => SerializableNodeWrapper::PropertyItem(v.to_serializable()),
      NodeWrapper::Reference(v) => SerializableNodeWrapper::Reference(v.to_serializable()),
      NodeWrapper::Return(v) => SerializableNodeWrapper::Return(v.to_serializable()),
      NodeWrapper::SelfKeyword(v) => SerializableNodeWrapper::SelfKeyword(v.to_serializable()),
      NodeWrapper::Silent(v) => SerializableNodeWrapper::Silent(v.to_serializable()),
      NodeWrapper::Static(v) => SerializableNodeWrapper::Static(v.to_serializable()),
      NodeWrapper::StaticKeyword(v) => SerializableNodeWrapper::StaticKeyword(v.to_serializable()),
      NodeWrapper::StaticLookup(v) => SerializableNodeWrapper::StaticLookup(v.to_serializable()),
      NodeWrapper::String(v) => SerializableNodeWrapper::String(v.to_serializable()),
      NodeWrapper::Switch(v) => SerializableNodeWrapper::Switch(v.to_serializable()),
      NodeWrapper::Ternary(v) => SerializableNodeWrapper::Ternary(v.to_serializable()),
      NodeWrapper::This(v) => SerializableNodeWrapper::This(v.to_serializable()),
      NodeWrapper::Trait(v) => SerializableNodeWrapper::Trait(v.to_serializable()),
      NodeWrapper::TraitUse(v) => SerializableNodeWrapper::TraitUse(v.to_serializable()),
      NodeWrapper::TraitUseAlias(v) => SerializableNodeWrapper::TraitUseAlias(v.to_serializable()),
      NodeWrapper::TraitUsePrecedence(v) =>
        SerializableNodeWrapper::TraitUsePrecedence(v.to_serializable()),
      NodeWrapper::Throw(v) => SerializableNodeWrapper::Throw(v.to_serializable()),
      NodeWrapper::Try(v) => SerializableNodeWrapper::Try(v.to_serializable()),
      NodeWrapper::Type(v) => SerializableNodeWrapper::Type(v.to_serializable()),
      NodeWrapper::UnionType(v) => SerializableNodeWrapper::UnionType(v.to_serializable()),
      NodeWrapper::Use(v) => SerializableNodeWrapper::Use(v.to_serializable()),
      NodeWrapper::UseItem(v) => SerializableNodeWrapper::UseItem(v.to_serializable()),
      NodeWrapper::Variable(v) => SerializableNodeWrapper::Variable(v.to_serializable()),
      NodeWrapper::Variadic(v) => SerializableNodeWrapper::Variadic(v.to_serializable()),
      NodeWrapper::While(v) => SerializableNodeWrapper::While(v.to_serializable()),
      NodeWrapper::Yield(v) => SerializableNodeWrapper::Yield(v.to_serializable()),
      NodeWrapper::YieldFrom(v) => SerializableNodeWrapper::YieldFrom(v.to_serializable()),
    };
    SerializableNode {
      node_type: self.node_type.clone(),
      wrapper,
      loc: self.loc.clone(),
      leadings: self.leadings.to_serializable(),
      trailings: self.trailings.to_serializable(),
    }
  }
}

impl<T, R> Serializable for Option<T> where T: Serializable<Return = R> {
  type Return = Option<R>;

  fn to_serializable(&self) -> Self::Return {
    self.as_ref().map(|x| x.to_serializable())
  }
}

impl<'arena, T, R> Serializable
  for bumpalo::collections::Vec<'arena, T>
  where T: Serializable<Return = R>
{
  type Return = Vec<R>;

  fn to_serializable(&self) -> Self::Return {
    self
      .iter()
      .map(|x| x.to_serializable())
      .collect()
  }
}

impl<'arena, T, R> Serializable for bumpalo::boxed::Box<'arena, T> where T: Serializable<Return = R> {
  type Return = Box<R>;

  fn to_serializable(&self) -> Self::Return {
    Box::new(self.as_ref().to_serializable())
  }
}

macro_rules! impl_serializable {
  ($($t:ty),*) => {
        $(
            impl Serializable for $t {
                type Return = Self;
                fn to_serializable(&self) -> Self::Return {
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
