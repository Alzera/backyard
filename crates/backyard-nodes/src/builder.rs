use bstr::BString;
use bumpalo::Bump;

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
  NodeType,
  PostType,
  PreType,
  Quote,
  UseItemModifier,
  Visibility,
};

pub struct Builder {}

impl Builder {
  pub fn new() -> Self {
    Self {}
  }
}

pub trait BlueprintBuildable<'arena> {
  type Result;
  fn build(&self, arena: &'arena Bump) -> Self::Result;
}

#[derive(Debug)]
pub struct Blueprint<'a> {
  pub leadings: &'a [Box<Blueprint<'a>>],
  pub trailings: &'a [Box<Blueprint<'a>>],
  pub node_type: NodeType,
  pub wrapper: BlueprintWrapper<'a>,
}

impl<'arena, 'a> BlueprintBuildable<'arena> for Blueprint<'a> {
  type Result = Node<'arena>;

  fn build(&self, arena: &'arena Bump) -> Self::Result {
    match &self.wrapper {
      BlueprintWrapper::AnonymousClass(bp) => bp.build(arena),
      BlueprintWrapper::AnonymousFunction(bp) => bp.build(arena),
      BlueprintWrapper::CallArgument(bp) => bp.build(arena),
      BlueprintWrapper::Array(bp) => bp.build(arena),
      BlueprintWrapper::ArrayItem(bp) => bp.build(arena),
      BlueprintWrapper::ArrayLookup(bp) => bp.build(arena),
      BlueprintWrapper::ArrowFunction(bp) => bp.build(arena),
      BlueprintWrapper::Assignment(bp) => bp.build(arena),
      BlueprintWrapper::Attribute(bp) => bp.build(arena),
      BlueprintWrapper::AttributeItem(bp) => bp.build(arena),
      BlueprintWrapper::Bin(bp) => bp.build(arena),
      BlueprintWrapper::Block(bp) => bp.build(arena),
      BlueprintWrapper::Boolean(bp) => bp.build(arena),
      BlueprintWrapper::Break(bp) => bp.build(arena),
      BlueprintWrapper::Call(bp) => bp.build(arena),
      BlueprintWrapper::Case(bp) => bp.build(arena),
      BlueprintWrapper::Cast(bp) => bp.build(arena),
      BlueprintWrapper::Catch(bp) => bp.build(arena),
      BlueprintWrapper::Class(bp) => bp.build(arena),
      BlueprintWrapper::ClassKeyword(bp) => bp.build(arena),
      BlueprintWrapper::Clone(bp) => bp.build(arena),
      BlueprintWrapper::CommentBlock(bp) => bp.build(arena),
      BlueprintWrapper::CommentDoc(bp) => bp.build(arena),
      BlueprintWrapper::CommentLine(bp) => bp.build(arena),
      BlueprintWrapper::Const(bp) => bp.build(arena),
      BlueprintWrapper::ConstProperty(bp) => bp.build(arena),
      BlueprintWrapper::ConstructorParameter(bp) => bp.build(arena),
      BlueprintWrapper::Continue(bp) => bp.build(arena),
      BlueprintWrapper::Declare(bp) => bp.build(arena),
      BlueprintWrapper::DeclareArgument(bp) => bp.build(arena),
      BlueprintWrapper::DoWhile(bp) => bp.build(arena),
      BlueprintWrapper::DoWhileCondition(bp) => bp.build(arena),
      BlueprintWrapper::Echo(bp) => bp.build(arena),
      BlueprintWrapper::Else(bp) => bp.build(arena),
      BlueprintWrapper::Encapsed(bp) => bp.build(arena),
      BlueprintWrapper::EncapsedPart(bp) => bp.build(arena),
      BlueprintWrapper::Enum(bp) => bp.build(arena),
      BlueprintWrapper::EnumItem(bp) => bp.build(arena),
      BlueprintWrapper::Eval(bp) => bp.build(arena),
      BlueprintWrapper::Exit(bp) => bp.build(arena),
      BlueprintWrapper::Finally(bp) => bp.build(arena),
      BlueprintWrapper::For(bp) => bp.build(arena),
      BlueprintWrapper::Foreach(bp) => bp.build(arena),
      BlueprintWrapper::Function(bp) => bp.build(arena),
      BlueprintWrapper::Global(bp) => bp.build(arena),
      BlueprintWrapper::Goto(bp) => bp.build(arena),
      BlueprintWrapper::HereDoc(bp) => bp.build(arena),
      BlueprintWrapper::Identifier(bp) => bp.build(arena),
      BlueprintWrapper::If(bp) => bp.build(arena),
      BlueprintWrapper::Include(bp) => bp.build(arena),
      BlueprintWrapper::Inline(bp) => bp.build(arena),
      BlueprintWrapper::Interface(bp) => bp.build(arena),
      BlueprintWrapper::IntersectionType(bp) => bp.build(arena),
      BlueprintWrapper::Label(bp) => bp.build(arena),
      BlueprintWrapper::List(bp) => bp.build(arena),
      BlueprintWrapper::Magic(bp) => bp.build(arena),
      BlueprintWrapper::MagicMethod(bp) => bp.build(arena),
      BlueprintWrapper::Match(bp) => bp.build(arena),
      BlueprintWrapper::MatchArm(bp) => bp.build(arena),
      BlueprintWrapper::Method(bp) => bp.build(arena),
      BlueprintWrapper::Namespace(bp) => bp.build(arena),
      BlueprintWrapper::Negate(bp) => bp.build(arena),
      BlueprintWrapper::New(bp) => bp.build(arena),
      BlueprintWrapper::NowDoc(bp) => bp.build(arena),
      BlueprintWrapper::Null(bp) => bp.build(arena),
      BlueprintWrapper::Number(bp) => bp.build(arena),
      BlueprintWrapper::ObjectAccess(bp) => bp.build(arena),
      BlueprintWrapper::Parameter(bp) => bp.build(arena),
      BlueprintWrapper::Parent(bp) => bp.build(arena),
      BlueprintWrapper::Parenthesis(bp) => bp.build(arena),
      BlueprintWrapper::Post(bp) => bp.build(arena),
      BlueprintWrapper::Pre(bp) => bp.build(arena),
      BlueprintWrapper::Print(bp) => bp.build(arena),
      BlueprintWrapper::Program(bp) => bp.build(arena),
      BlueprintWrapper::Property(bp) => bp.build(arena),
      BlueprintWrapper::PropertyHook(bp) => bp.build(arena),
      BlueprintWrapper::PropertyItem(bp) => bp.build(arena),
      BlueprintWrapper::Reference(bp) => bp.build(arena),
      BlueprintWrapper::Return(bp) => bp.build(arena),
      BlueprintWrapper::SelfKeyword(bp) => bp.build(arena),
      BlueprintWrapper::Silent(bp) => bp.build(arena),
      BlueprintWrapper::Static(bp) => bp.build(arena),
      BlueprintWrapper::StaticKeyword(bp) => bp.build(arena),
      BlueprintWrapper::StaticLookup(bp) => bp.build(arena),
      BlueprintWrapper::String(bp) => bp.build(arena),
      BlueprintWrapper::Switch(bp) => bp.build(arena),
      BlueprintWrapper::Ternary(bp) => bp.build(arena),
      BlueprintWrapper::This(bp) => bp.build(arena),
      BlueprintWrapper::Trait(bp) => bp.build(arena),
      BlueprintWrapper::TraitUse(bp) => bp.build(arena),
      BlueprintWrapper::TraitUseAlias(bp) => bp.build(arena),
      BlueprintWrapper::TraitUsePrecedence(bp) => bp.build(arena),
      BlueprintWrapper::Throw(bp) => bp.build(arena),
      BlueprintWrapper::Try(bp) => bp.build(arena),
      BlueprintWrapper::Type(bp) => bp.build(arena),
      BlueprintWrapper::UnionType(bp) => bp.build(arena),
      BlueprintWrapper::Use(bp) => bp.build(arena),
      BlueprintWrapper::UseItem(bp) => bp.build(arena),
      BlueprintWrapper::Variable(bp) => bp.build(arena),
      BlueprintWrapper::Variadic(bp) => bp.build(arena),
      BlueprintWrapper::While(bp) => bp.build(arena),
      BlueprintWrapper::Yield(bp) => bp.build(arena),
      BlueprintWrapper::YieldFrom(bp) => bp.build(arena),
    }
  }
}

#[derive(Debug)]
pub enum BlueprintWrapper<'a> {
  AnonymousClass(crate::AnonymousClassBlueprint<'a>),
  AnonymousFunction(crate::AnonymousFunctionBlueprint<'a>),
  CallArgument(crate::CallArgumentBlueprint<'a>),
  Array(crate::ArrayBlueprint<'a>),
  ArrayItem(crate::ArrayItemBlueprint<'a>),
  ArrayLookup(crate::ArrayLookupBlueprint<'a>),
  ArrowFunction(crate::ArrowFunctionBlueprint<'a>),
  Assignment(crate::AssignmentBlueprint<'a>),
  Attribute(crate::AttributeBlueprint<'a>),
  AttributeItem(crate::AttributeItemBlueprint<'a>),
  Bin(crate::BinBlueprint<'a>),
  Block(crate::BlockBlueprint<'a>),
  Boolean(crate::BooleanBlueprint),
  Break(crate::BreakBlueprint<'a>),
  Call(crate::CallBlueprint<'a>),
  Case(crate::CaseBlueprint<'a>),
  Cast(crate::CastBlueprint<'a>),
  Catch(crate::CatchBlueprint<'a>),
  Class(crate::ClassBlueprint<'a>),
  ClassKeyword(crate::ClassKeywordBlueprint),
  Clone(crate::CloneBlueprint<'a>),
  CommentBlock(crate::CommentBlockBlueprint<'a>),
  CommentDoc(crate::CommentDocBlueprint<'a>),
  CommentLine(crate::CommentLineBlueprint<'a>),
  Const(crate::ConstBlueprint<'a>),
  ConstProperty(crate::ConstPropertyBlueprint<'a>),
  ConstructorParameter(crate::ConstructorParameterBlueprint<'a>),
  Continue(crate::ContinueBlueprint<'a>),
  Declare(crate::DeclareBlueprint<'a>),
  DeclareArgument(crate::DeclareArgumentBlueprint<'a>),
  DoWhile(crate::DoWhileBlueprint<'a>),
  DoWhileCondition(crate::DoWhileConditionBlueprint<'a>),
  Echo(crate::EchoBlueprint<'a>),
  Else(crate::ElseBlueprint<'a>),
  Encapsed(crate::EncapsedBlueprint<'a>),
  EncapsedPart(crate::EncapsedPartBlueprint<'a>),
  Enum(crate::EnumBlueprint<'a>),
  EnumItem(crate::EnumItemBlueprint<'a>),
  Eval(crate::EvalBlueprint<'a>),
  Exit(crate::ExitBlueprint<'a>),
  Finally(crate::FinallyBlueprint<'a>),
  For(crate::ForBlueprint<'a>),
  Foreach(crate::ForeachBlueprint<'a>),
  Function(crate::FunctionBlueprint<'a>),
  Global(crate::GlobalBlueprint<'a>),
  Goto(crate::GotoBlueprint<'a>),
  HereDoc(crate::HereDocBlueprint<'a>),
  Identifier(crate::IdentifierBlueprint<'a>),
  If(crate::IfBlueprint<'a>),
  Include(crate::IncludeBlueprint<'a>),
  Inline(crate::InlineBlueprint<'a>),
  Interface(crate::InterfaceBlueprint<'a>),
  IntersectionType(crate::IntersectionTypeBlueprint<'a>),
  Label(crate::LabelBlueprint<'a>),
  List(crate::ListBlueprint<'a>),
  Magic(crate::MagicBlueprint),
  MagicMethod(crate::MagicMethodBlueprint),
  Match(crate::MatchBlueprint<'a>),
  MatchArm(crate::MatchArmBlueprint<'a>),
  Method(crate::MethodBlueprint<'a>),
  Namespace(crate::NamespaceBlueprint<'a>),
  Negate(crate::NegateBlueprint<'a>),
  New(crate::NewBlueprint<'a>),
  NowDoc(crate::NowDocBlueprint<'a>),
  Null(crate::NullBlueprint),
  Number(crate::NumberBlueprint<'a>),
  ObjectAccess(crate::ObjectAccessBlueprint<'a>),
  Parameter(crate::ParameterBlueprint<'a>),
  Parent(crate::ParentBlueprint),
  Parenthesis(crate::ParenthesisBlueprint<'a>),
  Post(crate::PostBlueprint<'a>),
  Pre(crate::PreBlueprint<'a>),
  Print(crate::PrintBlueprint<'a>),
  Program(crate::ProgramBlueprint<'a>),
  Property(crate::PropertyBlueprint<'a>),
  PropertyHook(crate::PropertyHookBlueprint<'a>),
  PropertyItem(crate::PropertyItemBlueprint<'a>),
  Reference(crate::ReferenceBlueprint<'a>),
  Return(crate::ReturnBlueprint<'a>),
  SelfKeyword(crate::SelfBlueprint),
  Silent(crate::SilentBlueprint<'a>),
  Static(crate::StaticBlueprint<'a>),
  StaticKeyword(crate::StaticKeywordBlueprint),
  StaticLookup(crate::StaticLookupBlueprint<'a>),
  String(crate::StringBlueprint<'a>),
  Switch(crate::SwitchBlueprint<'a>),
  Ternary(crate::TernaryBlueprint<'a>),
  This(crate::ThisBlueprint),
  Trait(crate::TraitBlueprint<'a>),
  TraitUse(crate::TraitUseBlueprint<'a>),
  TraitUseAlias(crate::TraitUseAliasBlueprint<'a>),
  TraitUsePrecedence(crate::TraitUsePrecedenceBlueprint<'a>),
  Throw(crate::ThrowBlueprint<'a>),
  Try(crate::TryBlueprint<'a>),
  Type(crate::TypeBlueprint<'a>),
  UnionType(crate::UnionTypeBlueprint<'a>),
  Use(crate::UseBlueprint<'a>),
  UseItem(crate::UseItemBlueprint<'a>),
  Variable(crate::VariableBlueprint<'a>),
  Variadic(crate::VariadicBlueprint<'a>),
  While(crate::WhileBlueprint<'a>),
  Yield(crate::YieldBlueprint<'a>),
  YieldFrom(crate::YieldFromBlueprint<'a>),
}

impl<'arena, 'a> BlueprintBuildable<'arena> for std::boxed::Box<Blueprint<'a>> {
  type Result = bumpalo::boxed::Box<'arena, Node<'arena>>;

  fn build(&self, arena: &'arena Bump) -> Self::Result {
    bumpalo::boxed::Box::new_in(self.as_ref().build(arena), arena)
  }
}

impl<'arena, T> BlueprintBuildable<'arena> for Option<T> where T: BlueprintBuildable<'arena> {
  type Result = Option<T::Result>;

  fn build(&self, arena: &'arena Bump) -> Self::Result {
    if let Some(i) = self { Some(i.build(arena)) } else { None }
  }
}

impl<'arena, 'a> BlueprintBuildable<'arena> for &'a [Box<Blueprint<'a>>] {
  type Result = bumpalo::collections::Vec<'arena, Node<'arena>>;

  fn build(&self, arena: &'arena Bump) -> Self::Result {
    bumpalo::collections::Vec::from_iter_in(
      self.iter().map(|i| i.as_ref().build(arena)),
      arena
    )
  }
}

impl<'arena, 'a> BlueprintBuildable<'arena> for &'a str {
  type Result = BString;

  fn build(&self, _: &'arena Bump) -> Self::Result {
    BString::new(self.as_bytes().to_vec())
  }
}

macro_rules! impl_build {
  ($($t:ty),*) => {
      $(
          impl<'arena> BlueprintBuildable<'arena> for $t {
            type Result = Self;

            fn build(&self, _: &'arena Bump) -> Self::Result {
              self.to_owned()
            }
          }
      )*
  };
}

impl_build!(
  bool,
  std::vec::Vec<Visibility>,
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
  use crate::{ builder::{ BlueprintBuildable, Builder }, AssignmentType };

  #[test]
  fn builder() {
    let arena = bumpalo::Bump::new();
    let b = Builder::new();
    let node = b
      .Program(
        &[b.Assignment(b.Variable(b.Identifier("a")), AssignmentType::Default, b.Number("21"))]
      )
      .build(&arena);
    insta::assert_yaml_snapshot!(node);
  }
}
