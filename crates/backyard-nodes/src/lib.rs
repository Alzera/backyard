pub mod macros;
pub mod utils;

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(feature = "walker")]
pub mod walker;

#[cfg(feature = "builder")]
pub mod builder;

#[cfg(feature = "printer")]
pub mod printer;

use std::fmt::{ self, Display, Formatter };

use bstr::BString;
use bumpalo::Bump;

#[cfg(feature = "serde")]
use ::serde::{ Serialize, Deserialize };

use crate::utils::CloneIn;

#[cfg(feature = "builder")]
use crate::builder::{ Blueprint, BlueprintBuildable, BlueprintWrapper, Builder };

#[cfg(feature = "walker")]
use crate::walker::{ Walkable, WalkerItem };

#[cfg(feature = "printer")]
use crate::printer::{ PrintBuilder, Printable, PrintType, PrintConfig };

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum BodyType {
  Basic,
  Short,
  Empty,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct RangeLocation {
  pub start: Location,
  pub end: Location,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
  pub line: u32,
  pub column: u32,
  pub offset: u32,
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, PartialEq)]
pub struct Node<'a> {
  pub node_type: NodeType,
  pub wrapper: NodeWrapper<'a>,
  pub loc: Option<RangeLocation>,
  pub leadings: Option<bumpalo::collections::Vec<'a, Node<'a>>>,
  pub trailings: Option<bumpalo::collections::Vec<'a, Node<'a>>>,
}

impl<'a> Node<'a> {
  pub fn new(node_type: NodeType, wrapper: NodeWrapper<'a>, loc: Option<RangeLocation>) -> Self {
    Self { node_type, wrapper, loc, leadings: None, trailings: None }
  }

  pub fn leadings_shift(&mut self, arena: &'a Bump, node: Node<'a>) {
    if let Some(leadings) = &mut self.leadings {
      leadings.insert(0, node);
    } else {
      self.leadings = Some(bumpalo::vec![in arena; node]);
    }
  }

  pub fn leadings_push(&mut self, arena: &'a Bump, node: Node<'a>) {
    if let Some(leadings) = &mut self.leadings {
      leadings.push(node);
    } else {
      self.leadings = Some(bumpalo::vec![in arena; node]);
    }
  }

  pub fn trailings_push(&mut self, arena: &'a Bump, node: Node<'a>) {
    if let Some(trailings) = &mut self.trailings {
      trailings.push(node);
    } else {
      self.trailings = Some(bumpalo::vec![in arena; node]);
    }
  }
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, PartialEq)]
pub enum NodeWrapper<'a> {
  AnonymousClass(AnonymousClassNode<'a>),
  AnonymousFunction(AnonymousFunctionNode<'a>),
  Argument(ArgumentNode<'a>),
  Array(ArrayNode<'a>),
  ArrayItem(ArrayItemNode<'a>),
  ArrayLookup(ArrayLookupNode<'a>),
  ArrowFunction(ArrowFunctionNode<'a>),
  Assignment(AssignmentNode<'a>),
  Attribute(AttributeNode<'a>),
  AttributeItem(AttributeItemNode<'a>),
  Bin(BinNode<'a>),
  Block(BlockNode<'a>),
  Boolean(BooleanNode),
  Break(BreakNode<'a>),
  Call(CallNode<'a>),
  Case(CaseNode<'a>),
  Cast(CastNode<'a>),
  Catch(CatchNode<'a>),
  Class(ClassNode<'a>),
  ClassKeyword(ClassKeywordNode),
  Clone(CloneNode<'a>),
  CommentBlock(CommentBlockNode),
  CommentDoc(CommentDocNode),
  CommentLine(CommentLineNode),
  Const(ConstNode<'a>),
  ConstProperty(ConstPropertyNode<'a>),
  ConstructorParameter(ConstructorParameterNode<'a>),
  Continue(ContinueNode<'a>),
  Declare(DeclareNode<'a>),
  DeclareArgument(DeclareArgumentNode<'a>),
  DoWhile(DoWhileNode<'a>),
  DoWhileCondition(DoWhileConditionNode<'a>),
  Echo(EchoNode<'a>),
  Else(ElseNode<'a>),
  Encapsed(EncapsedNode<'a>),
  EncapsedPart(EncapsedPartNode<'a>),
  Enum(EnumNode<'a>),
  EnumItem(EnumItemNode<'a>),
  Eval(EvalNode<'a>),
  Exit(ExitNode<'a>),
  Finally(FinallyNode<'a>),
  For(ForNode<'a>),
  Foreach(ForeachNode<'a>),
  Function(FunctionNode<'a>),
  Global(GlobalNode<'a>),
  Goto(GotoNode<'a>),
  HaltCompiler(HaltCompilerNode),
  HereDoc(HereDocNode<'a>),
  Identifier(IdentifierNode),
  If(IfNode<'a>),
  Include(IncludeNode<'a>),
  Inline(InlineNode),
  Interface(InterfaceNode<'a>),
  IntersectionType(IntersectionTypeNode<'a>),
  Label(LabelNode<'a>),
  List(ListNode<'a>),
  Magic(MagicNode),
  MagicMethod(MagicMethodNode),
  Match(MatchNode<'a>),
  MatchArm(MatchArmNode<'a>),
  Method(MethodNode<'a>),
  Namespace(NamespaceNode<'a>),
  Negate(NegateNode<'a>),
  New(NewNode<'a>),
  NowDoc(NowDocNode),
  Null(NullNode),
  Number(NumberNode),
  ObjectAccess(ObjectAccessNode<'a>),
  Parameter(ParameterNode<'a>),
  Parent(ParentNode),
  Parenthesis(ParenthesisNode<'a>),
  Post(PostNode<'a>),
  Pre(PreNode<'a>),
  Print(PrintNode<'a>),
  Program(ProgramNode<'a>),
  Property(PropertyNode<'a>),
  PropertyHook(PropertyHookNode<'a>),
  PropertyItem(PropertyItemNode<'a>),
  Reference(ReferenceNode<'a>),
  Return(ReturnNode<'a>),
  SelfKeyword(SelfNode),
  Silent(SilentNode<'a>),
  Static(StaticNode<'a>),
  StaticKeyword(StaticKeywordNode),
  StaticLookup(StaticLookupNode<'a>),
  String(StringNode),
  Switch(SwitchNode<'a>),
  Ternary(TernaryNode<'a>),
  This(ThisNode),
  Trait(TraitNode<'a>),
  TraitUse(TraitUseNode<'a>),
  TraitUseAlias(TraitUseAliasNode<'a>),
  TraitUsePrecedence(TraitUsePrecedenceNode<'a>),
  Throw(ThrowNode<'a>),
  Try(TryNode<'a>),
  Type(TypeNode),
  UnionType(UnionTypeNode<'a>),
  Use(UseNode<'a>),
  UseItem(UseItemNode<'a>),
  Variable(VariableNode<'a>),
  Variadic(VariadicNode<'a>),
  While(WhileNode<'a>),
  Yield(YieldNode<'a>),
  YieldFrom(YieldFromNode<'a>),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
  AnonymousClass,
  AnonymousFunction,
  Argument,
  Array,
  ArrayItem,
  ArrayLookup,
  ArrowFunction,
  Assignment,
  Attribute,
  AttributeItem,
  Bin,
  Block,
  Boolean,
  Break,
  Call,
  Case,
  Cast,
  Catch,
  Class,
  ClassKeyword,
  Clone,
  CommentBlock,
  CommentDoc,
  CommentLine,
  Const,
  ConstProperty,
  ConstructorParameter,
  Continue,
  Declare,
  DeclareArgument,
  DoWhile,
  DoWhileCondition,
  Echo,
  Else,
  Encapsed,
  EncapsedPart,
  Enum,
  EnumItem,
  Eval,
  Exit,
  Finally,
  For,
  Foreach,
  Function,
  Global,
  Goto,
  HaltCompiler,
  HereDoc,
  Identifier,
  If,
  Include,
  Inline,
  Interface,
  IntersectionType,
  Label,
  List,
  Magic,
  MagicMethod,
  Match,
  MatchArm,
  Method,
  Namespace,
  Negate,
  New,
  NowDoc,
  Null,
  Number,
  ObjectAccess,
  Parameter,
  Parent,
  Parenthesis,
  Post,
  Pre,
  Print,
  Program,
  Property,
  PropertyHook,
  PropertyItem,
  Reference,
  Return,
  SelfKeyword,
  Silent,
  Static,
  StaticKeyword,
  StaticLookup,
  String,
  Switch,
  Ternary,
  This,
  Trait,
  TraitUse,
  TraitUseAlias,
  TraitUsePrecedence,
  Throw,
  Try,
  Type,
  UnionType,
  Use,
  UseItem,
  Variable,
  Variadic,
  While,
  Yield,
  YieldFrom,
}

macro_rules! new_node {
  (
    $node_type:ident,
    $struct_name:ident < $lt:lifetime > { $($field_name:ident: $field_type:ty),* $(,)? },
    $blueprint_name:ident < $blt:lifetime > {
      $($blueprint_field_name:ident: $blueprint_field_type:ty),* $(,)?
    }
  ) => {
    #[cfg_attr(test, derive(Serialize))]
    #[derive(Debug, PartialEq)]
    pub struct $struct_name<$lt> {
      $(pub $field_name: $field_type),*
    }

    impl<$lt> $struct_name<$lt> {
      pub fn loc($($field_name: $field_type,)* loc: Option<RangeLocation>) -> Node<$lt> {
        Node {
          leadings: None,
          trailings: None,
          node_type: NodeType::$node_type,
          wrapper: NodeWrapper::$node_type(
            Self { $($field_name),* }
          ),
          loc
        }
      }
    }

    impl<'arena> CloneIn<'arena> for $struct_name<'_> {
      type Cloned = $struct_name<'arena>;

      #[inline]
      fn clone_in(&self, arena: &'arena Bump) -> Self::Cloned {
        $struct_name {
          $($field_name: self.$field_name.clone_in(arena)),*
        }
      }
    }

    #[cfg(feature = "walker")]
    impl<'arena> Walkable<'arena> for $struct_name<'arena> {
      fn populate_walks<'a>(&'a self, stack: &mut std::collections::VecDeque<WalkerItem<'arena, 'a>>, level: u16) {
        let next_level = level + 1;
        let mut scoped_stack = std::collections::VecDeque::new();
        $(self.$field_name.populate_walks(&mut scoped_stack, next_level);)*
        scoped_stack.into_iter().rev().for_each(|x| stack.push_back(x))
      }
    }

    #[cfg(feature = "builder")]
    #[derive(Debug)]
    pub struct $blueprint_name<$blt> {
      $(pub $blueprint_field_name: $blueprint_field_type),*
    }

    #[cfg(feature = "builder")]
    impl Builder {
      #[allow(non_snake_case)]
      pub fn $node_type<$blt>(&self, $($blueprint_field_name: $blueprint_field_type,)*) -> std::boxed::Box<Blueprint<$blt>> {
        std::boxed::Box::new(
          Blueprint {
            leadings: vec![],
            trailings: vec![],
            node_type: NodeType::$node_type,
            wrapper: BlueprintWrapper::$node_type(
              $blueprint_name {
                $($blueprint_field_name: $blueprint_field_name),*
              }
            )
          }
        )
      }
    }

    #[cfg(feature = "builder")]
    impl<'arena, $blt> BlueprintBuildable<'arena> for $blueprint_name<$blt> {
      type Result = Node<'arena>;

      fn build(&self, arena: &'arena Bump) -> Self::Result {
        $struct_name::loc(
          $(self.$field_name.build(arena),)*
          None
        )
      }
    }

    #[cfg(feature = "printer")]
    impl<$lt> Printable for $struct_name<$lt> {
      fn build_print(&self, config: &PrintConfig) -> PrintBuilder {
        let mut builder = PrintBuilder::new(PrintType::Object);
        builder.push_props(!(config.with_leading_trailing || config.with_location), &mut [
          $((stringify!($field_name), self.$field_name.build_print(config)),)*
        ]);
        builder.shift_new_line(stringify!($struct_name));
        builder
      }
    }
  };

  (
    $node_type:ident,
    $struct_name:ident { $($field_name:ident: $field_type:ty),* $(,)? },
    $blueprint_name:ident < $blt:lifetime > {
      $($blueprint_field_name:ident: $blueprint_field_type:ty),* $(,)?
    }
  ) => {
    #[cfg_attr(test, derive(Serialize))]
    #[derive(Debug, PartialEq)]
    pub struct $struct_name {
      $(pub $field_name: $field_type),*
    }

    impl $struct_name {
      pub fn loc<'a>($($field_name: $field_type,)* loc: Option<RangeLocation>) -> Node<'a> {
        Node::new(
          NodeType::$node_type,
          NodeWrapper::$node_type(
            Self { $($field_name),* }
          ),
          loc
        )
      }
    }

    impl<'arena> CloneIn<'arena> for $struct_name {
      type Cloned = $struct_name;

      #[inline]
      fn clone_in(&self, _: &'arena Bump) -> Self::Cloned {
        $struct_name {
          $($field_name: self.$field_name.clone()),*
        }
      }
    }

    #[cfg(feature = "walker")]
    impl<'arena> Walkable<'arena> for $struct_name {
      fn populate_walks<'a>(&'a self, stack: &mut std::collections::VecDeque<WalkerItem<'arena, 'a>>, level: u16) {
        let next_level = level + 1;
        let mut scoped_stack = std::collections::VecDeque::new();
        $(self.$field_name.populate_walks(&mut scoped_stack, next_level);)*
        scoped_stack.into_iter().rev().for_each(|x| stack.push_back(x))
      }
    }

    #[cfg(feature = "builder")]
    #[derive(Debug)]
    pub struct $blueprint_name<$blt> {
      $(pub $blueprint_field_name: $blueprint_field_type),*
    }

    #[cfg(feature = "builder")]
    impl Builder {
      #[allow(non_snake_case)]
      pub fn $node_type<$blt>(&self, $($blueprint_field_name: $blueprint_field_type,)*) -> std::boxed::Box<Blueprint<$blt>> {
        std::boxed::Box::new(
          Blueprint {
            leadings: vec![],
            trailings: vec![],
            node_type: NodeType::$node_type,
            wrapper: BlueprintWrapper::$node_type(
              $blueprint_name {
                $($blueprint_field_name: $blueprint_field_name),*
              }
            )
          }
        )
      }
    }

    #[cfg(feature = "builder")]
    impl<'arena, $blt> BlueprintBuildable<'arena> for $blueprint_name<$blt> {
      type Result = Node<'arena>;

      fn build(&self, arena: &'arena Bump) -> Self::Result {
        $struct_name::loc(
          $(self.$field_name.build(arena),)*
          None
        )
      }
    }

    #[cfg(feature = "printer")]
    impl Printable for $struct_name {
      fn build_print(&self, config: &PrintConfig) -> PrintBuilder {
        let mut builder = PrintBuilder::new(PrintType::Object);
        builder.push_props(!(config.with_leading_trailing || config.with_location), &mut [
          $((stringify!($field_name), self.$field_name.build_print(config)),)*
        ]);
        builder.shift_new_line(stringify!($struct_name));
        builder
      }
    }
  };

  (
    $node_type:ident,
    $struct_name:ident { $($field_name:ident: $field_type:ty),* $(,)? },
    $blueprint_name:ident { $($blueprint_field_name:ident: $blueprint_field_type:ty),* $(,)? }
  ) => {
    #[cfg_attr(test, derive(Serialize))]
    #[derive(Debug, PartialEq)]
    pub struct $struct_name {
      $(pub $field_name: $field_type),*
    }

    impl $struct_name {
      pub fn loc<'a>($($field_name: $field_type,)* loc: Option<RangeLocation>) -> Node<'a> {
        Node::new(
          NodeType::$node_type,
          NodeWrapper::$node_type(
            Self { $($field_name),* }
          ),
          loc
        )
      }
    }

    impl<'arena> CloneIn<'arena> for $struct_name {
      type Cloned = $struct_name;

      #[inline]
      fn clone_in(&self, _: &'arena Bump) -> Self::Cloned {
        $struct_name {
          $($field_name: self.$field_name.clone()),*
        }
      }
    }

    #[cfg(feature = "walker")]
    impl<'arena> Walkable<'arena> for $struct_name {

      #[allow(unused_variables, unused_mut)]
      fn populate_walks<'a>(&'a self, stack: &mut std::collections::VecDeque<WalkerItem<'arena, 'a>>, level: u16) {
        let next_level = level + 1;
        let mut scoped_stack = std::collections::VecDeque::new();
        $(self.$field_name.populate_walks(&mut scoped_stack, next_level);)*
        scoped_stack.into_iter().rev().for_each(|x| stack.push_back(x))
      }
    }

    #[cfg(feature = "builder")]
    #[derive(Debug)]
    pub struct $blueprint_name {
      $(pub $blueprint_field_name: $blueprint_field_type),*
    }

    #[cfg(feature = "builder")]
    impl Builder {
      #[allow(non_snake_case)]
      pub fn $node_type<'a>(&self, $($blueprint_field_name: $blueprint_field_type,)*) -> std::boxed::Box<Blueprint<'a>> {
        std::boxed::Box::new(
          Blueprint {
            leadings: vec![],
            trailings: vec![],
            node_type: NodeType::$node_type,
            wrapper: BlueprintWrapper::$node_type(
              $blueprint_name {
                $($blueprint_field_name: $blueprint_field_name),*
              }
            )
          }
        )
      }
    }

    #[cfg(feature = "builder")]
    impl<'arena> BlueprintBuildable<'arena> for $blueprint_name {
      type Result = Node<'arena>;

      #[allow(unused_variables)]
      fn build(&self, arena: &'arena Bump) -> Self::Result {
        $struct_name::loc(
          $(self.$field_name.build(arena),)*
          None
        )
      }
    }

    #[cfg(feature = "printer")]
    impl Printable for $struct_name {

      #[allow(unused_variables)]
      fn build_print(&self, config: &PrintConfig) -> PrintBuilder {
        let mut builder = PrintBuilder::new(PrintType::Object);
        builder.push_props(!(config.with_leading_trailing || config.with_location), &mut [
          $((stringify!($field_name), self.$field_name.build_print(config)),)*
        ]);
        builder.shift_new_line(stringify!($struct_name));
        builder
      }
    }
  };
}

new_node!(AnonymousClass, AnonymousClassNode<'a> { parameters: bumpalo::collections::Vec<'a, Node<'a>>, extends: Option<bumpalo::boxed::Box<'a, Node<'a>>>, implements: bumpalo::collections::Vec<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, AnonymousClassBlueprint<'b> { parameters: &'b [Box<Blueprint<'b>>], extends: Option<Box<Blueprint<'b>>>, implements: &'b [Box<Blueprint<'b>>], body: Box<Blueprint<'b>>, });
new_node!(AnonymousFunction, AnonymousFunctionNode<'a> { is_ref: bool, parameters: bumpalo::collections::Vec<'a, Node<'a>>, uses: bumpalo::collections::Vec<'a, Node<'a>>, return_type: Option<bumpalo::boxed::Box<'a, Node<'a>>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, AnonymousFunctionBlueprint<'b> { is_ref: bool, parameters: &'b [Box<Blueprint<'b>>], uses: &'b [Box<Blueprint<'b>>], return_type: Option<Box<Blueprint<'b>>>, body: Box<Blueprint<'b>>, });
new_node!(Argument, ArgumentNode<'a> { name: Option<bumpalo::boxed::Box<'a, Node<'a>>>, value: bumpalo::boxed::Box<'a, Node<'a>>, }, ArgumentBlueprint<'b> { name: Option<Box<Blueprint<'b>>>, value: Box<Blueprint<'b>>, });
new_node!(Array, ArrayNode<'a> { is_short: bool, items: bumpalo::collections::Vec<'a, Node<'a>>, }, ArrayBlueprint<'b> { is_short: bool, items: &'b [Box<Blueprint<'b>>], });
new_node!(ArrayItem, ArrayItemNode<'a> { key: Option<bumpalo::boxed::Box<'a, Node<'a>>>, value: bumpalo::boxed::Box<'a, Node<'a>>, }, ArrayItemBlueprint<'b> { key: Option<Box<Blueprint<'b>>>, value: Box<Blueprint<'b>>, });
new_node!(ArrayLookup, ArrayLookupNode<'a> { left: bumpalo::boxed::Box<'a, Node<'a>>, right: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, ArrayLookupBlueprint<'b> { left: Box<Blueprint<'b>>, right: Option<Box<Blueprint<'b>>>, });
new_node!(ArrowFunction, ArrowFunctionNode<'a> { is_ref: bool, parameters: bumpalo::collections::Vec<'a, Node<'a>>, return_type: Option<bumpalo::boxed::Box<'a, Node<'a>>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, ArrowFunctionBlueprint<'b> { is_ref: bool, parameters: &'b [Box<Blueprint<'b>>], return_type: Option<Box<Blueprint<'b>>>, body: Box<Blueprint<'b>>, });
new_node!(Assignment, AssignmentNode<'a> { left: bumpalo::boxed::Box<'a, Node<'a>>, operator: AssignmentType, right: bumpalo::boxed::Box<'a, Node<'a>>, }, AssignmentBlueprint<'b> { left: Box<Blueprint<'b>>, operator: AssignmentType, right: Box<Blueprint<'b>>, });
new_node!(Attribute, AttributeNode<'a> { items: bumpalo::collections::Vec<'a, Node<'a>>, }, AttributeBlueprint<'b> { items: &'b [Box<Blueprint<'b>>], });
new_node!(AttributeItem, AttributeItemNode<'a> { name: BString, arguments: bumpalo::collections::Vec<'a, Node<'a>>, }, AttributeItemBlueprint<'b> { name: &'b str, arguments: &'b [Box<Blueprint<'b>>], });
new_node!(Bin, BinNode<'a> { left: bumpalo::boxed::Box<'a, Node<'a>>, operator: BinaryType, right: bumpalo::boxed::Box<'a, Node<'a>>, }, BinBlueprint<'b> { left: Box<Blueprint<'b>>, operator: BinaryType, right: Box<Blueprint<'b>>, });
new_node!(Block, BlockNode<'a> { statements: bumpalo::collections::Vec<'a, Node<'a>>, }, BlockBlueprint<'b> { statements: &'b [Box<Blueprint<'b>>], });
new_node!(Boolean, BooleanNode { is_true: bool }, BooleanBlueprint { is_true: bool });
new_node!(Break, BreakNode<'a> { statement: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, BreakBlueprint<'b> { statement: Option<Box<Blueprint<'b>>>, });
new_node!(Call, CallNode<'a> { name: bumpalo::boxed::Box<'a, Node<'a>>, arguments: bumpalo::collections::Vec<'a, Node<'a>>, }, CallBlueprint<'b> { name: Box<Blueprint<'b>>, arguments: &'b [Box<Blueprint<'b>>], });
new_node!(Case, CaseNode<'a> { condition: Option<bumpalo::boxed::Box<'a, Node<'a>>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, CaseBlueprint<'b> { condition: Option<Box<Blueprint<'b>>>, body: Box<Blueprint<'b>>, });
new_node!(Cast, CastNode<'a> { cast_type: CastType, expression: bumpalo::boxed::Box<'a, Node<'a>>, }, CastBlueprint<'b> { cast_type: CastType, expression: Box<Blueprint<'b>>, });
new_node!(Catch, CatchNode<'a> { types: bumpalo::collections::Vec<'a, Node<'a>>, variable: Option<bumpalo::boxed::Box<'a, Node<'a>>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, CatchBlueprint<'b> { types: &'b [Box<Blueprint<'b>>], variable: Option<Box<Blueprint<'b>>>, body: Box<Blueprint<'b>>, });
new_node!(Class, ClassNode<'a> { inheritance: Option<Inheritance>, name: Option<bumpalo::boxed::Box<'a, Node<'a>>>, extends: Option<bumpalo::boxed::Box<'a, Node<'a>>>, implements: bumpalo::collections::Vec<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, is_readonly: bool, }, ClassBlueprint<'b> { inheritance: Option<Inheritance>, name: Option<Box<Blueprint<'b>>>, extends: Option<Box<Blueprint<'b>>>, implements: &'b [Box<Blueprint<'b>>], body: Box<Blueprint<'b>>, is_readonly: bool, });
new_node!(ClassKeyword, ClassKeywordNode {}, ClassKeywordBlueprint {});
new_node!(Clone, CloneNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, CloneBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(CommentBlock, CommentBlockNode { comment: BString, }, CommentBlockBlueprint<'b> { comment: &'b str, });
new_node!(CommentDoc, CommentDocNode { comment: BString, }, CommentDocBlueprint<'b> { comment: &'b str, });
new_node!(CommentLine, CommentLineNode { comment: BString, }, CommentLineBlueprint<'b> { comment: &'b str, });
new_node!(Const, ConstNode<'a> { items: bumpalo::collections::Vec<'a, Node<'a>>, }, ConstBlueprint<'b> { items: &'b [Box<Blueprint<'b>>], });
new_node!(ConstProperty, ConstPropertyNode<'a> { const_type: Option<bumpalo::boxed::Box<'a, Node<'a>>>, visibilities: Vec<Visibility>, items: bumpalo::collections::Vec<'a, Node<'a>>, }, ConstPropertyBlueprint<'b> { const_type: Option<Box<Blueprint<'b>>>, visibilities: Vec<Visibility>, items: &'b [Box<Blueprint<'b>>], });
new_node!(ConstructorParameter, ConstructorParameterNode<'a> { visibilities: Vec<Visibility>, modifier: Option<Modifier>, parameter: bumpalo::boxed::Box<'a, Node<'a>>, }, ConstructorParameterBlueprint<'b> { visibilities: Vec<Visibility>, modifier: Option<Modifier>, parameter: Box<Blueprint<'b>>, });
new_node!(Continue, ContinueNode<'a> { statement: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, ContinueBlueprint<'b> { statement: Option<Box<Blueprint<'b>>>, });
new_node!(Declare, DeclareNode<'a> { arguments: bumpalo::collections::Vec<'a, Node<'a>>, body: Option<bumpalo::boxed::Box<'a, Node<'a>>>, body_type: BodyType, }, DeclareBlueprint<'b> { arguments: &'b [Box<Blueprint<'b>>], body: Option<Box<Blueprint<'b>>>, body_type: BodyType, });
new_node!(DeclareArgument, DeclareArgumentNode<'a> { name: bumpalo::boxed::Box<'a, Node<'a>>, value: bumpalo::boxed::Box<'a, Node<'a>>, }, DeclareArgumentBlueprint<'b> { name: Box<Blueprint<'b>>, value: Box<Blueprint<'b>>, });
new_node!(DoWhile, DoWhileNode<'a> { condition: bumpalo::boxed::Box<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, DoWhileBlueprint<'b> { condition: Box<Blueprint<'b>>, body: Box<Blueprint<'b>>, });
new_node!(DoWhileCondition, DoWhileConditionNode<'a> { condition: bumpalo::boxed::Box<'a, Node<'a>>, }, DoWhileConditionBlueprint<'b> { condition: Box<Blueprint<'b>>, });
new_node!(Echo, EchoNode<'a> { items: bumpalo::collections::Vec<'a, Node<'a>>, }, EchoBlueprint<'b> { items: &'b [Box<Blueprint<'b>>], });
new_node!(Else, ElseNode<'a> { body: bumpalo::boxed::Box<'a, Node<'a>>, is_short: bool, }, ElseBlueprint<'b> { body: Box<Blueprint<'b>>, is_short: bool, });
new_node!(Encapsed, EncapsedNode<'a> { quote: Quote, values: bumpalo::collections::Vec<'a, Node<'a>>, }, EncapsedBlueprint<'b> { quote: Quote, values: &'b [Box<Blueprint<'b>>], });
new_node!(EncapsedPart, EncapsedPartNode<'a> { is_advanced: bool, value: bumpalo::boxed::Box<'a, Node<'a>>, }, EncapsedPartBlueprint<'b> { is_advanced: bool, value: Box<Blueprint<'b>>, });
new_node!(Enum, EnumNode<'a> { name: bumpalo::boxed::Box<'a, Node<'a>>, enum_type: Option<bumpalo::boxed::Box<'a, Node<'a>>>, implements: Option<bumpalo::boxed::Box<'a, Node<'a>>>, body: bumpalo::collections::Vec<'a, Node<'a>>, }, EnumBlueprint<'b> { name: Box<Blueprint<'b>>, enum_type: Option<Box<Blueprint<'b>>>, implements: Option<Box<Blueprint<'b>>>, body: &'b [Box<Blueprint<'b>>], });
new_node!(EnumItem, EnumItemNode<'a> { value: bumpalo::boxed::Box<'a, Node<'a>>, }, EnumItemBlueprint<'b> { value: Box<Blueprint<'b>>, });
new_node!(Eval, EvalNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, EvalBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(Exit, ExitNode<'a> { statement: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, ExitBlueprint<'b> { statement: Option<Box<Blueprint<'b>>>, });
new_node!(Finally, FinallyNode<'a> { body: bumpalo::boxed::Box<'a, Node<'a>>, }, FinallyBlueprint<'b> { body: Box<Blueprint<'b>>, });
new_node!(For, ForNode<'a> { inits: bumpalo::collections::Vec<'a, Node<'a>>, tests: bumpalo::collections::Vec<'a, Node<'a>>, increments: bumpalo::collections::Vec<'a, Node<'a>>, body: Option<bumpalo::boxed::Box<'a, Node<'a>>>, body_type: BodyType, }, ForBlueprint<'b> { inits: &'b [Box<Blueprint<'b>>], tests: &'b [Box<Blueprint<'b>>], increments: &'b [Box<Blueprint<'b>>], body: Option<Box<Blueprint<'b>>>, body_type: BodyType, });
new_node!(Foreach, ForeachNode<'a> { source: bumpalo::boxed::Box<'a, Node<'a>>, key: Option<bumpalo::boxed::Box<'a, Node<'a>>>, value: bumpalo::boxed::Box<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, is_short: bool, }, ForeachBlueprint<'b> { source: Box<Blueprint<'b>>, key: Option<Box<Blueprint<'b>>>, value: Box<Blueprint<'b>>, body: Box<Blueprint<'b>>, is_short: bool, });
new_node!(Function, FunctionNode<'a> { is_ref: bool, name: bumpalo::boxed::Box<'a, Node<'a>>, parameters: bumpalo::collections::Vec<'a, Node<'a>>, return_type: Option<bumpalo::boxed::Box<'a, Node<'a>>>, body: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, FunctionBlueprint<'b> { is_ref: bool, name: Box<Blueprint<'b>>, parameters: &'b [Box<Blueprint<'b>>], return_type: Option<Box<Blueprint<'b>>>, body: Option<Box<Blueprint<'b>>>, });
new_node!(Global, GlobalNode<'a> { items: bumpalo::collections::Vec<'a, Node<'a>>, }, GlobalBlueprint<'b> { items: &'b [Box<Blueprint<'b>>], });
new_node!(Goto, GotoNode<'a> { label: bumpalo::boxed::Box<'a, Node<'a>>, }, GotoBlueprint<'b> { label: Box<Blueprint<'b>>, });
new_node!(HaltCompiler, HaltCompilerNode {}, HaltCompilerBlueprint {});
new_node!(HereDoc, HereDocNode<'a> { label: BString, values: bumpalo::collections::Vec<'a, Node<'a>>, }, HereDocBlueprint<'b> { label: &'b str, values: &'b [Box<Blueprint<'b>>], });
new_node!(Identifier, IdentifierNode { name: BString, }, IdentifierBlueprint<'b> { name: &'b str, });
new_node!(If, IfNode<'a> { condition: bumpalo::boxed::Box<'a, Node<'a>>, valid: bumpalo::boxed::Box<'a, Node<'a>>, invalid: Option<bumpalo::boxed::Box<'a, Node<'a>>>, is_short: bool, }, IfBlueprint<'b> { condition: Box<Blueprint<'b>>, valid: Box<Blueprint<'b>>, invalid: Option<Box<Blueprint<'b>>>, is_short: bool, });
new_node!(Include, IncludeNode<'a> { use_parenthesis: bool, is_require: bool, is_once: bool, argument: bumpalo::boxed::Box<'a, Node<'a>>, }, IncludeBlueprint<'b> { use_parenthesis: bool, is_require: bool, is_once: bool, argument: Box<Blueprint<'b>>, });
new_node!(Inline, InlineNode { text: BString, }, InlineBlueprint<'b> { text: &'b str, });
new_node!(Interface, InterfaceNode<'a> { name: bumpalo::boxed::Box<'a, Node<'a>>, extends: bumpalo::collections::Vec<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, InterfaceBlueprint<'b> { name: Box<Blueprint<'b>>, extends: &'b [Box<Blueprint<'b>>], body: Box<Blueprint<'b>>, });
new_node!(IntersectionType, IntersectionTypeNode<'a> { types: bumpalo::collections::Vec<'a, Node<'a>>, }, IntersectionTypeBlueprint<'b> { types: &'b [Box<Blueprint<'b>>], });
new_node!(Label, LabelNode<'a> { label: bumpalo::boxed::Box<'a, Node<'a>>, }, LabelBlueprint<'b> { label: Box<Blueprint<'b>>, });
new_node!(List, ListNode<'a> { items: bumpalo::collections::Vec<'a, Node<'a>>, }, ListBlueprint<'b> { items: &'b [Box<Blueprint<'b>>], });
new_node!(Magic, MagicNode { name: MagicName }, MagicBlueprint { name: MagicName });
new_node!(
  MagicMethod,
  MagicMethodNode { name: MagicMethodName },
  MagicMethodBlueprint { name: MagicMethodName }
);
new_node!(Match, MatchNode<'a> { condition: bumpalo::boxed::Box<'a, Node<'a>>, arms: bumpalo::collections::Vec<'a, Node<'a>>, }, MatchBlueprint<'b> { condition: Box<Blueprint<'b>>, arms: &'b [Box<Blueprint<'b>>], });
new_node!(MatchArm, MatchArmNode<'a> { conditions: bumpalo::collections::Vec<'a, Node<'a>>, expr: bumpalo::boxed::Box<'a, Node<'a>>, }, MatchArmBlueprint<'b> { conditions: &'b [Box<Blueprint<'b>>], expr: Box<Blueprint<'b>>, });
new_node!(Method, MethodNode<'a> { visibility: Option<Visibility>, inheritance: Option<Inheritance>, is_static: bool, function: bumpalo::boxed::Box<'a, Node<'a>>, }, MethodBlueprint<'b> { visibility: Option<Visibility>, inheritance: Option<Inheritance>, is_static: bool, function: Box<Blueprint<'b>>, });
new_node!(Namespace, NamespaceNode<'a> { name: BString, body: bumpalo::boxed::Box<'a, Node<'a>>, is_bracket: bool, }, NamespaceBlueprint<'b> { name: &'b str, body: Box<Blueprint<'b>>, is_bracket: bool, });
new_node!(Negate, NegateNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, NegateBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(New, NewNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, NewBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(NowDoc, NowDocNode { label: BString, value: BString, }, NowDocBlueprint<'b> { label: &'b str, value: &'b str, });
new_node!(Null, NullNode {}, NullBlueprint {});
new_node!(Number, NumberNode { value: BString, }, NumberBlueprint<'b> { value: &'b str, });
new_node!(ObjectAccess, ObjectAccessNode<'a> { object: bumpalo::boxed::Box<'a, Node<'a>>, property: bumpalo::boxed::Box<'a, Node<'a>>, use_bracket: bool, is_nullsafe: bool, }, ObjectAccessBlueprint<'b> { object: Box<Blueprint<'b>>, property: Box<Blueprint<'b>>, use_bracket: bool, is_nullsafe: bool, });
new_node!(Parameter, ParameterNode<'a> { variable_type: Option<bumpalo::boxed::Box<'a, Node<'a>>>, is_ref: bool, is_ellipsis: bool, name: bumpalo::boxed::Box<'a, Node<'a>>, value: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, ParameterBlueprint<'b> { variable_type: Option<Box<Blueprint<'b>>>, is_ref: bool, is_ellipsis: bool, name: Box<Blueprint<'b>>, value: Option<Box<Blueprint<'b>>>, });
new_node!(Parent, ParentNode {}, ParentBlueprint {});
new_node!(Parenthesis, ParenthesisNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, ParenthesisBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(Post, PostNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, operator: PostType, }, PostBlueprint<'b> { statement: Box<Blueprint<'b>>, operator: PostType, });
new_node!(Pre, PreNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, operator: PreType, }, PreBlueprint<'b> { statement: Box<Blueprint<'b>>, operator: PreType, });
new_node!(Print, PrintNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, PrintBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(Program, ProgramNode<'a> { children: bumpalo::collections::Vec<'a, Node<'a>>, }, ProgramBlueprint<'b> { children: &'b [Box<Blueprint<'b>>], });
new_node!(Property, PropertyNode<'a> { visibilities: Vec<Visibility>, modifier: Option<Modifier>, hooks: bumpalo::collections::Vec<'a, Node<'a>>, items: bumpalo::collections::Vec<'a, Node<'a>>, }, PropertyBlueprint<'b> { visibilities: Vec<Visibility>, modifier: Option<Modifier>, hooks: &'b [Box<Blueprint<'b>>], items: &'b [Box<Blueprint<'b>>], });
new_node!(PropertyHook, PropertyHookNode<'a> { is_get: bool, is_ref: bool, parameters: bumpalo::collections::Vec<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, PropertyHookBlueprint<'b> { is_get: bool, is_ref: bool, parameters: &'b [Box<Blueprint<'b>>], body: Box<Blueprint<'b>>, });
new_node!(PropertyItem, PropertyItemNode<'a> { name: bumpalo::boxed::Box<'a, Node<'a>>, variable_type: Option<bumpalo::boxed::Box<'a, Node<'a>>>, value: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, PropertyItemBlueprint<'b> { name: Box<Blueprint<'b>>, variable_type: Option<Box<Blueprint<'b>>>, value: Option<Box<Blueprint<'b>>>, });
new_node!(Reference, ReferenceNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, ReferenceBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(Return, ReturnNode<'a> { statement: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, ReturnBlueprint<'b> { statement: Option<Box<Blueprint<'b>>>, });
new_node!(SelfKeyword, SelfNode {}, SelfBlueprint {});
new_node!(Silent, SilentNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, SilentBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(Static, StaticNode<'a> { items: bumpalo::collections::Vec<'a, Node<'a>>, }, StaticBlueprint<'b> { items: &'b [Box<Blueprint<'b>>], });
new_node!(StaticKeyword, StaticKeywordNode {}, StaticKeywordBlueprint {});
new_node!(StaticLookup, StaticLookupNode<'a> { left: bumpalo::boxed::Box<'a, Node<'a>>, right: bumpalo::boxed::Box<'a, Node<'a>>, use_bracket: bool, }, StaticLookupBlueprint<'b> { left: Box<Blueprint<'b>>, right: Box<Blueprint<'b>>, use_bracket: bool, });
new_node!(String, StringNode { quote: Quote, value: BString, }, StringBlueprint<'b> { quote: Quote, value: &'b str, });
new_node!(Switch, SwitchNode<'a> { condition: bumpalo::boxed::Box<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, is_short: bool, }, SwitchBlueprint<'b> { condition: Box<Blueprint<'b>>, body: Box<Blueprint<'b>>, is_short: bool, });
new_node!(Ternary, TernaryNode<'a> { condition: bumpalo::boxed::Box<'a, Node<'a>>, valid: bumpalo::boxed::Box<'a, Node<'a>>, invalid: bumpalo::boxed::Box<'a, Node<'a>>, }, TernaryBlueprint<'b> { condition: Box<Blueprint<'b>>, valid: Box<Blueprint<'b>>, invalid: Box<Blueprint<'b>>, });
new_node!(This, ThisNode {}, ThisBlueprint {});
new_node!(Throw, ThrowNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, ThrowBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(Trait, TraitNode<'a> { name: bumpalo::boxed::Box<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, TraitBlueprint<'b> { name: Box<Blueprint<'b>>, body: Box<Blueprint<'b>>, });
new_node!(TraitUse, TraitUseNode<'a> { traits: bumpalo::collections::Vec<'a, Node<'a>>, adaptations: bumpalo::collections::Vec<'a, Node<'a>>, }, TraitUseBlueprint<'b> { traits: &'b [Box<Blueprint<'b>>], adaptations: &'b [Box<Blueprint<'b>>], });
new_node!(TraitUseAlias, TraitUseAliasNode<'a> { trait_name: Option<bumpalo::boxed::Box<'a, Node<'a>>>, method: bumpalo::boxed::Box<'a, Node<'a>>, alias: Option<bumpalo::boxed::Box<'a, Node<'a>>>, visibility: Option<Visibility>, }, TraitUseAliasBlueprint<'b> { trait_name: Option<Box<Blueprint<'b>>>, method: Box<Blueprint<'b>>, alias: Option<Box<Blueprint<'b>>>, visibility: Option<Visibility>, });
new_node!(TraitUsePrecedence, TraitUsePrecedenceNode<'a> { trait_name: Option<bumpalo::boxed::Box<'a, Node<'a>>>, method: bumpalo::boxed::Box<'a, Node<'a>>, instead: bumpalo::boxed::Box<'a, Node<'a>>, }, TraitUsePrecedenceBlueprint<'b> { trait_name: Option<Box<Blueprint<'b>>>, method: Box<Blueprint<'b>>, instead: Box<Blueprint<'b>>, });
new_node!(Try, TryNode<'a> { body: bumpalo::boxed::Box<'a, Node<'a>>, catches: bumpalo::collections::Vec<'a, Node<'a>>, }, TryBlueprint<'b> { body: Box<Blueprint<'b>>, catches: &'b [Box<Blueprint<'b>>], });
new_node!(Type, TypeNode { is_nullable: bool, name: BString, }, TypeBlueprint<'b> { is_nullable: bool, name: &'b str, });
new_node!(UnionType, UnionTypeNode<'a> { types: bumpalo::collections::Vec<'a, Node<'a>>, }, UnionTypeBlueprint<'b> { types: &'b [Box<Blueprint<'b>>], });
new_node!(Use, UseNode<'a> { name: Option<BString>, items: bumpalo::collections::Vec<'a, Node<'a>>, }, UseBlueprint<'b> { name: Option<&'b str>, items: &'b [Box<Blueprint<'b>>], });
new_node!(UseItem, UseItemNode<'a> { modifier: Option<UseItemModifier>, name: BString, alias: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, UseItemBlueprint<'b> { modifier: Option<UseItemModifier>, name: &'b str, alias: Option<Box<Blueprint<'b>>>, });
new_node!(Variable, VariableNode<'a> { name: bumpalo::boxed::Box<'a, Node<'a>>, }, VariableBlueprint<'b> { name: Box<Blueprint<'b>>, });
new_node!(Variadic, VariadicNode<'a> { statement: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, VariadicBlueprint<'b> { statement: Option<Box<Blueprint<'b>>>, });
new_node!(While, WhileNode<'a> { condition: bumpalo::boxed::Box<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, is_short: bool, }, WhileBlueprint<'b> { condition: Box<Blueprint<'b>>, body: Box<Blueprint<'b>>, is_short: bool, });
new_node!(Yield, YieldNode<'a> { key: Option<bumpalo::boxed::Box<'a, Node<'a>>>, value: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, YieldBlueprint<'b> { key: Option<Box<Blueprint<'b>>>, value: Option<Box<Blueprint<'b>>>, });
new_node!(YieldFrom, YieldFromNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, YieldFromBlueprint<'b> { statement: Box<Blueprint<'b>>, });

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum MagicMethodName {
  Construct,
  Destruct,
  Call,
  CallStatic,
  Get,
  Set,
  Isset,
  Unset,
  Sleep,
  Wakeup,
  Serialize,
  Unserialize,
  ToString,
  Invoke,
  SetState,
  Clone,
  DebugInfo,
}

impl TryFrom<&BString> for MagicMethodName {
  type Error = String;

  fn try_from(value: &BString) -> Result<Self, Self::Error> {
    match value.as_slice() {
      b"__construct" => Ok(MagicMethodName::Construct),
      b"__destruct" => Ok(MagicMethodName::Destruct),
      b"__call" => Ok(MagicMethodName::Call),
      b"__callStatic" => Ok(MagicMethodName::CallStatic),
      b"__get" => Ok(MagicMethodName::Get),
      b"__set" => Ok(MagicMethodName::Set),
      b"__isset" => Ok(MagicMethodName::Isset),
      b"__unset" => Ok(MagicMethodName::Unset),
      b"__sleep" => Ok(MagicMethodName::Sleep),
      b"__wakeup" => Ok(MagicMethodName::Wakeup),
      b"__serialize" => Ok(MagicMethodName::Serialize),
      b"__unserialize" => Ok(MagicMethodName::Unserialize),
      b"__toString" => Ok(MagicMethodName::ToString),
      b"__invoke" => Ok(MagicMethodName::Invoke),
      b"__set_state" => Ok(MagicMethodName::SetState),
      b"__clone" => Ok(MagicMethodName::Clone),
      b"__debugInfo" => Ok(MagicMethodName::DebugInfo),
      _ => Err(format!("Invalid magic method name: {}", value)),
    }
  }
}

impl Display for MagicMethodName {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match self {
      MagicMethodName::Construct => "__construct",
      MagicMethodName::Destruct => "__destruct",
      MagicMethodName::Call => "__call",
      MagicMethodName::CallStatic => "__callStatic",
      MagicMethodName::Get => "__get",
      MagicMethodName::Set => "__set",
      MagicMethodName::Isset => "__isset",
      MagicMethodName::Unset => "__unset",
      MagicMethodName::Sleep => "__sleep",
      MagicMethodName::Wakeup => "__wakeup",
      MagicMethodName::Serialize => "__serialize",
      MagicMethodName::Unserialize => "__unserialize",
      MagicMethodName::ToString => "__toString",
      MagicMethodName::Invoke => "__invoke",
      MagicMethodName::SetState => "__set_state",
      MagicMethodName::Clone => "__clone",
      MagicMethodName::DebugInfo => "__debugInfo",
    })
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum MagicName {
  Class,
  Dir,
  File,
  Function,
  Line,
  Method,
  Namespace,
  Trait,
  Property,
}

impl TryFrom<&BString> for MagicName {
  type Error = String;

  fn try_from(value: &BString) -> Result<Self, Self::Error> {
    match value.as_slice() {
      b"__CLASS__" => Ok(MagicName::Class),
      b"__DIR__" => Ok(MagicName::Dir),
      b"__FILE__" => Ok(MagicName::File),
      b"__FUNCTION__" => Ok(MagicName::Function),
      b"__LINE__" => Ok(MagicName::Line),
      b"__METHOD__" => Ok(MagicName::Method),
      b"__NAMESPACE__" => Ok(MagicName::Namespace),
      b"__TRAIT__" => Ok(MagicName::Trait),
      b"__PROPERTY__" => Ok(MagicName::Property),
      _ => Err(format!("Invalid magic name: {}", value)),
    }
  }
}

impl Display for MagicName {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match self {
      MagicName::Class => "__CLASS__",
      MagicName::Dir => "__DIR__",
      MagicName::File => "__FILE__",
      MagicName::Function => "__FUNCTION__",
      MagicName::Line => "__LINE__",
      MagicName::Method => "__METHOD__",
      MagicName::Namespace => "__NAMESPACE__",
      MagicName::Trait => "__TRAIT__",
      MagicName::Property => "__PROPERTY__",
    })
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum PreType {
  Increment,
  Decrement,
  Addition,
  Subtraction,
}

impl TryFrom<&BString> for PreType {
  type Error = String;

  fn try_from(value: &BString) -> Result<Self, Self::Error> {
    match value.as_slice() {
      b"++" => Ok(PreType::Increment),
      b"--" => Ok(PreType::Decrement),
      b"+" => Ok(PreType::Addition),
      b"-" => Ok(PreType::Subtraction),
      _ => Err(format!("Invalid pre type: {}", value)),
    }
  }
}

impl Display for PreType {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match self {
      PreType::Increment => "++",
      PreType::Decrement => "--",
      PreType::Addition => "+",
      PreType::Subtraction => "-",
    })
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum PostType {
  Increment,
  Decrement,
}

impl TryFrom<&BString> for PostType {
  type Error = String;

  fn try_from(value: &BString) -> Result<Self, Self::Error> {
    match value.as_slice() {
      b"++" => Ok(PostType::Increment),
      b"--" => Ok(PostType::Decrement),
      _ => Err(format!("Invalid post type: {}", value)),
    }
  }
}

impl Display for PostType {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match self {
      PostType::Increment => "++",
      PostType::Decrement => "--",
    })
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum CastType {
  Int,
  Integer,
  Bool,
  Boolean,
  Float,
  Double,
  Real,
  String,
  Binary,
  Array,
  Object,
  Unset,
}

impl TryFrom<&BString> for CastType {
  type Error = String;

  fn try_from(value: &BString) -> Result<Self, Self::Error> {
    match value.as_slice() {
      b"int" => Ok(CastType::Int),
      b"integer" => Ok(CastType::Integer),
      b"bool" => Ok(CastType::Bool),
      b"boolean" => Ok(CastType::Boolean),
      b"float" => Ok(CastType::Float),
      b"double" => Ok(CastType::Double),
      b"real" => Ok(CastType::Real),
      b"string" => Ok(CastType::String),
      b"binary" => Ok(CastType::Binary),
      b"array" => Ok(CastType::Array),
      b"object" => Ok(CastType::Object),
      b"unset" => Ok(CastType::Unset),
      _ => Err(format!("Invalid cast type: {}", value)),
    }
  }
}

impl Display for CastType {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match self {
      CastType::Int => "int",
      CastType::Integer => "integer",
      CastType::Bool => "bool",
      CastType::Boolean => "boolean",
      CastType::Float => "float",
      CastType::Double => "double",
      CastType::Real => "real",
      CastType::String => "string",
      CastType::Binary => "binary",
      CastType::Array => "array",
      CastType::Object => "object",
      CastType::Unset => "unset",
    })
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryType {
  Addition,
  Subtraction,
  Multiplication,
  Division,
  Modulus,
  Exponentiation,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  BitwiseShiftLeft,
  BitwiseShiftRight,
  IsEqual,
  IsIdentical,
  IsNotEqual,
  IsNotIdentical,
  IsLesser,
  IsGreater,
  IsLesserOrEqual,
  IsGreaterOrEqual,
  Spaceship,
  Concatenation,
  BooleanAnd,
  BooleanOr,
  BooleanXor,
  Elvis,
  Coalesce,
  InstanceOf,
}

impl TryFrom<&BString> for BinaryType {
  type Error = String;

  fn try_from(value: &BString) -> Result<Self, Self::Error> {
    match value.as_slice() {
      b"+" => Ok(BinaryType::Addition),
      b"-" => Ok(BinaryType::Subtraction),
      b"*" => Ok(BinaryType::Multiplication),
      b"/" => Ok(BinaryType::Division),
      b"%" => Ok(BinaryType::Modulus),
      b"**" => Ok(BinaryType::Exponentiation),
      b"&" => Ok(BinaryType::BitwiseAnd),
      b"|" => Ok(BinaryType::BitwiseOr),
      b"^" => Ok(BinaryType::BitwiseXor),
      b"<<" => Ok(BinaryType::BitwiseShiftLeft),
      b">>" => Ok(BinaryType::BitwiseShiftRight),
      b"==" => Ok(BinaryType::IsEqual),
      b"===" => Ok(BinaryType::IsIdentical),
      b"!=" => Ok(BinaryType::IsNotEqual),
      b"!==" => Ok(BinaryType::IsNotIdentical),
      b"<" => Ok(BinaryType::IsLesser),
      b">" => Ok(BinaryType::IsGreater),
      b"<=" => Ok(BinaryType::IsLesserOrEqual),
      b">=" => Ok(BinaryType::IsGreaterOrEqual),
      b"<=>" => Ok(BinaryType::Spaceship),
      b"." => Ok(BinaryType::Concatenation),
      b"&&" | b"and" => Ok(BinaryType::BooleanAnd),
      b"||" | b"or" => Ok(BinaryType::BooleanOr),
      b"xor" => Ok(BinaryType::BooleanXor),
      b"?:" => Ok(BinaryType::Elvis),
      b"??" => Ok(BinaryType::Coalesce),
      b"instanceof" => Ok(BinaryType::InstanceOf),
      _ => Err(format!("Invalid binary type: {}", value)),
    }
  }
}

impl Display for BinaryType {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match self {
      BinaryType::Addition => "+",
      BinaryType::Subtraction => "-",
      BinaryType::Multiplication => "*",
      BinaryType::Division => "/",
      BinaryType::Modulus => "%",
      BinaryType::Exponentiation => "**",
      BinaryType::BitwiseAnd => "&",
      BinaryType::BitwiseOr => "|",
      BinaryType::BitwiseXor => "^",
      BinaryType::BitwiseShiftLeft => "<<",
      BinaryType::BitwiseShiftRight => ">>",
      BinaryType::IsEqual => "==",
      BinaryType::IsIdentical => "===",
      BinaryType::IsNotEqual => "!=",
      BinaryType::IsNotIdentical => "!==",
      BinaryType::IsLesser => "<",
      BinaryType::IsGreater => ">",
      BinaryType::IsLesserOrEqual => "<=",
      BinaryType::IsGreaterOrEqual => ">=",
      BinaryType::Spaceship => "<=>",
      BinaryType::Concatenation => ".",
      BinaryType::BooleanAnd => "&&",
      BinaryType::BooleanOr => "||",
      BinaryType::BooleanXor => "xor",
      BinaryType::Elvis => "?:",
      BinaryType::Coalesce => "??",
      BinaryType::InstanceOf => "instanceof",
    })
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentType {
  Default,
  Reference,
  Coalesce,
  Concatenation,
  Addition,
  Subtraction,
  Multiplication,
  Division,
  Exponentiation,
  Modulus,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  BitwiseShiftRight,
  BitwiseShiftLeft,
}

impl TryFrom<&BString> for AssignmentType {
  type Error = String;

  fn try_from(value: &BString) -> Result<Self, Self::Error> {
    match value.as_slice() {
      b"=" => Ok(AssignmentType::Default),
      b"=&" => Ok(AssignmentType::Reference),
      b"??=" => Ok(AssignmentType::Coalesce),
      b".=" => Ok(AssignmentType::Concatenation),
      b"+=" => Ok(AssignmentType::Addition),
      b"-=" => Ok(AssignmentType::Subtraction),
      b"*=" => Ok(AssignmentType::Multiplication),
      b"/=" => Ok(AssignmentType::Division),
      b"**=" => Ok(AssignmentType::Exponentiation),
      b"%=" => Ok(AssignmentType::Modulus),
      b"&=" => Ok(AssignmentType::BitwiseAnd),
      b"|=" => Ok(AssignmentType::BitwiseOr),
      b"^=" => Ok(AssignmentType::BitwiseXor),
      b">>=" => Ok(AssignmentType::BitwiseShiftRight),
      b"<<=" => Ok(AssignmentType::BitwiseShiftLeft),
      _ => Err(format!("Invalid assignment type: {}", value)),
    }
  }
}

impl Display for AssignmentType {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match self {
      AssignmentType::Default => "=",
      AssignmentType::Reference => "=&",
      AssignmentType::Coalesce => "??=",
      AssignmentType::Concatenation => ".=",
      AssignmentType::Addition => "+=",
      AssignmentType::Subtraction => "-=",
      AssignmentType::Multiplication => "*=",
      AssignmentType::Division => "/=",
      AssignmentType::Exponentiation => "**=",
      AssignmentType::Modulus => "%=",
      AssignmentType::BitwiseAnd => "&=",
      AssignmentType::BitwiseOr => "|=",
      AssignmentType::BitwiseXor => "^=",
      AssignmentType::BitwiseShiftRight => ">>=",
      AssignmentType::BitwiseShiftLeft => "<<=",
    })
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum UseItemModifier {
  Function,
  Const,
}

impl TryFrom<&BString> for UseItemModifier {
  type Error = String;

  fn try_from(value: &BString) -> Result<Self, Self::Error> {
    match value.as_slice() {
      b"function" => Ok(UseItemModifier::Function),
      b"const" => Ok(UseItemModifier::Const),
      _ => Err(format!("Invalid use item modifier: {}", value)),
    }
  }
}

impl Display for UseItemModifier {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match self {
      UseItemModifier::Function => "function",
      UseItemModifier::Const => "const",
    })
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Modifier {
  Static,
  Readonly,
}

impl TryFrom<&BString> for Modifier {
  type Error = String;

  fn try_from(value: &BString) -> Result<Self, Self::Error> {
    match value.as_slice() {
      b"static" => Ok(Modifier::Static),
      b"readonly" => Ok(Modifier::Readonly),
      _ => Err(format!("Invalid modifier: {}", value)),
    }
  }
}

impl Display for Modifier {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match self {
      Modifier::Static => "static",
      Modifier::Readonly => "readonly",
    })
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Quote {
  Single,
  Double,
  Backtick,
}

impl TryFrom<&BString> for Quote {
  type Error = String;

  fn try_from(value: &BString) -> Result<Self, Self::Error> {
    match value.as_slice() {
      b"'" => Ok(Quote::Single),
      b"\"" => Ok(Quote::Double),
      b"`" => Ok(Quote::Backtick),
      _ => Err(format!("Invalid quote: {}", value)),
    }
  }
}

impl Display for Quote {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match self {
      Quote::Single => "'",
      Quote::Double => "\"",
      Quote::Backtick => "`",
    })
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Inheritance {
  Abstract,
  Final,
}

impl TryFrom<&BString> for Inheritance {
  type Error = String;

  fn try_from(value: &BString) -> Result<Self, Self::Error> {
    match value.as_slice() {
      b"abstract" => Ok(Inheritance::Abstract),
      b"final" => Ok(Inheritance::Final),
      _ => Err(format!("Invalid inheritance: {}", value)),
    }
  }
}

impl Display for Inheritance {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match self {
      Inheritance::Abstract => "abstract",
      Inheritance::Final => "final",
    })
  }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Visibility {
  Public,
  PublicGet,
  PublicSet,
  Private,
  PrivateGet,
  PrivateSet,
  Protected,
  ProtectedGet,
  ProtectedSet,
}

impl TryFrom<&BString> for Visibility {
  type Error = String;

  fn try_from(value: &BString) -> Result<Self, Self::Error> {
    match value.as_slice() {
      b"public" => Ok(Visibility::Public),
      b"public(get)" => Ok(Visibility::PublicGet),
      b"public(set)" => Ok(Visibility::PublicSet),
      b"private" => Ok(Visibility::Private),
      b"private(get)" => Ok(Visibility::PrivateGet),
      b"private(set)" => Ok(Visibility::PrivateSet),
      b"protected" => Ok(Visibility::Protected),
      b"protected(get)" => Ok(Visibility::ProtectedGet),
      b"protected(set)" => Ok(Visibility::ProtectedSet),
      _ => Err(format!("Invalid visibility: {}", value)),
    }
  }
}

impl Display for Visibility {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match self {
      Visibility::Public => "public",
      Visibility::PublicGet => "public(get)",
      Visibility::PublicSet => "public(set)",
      Visibility::Private => "private",
      Visibility::PrivateGet => "private(get)",
      Visibility::PrivateSet => "private(set)",
      Visibility::Protected => "protected",
      Visibility::ProtectedGet => "protected(get)",
      Visibility::ProtectedSet => "protected(set)",
    })
  }
}

#[cfg(test)]
mod tests {
  use bstr::BString;

  use crate::{
    AssignmentType,
    BinaryType,
    CastType,
    Inheritance,
    MagicMethodName,
    MagicName,
    Modifier,
    PostType,
    PreType,
    Quote,
    UseItemModifier,
    Visibility,
  };

  #[test]
  fn magic_name() {
    assert_eq!(
      MagicName::try_from(&BString::new("__CLASS__".as_bytes().to_vec())),
      Ok(MagicName::Class)
    );
    assert_eq!(
      MagicName::try_from(&BString::new("__DIR__".as_bytes().to_vec())),
      Ok(MagicName::Dir)
    );
    assert_eq!(
      MagicName::try_from(&BString::new("__FILE__".as_bytes().to_vec())),
      Ok(MagicName::File)
    );
    assert_eq!(
      MagicName::try_from(&BString::new("__FUNCTION__".as_bytes().to_vec())),
      Ok(MagicName::Function)
    );
    assert_eq!(
      MagicName::try_from(&BString::new("__LINE__".as_bytes().to_vec())),
      Ok(MagicName::Line)
    );
    assert_eq!(
      MagicName::try_from(&BString::new("__METHOD__".as_bytes().to_vec())),
      Ok(MagicName::Method)
    );
    assert_eq!(
      MagicName::try_from(&BString::new("__NAMESPACE__".as_bytes().to_vec())),
      Ok(MagicName::Namespace)
    );
    assert_eq!(
      MagicName::try_from(&BString::new("__TRAIT__".as_bytes().to_vec())),
      Ok(MagicName::Trait)
    );
    assert_eq!(
      MagicName::try_from(&BString::new("__PROPERTY__".as_bytes().to_vec())),
      Ok(MagicName::Property)
    );
    assert!(MagicName::try_from(&BString::new("none".as_bytes().to_vec())).is_err());

    assert_eq!("__CLASS__", format!("{}", MagicName::Class));
    assert_eq!("__DIR__", format!("{}", MagicName::Dir));
    assert_eq!("__FILE__", format!("{}", MagicName::File));
    assert_eq!("__FUNCTION__", format!("{}", MagicName::Function));
    assert_eq!("__LINE__", format!("{}", MagicName::Line));
    assert_eq!("__METHOD__", format!("{}", MagicName::Method));
    assert_eq!("__NAMESPACE__", format!("{}", MagicName::Namespace));
    assert_eq!("__TRAIT__", format!("{}", MagicName::Trait));
    assert_eq!("__PROPERTY__", format!("{}", MagicName::Property));
  }

  #[test]
  fn magic_method_name() {
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__construct".as_bytes().to_vec())),
      Ok(MagicMethodName::Construct)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__destruct".as_bytes().to_vec())),
      Ok(MagicMethodName::Destruct)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__call".as_bytes().to_vec())),
      Ok(MagicMethodName::Call)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__callStatic".as_bytes().to_vec())),
      Ok(MagicMethodName::CallStatic)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__get".as_bytes().to_vec())),
      Ok(MagicMethodName::Get)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__set".as_bytes().to_vec())),
      Ok(MagicMethodName::Set)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__isset".as_bytes().to_vec())),
      Ok(MagicMethodName::Isset)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__unset".as_bytes().to_vec())),
      Ok(MagicMethodName::Unset)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__sleep".as_bytes().to_vec())),
      Ok(MagicMethodName::Sleep)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__wakeup".as_bytes().to_vec())),
      Ok(MagicMethodName::Wakeup)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__serialize".as_bytes().to_vec())),
      Ok(MagicMethodName::Serialize)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__unserialize".as_bytes().to_vec())),
      Ok(MagicMethodName::Unserialize)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__toString".as_bytes().to_vec())),
      Ok(MagicMethodName::ToString)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__invoke".as_bytes().to_vec())),
      Ok(MagicMethodName::Invoke)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__set_state".as_bytes().to_vec())),
      Ok(MagicMethodName::SetState)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__clone".as_bytes().to_vec())),
      Ok(MagicMethodName::Clone)
    );
    assert_eq!(
      MagicMethodName::try_from(&BString::new("__debugInfo".as_bytes().to_vec())),
      Ok(MagicMethodName::DebugInfo)
    );
    assert!(MagicMethodName::try_from(&BString::new("none".as_bytes().to_vec())).is_err());

    assert_eq!("__construct", format!("{}", MagicMethodName::Construct));
    assert_eq!("__destruct", format!("{}", MagicMethodName::Destruct));
    assert_eq!("__call", format!("{}", MagicMethodName::Call));
    assert_eq!("__callStatic", format!("{}", MagicMethodName::CallStatic));
    assert_eq!("__get", format!("{}", MagicMethodName::Get));
    assert_eq!("__set", format!("{}", MagicMethodName::Set));
    assert_eq!("__isset", format!("{}", MagicMethodName::Isset));
    assert_eq!("__unset", format!("{}", MagicMethodName::Unset));
    assert_eq!("__sleep", format!("{}", MagicMethodName::Sleep));
    assert_eq!("__wakeup", format!("{}", MagicMethodName::Wakeup));
    assert_eq!("__serialize", format!("{}", MagicMethodName::Serialize));
    assert_eq!("__unserialize", format!("{}", MagicMethodName::Unserialize));
    assert_eq!("__toString", format!("{}", MagicMethodName::ToString));
    assert_eq!("__invoke", format!("{}", MagicMethodName::Invoke));
    assert_eq!("__set_state", format!("{}", MagicMethodName::SetState));
    assert_eq!("__clone", format!("{}", MagicMethodName::Clone));
    assert_eq!("__debugInfo", format!("{}", MagicMethodName::DebugInfo));
  }

  #[test]
  fn pre_type() {
    assert_eq!(PreType::try_from(&BString::new("++".as_bytes().to_vec())), Ok(PreType::Increment));
    assert_eq!(PreType::try_from(&BString::new("--".as_bytes().to_vec())), Ok(PreType::Decrement));
    assert_eq!(PreType::try_from(&BString::new("+".as_bytes().to_vec())), Ok(PreType::Addition));
    assert_eq!(PreType::try_from(&BString::new("-".as_bytes().to_vec())), Ok(PreType::Subtraction));
    assert!(PreType::try_from(&BString::new("none".as_bytes().to_vec())).is_err());

    assert_eq!("++", format!("{}", PreType::Increment));
    assert_eq!("--", format!("{}", PreType::Decrement));
    assert_eq!("+", format!("{}", PreType::Addition));
    assert_eq!("-", format!("{}", PreType::Subtraction));
  }

  #[test]
  fn post_type() {
    assert_eq!(
      PostType::try_from(&BString::new("++".as_bytes().to_vec())),
      Ok(PostType::Increment)
    );
    assert_eq!(
      PostType::try_from(&BString::new("--".as_bytes().to_vec())),
      Ok(PostType::Decrement)
    );
    assert!(PostType::try_from(&BString::new("none".as_bytes().to_vec())).is_err());

    assert_eq!("++", format!("{}", PostType::Increment));
    assert_eq!("--", format!("{}", PostType::Decrement));
  }

  #[test]
  fn cast_type() {
    assert_eq!(CastType::try_from(&BString::new("int".as_bytes().to_vec())), Ok(CastType::Int));
    assert_eq!(
      CastType::try_from(&BString::new("integer".as_bytes().to_vec())),
      Ok(CastType::Integer)
    );
    assert_eq!(CastType::try_from(&BString::new("bool".as_bytes().to_vec())), Ok(CastType::Bool));
    assert_eq!(
      CastType::try_from(&BString::new("boolean".as_bytes().to_vec())),
      Ok(CastType::Boolean)
    );
    assert_eq!(CastType::try_from(&BString::new("float".as_bytes().to_vec())), Ok(CastType::Float));
    assert_eq!(
      CastType::try_from(&BString::new("double".as_bytes().to_vec())),
      Ok(CastType::Double)
    );
    assert_eq!(CastType::try_from(&BString::new("real".as_bytes().to_vec())), Ok(CastType::Real));
    assert_eq!(
      CastType::try_from(&BString::new("string".as_bytes().to_vec())),
      Ok(CastType::String)
    );
    assert_eq!(
      CastType::try_from(&BString::new("binary".as_bytes().to_vec())),
      Ok(CastType::Binary)
    );
    assert_eq!(CastType::try_from(&BString::new("array".as_bytes().to_vec())), Ok(CastType::Array));
    assert_eq!(
      CastType::try_from(&BString::new("object".as_bytes().to_vec())),
      Ok(CastType::Object)
    );
    assert_eq!(CastType::try_from(&BString::new("unset".as_bytes().to_vec())), Ok(CastType::Unset));
    assert!(CastType::try_from(&BString::new("none".as_bytes().to_vec())).is_err());

    assert_eq!("int", format!("{}", CastType::Int));
    assert_eq!("integer", format!("{}", CastType::Integer));
    assert_eq!("bool", format!("{}", CastType::Bool));
    assert_eq!("boolean", format!("{}", CastType::Boolean));
    assert_eq!("float", format!("{}", CastType::Float));
    assert_eq!("double", format!("{}", CastType::Double));
    assert_eq!("real", format!("{}", CastType::Real));
    assert_eq!("string", format!("{}", CastType::String));
    assert_eq!("binary", format!("{}", CastType::Binary));
    assert_eq!("array", format!("{}", CastType::Array));
    assert_eq!("object", format!("{}", CastType::Object));
    assert_eq!("unset", format!("{}", CastType::Unset));
  }

  #[test]
  fn binary_type() {
    assert_eq!(
      BinaryType::try_from(&BString::new("+".as_bytes().to_vec())),
      Ok(BinaryType::Addition)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("-".as_bytes().to_vec())),
      Ok(BinaryType::Subtraction)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("*".as_bytes().to_vec())),
      Ok(BinaryType::Multiplication)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("/".as_bytes().to_vec())),
      Ok(BinaryType::Division)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("%".as_bytes().to_vec())),
      Ok(BinaryType::Modulus)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("**".as_bytes().to_vec())),
      Ok(BinaryType::Exponentiation)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("&".as_bytes().to_vec())),
      Ok(BinaryType::BitwiseAnd)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("|".as_bytes().to_vec())),
      Ok(BinaryType::BitwiseOr)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("^".as_bytes().to_vec())),
      Ok(BinaryType::BitwiseXor)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("<<".as_bytes().to_vec())),
      Ok(BinaryType::BitwiseShiftLeft)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new(">>".as_bytes().to_vec())),
      Ok(BinaryType::BitwiseShiftRight)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("==".as_bytes().to_vec())),
      Ok(BinaryType::IsEqual)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("===".as_bytes().to_vec())),
      Ok(BinaryType::IsIdentical)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("!=".as_bytes().to_vec())),
      Ok(BinaryType::IsNotEqual)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("!==".as_bytes().to_vec())),
      Ok(BinaryType::IsNotIdentical)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("<".as_bytes().to_vec())),
      Ok(BinaryType::IsLesser)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new(">".as_bytes().to_vec())),
      Ok(BinaryType::IsGreater)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("<=".as_bytes().to_vec())),
      Ok(BinaryType::IsLesserOrEqual)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new(">=".as_bytes().to_vec())),
      Ok(BinaryType::IsGreaterOrEqual)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("<=>".as_bytes().to_vec())),
      Ok(BinaryType::Spaceship)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new(".".as_bytes().to_vec())),
      Ok(BinaryType::Concatenation)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("&&".as_bytes().to_vec())),
      Ok(BinaryType::BooleanAnd)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("and".as_bytes().to_vec())),
      Ok(BinaryType::BooleanAnd)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("||".as_bytes().to_vec())),
      Ok(BinaryType::BooleanOr)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("or".as_bytes().to_vec())),
      Ok(BinaryType::BooleanOr)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("xor".as_bytes().to_vec())),
      Ok(BinaryType::BooleanXor)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("?:".as_bytes().to_vec())),
      Ok(BinaryType::Elvis)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("??".as_bytes().to_vec())),
      Ok(BinaryType::Coalesce)
    );
    assert_eq!(
      BinaryType::try_from(&BString::new("instanceof".as_bytes().to_vec())),
      Ok(BinaryType::InstanceOf)
    );
    assert!(BinaryType::try_from(&BString::new("none".as_bytes().to_vec())).is_err());

    assert_eq!("+", format!("{}", BinaryType::Addition));
    assert_eq!("-", format!("{}", BinaryType::Subtraction));
    assert_eq!("*", format!("{}", BinaryType::Multiplication));
    assert_eq!("/", format!("{}", BinaryType::Division));
    assert_eq!("%", format!("{}", BinaryType::Modulus));
    assert_eq!("**", format!("{}", BinaryType::Exponentiation));
    assert_eq!("&", format!("{}", BinaryType::BitwiseAnd));
    assert_eq!("|", format!("{}", BinaryType::BitwiseOr));
    assert_eq!("^", format!("{}", BinaryType::BitwiseXor));
    assert_eq!("<<", format!("{}", BinaryType::BitwiseShiftLeft));
    assert_eq!(">>", format!("{}", BinaryType::BitwiseShiftRight));
    assert_eq!("==", format!("{}", BinaryType::IsEqual));
    assert_eq!("===", format!("{}", BinaryType::IsIdentical));
    assert_eq!("!=", format!("{}", BinaryType::IsNotEqual));
    assert_eq!("!==", format!("{}", BinaryType::IsNotIdentical));
    assert_eq!("<", format!("{}", BinaryType::IsLesser));
    assert_eq!(">", format!("{}", BinaryType::IsGreater));
    assert_eq!("<=", format!("{}", BinaryType::IsLesserOrEqual));
    assert_eq!(">=", format!("{}", BinaryType::IsGreaterOrEqual));
    assert_eq!("<=>", format!("{}", BinaryType::Spaceship));
    assert_eq!(".", format!("{}", BinaryType::Concatenation));
    assert_eq!("&&", format!("{}", BinaryType::BooleanAnd));
    assert_eq!("||", format!("{}", BinaryType::BooleanOr));
    assert_eq!("xor", format!("{}", BinaryType::BooleanXor));
    assert_eq!("?:", format!("{}", BinaryType::Elvis));
    assert_eq!("??", format!("{}", BinaryType::Coalesce));
    assert_eq!("instanceof", format!("{}", BinaryType::InstanceOf));
  }

  #[test]
  fn assignment_type() {
    assert_eq!(
      AssignmentType::try_from(&BString::new("=".as_bytes().to_vec())),
      Ok(AssignmentType::Default)
    );
    assert_eq!(
      AssignmentType::try_from(&BString::new("=&".as_bytes().to_vec())),
      Ok(AssignmentType::Reference)
    );
    assert_eq!(
      AssignmentType::try_from(&BString::new("??=".as_bytes().to_vec())),
      Ok(AssignmentType::Coalesce)
    );
    assert_eq!(
      AssignmentType::try_from(&BString::new("**=".as_bytes().to_vec())),
      Ok(AssignmentType::Exponentiation)
    );
    assert_eq!(
      AssignmentType::try_from(&BString::new("*=".as_bytes().to_vec())),
      Ok(AssignmentType::Multiplication)
    );
    assert_eq!(
      AssignmentType::try_from(&BString::new("/=".as_bytes().to_vec())),
      Ok(AssignmentType::Division)
    );
    assert_eq!(
      AssignmentType::try_from(&BString::new("+=".as_bytes().to_vec())),
      Ok(AssignmentType::Addition)
    );
    assert_eq!(
      AssignmentType::try_from(&BString::new("-=".as_bytes().to_vec())),
      Ok(AssignmentType::Subtraction)
    );
    assert_eq!(
      AssignmentType::try_from(&BString::new("%=".as_bytes().to_vec())),
      Ok(AssignmentType::Modulus)
    );
    assert_eq!(
      AssignmentType::try_from(&BString::new("&=".as_bytes().to_vec())),
      Ok(AssignmentType::BitwiseAnd)
    );
    assert_eq!(
      AssignmentType::try_from(&BString::new("|=".as_bytes().to_vec())),
      Ok(AssignmentType::BitwiseOr)
    );
    assert_eq!(
      AssignmentType::try_from(&BString::new("^=".as_bytes().to_vec())),
      Ok(AssignmentType::BitwiseXor)
    );
    assert_eq!(
      AssignmentType::try_from(&BString::new(">>=".as_bytes().to_vec())),
      Ok(AssignmentType::BitwiseShiftRight)
    );
    assert_eq!(
      AssignmentType::try_from(&BString::new("<<=".as_bytes().to_vec())),
      Ok(AssignmentType::BitwiseShiftLeft)
    );
    assert!(AssignmentType::try_from(&BString::new("none".as_bytes().to_vec())).is_err());

    assert_eq!("=", format!("{}", AssignmentType::Default));
    assert_eq!("=&", format!("{}", AssignmentType::Reference));
    assert_eq!("??=", format!("{}", AssignmentType::Coalesce));
    assert_eq!("**=", format!("{}", AssignmentType::Exponentiation));
    assert_eq!("*=", format!("{}", AssignmentType::Multiplication));
    assert_eq!("/=", format!("{}", AssignmentType::Division));
    assert_eq!("+=", format!("{}", AssignmentType::Addition));
    assert_eq!("-=", format!("{}", AssignmentType::Subtraction));
    assert_eq!("%=", format!("{}", AssignmentType::Modulus));
    assert_eq!("&=", format!("{}", AssignmentType::BitwiseAnd));
    assert_eq!("|=", format!("{}", AssignmentType::BitwiseOr));
    assert_eq!("^=", format!("{}", AssignmentType::BitwiseXor));
    assert_eq!(">>=", format!("{}", AssignmentType::BitwiseShiftRight));
    assert_eq!("<<=", format!("{}", AssignmentType::BitwiseShiftLeft));
  }

  #[test]
  fn use_item_modifier() {
    assert_eq!(
      UseItemModifier::try_from(&BString::new("const".as_bytes().to_vec())),
      Ok(UseItemModifier::Const)
    );
    assert_eq!(
      UseItemModifier::try_from(&BString::new("function".as_bytes().to_vec())),
      Ok(UseItemModifier::Function)
    );
    assert!(UseItemModifier::try_from(&BString::new("none".as_bytes().to_vec())).is_err());

    assert_eq!("const", format!("{}", UseItemModifier::Const));
    assert_eq!("function", format!("{}", UseItemModifier::Function));
  }

  #[test]
  fn modifier() {
    assert_eq!(
      Modifier::try_from(&BString::new("static".as_bytes().to_vec())),
      Ok(Modifier::Static)
    );
    assert_eq!(
      Modifier::try_from(&BString::new("readonly".as_bytes().to_vec())),
      Ok(Modifier::Readonly)
    );
    assert!(Modifier::try_from(&BString::new("none".as_bytes().to_vec())).is_err());

    assert_eq!("static", format!("{}", Modifier::Static));
    assert_eq!("readonly", format!("{}", Modifier::Readonly));
  }

  #[test]
  fn quote() {
    assert_eq!(Quote::try_from(&BString::new("'".as_bytes().to_vec())), Ok(Quote::Single));
    assert_eq!(Quote::try_from(&BString::new("\"".as_bytes().to_vec())), Ok(Quote::Double));
    assert_eq!(Quote::try_from(&BString::new("`".as_bytes().to_vec())), Ok(Quote::Backtick));
    assert!(Quote::try_from(&BString::new("none".as_bytes().to_vec())).is_err());

    assert_eq!("'", format!("{}", Quote::Single));
    assert_eq!("\"", format!("{}", Quote::Double));
    assert_eq!("`", format!("{}", Quote::Backtick));
  }

  #[test]
  fn inheritance() {
    assert_eq!(
      Inheritance::try_from(&BString::new("abstract".as_bytes().to_vec())),
      Ok(Inheritance::Abstract)
    );
    assert_eq!(
      Inheritance::try_from(&BString::new("final".as_bytes().to_vec())),
      Ok(Inheritance::Final)
    );
    assert!(Inheritance::try_from(&BString::new("none".as_bytes().to_vec())).is_err());

    assert_eq!("abstract", format!("{}", Inheritance::Abstract));
    assert_eq!("final", format!("{}", Inheritance::Final));
  }

  #[test]
  fn visibility() {
    assert_eq!(
      Visibility::try_from(&BString::new("private".as_bytes().to_vec())),
      Ok(Visibility::Private)
    );
    assert_eq!(
      Visibility::try_from(&BString::new("private(get)".as_bytes().to_vec())),
      Ok(Visibility::PrivateGet)
    );
    assert_eq!(
      Visibility::try_from(&BString::new("private(set)".as_bytes().to_vec())),
      Ok(Visibility::PrivateSet)
    );
    assert_eq!(
      Visibility::try_from(&BString::new("protected".as_bytes().to_vec())),
      Ok(Visibility::Protected)
    );
    assert_eq!(
      Visibility::try_from(&BString::new("protected(get)".as_bytes().to_vec())),
      Ok(Visibility::ProtectedGet)
    );
    assert_eq!(
      Visibility::try_from(&BString::new("protected(set)".as_bytes().to_vec())),
      Ok(Visibility::ProtectedSet)
    );
    assert_eq!(
      Visibility::try_from(&BString::new("public".as_bytes().to_vec())),
      Ok(Visibility::Public)
    );
    assert_eq!(
      Visibility::try_from(&BString::new("public(get)".as_bytes().to_vec())),
      Ok(Visibility::PublicGet)
    );
    assert_eq!(
      Visibility::try_from(&BString::new("public(set)".as_bytes().to_vec())),
      Ok(Visibility::PublicSet)
    );
    assert!(Visibility::try_from(&BString::new("none".as_bytes().to_vec())).is_err());

    assert_eq!("private", format!("{}", Visibility::Private));
    assert_eq!("private(get)", format!("{}", Visibility::PrivateGet));
    assert_eq!("private(set)", format!("{}", Visibility::PrivateSet));
    assert_eq!("protected", format!("{}", Visibility::Protected));
    assert_eq!("protected(get)", format!("{}", Visibility::ProtectedGet));
    assert_eq!("protected(set)", format!("{}", Visibility::ProtectedSet));
    assert_eq!("public", format!("{}", Visibility::Public));
    assert_eq!("public(get)", format!("{}", Visibility::PublicGet));
    assert_eq!("public(set)", format!("{}", Visibility::PublicSet));
  }
}
