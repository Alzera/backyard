// For now, it's incredibly hard to serde deserialize bumpalo Vec and Box.
// So we just use this SerializableNode as work around to make deserialize possible.

use bstr::BString;
use serde::{ Deserialize, Serialize };

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
  NodeWrapper,
  PostType,
  PreType,
  Quote,
  RangeLocation,
  UseItemModifier,
  Visibility,
};

impl<'arena> Node<'arena> {
  pub fn serializable(&self) -> SerializableNode {
    self.to_serializable()
  }
}

pub(crate) trait Serializable {
  type Return: ?Sized;
  fn to_serializable(&self) -> Self::Return;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializableNode {
  pub node_type: NodeType,
  #[serde(flatten)]
  pub wrapper: SerializableNodeWrapper,
  pub loc: Option<RangeLocation>,
  pub leadings: Option<Vec<SerializableNode>>,
  pub trailings: Option<Vec<SerializableNode>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SerializableNodeWrapper {
  AnonymousClass(AnonymousClassSNode),
  AnonymousFunction(AnonymousFunctionSNode),
  Argument(ArgumentSNode),
  Array(ArraySNode),
  ArrayItem(ArrayItemSNode),
  ArrayLookup(ArrayLookupSNode),
  ArrowFunction(ArrowFunctionSNode),
  Assignment(AssignmentSNode),
  Attribute(AttributeSNode),
  AttributeItem(AttributeItemSNode),
  Bin(BinSNode),
  Block(BlockSNode),
  Boolean(BooleanSNode),
  Break(BreakSNode),
  Call(CallSNode),
  Case(CaseSNode),
  Cast(CastSNode),
  Catch(CatchSNode),
  Class(ClassSNode),
  ClassKeyword(ClassKeywordSNode),
  Clone(CloneSNode),
  CommentBlock(CommentBlockSNode),
  CommentDoc(CommentDocSNode),
  CommentLine(CommentLineSNode),
  Const(ConstSNode),
  ConstProperty(ConstPropertySNode),
  ConstructorParameter(ConstructorParameterSNode),
  Continue(ContinueSNode),
  Declare(DeclareSNode),
  DeclareArgument(DeclareArgumentSNode),
  DoWhile(DoWhileSNode),
  DoWhileCondition(DoWhileConditionSNode),
  Echo(EchoSNode),
  Else(ElseSNode),
  Encapsed(EncapsedSNode),
  EncapsedPart(EncapsedPartSNode),
  Enum(EnumSNode),
  EnumItem(EnumItemSNode),
  Eval(EvalSNode),
  Exit(ExitSNode),
  Finally(FinallySNode),
  For(ForSNode),
  Foreach(ForeachSNode),
  Function(FunctionSNode),
  Global(GlobalSNode),
  Goto(GotoSNode),
  HaltCompiler(HaltCompilerSNode),
  HereDoc(HereDocSNode),
  Identifier(IdentifierSNode),
  If(IfSNode),
  Include(IncludeSNode),
  Inline(InlineSNode),
  Interface(InterfaceSNode),
  IntersectionType(IntersectionTypeSNode),
  Label(LabelSNode),
  List(ListSNode),
  Magic(MagicSNode),
  MagicMethod(MagicMethodSNode),
  Match(MatchSNode),
  MatchArm(MatchArmSNode),
  Method(MethodSNode),
  Namespace(NamespaceSNode),
  Negate(NegateSNode),
  New(NewSNode),
  NowDoc(NowDocSNode),
  Null(NullSNode),
  Number(NumberSNode),
  ObjectAccess(ObjectAccessSNode),
  Parameter(ParameterSNode),
  Parent(ParentSNode),
  Parenthesis(ParenthesisSNode),
  Post(PostSNode),
  Pre(PreSNode),
  Print(PrintSNode),
  Program(ProgramSNode),
  Property(PropertySNode),
  PropertyHook(PropertyHookSNode),
  PropertyItem(PropertyItemSNode),
  Reference(ReferenceSNode),
  Return(ReturnSNode),
  SelfKeyword(SelfSNode),
  Silent(SilentSNode),
  Static(StaticSNode),
  StaticKeyword(StaticKeywordSNode),
  StaticLookup(StaticLookupSNode),
  String(StringSNode),
  Switch(SwitchSNode),
  Ternary(TernarySNode),
  This(ThisSNode),
  Trait(TraitSNode),
  TraitUse(TraitUseSNode),
  TraitUseAlias(TraitUseAliasSNode),
  TraitUsePrecedence(TraitUsePrecedenceSNode),
  Throw(ThrowSNode),
  Try(TrySNode),
  Type(TypeSNode),
  UnionType(UnionTypeSNode),
  Use(UseSNode),
  UseItem(UseItemSNode),
  Variable(VariableSNode),
  Variadic(VariadicSNode),
  While(WhileSNode),
  Yield(YieldSNode),
  YieldFrom(YieldFromSNode),
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

macro_rules! new_serializable_node {
  (
    $node_type:ident,
    $struct_name:ident < $lt:lifetime > ,
    $serializable_node_name:ident { $($field_name:ident: $field_type:ty),* $(,)? }
  ) => {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct $serializable_node_name {
      $(pub $field_name: $field_type),*
    }

    impl<$lt> Serializable for crate::$struct_name<$lt> {
      type Return = $serializable_node_name;
      fn to_serializable(&self) -> Self::Return {
        $serializable_node_name {
          $($field_name: self.$field_name.to_serializable()),*
        }
      }
    }
  };

  (
    $node_type:ident,
    $struct_name:ident,
    $serializable_node_name:ident { $($field_name:ident: $field_type:ty),* $(,)? }
  ) => {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct $serializable_node_name {
      $(pub $field_name: $field_type),*
    }

    impl Serializable for crate::$struct_name {
      type Return = $serializable_node_name;
      fn to_serializable(&self) -> Self::Return {
        $serializable_node_name {
          $($field_name: self.$field_name.to_serializable()),*
        }
      }
    }
  };
}
new_serializable_node!(AnonymousClass, AnonymousClassNode<'a>, AnonymousClassSNode { parameters: Vec<SerializableNode>, extends: Option<Box<SerializableNode>>, implements: Vec<SerializableNode>, body: Box<SerializableNode>, });
new_serializable_node!(AnonymousFunction, AnonymousFunctionNode<'a>, AnonymousFunctionSNode { is_ref: bool, parameters: Vec<SerializableNode>, uses: Vec<SerializableNode>, return_type: Option<Box<SerializableNode>>, body: Box<SerializableNode>, });
new_serializable_node!(Argument, ArgumentNode<'a>, ArgumentSNode { name: Option<Box<SerializableNode>>, value: Box<SerializableNode>, });
new_serializable_node!(Array, ArrayNode<'a>, ArraySNode { is_short: bool, items: Vec<SerializableNode>, });
new_serializable_node!(ArrayItem, ArrayItemNode<'a>, ArrayItemSNode { key: Option<Box<SerializableNode>>, value: Box<SerializableNode>, });
new_serializable_node!(ArrayLookup, ArrayLookupNode<'a>, ArrayLookupSNode { left: Box<SerializableNode>, right: Option<Box<SerializableNode>>, });
new_serializable_node!(ArrowFunction, ArrowFunctionNode<'a>, ArrowFunctionSNode { is_ref: bool, parameters: Vec<SerializableNode>, return_type: Option<Box<SerializableNode>>, body: Box<SerializableNode>, });
new_serializable_node!(Assignment, AssignmentNode<'a>, AssignmentSNode { left: Box<SerializableNode>, operator: AssignmentType, right: Box<SerializableNode>, });
new_serializable_node!(Attribute, AttributeNode<'a>, AttributeSNode { items: Vec<SerializableNode>, });
new_serializable_node!(AttributeItem, AttributeItemNode<'a>, AttributeItemSNode { name: BString, arguments: Vec<SerializableNode>, });
new_serializable_node!(Bin, BinNode<'a>, BinSNode { left: Box<SerializableNode>, operator: BinaryType, right: Box<SerializableNode>, });
new_serializable_node!(Block, BlockNode<'a>, BlockSNode { statements: Vec<SerializableNode>, });
new_serializable_node!(Boolean, BooleanNode, BooleanSNode { is_true: bool });
new_serializable_node!(Break, BreakNode<'a>, BreakSNode { statement: Option<Box<SerializableNode>>, });
new_serializable_node!(Call, CallNode<'a>, CallSNode { name: Box<SerializableNode>, arguments: Vec<SerializableNode>, });
new_serializable_node!(Case, CaseNode<'a>, CaseSNode { condition: Option<Box<SerializableNode>>, body: Box<SerializableNode>, });
new_serializable_node!(Cast, CastNode<'a>, CastSNode { cast_type: CastType, expression: Box<SerializableNode>, });
new_serializable_node!(Catch, CatchNode<'a>, CatchSNode { types: Vec<SerializableNode>, variable: Option<Box<SerializableNode>>, body: Box<SerializableNode>, });
new_serializable_node!(Class, ClassNode<'a>, ClassSNode { inheritance: Option<Inheritance>, name: Option<Box<SerializableNode>>, extends: Option<Box<SerializableNode>>, implements: Vec<SerializableNode>, body: Box<SerializableNode>, is_readonly: bool, });
new_serializable_node!(ClassKeyword, ClassKeywordNode, ClassKeywordSNode {});
new_serializable_node!(Clone, CloneNode<'a>, CloneSNode { statement: Box<SerializableNode>, });
new_serializable_node!(CommentBlock, CommentBlockNode, CommentBlockSNode { comment: BString });
new_serializable_node!(CommentDoc, CommentDocNode, CommentDocSNode { comment: BString });
new_serializable_node!(CommentLine, CommentLineNode, CommentLineSNode { comment: BString });
new_serializable_node!(Const, ConstNode<'a>, ConstSNode { items: Vec<SerializableNode>, });
new_serializable_node!(ConstProperty, ConstPropertyNode<'a>, ConstPropertySNode { const_type: Option<Box<SerializableNode>>, visibilities: Vec<Visibility>, items: Vec<SerializableNode>, });
new_serializable_node!(ConstructorParameter, ConstructorParameterNode<'a>, ConstructorParameterSNode { visibilities: Vec<Visibility>, modifier: Option<Modifier>, parameter: Box<SerializableNode>, });
new_serializable_node!(Continue, ContinueNode<'a>, ContinueSNode { statement: Option<Box<SerializableNode>>, });
new_serializable_node!(Declare, DeclareNode<'a>, DeclareSNode { arguments: Vec<SerializableNode>, body: Option<Box<SerializableNode>>, body_type: BodyType, });
new_serializable_node!(DeclareArgument, DeclareArgumentNode<'a>, DeclareArgumentSNode { name: Box<SerializableNode>, value: Box<SerializableNode>, });
new_serializable_node!(DoWhile, DoWhileNode<'a>, DoWhileSNode { condition: Box<SerializableNode>, body: Box<SerializableNode>, });
new_serializable_node!(DoWhileCondition, DoWhileConditionNode<'a>, DoWhileConditionSNode { condition: Box<SerializableNode>, });
new_serializable_node!(Echo, EchoNode<'a>, EchoSNode { items: Vec<SerializableNode>, });
new_serializable_node!(Else, ElseNode<'a>, ElseSNode { body: Box<SerializableNode>, is_short: bool, });
new_serializable_node!(Encapsed, EncapsedNode<'a>, EncapsedSNode { quote: Quote, values: Vec<SerializableNode>, });
new_serializable_node!(EncapsedPart, EncapsedPartNode<'a>, EncapsedPartSNode { is_advanced: bool, value: Box<SerializableNode>, });
new_serializable_node!(Enum, EnumNode<'a>, EnumSNode { name: Box<SerializableNode>, enum_type: Option<Box<SerializableNode>>, implements: Option<Box<SerializableNode>>, body: Vec<SerializableNode>, });
new_serializable_node!(EnumItem, EnumItemNode<'a>, EnumItemSNode { value: Box<SerializableNode>, });
new_serializable_node!(Eval, EvalNode<'a>, EvalSNode { statement: Box<SerializableNode>, });
new_serializable_node!(Exit, ExitNode<'a>, ExitSNode { statement: Option<Box<SerializableNode>>, });
new_serializable_node!(Finally, FinallyNode<'a>, FinallySNode { body: Box<SerializableNode>, });
new_serializable_node!(For, ForNode<'a>, ForSNode { inits: Vec<SerializableNode>, tests: Vec<SerializableNode>, increments: Vec<SerializableNode>, body: Option<Box<SerializableNode>>, body_type: BodyType, });
new_serializable_node!(Foreach, ForeachNode<'a>, ForeachSNode { source: Box<SerializableNode>, key: Option<Box<SerializableNode>>, value: Box<SerializableNode>, body: Box<SerializableNode>, is_short: bool, });
new_serializable_node!(Function, FunctionNode<'a>, FunctionSNode { is_ref: bool, name: Box<SerializableNode>, parameters: Vec<SerializableNode>, return_type: Option<Box<SerializableNode>>, body: Option<Box<SerializableNode>>, });
new_serializable_node!(Global, GlobalNode<'a>, GlobalSNode { items: Vec<SerializableNode>, });
new_serializable_node!(Goto, GotoNode<'a>, GotoSNode { label: Box<SerializableNode>, });
new_serializable_node!(HaltCompiler, HaltCompilerNode, HaltCompilerSNode {});
new_serializable_node!(HereDoc, HereDocNode<'a>, HereDocSNode { label: BString, values: Vec<SerializableNode>, });
new_serializable_node!(Identifier, IdentifierNode, IdentifierSNode { name: BString });
new_serializable_node!(If, IfNode<'a>, IfSNode { condition: Box<SerializableNode>, valid: Box<SerializableNode>, invalid: Option<Box<SerializableNode>>, is_short: bool, });
new_serializable_node!(Include, IncludeNode<'a>, IncludeSNode { use_parenthesis: bool, is_require: bool, is_once: bool, argument: Box<SerializableNode>, });
new_serializable_node!(Inline, InlineNode, InlineSNode { text: BString });
new_serializable_node!(Interface, InterfaceNode<'a>, InterfaceSNode { name: Box<SerializableNode>, extends: Vec<SerializableNode>, body: Box<SerializableNode>, });
new_serializable_node!(IntersectionType, IntersectionTypeNode<'a>, IntersectionTypeSNode { types: Vec<SerializableNode>, });
new_serializable_node!(Label, LabelNode<'a>, LabelSNode { label: Box<SerializableNode>, });
new_serializable_node!(List, ListNode<'a>, ListSNode { items: Vec<SerializableNode>, });
new_serializable_node!(Magic, MagicNode, MagicSNode { name: MagicName });
new_serializable_node!(MagicMethod, MagicMethodNode, MagicMethodSNode { name: MagicMethodName });
new_serializable_node!(Match, MatchNode<'a>, MatchSNode { condition: Box<SerializableNode>, arms: Vec<SerializableNode>, });
new_serializable_node!(MatchArm, MatchArmNode<'a>, MatchArmSNode { conditions: Vec<SerializableNode>, expr: Box<SerializableNode>, });
new_serializable_node!(Method, MethodNode<'a>, MethodSNode { visibility: Option<Visibility>, inheritance: Option<Inheritance>, is_static: bool, function: Box<SerializableNode>, });
new_serializable_node!(Namespace, NamespaceNode<'a>, NamespaceSNode { name: BString, body: Box<SerializableNode>, is_bracket: bool, });
new_serializable_node!(Negate, NegateNode<'a>, NegateSNode { statement: Box<SerializableNode>, });
new_serializable_node!(New, NewNode<'a>, NewSNode { statement: Box<SerializableNode>, });
new_serializable_node!(NowDoc, NowDocNode, NowDocSNode { label: BString, value: BString });
new_serializable_node!(Null, NullNode, NullSNode {});
new_serializable_node!(Number, NumberNode, NumberSNode { value: BString });
new_serializable_node!(ObjectAccess, ObjectAccessNode<'a>, ObjectAccessSNode { object: Box<SerializableNode>, property: Box<SerializableNode>, use_bracket: bool, is_nullsafe: bool, });
new_serializable_node!(Parameter, ParameterNode<'a>, ParameterSNode { variable_type: Option<Box<SerializableNode>>, is_ref: bool, is_ellipsis: bool, name: Box<SerializableNode>, value: Option<Box<SerializableNode>>, });
new_serializable_node!(Parent, ParentNode, ParentSNode {});
new_serializable_node!(Parenthesis, ParenthesisNode<'a>, ParenthesisSNode { statement: Box<SerializableNode>, });
new_serializable_node!(Post, PostNode<'a>, PostSNode { statement: Box<SerializableNode>, operator: PostType, });
new_serializable_node!(Pre, PreNode<'a>, PreSNode { statement: Box<SerializableNode>, operator: PreType, });
new_serializable_node!(Print, PrintNode<'a>, PrintSNode { statement: Box<SerializableNode>, });
new_serializable_node!(Program, ProgramNode<'a>, ProgramSNode { children: Vec<SerializableNode>, });
new_serializable_node!(Property, PropertyNode<'a>, PropertySNode { visibilities: Vec<Visibility>, modifier: Option<Modifier>, hooks: Vec<SerializableNode>, items: Vec<SerializableNode>, });
new_serializable_node!(PropertyHook, PropertyHookNode<'a>, PropertyHookSNode { is_get: bool, is_ref: bool, parameters: Vec<SerializableNode>, body: Box<SerializableNode>, });
new_serializable_node!(PropertyItem, PropertyItemNode<'a>, PropertyItemSNode { name: Box<SerializableNode>, variable_type: Option<Box<SerializableNode>>, value: Option<Box<SerializableNode>>, });
new_serializable_node!(Reference, ReferenceNode<'a>, ReferenceSNode { statement: Box<SerializableNode>, });
new_serializable_node!(Return, ReturnNode<'a>, ReturnSNode { statement: Option<Box<SerializableNode>>, });
new_serializable_node!(SelfKeyword, SelfNode, SelfSNode {});
new_serializable_node!(Silent, SilentNode<'a>, SilentSNode { statement: Box<SerializableNode>, });
new_serializable_node!(Static, StaticNode<'a>, StaticSNode { items: Vec<SerializableNode>, });
new_serializable_node!(StaticKeyword, StaticKeywordNode, StaticKeywordSNode {});
new_serializable_node!(StaticLookup, StaticLookupNode<'a>, StaticLookupSNode { left: Box<SerializableNode>, right: Box<SerializableNode>, use_bracket: bool, });
new_serializable_node!(String, StringNode, StringSNode { quote: Quote, value: BString });
new_serializable_node!(Switch, SwitchNode<'a>, SwitchSNode { condition: Box<SerializableNode>, body: Box<SerializableNode>, is_short: bool, });
new_serializable_node!(Ternary, TernaryNode<'a>, TernarySNode { condition: Box<SerializableNode>, valid: Box<SerializableNode>, invalid: Box<SerializableNode>, });
new_serializable_node!(This, ThisNode, ThisSNode {});
new_serializable_node!(Throw, ThrowNode<'a>, ThrowSNode { statement: Box<SerializableNode>, });
new_serializable_node!(Trait, TraitNode<'a>, TraitSNode { name: Box<SerializableNode>, body: Box<SerializableNode>, });
new_serializable_node!(TraitUse, TraitUseNode<'a>, TraitUseSNode { traits: Vec<SerializableNode>, adaptations: Vec<SerializableNode>, });
new_serializable_node!(TraitUseAlias, TraitUseAliasNode<'a>, TraitUseAliasSNode { trait_name: Option<Box<SerializableNode>>, method: Box<SerializableNode>, alias: Option<Box<SerializableNode>>, visibility: Option<Visibility>, });
new_serializable_node!(TraitUsePrecedence, TraitUsePrecedenceNode<'a>, TraitUsePrecedenceSNode { trait_name: Option<Box<SerializableNode>>, method: Box<SerializableNode>, instead: Box<SerializableNode>, });
new_serializable_node!(Try, TryNode<'a>, TrySNode { body: Box<SerializableNode>, catches: Vec<SerializableNode>, });
new_serializable_node!(Type, TypeNode, TypeSNode { is_nullable: bool, name: BString });
new_serializable_node!(UnionType, UnionTypeNode<'a>, UnionTypeSNode { types: Vec<SerializableNode>, });
new_serializable_node!(Use, UseNode<'a>, UseSNode { name: Option<BString>, items: Vec<SerializableNode>, });
new_serializable_node!(UseItem, UseItemNode<'a>, UseItemSNode { modifier: Option<UseItemModifier>, name: BString, alias: Option<Box<SerializableNode>>, });
new_serializable_node!(Variable, VariableNode<'a>, VariableSNode { name: Box<SerializableNode>, });
new_serializable_node!(Variadic, VariadicNode<'a>, VariadicSNode { statement: Option<Box<SerializableNode>>, });
new_serializable_node!(While, WhileNode<'a>, WhileSNode { condition: Box<SerializableNode>, body: Box<SerializableNode>, is_short: bool, });
new_serializable_node!(Yield, YieldNode<'a>, YieldSNode { key: Option<Box<SerializableNode>>, value: Option<Box<SerializableNode>>, });
new_serializable_node!(YieldFrom, YieldFromNode<'a>, YieldFromSNode { statement: Box<SerializableNode>, });

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
