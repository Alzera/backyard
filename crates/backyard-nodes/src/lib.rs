pub mod macros;
pub mod utils;
#[cfg(feature = "walker")]
pub mod walker;
#[cfg(feature = "builder")]
pub mod builder;

use std::fmt::{ self, Display, Formatter };

use bumpalo::{ vec, Bump };
use compact_str::CompactString;
use serde::Serialize;

use crate::utils::CloneIn;

#[cfg(feature = "builder")]
use crate::builder::{ Blueprint, BlueprintBuildable, BlueprintWrapper, Builder };

#[cfg(feature = "walker")]
use crate::walker::{ MapIntoWalkerStack, Walkable };

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum UseItemModifier {
  Function,
  Const,
}

impl TryFrom<&str> for UseItemModifier {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "function" => Ok(UseItemModifier::Function),
      "const" => Ok(UseItemModifier::Const),
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

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Modifier {
  Static,
  Readonly,
}

impl TryFrom<&str> for Modifier {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "static" => Ok(Modifier::Static),
      "readonly" => Ok(Modifier::Readonly),
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

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Quote {
  Single,
  Double,
  Backtick,
}

impl TryFrom<&str> for Quote {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "'" => Ok(Quote::Single),
      "\"" => Ok(Quote::Double),
      "`" => Ok(Quote::Backtick),
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

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Inheritance {
  Abstract,
  Final,
}

impl TryFrom<&str> for Inheritance {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "abstract" => Ok(Inheritance::Abstract),
      "final" => Ok(Inheritance::Final),
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

#[derive(Debug, PartialEq, Clone, Serialize)]
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

impl TryFrom<&str> for Visibility {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "public" => Ok(Visibility::Public),
      "public(get)" => Ok(Visibility::PublicGet),
      "public(set)" => Ok(Visibility::PublicSet),
      "private" => Ok(Visibility::Private),
      "private(get)" => Ok(Visibility::PrivateGet),
      "private(set)" => Ok(Visibility::PrivateSet),
      "protected" => Ok(Visibility::Protected),
      "protected(get)" => Ok(Visibility::ProtectedGet),
      "protected(set)" => Ok(Visibility::ProtectedSet),
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

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum BodyType {
  Basic,
  Short,
  Empty,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RangeLocation {
  pub start: Location,
  pub end: Location,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Location {
  pub line: usize,
  pub column: usize,
  pub offset: usize,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Node<'a> {
  pub node_type: NodeType,
  #[serde(flatten)]
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
      self.leadings = Some(vec![in arena; node]);
    }
  }

  pub fn leadings_push(&mut self, arena: &'a Bump, node: Node<'a>) {
    if let Some(leadings) = &mut self.leadings {
      leadings.push(node);
    } else {
      self.leadings = Some(vec![in arena; node]);
    }
  }

  pub fn trailings_push(&mut self, arena: &'a Bump, node: Node<'a>) {
    if let Some(trailings) = &mut self.trailings {
      trailings.push(node);
    } else {
      self.trailings = Some(vec![in arena; node]);
    }
  }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum NodeWrapper<'a> {
  AnonymousClass(AnonymousClassNode<'a>),
  AnonymousFunction(AnonymousFunctionNode<'a>),
  CallArgument(CallArgumentNode<'a>),
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

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
  AnonymousClass,
  AnonymousFunction,
  CallArgument,
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

#[cfg(test)]
mod tests {
  use crate::{ Inheritance, Modifier, Quote, UseItemModifier, Visibility };

  #[test]
  fn use_item_modifier() {
    assert_eq!(UseItemModifier::try_from("const"), Ok(UseItemModifier::Const));
    assert_eq!(UseItemModifier::try_from("function"), Ok(UseItemModifier::Function));
    assert!(UseItemModifier::try_from("none").is_err());

    assert_eq!("const", format!("{}", UseItemModifier::Const));
    assert_eq!("function", format!("{}", UseItemModifier::Function));
  }

  #[test]
  fn modifier() {
    assert_eq!(Modifier::try_from("static"), Ok(Modifier::Static));
    assert_eq!(Modifier::try_from("readonly"), Ok(Modifier::Readonly));
    assert!(Modifier::try_from("none").is_err());

    assert_eq!("static", format!("{}", Modifier::Static));
    assert_eq!("readonly", format!("{}", Modifier::Readonly));
  }

  #[test]
  fn quote() {
    assert_eq!(Quote::try_from("'"), Ok(Quote::Single));
    assert_eq!(Quote::try_from("\""), Ok(Quote::Double));
    assert_eq!(Quote::try_from("`"), Ok(Quote::Backtick));
    assert!(Quote::try_from("none").is_err());

    assert_eq!("'", format!("{}", Quote::Single));
    assert_eq!("\"", format!("{}", Quote::Double));
    assert_eq!("`", format!("{}", Quote::Backtick));
  }

  #[test]
  fn inheritance() {
    assert_eq!(Inheritance::try_from("abstract"), Ok(Inheritance::Abstract));
    assert_eq!(Inheritance::try_from("final"), Ok(Inheritance::Final));
    assert!(Inheritance::try_from("none").is_err());

    assert_eq!("abstract", format!("{}", Inheritance::Abstract));
    assert_eq!("final", format!("{}", Inheritance::Final));
  }

  #[test]
  fn visibility() {
    assert_eq!(Visibility::try_from("private"), Ok(Visibility::Private));
    assert_eq!(Visibility::try_from("private(get)"), Ok(Visibility::PrivateGet));
    assert_eq!(Visibility::try_from("private(set)"), Ok(Visibility::PrivateSet));
    assert_eq!(Visibility::try_from("protected"), Ok(Visibility::Protected));
    assert_eq!(Visibility::try_from("protected(get)"), Ok(Visibility::ProtectedGet));
    assert_eq!(Visibility::try_from("protected(set)"), Ok(Visibility::ProtectedSet));
    assert_eq!(Visibility::try_from("public"), Ok(Visibility::Public));
    assert_eq!(Visibility::try_from("public(get)"), Ok(Visibility::PublicGet));
    assert_eq!(Visibility::try_from("public(set)"), Ok(Visibility::PublicSet));
    assert!(Visibility::try_from("none").is_err());

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

macro_rules! new_node {
  (
    $node_type:ident,
    $struct_name:ident < $lt:lifetime > { $($field_name:ident: $field_type:ty),* $(,)? },
    $blueprint_name:ident < $blt:lifetime > {
      $($blueprint_field_name:ident: $blueprint_field_type:ty),* $(,)?
    }
  ) => {
    #[derive(Debug, PartialEq, Serialize)]
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
      fn populate_walks<'a>(&'a self) -> std::collections::VecDeque<&'a Node<'arena>> {
        let mut stack = std::collections::VecDeque::new();
        $(self.$field_name.map_into_walker_stack(&mut stack);)*
        stack.into_iter().rev().collect::<std::collections::VecDeque<_>>()
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
            leadings: &[],
            trailings: &[],
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
  };

  (
    $node_type:ident,
    $struct_name:ident { $($field_name:ident: $field_type:ty),* $(,)? },
    $blueprint_name:ident < $blt:lifetime > {
      $($blueprint_field_name:ident: $blueprint_field_type:ty),* $(,)?
    }
  ) => {
    #[derive(Debug, PartialEq, Serialize)]
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
      fn populate_walks<'a>(&'a self) -> std::collections::VecDeque<&'a Node<'arena>> {
        std::collections::VecDeque::new()
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
            leadings: &[],
            trailings: &[],
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
  };

  (
    $node_type:ident,
    $struct_name:ident { $($field_name:ident: $field_type:ty),* $(,)? },
    $blueprint_name:ident { $($blueprint_field_name:ident: $blueprint_field_type:ty),* $(,)? }
  ) => {
    #[derive(Debug, PartialEq, Serialize)]
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
      fn populate_walks<'a>(&'a self) -> std::collections::VecDeque<&'a Node<'arena>> {
        std::collections::VecDeque::new()
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
            leadings: &[],
            trailings: &[],
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

      fn build(&self, _: &'arena Bump) -> Self::Result {
        $struct_name::loc(
          $(self.$field_name,)*
          None
        )
      }
    }
  };
}

new_node!(AnonymousClass, AnonymousClassNode<'a> { parameters: bumpalo::collections::Vec<'a, Node<'a>>, extends: Option<bumpalo::boxed::Box<'a, Node<'a>>>, implements: bumpalo::collections::Vec<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, AnonymousClassBlueprint<'b> { parameters: &'b [Box<Blueprint<'b>>], extends: Option<Box<Blueprint<'b>>>, implements: &'b [Box<Blueprint<'b>>], body: Box<Blueprint<'b>>, });
new_node!(AnonymousFunction, AnonymousFunctionNode<'a> { is_ref: bool, parameters: bumpalo::collections::Vec<'a, Node<'a>>, uses: bumpalo::collections::Vec<'a, Node<'a>>, return_type: Option<bumpalo::boxed::Box<'a, Node<'a>>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, AnonymousFunctionBlueprint<'b> { is_ref: bool, parameters: &'b [Box<Blueprint<'b>>], uses: &'b [Box<Blueprint<'b>>], return_type: Option<Box<Blueprint<'b>>>, body: Box<Blueprint<'b>>, });
new_node!(CallArgument, CallArgumentNode<'a> { name: Option<bumpalo::boxed::Box<'a, Node<'a>>>, value: bumpalo::boxed::Box<'a, Node<'a>>, }, CallArgumentBlueprint<'b> { name: Option<Box<Blueprint<'b>>>, value: Box<Blueprint<'b>>, });
new_node!(Array, ArrayNode<'a> { is_short: bool, items: bumpalo::collections::Vec<'a, Node<'a>>, }, ArrayBlueprint<'b> { is_short: bool, items: &'b [Box<Blueprint<'b>>], });
new_node!(ArrayItem, ArrayItemNode<'a> { key: Option<bumpalo::boxed::Box<'a, Node<'a>>>, value: bumpalo::boxed::Box<'a, Node<'a>>, }, ArrayItemBlueprint<'b> { key: Option<Box<Blueprint<'b>>>, value: Box<Blueprint<'b>>, });
new_node!(ArrayLookup, ArrayLookupNode<'a> { left: bumpalo::boxed::Box<'a, Node<'a>>, right: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, ArrayLookupBlueprint<'b> { left: Box<Blueprint<'b>>, right: Option<Box<Blueprint<'b>>>, });
new_node!(ArrowFunction, ArrowFunctionNode<'a> { is_ref: bool, parameters: bumpalo::collections::Vec<'a, Node<'a>>, return_type: Option<bumpalo::boxed::Box<'a, Node<'a>>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, ArrowFunctionBlueprint<'b> { is_ref: bool, parameters: &'b [Box<Blueprint<'b>>], return_type: Option<Box<Blueprint<'b>>>, body: Box<Blueprint<'b>>, });
new_node!(Assignment, AssignmentNode<'a> { left: bumpalo::boxed::Box<'a, Node<'a>>, operator: CompactString, right: bumpalo::boxed::Box<'a, Node<'a>>, }, AssignmentBlueprint<'b> { left: Box<Blueprint<'b>>, operator: &'b str, right: Box<Blueprint<'b>>, });
new_node!(Attribute, AttributeNode<'a> { items: bumpalo::collections::Vec<'a, Node<'a>>, }, AttributeBlueprint<'b> { items: &'b [Box<Blueprint<'b>>], });
new_node!(AttributeItem, AttributeItemNode<'a> { name: CompactString, arguments: bumpalo::collections::Vec<'a, Node<'a>>, }, AttributeItemBlueprint<'b> { name: &'b str, arguments: &'b [Box<Blueprint<'b>>], });
new_node!(Bin, BinNode<'a> { left: bumpalo::boxed::Box<'a, Node<'a>>, operator: CompactString, right: bumpalo::boxed::Box<'a, Node<'a>>, }, BinBlueprint<'b> { left: Box<Blueprint<'b>>, operator: &'b str, right: Box<Blueprint<'b>>, });
new_node!(Block, BlockNode<'a> { statements: bumpalo::collections::Vec<'a, Node<'a>>, }, BlockBlueprint<'b> { statements: &'b [Box<Blueprint<'b>>], });
new_node!(Boolean, BooleanNode { is_true: bool }, BooleanBlueprint { is_true: bool });
new_node!(Break, BreakNode<'a> { statement: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, BreakBlueprint<'b> { statement: Option<Box<Blueprint<'b>>>, });
new_node!(Call, CallNode<'a> { name: bumpalo::boxed::Box<'a, Node<'a>>, arguments: bumpalo::collections::Vec<'a, Node<'a>>, }, CallBlueprint<'b> { name: Box<Blueprint<'b>>, arguments: &'b [Box<Blueprint<'b>>], });
new_node!(Case, CaseNode<'a> { condition: Option<bumpalo::boxed::Box<'a, Node<'a>>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, CaseBlueprint<'b> { condition: Option<Box<Blueprint<'b>>>, body: Box<Blueprint<'b>>, });
new_node!(Cast, CastNode<'a> { cast_type: CompactString, expression: bumpalo::boxed::Box<'a, Node<'a>>, }, CastBlueprint<'b> { cast_type: &'b str, expression: Box<Blueprint<'b>>, });
new_node!(Catch, CatchNode<'a> { types: bumpalo::collections::Vec<'a, Node<'a>>, variable: Option<bumpalo::boxed::Box<'a, Node<'a>>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, CatchBlueprint<'b> { types: &'b [Box<Blueprint<'b>>], variable: Option<Box<Blueprint<'b>>>, body: Box<Blueprint<'b>>, });
new_node!(Class, ClassNode<'a> { inheritance: Option<Inheritance>, name: Option<bumpalo::boxed::Box<'a, Node<'a>>>, extends: Option<bumpalo::boxed::Box<'a, Node<'a>>>, implements: bumpalo::collections::Vec<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, is_readonly: bool, }, ClassBlueprint<'b> { inheritance: Option<Inheritance>, name: Option<Box<Blueprint<'b>>>, extends: Option<Box<Blueprint<'b>>>, implements: &'b [Box<Blueprint<'b>>], body: Box<Blueprint<'b>>, is_readonly: bool, });
new_node!(ClassKeyword, ClassKeywordNode {}, ClassKeywordBlueprint {});
new_node!(Clone, CloneNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, CloneBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(CommentBlock, CommentBlockNode { comment: CompactString, }, CommentBlockBlueprint<'b> { comment: &'b str, });
new_node!(CommentDoc, CommentDocNode { comment: CompactString, }, CommentDocBlueprint<'b> { comment: &'b str, });
new_node!(CommentLine, CommentLineNode { comment: CompactString, }, CommentLineBlueprint<'b> { comment: &'b str, });
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
new_node!(HereDoc, HereDocNode<'a> { label: CompactString, values: bumpalo::collections::Vec<'a, Node<'a>>, }, HereDocBlueprint<'b> { label: &'b str, values: &'b [Box<Blueprint<'b>>], });
new_node!(Identifier, IdentifierNode { name: CompactString, }, IdentifierBlueprint<'b> { name: &'b str, });
new_node!(If, IfNode<'a> { condition: bumpalo::boxed::Box<'a, Node<'a>>, valid: bumpalo::boxed::Box<'a, Node<'a>>, invalid: Option<bumpalo::boxed::Box<'a, Node<'a>>>, is_short: bool, }, IfBlueprint<'b> { condition: Box<Blueprint<'b>>, valid: Box<Blueprint<'b>>, invalid: Option<Box<Blueprint<'b>>>, is_short: bool, });
new_node!(Include, IncludeNode<'a> { use_parenthesis: bool, is_require: bool, is_once: bool, argument: bumpalo::boxed::Box<'a, Node<'a>>, }, IncludeBlueprint<'b> { use_parenthesis: bool, is_require: bool, is_once: bool, argument: Box<Blueprint<'b>>, });
new_node!(Inline, InlineNode { text: CompactString, }, InlineBlueprint<'b> { text: &'b str, });
new_node!(Interface, InterfaceNode<'a> { name: bumpalo::boxed::Box<'a, Node<'a>>, extends: bumpalo::collections::Vec<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, InterfaceBlueprint<'b> { name: Box<Blueprint<'b>>, extends: &'b [Box<Blueprint<'b>>], body: Box<Blueprint<'b>>, });
new_node!(IntersectionType, IntersectionTypeNode<'a> { types: bumpalo::collections::Vec<'a, Node<'a>>, }, IntersectionTypeBlueprint<'b> { types: &'b [Box<Blueprint<'b>>], });
new_node!(Label, LabelNode<'a> { label: bumpalo::boxed::Box<'a, Node<'a>>, }, LabelBlueprint<'b> { label: Box<Blueprint<'b>>, });
new_node!(List, ListNode<'a> { items: bumpalo::collections::Vec<'a, Node<'a>>, }, ListBlueprint<'b> { items: &'b [Box<Blueprint<'b>>], });
new_node!(Magic, MagicNode { name: CompactString, }, MagicBlueprint<'b> { name: &'b str, });
new_node!(Match, MatchNode<'a> { condition: bumpalo::boxed::Box<'a, Node<'a>>, arms: bumpalo::collections::Vec<'a, Node<'a>>, }, MatchBlueprint<'b> { condition: Box<Blueprint<'b>>, arms: &'b [Box<Blueprint<'b>>], });
new_node!(MatchArm, MatchArmNode<'a> { conditions: bumpalo::collections::Vec<'a, Node<'a>>, expr: bumpalo::boxed::Box<'a, Node<'a>>, }, MatchArmBlueprint<'b> { conditions: &'b [Box<Blueprint<'b>>], expr: Box<Blueprint<'b>>, });
new_node!(Method, MethodNode<'a> { visibility: Option<Visibility>, inheritance: Option<Inheritance>, is_static: bool, function: bumpalo::boxed::Box<'a, Node<'a>>, }, MethodBlueprint<'b> { visibility: Option<Visibility>, inheritance: Option<Inheritance>, is_static: bool, function: Box<Blueprint<'b>>, });
new_node!(Namespace, NamespaceNode<'a> { name: CompactString, body: bumpalo::boxed::Box<'a, Node<'a>>, is_bracket: bool, }, NamespaceBlueprint<'b> { name: &'b str, body: Box<Blueprint<'b>>, is_bracket: bool, });
new_node!(Negate, NegateNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, NegateBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(New, NewNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, NewBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(NowDoc, NowDocNode { label: CompactString, value: CompactString, }, NowDocBlueprint<'b> { label: &'b str, value: &'b str, });
new_node!(Null, NullNode {}, NullBlueprint {});
new_node!(Number, NumberNode { value: CompactString, }, NumberBlueprint<'b> { value: &'b str, });
new_node!(ObjectAccess, ObjectAccessNode<'a> { object: bumpalo::boxed::Box<'a, Node<'a>>, property: bumpalo::boxed::Box<'a, Node<'a>>, use_bracket: bool, is_nullsafe: bool, }, ObjectAccessBlueprint<'b> { object: Box<Blueprint<'b>>, property: Box<Blueprint<'b>>, use_bracket: bool, is_nullsafe: bool, });
new_node!(Parameter, ParameterNode<'a> { variable_type: Option<bumpalo::boxed::Box<'a, Node<'a>>>, is_ref: bool, is_ellipsis: bool, name: bumpalo::boxed::Box<'a, Node<'a>>, value: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, ParameterBlueprint<'b> { variable_type: Option<Box<Blueprint<'b>>>, is_ref: bool, is_ellipsis: bool, name: Box<Blueprint<'b>>, value: Option<Box<Blueprint<'b>>>, });
new_node!(Parent, ParentNode {}, ParentBlueprint {});
new_node!(Parenthesis, ParenthesisNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, ParenthesisBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(Post, PostNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, operator: CompactString, }, PostBlueprint<'b> { statement: Box<Blueprint<'b>>, operator: &'b str, });
new_node!(Pre, PreNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, operator: CompactString, }, PreBlueprint<'b> { statement: Box<Blueprint<'b>>, operator: &'b str, });
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
new_node!(String, StringNode { quote: Quote, value: CompactString, }, StringBlueprint<'b> { quote: Quote, value: &'b str, });
new_node!(Switch, SwitchNode<'a> { condition: bumpalo::boxed::Box<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, is_short: bool, }, SwitchBlueprint<'b> { condition: Box<Blueprint<'b>>, body: Box<Blueprint<'b>>, is_short: bool, });
new_node!(Ternary, TernaryNode<'a> { condition: bumpalo::boxed::Box<'a, Node<'a>>, valid: bumpalo::boxed::Box<'a, Node<'a>>, invalid: bumpalo::boxed::Box<'a, Node<'a>>, }, TernaryBlueprint<'b> { condition: Box<Blueprint<'b>>, valid: Box<Blueprint<'b>>, invalid: Box<Blueprint<'b>>, });
new_node!(This, ThisNode {}, ThisBlueprint {});
new_node!(Throw, ThrowNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, ThrowBlueprint<'b> { statement: Box<Blueprint<'b>>, });
new_node!(Trait, TraitNode<'a> { name: bumpalo::boxed::Box<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, }, TraitBlueprint<'b> { name: Box<Blueprint<'b>>, body: Box<Blueprint<'b>>, });
new_node!(TraitUse, TraitUseNode<'a> { traits: bumpalo::collections::Vec<'a, Node<'a>>, adaptations: bumpalo::collections::Vec<'a, Node<'a>>, }, TraitUseBlueprint<'b> { traits: &'b [Box<Blueprint<'b>>], adaptations: &'b [Box<Blueprint<'b>>], });
new_node!(TraitUseAlias, TraitUseAliasNode<'a> { trait_name: Option<bumpalo::boxed::Box<'a, Node<'a>>>, method: bumpalo::boxed::Box<'a, Node<'a>>, alias: Option<bumpalo::boxed::Box<'a, Node<'a>>>, visibility: Option<Visibility>, }, TraitUseAliasBlueprint<'b> { trait_name: Option<Box<Blueprint<'b>>>, method: Box<Blueprint<'b>>, alias: Option<Box<Blueprint<'b>>>, visibility: Option<Visibility>, });
new_node!(TraitUsePrecedence, TraitUsePrecedenceNode<'a> { trait_name: Option<bumpalo::boxed::Box<'a, Node<'a>>>, method: bumpalo::boxed::Box<'a, Node<'a>>, instead: bumpalo::boxed::Box<'a, Node<'a>>, }, TraitUsePrecedenceBlueprint<'b> { trait_name: Option<Box<Blueprint<'b>>>, method: Box<Blueprint<'b>>, instead: Box<Blueprint<'b>>, });
new_node!(Try, TryNode<'a> { body: bumpalo::boxed::Box<'a, Node<'a>>, catches: bumpalo::collections::Vec<'a, Node<'a>>, }, TryBlueprint<'b> { body: Box<Blueprint<'b>>, catches: &'b [Box<Blueprint<'b>>], });
new_node!(Type, TypeNode { is_nullable: bool, name: CompactString, }, TypeBlueprint<'b> { is_nullable: bool, name: &'b str, });
new_node!(UnionType, UnionTypeNode<'a> { types: bumpalo::collections::Vec<'a, Node<'a>>, }, UnionTypeBlueprint<'b> { types: &'b [Box<Blueprint<'b>>], });
new_node!(Use, UseNode<'a> { name: Option<CompactString>, items: bumpalo::collections::Vec<'a, Node<'a>>, }, UseBlueprint<'b> { name: Option<&'b str>, items: &'b [Box<Blueprint<'b>>], });
new_node!(UseItem, UseItemNode<'a> { modifier: Option<UseItemModifier>, name: CompactString, alias: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, UseItemBlueprint<'b> { modifier: Option<UseItemModifier>, name: &'b str, alias: Option<Box<Blueprint<'b>>>, });
new_node!(Variable, VariableNode<'a> { name: bumpalo::boxed::Box<'a, Node<'a>>, }, VariableBlueprint<'b> { name: Box<Blueprint<'b>>, });
new_node!(Variadic, VariadicNode<'a> { statement: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, VariadicBlueprint<'b> { statement: Option<Box<Blueprint<'b>>>, });
new_node!(While, WhileNode<'a> { condition: bumpalo::boxed::Box<'a, Node<'a>>, body: bumpalo::boxed::Box<'a, Node<'a>>, is_short: bool, }, WhileBlueprint<'b> { condition: Box<Blueprint<'b>>, body: Box<Blueprint<'b>>, is_short: bool, });
new_node!(Yield, YieldNode<'a> { key: Option<bumpalo::boxed::Box<'a, Node<'a>>>, value: Option<bumpalo::boxed::Box<'a, Node<'a>>>, }, YieldBlueprint<'b> { key: Option<Box<Blueprint<'b>>>, value: Option<Box<Blueprint<'b>>>, });
new_node!(YieldFrom, YieldFromNode<'a> { statement: bumpalo::boxed::Box<'a, Node<'a>>, }, YieldFromBlueprint<'b> { statement: Box<Blueprint<'b>>, });
