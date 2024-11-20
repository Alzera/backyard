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
  pub leading_comments: Vec<Box<Node>>,
  pub trailing_comments: Vec<Box<Node>>,
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
        let mut leading_comments = None;
        let mut trailing_comments = None;
        let mut node_type = None;
        let mut node_data = None;

        while let Some(key) = map.next_key::<String>()? {
          match key.as_str() {
            "leading_comments" => {
              leading_comments = Some(map.next_value()?);
            }
            "trailing_comments" => {
              trailing_comments = Some(map.next_value()?);
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

        let leading_comments = leading_comments.unwrap_or_default();
        let trailing_comments = trailing_comments.unwrap_or_default();
        let node_type = node_type.ok_or_else(|| de::Error::missing_field("node_type"))?;
        let node_data = node_data.unwrap_or_else(|| serde_json::Value::Object(Default::default()));

        let node: NodeWrapper = (
          match node_type {
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
            NodeType::Bin => { serde_json::from_value(node_data).map(NodeWrapper::Bin) }
            NodeType::Block => { serde_json::from_value(node_data).map(NodeWrapper::Block) }
            NodeType::Boolean => { serde_json::from_value(node_data).map(NodeWrapper::Boolean) }
            NodeType::Break => { serde_json::from_value(node_data).map(NodeWrapper::Break) }
            NodeType::Call => { serde_json::from_value(node_data).map(NodeWrapper::Call) }
            NodeType::Case => { serde_json::from_value(node_data).map(NodeWrapper::Case) }
            NodeType::Cast => { serde_json::from_value(node_data).map(NodeWrapper::Cast) }
            NodeType::Catch => { serde_json::from_value(node_data).map(NodeWrapper::Catch) }
            NodeType::Class => { serde_json::from_value(node_data).map(NodeWrapper::Class) }
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
            NodeType::Echo => { serde_json::from_value(node_data).map(NodeWrapper::Echo) }
            NodeType::Elvis => { serde_json::from_value(node_data).map(NodeWrapper::Elvis) }
            NodeType::Encapsed => { serde_json::from_value(node_data).map(NodeWrapper::Encapsed) }
            NodeType::EncapsedPart => {
              serde_json::from_value(node_data).map(NodeWrapper::EncapsedPart)
            }
            NodeType::Enum => { serde_json::from_value(node_data).map(NodeWrapper::Enum) }
            NodeType::EnumItem => { serde_json::from_value(node_data).map(NodeWrapper::EnumItem) }
            NodeType::Eval => { serde_json::from_value(node_data).map(NodeWrapper::Eval) }
            NodeType::Exit => { serde_json::from_value(node_data).map(NodeWrapper::Exit) }
            NodeType::For => { serde_json::from_value(node_data).map(NodeWrapper::For) }
            NodeType::Foreach => { serde_json::from_value(node_data).map(NodeWrapper::Foreach) }
            NodeType::Function => { serde_json::from_value(node_data).map(NodeWrapper::Function) }
            NodeType::Global => { serde_json::from_value(node_data).map(NodeWrapper::Global) }
            NodeType::Goto => { serde_json::from_value(node_data).map(NodeWrapper::Goto) }
            NodeType::Identifier => {
              serde_json::from_value(node_data).map(NodeWrapper::Identifier)
            }
            NodeType::If => { serde_json::from_value(node_data).map(NodeWrapper::If) }
            NodeType::Include => { serde_json::from_value(node_data).map(NodeWrapper::Include) }
            NodeType::InstanceOf => {
              serde_json::from_value(node_data).map(NodeWrapper::InstanceOf)
            }
            NodeType::Interface => { serde_json::from_value(node_data).map(NodeWrapper::Interface) }
            NodeType::Label => { serde_json::from_value(node_data).map(NodeWrapper::Label) }
            NodeType::List => { serde_json::from_value(node_data).map(NodeWrapper::List) }
            NodeType::Magic => { serde_json::from_value(node_data).map(NodeWrapper::Magic) }
            NodeType::Match => { serde_json::from_value(node_data).map(NodeWrapper::Match) }
            NodeType::MatchArm => { serde_json::from_value(node_data).map(NodeWrapper::MatchArm) }
            NodeType::Method => { serde_json::from_value(node_data).map(NodeWrapper::Method) }
            NodeType::Namespace => { serde_json::from_value(node_data).map(NodeWrapper::Namespace) }
            NodeType::Negate => { serde_json::from_value(node_data).map(NodeWrapper::Negate) }
            NodeType::Negative => { serde_json::from_value(node_data).map(NodeWrapper::Negative) }
            NodeType::New => { serde_json::from_value(node_data).map(NodeWrapper::New) }
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
            NodeType::Return => { serde_json::from_value(node_data).map(NodeWrapper::Return) }
            NodeType::Silent => { serde_json::from_value(node_data).map(NodeWrapper::Silent) }
            NodeType::Static => { serde_json::from_value(node_data).map(NodeWrapper::Static) }
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
            NodeType::Use => { serde_json::from_value(node_data).map(NodeWrapper::Use) }
            NodeType::Variable => { serde_json::from_value(node_data).map(NodeWrapper::Variable) }
            NodeType::While => { serde_json::from_value(node_data).map(NodeWrapper::While) }
            NodeType::Yield => { serde_json::from_value(node_data).map(NodeWrapper::Yield) }
            NodeType::YieldFrom => { serde_json::from_value(node_data).map(NodeWrapper::YieldFrom) }
          }
        ).map_err(de::Error::custom)?;

        Ok(Node {
          leading_comments,
          trailing_comments,
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
  AnonymousFunction(AnonymousFunctionNode),
  Argument(ArgumentNode),
  Array(ArrayNode),
  ArrayItem(ArrayItemNode),
  ArrayLookup(ArrayLookupNode),
  ArrowFunction(ArrowFunctionNode),
  Assignment(AssignmentNode),
  Bin(BinNode),
  Block(BlockNode),
  Boolean(BooleanNode),
  Break(BreakNode),
  Call(CallNode),
  Case(CaseNode),
  Cast(CastNode),
  Catch(CatchNode),
  Class(ClassNode),
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
  Echo(EchoNode),
  Elvis(ElvisNode),
  Encapsed(EncapsedNode),
  EncapsedPart(EncapsedPartNode),
  Enum(EnumNode),
  EnumItem(EnumItemNode),
  Eval(EvalNode),
  Exit(ExitNode),
  For(ForNode),
  Foreach(ForeachNode),
  Function(FunctionNode),
  Global(GlobalNode),
  Goto(GotoNode),
  Identifier(IdentifierNode),
  If(IfNode),
  Include(IncludeNode),
  InstanceOf(InstanceOfNode),
  Interface(InterfaceNode),
  Label(LabelNode),
  List(ListNode),
  Magic(MagicNode),
  Match(MatchNode),
  MatchArm(MatchArmNode),
  Method(MethodNode),
  Namespace(NamespaceNode),
  Negate(NegateNode),
  Negative(NegativeNode),
  New(NewNode),
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
  Return(ReturnNode),
  Silent(SilentNode),
  Static(StaticNode),
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
  Use(UseNode),
  Variable(VariableNode),
  While(WhileNode),
  Yield(YieldNode),
  YieldFrom(YieldFromNode),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
  AnonymousFunction,
  Argument,
  Array,
  ArrayItem,
  ArrayLookup,
  ArrowFunction,
  Assignment,
  Bin,
  Block,
  Boolean,
  Break,
  Call,
  Case,
  Cast,
  Catch,
  Class,
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
  Echo,
  Elvis,
  Encapsed,
  EncapsedPart,
  Enum,
  EnumItem,
  Eval,
  Exit,
  For,
  Foreach,
  Function,
  Global,
  Goto,
  Identifier,
  If,
  Include,
  InstanceOf,
  Interface,
  Label,
  List,
  Magic,
  Match,
  MatchArm,
  Method,
  Namespace,
  Negate,
  Negative,
  New,
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
  Return,
  Silent,
  Static,
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
  Use,
  Variable,
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
            leading_comments: vec![],
            trailing_comments: vec![],
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

new_node!(Array, ArrayNode {
  is_ellipsis: bool,
  items: Vec<Box<Node>>,
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
  name: Box<Node>,
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

new_node!(Enum, EnumNode {
  name: Box<Node>,
  items: Vec<Box<Node>>,
});

new_node!(EnumItem, EnumItemNode {
  value: Box<Node>,
});

new_node!(Eval, EvalNode {
  argument: Box<Node>,
});

new_node!(Exit, ExitNode {
  argument: Box<Node>,
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

new_node!(Include, IncludeNode {
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
  implements: Vec<Box<Node>>,
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
  names: Vec<Box<Node>>, 
  body: Box<Node>,
  is_bracket: bool,
});

new_node!(Number, NumberNode {
  value: String,
});

new_node!(ObjectAccess, ObjectAccessNode {
  object: Box<Node>,
  property: Box<Node>,
});

new_node!(Parenthesis, ParenthesisNode {
  statement: Box<Node>,
});

new_node!(Cast, CastNode {
  target: Box<Node>,
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

new_node!(Negative, NegativeNode {
  variable: Box<Node>,
});

new_node!(Program, ProgramNode {
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
  argument: Box<Node>,
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

new_node!(Static, StaticNode {});

new_node!(Boolean, BooleanNode {
  is_true: bool,
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
  value: String,
});

new_node!(Encapsed, EncapsedNode {
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
  alias: Box<Node>,
  visibility: String,
});

new_node!(TraitUsePrecedence, TraitUsePrecedenceNode {
  trait_name: Box<Node>,
  method: Box<Node>,
  instead: Box<Node>,
});

new_node!(Try, TryNode {
  body: Box<Node>,
  catches: Vec<Box<Node>>,
  finally: Option<Box<Node>>,
});

new_node!(Catch, CatchNode {
  types: Vec<Box<Node>>,
  variable: Option<Box<Node>>,
  body: Box<Node>,
});

new_node!(Type, TypeNode {
  is_nullable: bool,
  name: Vec<String>,
});

new_node!(Use, UseNode {
  modifier: String,
  names: Vec<Box<Node>>,
  items: Vec<Box<Node>>,
});

new_node!(Variable, VariableNode {
  is_ref: bool,
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
  value: Box<Node>,
});
