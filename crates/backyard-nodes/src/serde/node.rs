use bumpalo::Bump;
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
  NodeType,
  PostType,
  PreType,
  Quote,
  RangeLocation,
  UseItemModifier,
  Visibility,
  serde::serialize::Serializable,
  serde::deserialize::Deserializable,
};

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

    impl<'arena> Deserializable<'arena> for $serializable_node_name {
      type Return = crate::$struct_name<'arena>;
      fn to_deserialize_in(&self, arena: &'arena Bump) -> Self::Return {
        crate::$struct_name {
          $($field_name: self.$field_name.to_deserialize_in(arena)),*
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

    #[allow(unused_variables)]
    impl<'arena> Deserializable<'arena> for $serializable_node_name {
      type Return = crate::$struct_name;
      fn to_deserialize_in(&self, arena: &'arena Bump) -> Self::Return {
        crate::$struct_name {
          $($field_name: self.$field_name.to_deserialize_in(arena)),*
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

#[cfg(test)]
mod tests {
  use crate::{ builder::{ BlueprintBuildable, BoxBlueprint, Builder }, AssignmentType };

  #[test]
  fn serde() {
    let arena = bumpalo::Bump::new();
    let b = Builder::new();
    let node = b
      .Program(
        &[
          b
            .Assignment(b.Variable(b.Identifier("a")), AssignmentType::Default, b.Number("21"))
            .add_leading(b.CommentLine("Test leading")),
        ]
      )
      .build(&arena);
    let serialized = node.serializable();
    let deserialized = serialized.deserialize_in(&arena);
    assert!(node.as_ref() == &deserialized);
  }
}
