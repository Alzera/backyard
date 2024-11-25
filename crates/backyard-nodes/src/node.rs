use std::fmt;

use serde::{ de::{ self, MapAccess, Visitor }, Deserialize, Deserializer, Serialize };
use ts_rs::TS;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum BodyType {
  Basic,
  Short,
  Empty,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct Node {
  pub leadings: Vec<Box<Node>>,
  pub trailings: Vec<Box<Node>>,
  pub node_type: NodeType,
  #[serde(flatten)]
  pub node: NodeWrapper,
}

impl<'de> Deserialize<'de> for Node {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
    struct NodeVisitor;

    impl<'de> Visitor<'de> for NodeVisitor {
      type Value = Node;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid Node structure")
      }

      fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error> where M: MapAccess<'de> {
        let mut leadings = None;
        let mut trailings = None;
        let mut node_type = None;
        let mut node_data = None;

        while let Some(key) = map.next_key::<String>()? {
          match key.as_str() {
            "leadings" => {
              leadings = Some(map.next_value()?);
            }
            "trailings" => {
              trailings = Some(map.next_value()?);
            }
            "node_type" => {
              node_type = Some(map.next_value()?);
            }
            _ => {
              // Assuming `#[serde(flatten)]` attributes allow arbitrary extra fields
              if node_data.is_none() {
                node_data = Some(serde_json::Value::Object(Default::default()));
              }
              if let Some(ref mut obj) = node_data {
                if let serde_json::Value::Object(map_obj) = obj {
                  map_obj.insert(key, map.next_value()?);
                }
              }
            }
          }
        }

        let leadings = leadings.unwrap_or_default();
        let trailings = trailings.unwrap_or_default();
        let node_type = node_type.ok_or_else(|| de::Error::missing_field("node_type"))?;
        let node_data = node_data.unwrap_or_else(|| serde_json::Value::Object(Default::default()));

        let node: NodeWrapper = (
          match node_type {
            NodeType::AnonymousClass => {
              serde_json::from_value(node_data).map(NodeWrapper::AnonymousClass)
            }
            NodeType::AnonymousFunction => {
              serde_json::from_value(node_data).map(NodeWrapper::AnonymousFunction)
            }
            NodeType::Argument => { serde_json::from_value(node_data).map(NodeWrapper::Argument) }
            NodeType::Array => { serde_json::from_value(node_data).map(NodeWrapper::Array) }
            NodeType::ArrayItem => { serde_json::from_value(node_data).map(NodeWrapper::ArrayItem) }
            NodeType::ArrayLookup => {
              serde_json::from_value(node_data).map(NodeWrapper::ArrayLookup)
            }
            NodeType::ArrowFunction => {
              serde_json::from_value(node_data).map(NodeWrapper::ArrowFunction)
            }
            NodeType::Assignment => {
              serde_json::from_value(node_data).map(NodeWrapper::Assignment)
            }
            NodeType::Attribute => { serde_json::from_value(node_data).map(NodeWrapper::Attribute) }
            NodeType::AttributeItem => {
              serde_json::from_value(node_data).map(NodeWrapper::AttributeItem)
            }
            NodeType::Bin => { serde_json::from_value(node_data).map(NodeWrapper::Bin) }
            NodeType::Block => { serde_json::from_value(node_data).map(NodeWrapper::Block) }
            NodeType::Boolean => { serde_json::from_value(node_data).map(NodeWrapper::Boolean) }
            NodeType::Break => { serde_json::from_value(node_data).map(NodeWrapper::Break) }
            NodeType::Call => { serde_json::from_value(node_data).map(NodeWrapper::Call) }
            NodeType::Case => { serde_json::from_value(node_data).map(NodeWrapper::Case) }
            NodeType::Cast => { serde_json::from_value(node_data).map(NodeWrapper::Cast) }
            NodeType::Catch => { serde_json::from_value(node_data).map(NodeWrapper::Catch) }
            NodeType::Class => { serde_json::from_value(node_data).map(NodeWrapper::Class) }
            NodeType::ClassKeyword => {
              serde_json::from_value(node_data).map(NodeWrapper::ClassKeyword)
            }
            NodeType::Clone => { serde_json::from_value(node_data).map(NodeWrapper::Clone) }
            NodeType::CommentBlock => {
              serde_json::from_value(node_data).map(NodeWrapper::CommentBlock)
            }
            NodeType::CommentDoc => {
              serde_json::from_value(node_data).map(NodeWrapper::CommentDoc)
            }
            NodeType::CommentLine => {
              serde_json::from_value(node_data).map(NodeWrapper::CommentLine)
            }
            NodeType::Const => { serde_json::from_value(node_data).map(NodeWrapper::Const) }
            NodeType::ConstProperty => {
              serde_json::from_value(node_data).map(NodeWrapper::ConstProperty)
            }
            NodeType::Continue => { serde_json::from_value(node_data).map(NodeWrapper::Continue) }
            NodeType::Declare => { serde_json::from_value(node_data).map(NodeWrapper::Declare) }
            NodeType::DeclareArgument => {
              serde_json::from_value(node_data).map(NodeWrapper::DeclareArgument)
            }
            NodeType::DoWhile => { serde_json::from_value(node_data).map(NodeWrapper::DoWhile) }
            NodeType::DoWhileCondition => {
              serde_json::from_value(node_data).map(NodeWrapper::DoWhileCondition)
            }
            NodeType::Echo => { serde_json::from_value(node_data).map(NodeWrapper::Echo) }
            NodeType::Elvis => { serde_json::from_value(node_data).map(NodeWrapper::Elvis) }
            NodeType::Else => { serde_json::from_value(node_data).map(NodeWrapper::Else) }
            NodeType::Encapsed => { serde_json::from_value(node_data).map(NodeWrapper::Encapsed) }
            NodeType::EncapsedPart => {
              serde_json::from_value(node_data).map(NodeWrapper::EncapsedPart)
            }
            NodeType::Enum => { serde_json::from_value(node_data).map(NodeWrapper::Enum) }
            NodeType::EnumItem => { serde_json::from_value(node_data).map(NodeWrapper::EnumItem) }
            NodeType::Eval => { serde_json::from_value(node_data).map(NodeWrapper::Eval) }
            NodeType::Exit => { serde_json::from_value(node_data).map(NodeWrapper::Exit) }
            NodeType::Finally => { serde_json::from_value(node_data).map(NodeWrapper::Finally) }
            NodeType::For => { serde_json::from_value(node_data).map(NodeWrapper::For) }
            NodeType::Foreach => { serde_json::from_value(node_data).map(NodeWrapper::Foreach) }
            NodeType::Function => { serde_json::from_value(node_data).map(NodeWrapper::Function) }
            NodeType::Global => { serde_json::from_value(node_data).map(NodeWrapper::Global) }
            NodeType::Goto => { serde_json::from_value(node_data).map(NodeWrapper::Goto) }
            NodeType::HereDoc => { serde_json::from_value(node_data).map(NodeWrapper::HereDoc) }
            NodeType::Identifier => {
              serde_json::from_value(node_data).map(NodeWrapper::Identifier)
            }
            NodeType::If => { serde_json::from_value(node_data).map(NodeWrapper::If) }
            NodeType::Include => { serde_json::from_value(node_data).map(NodeWrapper::Include) }
            NodeType::Inline => { serde_json::from_value(node_data).map(NodeWrapper::Inline) }
            NodeType::InstanceOf => {
              serde_json::from_value(node_data).map(NodeWrapper::InstanceOf)
            }
            NodeType::Interface => { serde_json::from_value(node_data).map(NodeWrapper::Interface) }
            NodeType::IntersectionType => {
              serde_json::from_value(node_data).map(NodeWrapper::IntersectionType)
            }
            NodeType::Label => { serde_json::from_value(node_data).map(NodeWrapper::Label) }
            NodeType::List => { serde_json::from_value(node_data).map(NodeWrapper::List) }
            NodeType::Magic => { serde_json::from_value(node_data).map(NodeWrapper::Magic) }
            NodeType::Match => { serde_json::from_value(node_data).map(NodeWrapper::Match) }
            NodeType::MatchArm => { serde_json::from_value(node_data).map(NodeWrapper::MatchArm) }
            NodeType::Method => { serde_json::from_value(node_data).map(NodeWrapper::Method) }
            NodeType::Namespace => { serde_json::from_value(node_data).map(NodeWrapper::Namespace) }
            NodeType::Negate => { serde_json::from_value(node_data).map(NodeWrapper::Negate) }
            NodeType::New => { serde_json::from_value(node_data).map(NodeWrapper::New) }
            NodeType::NowDoc => { serde_json::from_value(node_data).map(NodeWrapper::NowDoc) }
            NodeType::Null => { serde_json::from_value(node_data).map(NodeWrapper::Null) }
            NodeType::Number => { serde_json::from_value(node_data).map(NodeWrapper::Number) }
            NodeType::ObjectAccess => {
              serde_json::from_value(node_data).map(NodeWrapper::ObjectAccess)
            }
            NodeType::Parameter => { serde_json::from_value(node_data).map(NodeWrapper::Parameter) }
            NodeType::Parent => { serde_json::from_value(node_data).map(NodeWrapper::Parent) }
            NodeType::Parenthesis => {
              serde_json::from_value(node_data).map(NodeWrapper::Parenthesis)
            }
            NodeType::Post => { serde_json::from_value(node_data).map(NodeWrapper::Post) }
            NodeType::Pre => { serde_json::from_value(node_data).map(NodeWrapper::Pre) }
            NodeType::Print => { serde_json::from_value(node_data).map(NodeWrapper::Print) }
            NodeType::Program => { serde_json::from_value(node_data).map(NodeWrapper::Program) }
            NodeType::Property => { serde_json::from_value(node_data).map(NodeWrapper::Property) }
            NodeType::PropertyItem => {
              serde_json::from_value(node_data).map(NodeWrapper::PropertyItem)
            }
            NodeType::Reference => { serde_json::from_value(node_data).map(NodeWrapper::Reference) }
            NodeType::Return => { serde_json::from_value(node_data).map(NodeWrapper::Return) }
            NodeType::SelfKeyword => {
              serde_json::from_value(node_data).map(NodeWrapper::SelfKeyword)
            }
            NodeType::Silent => { serde_json::from_value(node_data).map(NodeWrapper::Silent) }
            NodeType::Static => { serde_json::from_value(node_data).map(NodeWrapper::Static) }
            NodeType::StaticKeyword => {
              serde_json::from_value(node_data).map(NodeWrapper::StaticKeyword)
            }
            NodeType::StaticLookup => {
              serde_json::from_value(node_data).map(NodeWrapper::StaticLookup)
            }
            NodeType::String => { serde_json::from_value(node_data).map(NodeWrapper::String) }
            NodeType::Switch => { serde_json::from_value(node_data).map(NodeWrapper::Switch) }
            NodeType::Ternary => { serde_json::from_value(node_data).map(NodeWrapper::Ternary) }
            NodeType::This => { serde_json::from_value(node_data).map(NodeWrapper::This) }
            NodeType::Trait => { serde_json::from_value(node_data).map(NodeWrapper::Trait) }
            NodeType::TraitUse => { serde_json::from_value(node_data).map(NodeWrapper::TraitUse) }
            NodeType::TraitUseAlias => {
              serde_json::from_value(node_data).map(NodeWrapper::TraitUseAlias)
            }
            NodeType::TraitUsePrecedence => {
              serde_json::from_value(node_data).map(NodeWrapper::TraitUsePrecedence)
            }
            NodeType::Throw => { serde_json::from_value(node_data).map(NodeWrapper::Throw) }
            NodeType::Try => { serde_json::from_value(node_data).map(NodeWrapper::Try) }
            NodeType::Type => { serde_json::from_value(node_data).map(NodeWrapper::Type) }
            NodeType::UnionType => { serde_json::from_value(node_data).map(NodeWrapper::UnionType) }
            NodeType::Use => { serde_json::from_value(node_data).map(NodeWrapper::Use) }
            NodeType::UseItem => { serde_json::from_value(node_data).map(NodeWrapper::UseItem) }
            NodeType::Variable => { serde_json::from_value(node_data).map(NodeWrapper::Variable) }
            NodeType::Variadic => { serde_json::from_value(node_data).map(NodeWrapper::Variadic) }
            NodeType::While => { serde_json::from_value(node_data).map(NodeWrapper::While) }
            NodeType::Yield => { serde_json::from_value(node_data).map(NodeWrapper::Yield) }
            NodeType::YieldFrom => { serde_json::from_value(node_data).map(NodeWrapper::YieldFrom) }
          }
        ).map_err(de::Error::custom)?;

        Ok(Node {
          leadings,
          trailings,
          node_type,
          node,
        })
      }
    }

    deserializer.deserialize_map(NodeVisitor)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(untagged)]
pub enum NodeWrapper {
  AnonymousClass(AnonymousClassNode),
  AnonymousFunction(AnonymousFunctionNode),
  Argument(ArgumentNode),
  Array(ArrayNode),
  ArrayItem(ArrayItemNode),
  ArrayLookup(ArrayLookupNode),
  ArrowFunction(ArrowFunctionNode),
  Assignment(AssignmentNode),
  Attribute(AttributeNode),
  AttributeItem(AttributeItemNode),
  Bin(BinNode),
  Block(BlockNode),
  Boolean(BooleanNode),
  Break(BreakNode),
  Call(CallNode),
  Case(CaseNode),
  Cast(CastNode),
  Catch(CatchNode),
  Class(ClassNode),
  ClassKeyword(ClassKeywordNode),
  Clone(CloneNode),
  CommentBlock(CommentBlockNode),
  CommentDoc(CommentDocNode),
  CommentLine(CommentLineNode),
  Const(ConstNode),
  ConstProperty(ConstPropertyNode),
  Continue(ContinueNode),
  Declare(DeclareNode),
  DeclareArgument(DeclareArgumentNode),
  DoWhile(DoWhileNode),
  DoWhileCondition(DoWhileConditionNode),
  Echo(EchoNode),
  Else(ElseNode),
  Elvis(ElvisNode),
  Encapsed(EncapsedNode),
  EncapsedPart(EncapsedPartNode),
  Enum(EnumNode),
  EnumItem(EnumItemNode),
  Eval(EvalNode),
  Exit(ExitNode),
  Finally(FinallyNode),
  For(ForNode),
  Foreach(ForeachNode),
  Function(FunctionNode),
  Global(GlobalNode),
  Goto(GotoNode),
  HereDoc(HereDocNode),
  Identifier(IdentifierNode),
  If(IfNode),
  Include(IncludeNode),
  Inline(InlineNode),
  InstanceOf(InstanceOfNode),
  Interface(InterfaceNode),
  IntersectionType(IntersectionTypeNode),
  Label(LabelNode),
  List(ListNode),
  Magic(MagicNode),
  Match(MatchNode),
  MatchArm(MatchArmNode),
  Method(MethodNode),
  Namespace(NamespaceNode),
  Negate(NegateNode),
  New(NewNode),
  NowDoc(NowDocNode),
  Null(NullNode),
  Number(NumberNode),
  ObjectAccess(ObjectAccessNode),
  Parameter(ParameterNode),
  Parent(ParentNode),
  Parenthesis(ParenthesisNode),
  Post(PostNode),
  Pre(PreNode),
  Print(PrintNode),
  Program(ProgramNode),
  Property(PropertyNode),
  PropertyItem(PropertyItemNode),
  Reference(ReferenceNode),
  Return(ReturnNode),
  SelfKeyword(SelfNode),
  Silent(SilentNode),
  Static(StaticNode),
  StaticKeyword(StaticKeywordNode),
  StaticLookup(StaticLookupNode),
  String(StringNode),
  Switch(SwitchNode),
  Ternary(TernaryNode),
  This(ThisNode),
  Trait(TraitNode),
  TraitUse(TraitUseNode),
  TraitUseAlias(TraitUseAliasNode),
  TraitUsePrecedence(TraitUsePrecedenceNode),
  Throw(ThrowNode),
  Try(TryNode),
  Type(TypeNode),
  UnionType(UnionTypeNode),
  Use(UseNode),
  UseItem(UseItemNode),
  Variable(VariableNode),
  Variadic(VariadicNode),
  While(WhileNode),
  Yield(YieldNode),
  YieldFrom(YieldFromNode),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
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
  Continue,
  Declare,
  DeclareArgument,
  DoWhile,
  DoWhileCondition,
  Echo,
  Else,
  Elvis,
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
  InstanceOf,
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
  ($node_type:ident, $struct_name:ident { $($field_name:ident: $field_type:ty),* $(,)? }) => {
    #[derive(Debug, Clone, Serialize, Deserialize, TS)]
    #[ts(export)]
    pub struct $struct_name {
      $(pub $field_name: $field_type),*
    }
    impl $struct_name {
      pub fn new($($field_name: $field_type),*) -> Box<Node> {
        Box::new(
          Node {
            leadings: vec![],
            trailings: vec![],
            node_type: NodeType::$node_type,
            node: NodeWrapper::$node_type(
              Self { $($field_name),* }
            )
          }
        )
      }
    }
  };
}

new_node!(Attribute, AttributeNode {
  items: Vec<Box<Node>>,
});

new_node!(AttributeItem, AttributeItemNode {
  name: String,
  arguments: Vec<Box<Node>>,
});

new_node!(Array, ArrayNode {
  is_short: bool,
  items: Vec<Box<Node>>,
});

new_node!(Variadic, VariadicNode {
  expr: Option<Box<Node>>,
});

new_node!(ArrayItem, ArrayItemNode {
  key: Option<Box<Node>>,
  value: Box<Node>,
});

new_node!(ArrayLookup, ArrayLookupNode {
  target: Box<Node>,
  on: Option<Box<Node>>,
});

new_node!(Assignment, AssignmentNode {
  left: Box<Node>,
  operator: String,
  right: Box<Node>,
});

new_node!(Bin, BinNode {
  left: Box<Node>,
  operator: String,
  right: Box<Node>,
});

new_node!(Block, BlockNode {
  statements: Vec<Box<Node>>,
});

new_node!(Call, CallNode {
  name: Box<Node>,
  arguments: Vec<Box<Node>>,
});

new_node!(Argument, ArgumentNode {
  name: Option<Box<Node>>,
  value: Box<Node>,
});

new_node!(Class, ClassNode {
  modifier: String,
  name: Option<Box<Node>>,
  extends: Option<Box<Node>>,
  implements: Vec<Box<Node>>,
  body: Box<Node>,
  is_readonly: bool,
});

new_node!(AnonymousClass, AnonymousClassNode {
  parameters: Vec<Box<Node>>,
  extends: Option<Box<Node>>,
  implements: Vec<Box<Node>>,
  body: Box<Node>,
});

new_node!(CommentBlock, CommentBlockNode {
  comment: String,
});

new_node!(CommentDoc, CommentDocNode {
  comment: String,
});

new_node!(CommentLine, CommentLineNode {
  comment: String,
});

new_node!(Const, ConstNode {
  consts: Vec<Box<Node>>,
});

new_node!(ConstProperty, ConstPropertyNode {
  visibility: String,
  consts: Vec<Box<Node>>,
});

new_node!(Declare, DeclareNode {
  arguments: Vec<Box<Node>>,
  body: Option<Box<Node>>,
  body_type: BodyType,
});

new_node!(DeclareArgument, DeclareArgumentNode {
  name: Box<Node>,
  value: Box<Node>,
});

new_node!(DoWhile, DoWhileNode {
  condition: Box<Node>,
  body: Box<Node>,
});

new_node!(DoWhileCondition, DoWhileConditionNode {
  condition: Box<Node>,
});

new_node!(Enum, EnumNode {
  name: Box<Node>,
  enum_type: Option<Box<Node>>,
  implements: Option<Box<Node>>,
  items: Vec<Box<Node>>,
});

new_node!(EnumItem, EnumItemNode {
  value: Box<Node>,
});

new_node!(Eval, EvalNode {
  argument: Box<Node>,
});

new_node!(Exit, ExitNode {
  argument: Option<Box<Node>>,
});

new_node!(Foreach, ForeachNode {
  source: Box<Node>,
  key: Option<Box<Node>>,
  value: Box<Node>,
  body: Box<Node>,
  is_short: bool,
});

new_node!(For, ForNode {
  inits: Vec<Box<Node>>,
  tests: Vec<Box<Node>>,
  increments: Vec<Box<Node>>,
  body: Option<Box<Node>>,
  body_type: BodyType,
});

new_node!(Function, FunctionNode {
  is_ref: bool,
  name: Box<Node>,
  parameters: Vec<Box<Node>>,
  return_type: Option<Box<Node>>,
  body: Option<Box<Node>>,
});

new_node!(ArrowFunction, ArrowFunctionNode {
  is_ref: bool,
  parameters: Vec<Box<Node>>,
  return_type: Option<Box<Node>>,
  body: Box<Node>,
});

new_node!(AnonymousFunction, AnonymousFunctionNode {
  is_ref: bool,
  parameters: Vec<Box<Node>>,
  uses: Vec<Box<Node>>,
  return_type: Option<Box<Node>>,
  body: Box<Node>,
});

new_node!(Parameter, ParameterNode {
  variable_type: Option<Box<Node>>,
  is_ref: bool,
  is_ellipsis: bool,
  name: Box<Node>,
  value: Option<Box<Node>>,
});

new_node!(Identifier, IdentifierNode {
  name: String,
});

new_node!(If, IfNode {
  condition: Box<Node>,
  valid: Box<Node>,
  invalid: Option<Box<Node>>,
  is_short: bool,
});

new_node!(Else, ElseNode {
  body: Box<Node>,
  is_short: bool,
});

new_node!(Include, IncludeNode {
  use_parenthesis: bool,
  is_require: bool,
  is_once: bool,
  argument: Box<Node>,
});

new_node!(InstanceOf, InstanceOfNode {
  left: Box<Node>,
  right: Box<Node>,
});

new_node!(Interface, InterfaceNode {
  name: Box<Node>,
  extends: Vec<Box<Node>>,
  body: Box<Node>,
});

new_node!(Label, LabelNode {
  label: Box<Node>,
});

new_node!(List, ListNode {
  values: Vec<Box<Node>>,
});

new_node!(Magic, MagicNode {
  name: String,
});

new_node!(MatchArm, MatchArmNode {
  conditions: Vec<Box<Node>>,
  body: Box<Node>,
});

new_node!(Match, MatchNode {
  condition: Box<Node>,
  arms: Vec<Box<Node>>,
});

new_node!(Method, MethodNode {
  visibility: String,
  modifier: String,
  is_static: bool,
  function: Box<Node>,
});

new_node!(Namespace, NamespaceNode {
  name: String, 
  body: Box<Node>,
  is_bracket: bool,
});

new_node!(Number, NumberNode {
  value: String,
});

new_node!(ObjectAccess, ObjectAccessNode {
  object: Box<Node>,
  property: Box<Node>,
  bracket: bool,
});

new_node!(Parenthesis, ParenthesisNode {
  statement: Box<Node>,
});

new_node!(Cast, CastNode {
  target: String,
  expression: Box<Node>,
});

new_node!(Post, PostNode {
  variable: Box<Node>,
  operator: String,
});

new_node!(Pre, PreNode {
  variable: Box<Node>,
  operator: String,
});

new_node!(Negate, NegateNode {
  variable: Box<Node>,
});

new_node!(Silent, SilentNode {
  variable: Box<Node>,
});

new_node!(Reference, ReferenceNode {
  expr: Box<Node>,
});

new_node!(Program, ProgramNode {
  opentag: String,
  children: Vec<Box<Node>>,
});

new_node!(PropertyItem, PropertyItemNode {
  name: Box<Node>,
  variable_type: Option<Box<Node>>,
  value: Option<Box<Node>>,
});

new_node!(Property, PropertyNode {
  visibility: String,
  modifier: String,
  items: Vec<Box<Node>>,
});

new_node!(Break, BreakNode {
  argument: Option<Box<Node>>,
});

new_node!(Continue, ContinueNode {
  argument: Option<Box<Node>>,
});

new_node!(Return, ReturnNode {
  argument: Option<Box<Node>>,
});

new_node!(Echo, EchoNode {
  arguments: Vec<Box<Node>>,
});

new_node!(Elvis, ElvisNode {
  left: Box<Node>,
  right: Box<Node>,
});

new_node!(New, NewNode {
  argument: Box<Node>,
});

new_node!(Throw, ThrowNode {
  argument: Box<Node>,
});

new_node!(Print, PrintNode {
  argument: Box<Node>,
});

new_node!(Parent, ParentNode {});

new_node!(Static, StaticNode {
  items: Vec<Box<Node>>,
});

new_node!(StaticKeyword, StaticKeywordNode {});

new_node!(ClassKeyword, ClassKeywordNode {});

new_node!(Null, NullNode {});

new_node!(Boolean, BooleanNode {
  is_true: bool,
});

new_node!(Inline, InlineNode {
  text: String,
});

new_node!(Clone, CloneNode {
  argument: Box<Node>,
});

new_node!(Global, GlobalNode {
  argument: Box<Node>,
});

new_node!(Goto, GotoNode {
  label: Box<Node>,
});

new_node!(StaticLookup, StaticLookupNode {
  target: Box<Node>,
  on: Box<Node>,
});

new_node!(String, StringNode {
  quote: String,
  value: String,
});

new_node!(NowDoc, NowDocNode {
  label: String,
  value: String,
});

new_node!(HereDoc, HereDocNode {
  label: String,
  values: Vec<Box<Node>>,
});

new_node!(Encapsed, EncapsedNode {
  quote: String,
  values: Vec<Box<Node>>,
});

new_node!(EncapsedPart, EncapsedPartNode {
  is_advanced: bool,
  value: Box<Node>,
});

new_node!(Switch, SwitchNode {
  condition: Box<Node>,
  body: Box<Node>,
  is_short: bool,
});

new_node!(Case, CaseNode {
  condition: Option<Box<Node>>,
  body: Box<Node>,
});

new_node!(Ternary, TernaryNode {
  condition: Box<Node>,
  valid: Box<Node>,
  invalid: Box<Node>,
});

new_node!(This, ThisNode {});

new_node!(SelfKeyword, SelfNode {});

new_node!(Trait, TraitNode {
  name: Box<Node>,
  body: Box<Node>,
});

new_node!(TraitUse, TraitUseNode {
  traits: Vec<Box<Node>>,
  adaptations: Vec<Box<Node>>,
});

new_node!(TraitUseAlias, TraitUseAliasNode {
  trait_name: Option<Box<Node>>,
  method: Box<Node>,
  alias: Option<Box<Node>>,
  visibility: String,
});

new_node!(TraitUsePrecedence, TraitUsePrecedenceNode {
  trait_name: Option<Box<Node>>,
  method: Box<Node>,
  instead: Box<Node>,
});

new_node!(Try, TryNode {
  body: Box<Node>,
  catches: Vec<Box<Node>>,
});

new_node!(Catch, CatchNode {
  types: Vec<Box<Node>>,
  variable: Option<Box<Node>>,
  body: Box<Node>,
});

new_node!(Finally, FinallyNode {
  body: Box<Node>,
});

new_node!(Type, TypeNode {
  is_nullable: bool,
  name: String,
});

new_node!(UnionType, UnionTypeNode {
  types: Vec<String>,
});

new_node!(IntersectionType, IntersectionTypeNode {
  types: Vec<String>,
});

new_node!(Use, UseNode {
  name: Option<String>,
  items: Vec<Box<Node>>,
});

new_node!(UseItem, UseItemNode {
  modifier: String,
  name: String,
  alias: Option<Box<Node>>,
});

new_node!(Variable, VariableNode {
  name: Box<Node>,
});

new_node!(While, WhileNode {
  condition: Box<Node>,
  body: Box<Node>,
  is_short: bool,
});

new_node!(YieldFrom, YieldFromNode {
  value: Box<Node>,
});

new_node!(Yield, YieldNode {
  key: Option<Box<Node>>,
  value: Option<Box<Node>>,
});
