use bumpalo::Bump;
use compact_str::CompactString;

use crate::node::{ Node, NodeType };

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
  AnonymousClass(crate::node::AnonymousClassBlueprint<'a>),
  AnonymousFunction(crate::node::AnonymousFunctionBlueprint<'a>),
  CallArgument(crate::node::CallArgumentBlueprint<'a>),
  Array(crate::node::ArrayBlueprint<'a>),
  ArrayItem(crate::node::ArrayItemBlueprint<'a>),
  ArrayLookup(crate::node::ArrayLookupBlueprint<'a>),
  ArrowFunction(crate::node::ArrowFunctionBlueprint<'a>),
  Assignment(crate::node::AssignmentBlueprint<'a>),
  Attribute(crate::node::AttributeBlueprint<'a>),
  AttributeItem(crate::node::AttributeItemBlueprint<'a>),
  Bin(crate::node::BinBlueprint<'a>),
  Block(crate::node::BlockBlueprint<'a>),
  Boolean(crate::node::BooleanBlueprint),
  Break(crate::node::BreakBlueprint<'a>),
  Call(crate::node::CallBlueprint<'a>),
  Case(crate::node::CaseBlueprint<'a>),
  Cast(crate::node::CastBlueprint<'a>),
  Catch(crate::node::CatchBlueprint<'a>),
  Class(crate::node::ClassBlueprint<'a>),
  ClassKeyword(crate::node::ClassKeywordBlueprint),
  Clone(crate::node::CloneBlueprint<'a>),
  CommentBlock(crate::node::CommentBlockBlueprint<'a>),
  CommentDoc(crate::node::CommentDocBlueprint<'a>),
  CommentLine(crate::node::CommentLineBlueprint<'a>),
  Const(crate::node::ConstBlueprint<'a>),
  ConstProperty(crate::node::ConstPropertyBlueprint<'a>),
  ConstructorParameter(crate::node::ConstructorParameterBlueprint<'a>),
  Continue(crate::node::ContinueBlueprint<'a>),
  Declare(crate::node::DeclareBlueprint<'a>),
  DeclareArgument(crate::node::DeclareArgumentBlueprint<'a>),
  DoWhile(crate::node::DoWhileBlueprint<'a>),
  DoWhileCondition(crate::node::DoWhileConditionBlueprint<'a>),
  Echo(crate::node::EchoBlueprint<'a>),
  Else(crate::node::ElseBlueprint<'a>),
  Encapsed(crate::node::EncapsedBlueprint<'a>),
  EncapsedPart(crate::node::EncapsedPartBlueprint<'a>),
  Enum(crate::node::EnumBlueprint<'a>),
  EnumItem(crate::node::EnumItemBlueprint<'a>),
  Eval(crate::node::EvalBlueprint<'a>),
  Exit(crate::node::ExitBlueprint<'a>),
  Finally(crate::node::FinallyBlueprint<'a>),
  For(crate::node::ForBlueprint<'a>),
  Foreach(crate::node::ForeachBlueprint<'a>),
  Function(crate::node::FunctionBlueprint<'a>),
  Global(crate::node::GlobalBlueprint<'a>),
  Goto(crate::node::GotoBlueprint<'a>),
  HereDoc(crate::node::HereDocBlueprint<'a>),
  Identifier(crate::node::IdentifierBlueprint<'a>),
  If(crate::node::IfBlueprint<'a>),
  Include(crate::node::IncludeBlueprint<'a>),
  Inline(crate::node::InlineBlueprint<'a>),
  Interface(crate::node::InterfaceBlueprint<'a>),
  IntersectionType(crate::node::IntersectionTypeBlueprint<'a>),
  Label(crate::node::LabelBlueprint<'a>),
  List(crate::node::ListBlueprint<'a>),
  Magic(crate::node::MagicBlueprint<'a>),
  Match(crate::node::MatchBlueprint<'a>),
  MatchArm(crate::node::MatchArmBlueprint<'a>),
  Method(crate::node::MethodBlueprint<'a>),
  Namespace(crate::node::NamespaceBlueprint<'a>),
  Negate(crate::node::NegateBlueprint<'a>),
  New(crate::node::NewBlueprint<'a>),
  NowDoc(crate::node::NowDocBlueprint<'a>),
  Null(crate::node::NullBlueprint),
  Number(crate::node::NumberBlueprint<'a>),
  ObjectAccess(crate::node::ObjectAccessBlueprint<'a>),
  Parameter(crate::node::ParameterBlueprint<'a>),
  Parent(crate::node::ParentBlueprint),
  Parenthesis(crate::node::ParenthesisBlueprint<'a>),
  Post(crate::node::PostBlueprint<'a>),
  Pre(crate::node::PreBlueprint<'a>),
  Print(crate::node::PrintBlueprint<'a>),
  Program(crate::node::ProgramBlueprint<'a>),
  Property(crate::node::PropertyBlueprint<'a>),
  PropertyHook(crate::node::PropertyHookBlueprint<'a>),
  PropertyItem(crate::node::PropertyItemBlueprint<'a>),
  Reference(crate::node::ReferenceBlueprint<'a>),
  Return(crate::node::ReturnBlueprint<'a>),
  SelfKeyword(crate::node::SelfBlueprint),
  Silent(crate::node::SilentBlueprint<'a>),
  Static(crate::node::StaticBlueprint<'a>),
  StaticKeyword(crate::node::StaticKeywordBlueprint),
  StaticLookup(crate::node::StaticLookupBlueprint<'a>),
  String(crate::node::StringBlueprint<'a>),
  Switch(crate::node::SwitchBlueprint<'a>),
  Ternary(crate::node::TernaryBlueprint<'a>),
  This(crate::node::ThisBlueprint),
  Trait(crate::node::TraitBlueprint<'a>),
  TraitUse(crate::node::TraitUseBlueprint<'a>),
  TraitUseAlias(crate::node::TraitUseAliasBlueprint<'a>),
  TraitUsePrecedence(crate::node::TraitUsePrecedenceBlueprint<'a>),
  Throw(crate::node::ThrowBlueprint<'a>),
  Try(crate::node::TryBlueprint<'a>),
  Type(crate::node::TypeBlueprint<'a>),
  UnionType(crate::node::UnionTypeBlueprint<'a>),
  Use(crate::node::UseBlueprint<'a>),
  UseItem(crate::node::UseItemBlueprint<'a>),
  Variable(crate::node::VariableBlueprint<'a>),
  Variadic(crate::node::VariadicBlueprint<'a>),
  While(crate::node::WhileBlueprint<'a>),
  Yield(crate::node::YieldBlueprint<'a>),
  YieldFrom(crate::node::YieldFromBlueprint<'a>),
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
  type Result = CompactString;

  fn build(&self, _: &'arena Bump) -> Self::Result {
    CompactString::new(self)
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
  Vec<crate::node::Visibility>,
  crate::node::BodyType,
  crate::node::UseItemModifier,
  crate::node::Visibility,
  crate::node::Inheritance,
  crate::node::Quote,
  crate::node::Modifier
);
