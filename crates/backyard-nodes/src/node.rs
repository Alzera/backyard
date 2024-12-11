use std::fmt::{ self, Display, Formatter };

use bumpalo::{ boxed::Box, collections::Vec, vec, Bump };
use compact_str::CompactString;
use serde::Serialize;

use crate::utils::CloneIn;

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
  pub node: NodeWrapper<'a>,
  pub loc: Option<RangeLocation>,
  pub leadings: Option<Vec<'a, Node<'a>>>,
  pub trailings: Option<Vec<'a, Node<'a>>>,
}

impl<'a> Node<'a> {
  pub fn new(node_type: NodeType, node: NodeWrapper<'a>, loc: Option<RangeLocation>) -> Self {
    Self { node_type, node, loc, leadings: None, trailings: None }
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

macro_rules! new_node {
  (
    $node_type:ident,
    $struct_name:ident < $lt:lifetime > { $($field_name:ident: $field_type:ty),* $(,)? }
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
          node: NodeWrapper::$node_type(
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
  };

  ($node_type:ident, $struct_name:ident { $($field_name:ident: $field_type:ty),* $(,)? }) => {
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
  };
}

new_node!(AnonymousClass, AnonymousClassNode<'a> {
  parameters: Vec<'a, Node<'a>>,
  extends: Option<Box<'a, Node<'a>>>,
  implements: Vec<'a, Node<'a>>,
  body: Box<'a, Node<'a>>,
});
new_node!(AnonymousFunction, AnonymousFunctionNode<'a> {
  is_ref: bool,
  parameters: Vec<'a, Node<'a>>,
  uses: Vec<'a, Node<'a>>,
  return_type: Option<Box<'a, Node<'a>>>,
  body: Box<'a, Node<'a>>,
});
new_node!(CallArgument, CallArgumentNode<'a> {
  name: Option<Box<'a, Node<'a>>>,
  value: Box<'a, Node<'a>>,
});
new_node!(Array, ArrayNode<'a> {
  is_short: bool,
  items: Vec<'a, Node<'a>>,
});
new_node!(ArrayItem, ArrayItemNode<'a> {
  key: Option<Box<'a, Node<'a>>>,
  value: Box<'a, Node<'a>>,
});
new_node!(ArrayLookup, ArrayLookupNode<'a> {
  left: Box<'a, Node<'a>>,
  right: Option<Box<'a, Node<'a>>>,
});
new_node!(ArrowFunction, ArrowFunctionNode<'a> {
  is_ref: bool,
  parameters: Vec<'a, Node<'a>>,
  return_type: Option<Box<'a, Node<'a>>>,
  body: Box<'a, Node<'a>>,
});
new_node!(Assignment, AssignmentNode<'a> {
  left: Box<'a, Node<'a>>,
  operator: CompactString,
  right: Box<'a, Node<'a>>,
});
new_node!(Attribute, AttributeNode<'a> {
  items: Vec<'a, Node<'a>>,
});
new_node!(AttributeItem, AttributeItemNode<'a> {
  name: CompactString,
  arguments: Vec<'a, Node<'a>>,
});
new_node!(Bin, BinNode<'a> {
  left: Box<'a, Node<'a>>,
  operator: CompactString,
  right: Box<'a, Node<'a>>,
});
new_node!(Block, BlockNode<'a> {
  statements: Vec<'a, Node<'a>>,
});
new_node!(Boolean, BooleanNode {
  is_true: bool,
});
new_node!(Break, BreakNode<'a> {
  statement: Option<Box<'a, Node<'a>>>,
});
new_node!(Call, CallNode<'a> {
  name: Box<'a, Node<'a>>,
  arguments: Vec<'a, Node<'a>>,
});
new_node!(Case, CaseNode<'a> {
  condition: Option<Box<'a, Node<'a>>>,
  body: Box<'a, Node<'a>>,
});
new_node!(Cast, CastNode<'a> {
  cast_type: CompactString,
  expression: Box<'a, Node<'a>>,
});
new_node!(Catch, CatchNode<'a> {
  types: Vec<'a, Node<'a>>,
  variable: Option<Box<'a, Node<'a>>>,
  body: Box<'a, Node<'a>>,
});
new_node!(Class, ClassNode<'a> {
  inheritance: Option<Inheritance>,
  name: Option<Box<'a, Node<'a>>>,
  extends: Option<Box<'a, Node<'a>>>,
  implements: Vec<'a, Node<'a>>,
  body: Box<'a, Node<'a>>,
  is_readonly: bool,
});
new_node!(ClassKeyword, ClassKeywordNode {});
new_node!(Clone, CloneNode<'a> {
  statement: Box<'a, Node<'a>>,
});
new_node!(CommentBlock, CommentBlockNode {
  comment: CompactString,
});
new_node!(CommentDoc, CommentDocNode {
  comment: CompactString,
});
new_node!(CommentLine, CommentLineNode {
  comment: CompactString,
});
new_node!(Const, ConstNode<'a> {
  items: Vec<'a, Node<'a>>,
});
new_node!(ConstProperty, ConstPropertyNode<'a> {
  const_type: Option<Box<'a, Node<'a>>>,
  visibilities: std::vec::Vec<Visibility>,
  items: Vec<'a, Node<'a>>,
});
new_node!(ConstructorParameter, ConstructorParameterNode<'a> {
  visibilities: std::vec::Vec<Visibility>,
  modifier: Option<Modifier>,
  parameter: Box<'a, Node<'a>>,
});
new_node!(Continue, ContinueNode<'a> {
  statement: Option<Box<'a, Node<'a>>>,
});
new_node!(Declare, DeclareNode<'a> {
  arguments: Vec<'a, Node<'a>>,
  body: Option<Box<'a, Node<'a>>>,
  body_type: BodyType,
});
new_node!(DeclareArgument, DeclareArgumentNode<'a> {
  name: Box<'a, Node<'a>>,
  value: Box<'a, Node<'a>>,
});
new_node!(DoWhile, DoWhileNode<'a> {
  condition: Box<'a, Node<'a>>,
  body: Box<'a, Node<'a>>,
});
new_node!(DoWhileCondition, DoWhileConditionNode<'a> {
  condition: Box<'a, Node<'a>>,
});
new_node!(Echo, EchoNode<'a> {
  items: Vec<'a, Node<'a>>,
});
new_node!(Else, ElseNode<'a> {
  body: Box<'a, Node<'a>>,
  is_short: bool,
});
new_node!(Encapsed, EncapsedNode<'a> {
  quote: Quote,
  values: Vec<'a, Node<'a>>,
});
new_node!(EncapsedPart, EncapsedPartNode<'a> {
  is_advanced: bool,
  value: Box<'a, Node<'a>>,
});
new_node!(Enum, EnumNode<'a> {
  name: Box<'a, Node<'a>>,
  enum_type: Option<Box<'a, Node<'a>>>,
  implements: Option<Box<'a, Node<'a>>>,
  body: Vec<'a, Node<'a>>,
});
new_node!(EnumItem, EnumItemNode<'a> {
  value: Box<'a, Node<'a>>,
});
new_node!(Eval, EvalNode<'a> {
  statement: Box<'a, Node<'a>>,
});
new_node!(Exit, ExitNode<'a> {
  statement: Option<Box<'a, Node<'a>>>,
});
new_node!(Finally, FinallyNode<'a> {
  body: Box<'a, Node<'a>>,
});
new_node!(For, ForNode<'a> {
  inits: Vec<'a, Node<'a>>,
  tests: Vec<'a, Node<'a>>,
  increments: Vec<'a, Node<'a>>,
  body: Option<Box<'a, Node<'a>>>,
  body_type: BodyType,
});
new_node!(Foreach, ForeachNode<'a> {
  source: Box<'a, Node<'a>>,
  key: Option<Box<'a, Node<'a>>>,
  value: Box<'a, Node<'a>>,
  body: Box<'a, Node<'a>>,
  is_short: bool,
});
new_node!(Function, FunctionNode<'a> {
  is_ref: bool,
  name: Box<'a, Node<'a>>,
  parameters: Vec<'a, Node<'a>>,
  return_type: Option<Box<'a, Node<'a>>>,
  body: Option<Box<'a, Node<'a>>>,
});
new_node!(Global, GlobalNode<'a> {
  items: Vec<'a, Node<'a>>,
});
new_node!(Goto, GotoNode<'a> {
  label: Box<'a, Node<'a>>,
});
new_node!(HereDoc, HereDocNode<'a> {
  label: CompactString,
  values: Vec<'a, Node<'a>>,
});
new_node!(Identifier, IdentifierNode {
  name: CompactString,
});
new_node!(If, IfNode<'a> {
  condition: Box<'a, Node<'a>>,
  valid: Box<'a, Node<'a>>,
  invalid: Option<Box<'a, Node<'a>>>,
  is_short: bool,
});
new_node!(Include, IncludeNode<'a> {
  use_parenthesis: bool,
  is_require: bool,
  is_once: bool,
  argument: Box<'a, Node<'a>>,
});
new_node!(Inline, InlineNode {
  text: CompactString,
});
new_node!(Interface, InterfaceNode<'a> {
  name: Box<'a, Node<'a>>,
  extends: Vec<'a, Node<'a>>,
  body: Box<'a, Node<'a>>,
});
new_node!(IntersectionType, IntersectionTypeNode<'a> {
  types: Vec<'a, Node<'a>>,
});
new_node!(Label, LabelNode<'a> {
  label: Box<'a, Node<'a>>,
});
new_node!(List, ListNode<'a> {
  items: Vec<'a, Node<'a>>,
});
new_node!(Magic, MagicNode {
  name: CompactString,
});
new_node!(Match, MatchNode<'a> {
  condition: Box<'a, Node<'a>>,
  arms: Vec<'a, Node<'a>>,
});
new_node!(MatchArm, MatchArmNode<'a> {
  conditions: Vec<'a, Node<'a>>,
  expr: Box<'a, Node<'a>>,
});
new_node!(Method, MethodNode<'a> {
  visibility: Option<Visibility>,
  inheritance: Option<Inheritance>,
  is_static: bool,
  function: Box<'a, Node<'a>>,
});
new_node!(Namespace, NamespaceNode<'a> {
  name: CompactString,
  body: Box<'a, Node<'a>>,
  is_bracket: bool,
});
new_node!(Negate, NegateNode<'a> {
  statement: Box<'a, Node<'a>>,
});
new_node!(New, NewNode<'a> {
  statement: Box<'a, Node<'a>>,
});
new_node!(NowDoc, NowDocNode {
  label: CompactString,
  value: CompactString,
});
new_node!(Null, NullNode {});
new_node!(Number, NumberNode {
  value: CompactString,
});
new_node!(ObjectAccess, ObjectAccessNode<'a> {
  object: Box<'a, Node<'a>>,
  property: Box<'a, Node<'a>>,
  use_bracket: bool,
  is_nullsafe: bool,
});
new_node!(Parameter, ParameterNode<'a> {
  variable_type: Option<Box<'a, Node<'a>>>,
  is_ref: bool,
  is_ellipsis: bool,
  name: Box<'a, Node<'a>>,
  value: Option<Box<'a, Node<'a>>>,
});
new_node!(Parent, ParentNode {});
new_node!(Parenthesis, ParenthesisNode<'a> {
  statement: Box<'a, Node<'a>>,
});
new_node!(Post, PostNode<'a> {
  statement: Box<'a, Node<'a>>,
  operator: CompactString,
});
new_node!(Pre, PreNode<'a> {
  statement: Box<'a, Node<'a>>,
  operator: CompactString,
});
new_node!(Print, PrintNode<'a> {
  statement: Box<'a, Node<'a>>,
});
new_node!(Program, ProgramNode<'a> {
  children: Vec<'a, Node<'a>>,
});
new_node!(Property, PropertyNode<'a> {
  visibilities: std::vec::Vec<Visibility>,
  modifier: Option<Modifier>,
  hooks: Vec<'a, Node<'a>>,
  items: Vec<'a, Node<'a>>,
});
new_node!(PropertyHook, PropertyHookNode<'a> {
  is_get: bool,
  is_ref: bool,
  parameters: Vec<'a, Node<'a>>,
  body: Box<'a, Node<'a>>,
});
new_node!(PropertyItem, PropertyItemNode<'a> {
  name: Box<'a, Node<'a>>,
  variable_type: Option<Box<'a, Node<'a>>>,
  value: Option<Box<'a, Node<'a>>>,
});
new_node!(Reference, ReferenceNode<'a> {
  statement: Box<'a, Node<'a>>,
});
new_node!(Return, ReturnNode<'a> {
  statement: Option<Box<'a, Node<'a>>>,
});
new_node!(SelfKeyword, SelfNode {});
new_node!(Silent, SilentNode<'a> {
  statement: Box<'a, Node<'a>>,
});
new_node!(Static, StaticNode<'a> {
  items: Vec<'a, Node<'a>>,
});
new_node!(StaticKeyword, StaticKeywordNode {});
new_node!(StaticLookup, StaticLookupNode<'a> {
  left: Box<'a, Node<'a>>,
  right: Box<'a, Node<'a>>,
  use_bracket: bool,
});
new_node!(String, StringNode {
  quote: Quote,
  value: CompactString,
});
new_node!(Switch, SwitchNode<'a> {
  condition: Box<'a, Node<'a>>,
  body: Box<'a, Node<'a>>,
  is_short: bool,
});
new_node!(Ternary, TernaryNode<'a> {
  condition: Box<'a, Node<'a>>,
  valid: Box<'a, Node<'a>>,
  invalid: Box<'a, Node<'a>>,
});
new_node!(This, ThisNode {});
new_node!(Throw, ThrowNode<'a> {
  statement: Box<'a, Node<'a>>,
});
new_node!(Trait, TraitNode<'a> {
  name: Box<'a, Node<'a>>,
  body: Box<'a, Node<'a>>,
});
new_node!(TraitUse, TraitUseNode<'a> {
  traits: Vec<'a, Node<'a>>,
  adaptations: Vec<'a, Node<'a>>,
});
new_node!(TraitUseAlias, TraitUseAliasNode<'a> {
  trait_name: Option<Box<'a, Node<'a>>>,
  method: Box<'a, Node<'a>>,
  alias: Option<Box<'a, Node<'a>>>,
  visibility: Option<Visibility>,
});
new_node!(TraitUsePrecedence, TraitUsePrecedenceNode<'a> {
  trait_name: Option<Box<'a, Node<'a>>>,
  method: Box<'a, Node<'a>>,
  instead: Box<'a, Node<'a>>,
});
new_node!(Try, TryNode<'a> {
  body: Box<'a, Node<'a>>,
  catches: Vec<'a, Node<'a>>,
});
new_node!(Type, TypeNode {
  is_nullable: bool,
  name: CompactString,
});
new_node!(UnionType, UnionTypeNode<'a> {
  types: Vec<'a, Node<'a>>,
});
new_node!(Use, UseNode<'a> {
  name: Option<CompactString>,
  items: Vec<'a, Node<'a>>,
});
new_node!(UseItem, UseItemNode<'a> {
  modifier: Option<UseItemModifier>,
  name: CompactString,
  alias: Option<Box<'a, Node<'a>>>,
});
new_node!(Variable, VariableNode<'a> {
  name: Box<'a, Node<'a>>,
});
new_node!(Variadic, VariadicNode<'a> {
  statement: Option<Box<'a, Node<'a>>>,
});
new_node!(While, WhileNode<'a> {
  condition: Box<'a, Node<'a>>,
  body: Box<'a, Node<'a>>,
  is_short: bool,
});
new_node!(Yield, YieldNode<'a> {
  key: Option<Box<'a, Node<'a>>>,
  value: Option<Box<'a, Node<'a>>>,
});
new_node!(YieldFrom, YieldFromNode<'a> {
  statement: Box<'a, Node<'a>>,
});
