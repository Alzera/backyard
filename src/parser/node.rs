use std::{ any::Any, fmt::Debug };

use dyn_clone::DynClone;
use napi::bindgen_prelude::{ FromNapiValue, ToNapiValue };

use crate::guard;

use super::nodes::{
  array::*,
  arraylookup::*,
  assignment::*,
  bin::*,
  block::*,
  call::*,
  class::*,
  comment::*,
  consts::*,
  declare::*,
  dowhile::*,
  enums::*,
  eval::*,
  exit::*,
  foreach::*,
  fors::*,
  function::*,
  global::*,
  goto::*,
  identifier::*,
  ifs::*,
  include::*,
  instanceof::*,
  interface::*,
  label::*,
  list::*,
  magic::*,
  matchs::*,
  method::*,
  namespace::*,
  number::*,
  objectaccess::*,
  parenthesis::*,
  post::*,
  pre::*,
  program::*,
  property::*,
  singles::*,
  staticlookup::*,
  string::*,
  switch::*,
  ternary::*,
  traits::*,
  traituse::*,
  tries::*,
  types::*,
  uses::*,
  variable::*,
  whiles::*,
  yields::*,
};

#[derive(Debug, Clone, PartialEq)]
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
  Break,
  Call,
  Case,
  Cast,
  Catch,
  Class,
  Clone,
  CommentBlock,
  CommentLine,
  Const,
  ConstProperty,
  Continue,
  Declare,
  DeclareArgument,
  DoWhile,
  Echo,
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
  Static,
  StaticLookup,
  String,
  Switch,
  Ternary,
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

impl NodeType {
  pub fn to_string(&self) -> &'static str {
    match self {
      NodeType::AnonymousFunction => "anonymous_function",
      NodeType::Argument => "argument",
      NodeType::Array => "array",
      NodeType::ArrayItem => "array_item",
      NodeType::ArrayLookup => "array_lookup",
      NodeType::ArrowFunction => "arrow_function",
      NodeType::Assignment => "assignment",
      NodeType::Bin => "bin",
      NodeType::Block => "block",
      NodeType::Break => "break",
      NodeType::Call => "call",
      NodeType::Case => "case",
      NodeType::Cast => "cast",
      NodeType::Catch => "catch",
      NodeType::Class => "class",
      NodeType::Clone => "clone",
      NodeType::CommentBlock => "comment_block",
      NodeType::CommentLine => "comment_line",
      NodeType::Const => "const",
      NodeType::ConstProperty => "const_property",
      NodeType::Continue => "continue",
      NodeType::Declare => "declare",
      NodeType::DeclareArgument => "declare_argument",
      NodeType::DoWhile => "do_while",
      NodeType::Echo => "echo",
      NodeType::Encapsed => "encapsed",
      NodeType::EncapsedPart => "encapsed_part",
      NodeType::Enum => "enum",
      NodeType::EnumItem => "enum_item",
      NodeType::Eval => "eval",
      NodeType::Exit => "exit",
      NodeType::For => "for",
      NodeType::Foreach => "foreach",
      NodeType::Function => "function",
      NodeType::Global => "global",
      NodeType::Goto => "goto",
      NodeType::Identifier => "identifier",
      NodeType::If => "if",
      NodeType::Include => "include",
      NodeType::InstanceOf => "instanceof",
      NodeType::Interface => "interface",
      NodeType::Label => "label",
      NodeType::List => "list",
      NodeType::Magic => "magic",
      NodeType::Match => "match",
      NodeType::MatchArm => "match_arm",
      NodeType::Method => "method",
      NodeType::Namespace => "namespace",
      NodeType::New => "new",
      NodeType::Number => "number",
      NodeType::ObjectAccess => "object_access",
      NodeType::Parameter => "parameter",
      NodeType::Parent => "parent",
      NodeType::Parenthesis => "parenthesis",
      NodeType::Post => "post",
      NodeType::Pre => "pre",
      NodeType::Print => "print",
      NodeType::Program => "program",
      NodeType::Property => "property",
      NodeType::PropertyItem => "property_item",
      NodeType::Return => "return",
      NodeType::Static => "static",
      NodeType::StaticLookup => "static_lookup",
      NodeType::String => "string",
      NodeType::Switch => "switch",
      NodeType::Ternary => "ternary",
      NodeType::Trait => "trait",
      NodeType::TraitUse => "trait_use",
      NodeType::TraitUseAlias => "trait_use_alias",
      NodeType::TraitUsePrecedence => "trait_use_precedence",
      NodeType::Throw => "throw",
      NodeType::Try => "try",
      NodeType::Type => "type",
      NodeType::Use => "use",
      NodeType::Variable => "variable",
      NodeType::While => "while",
      NodeType::Yield => "yield",
      NodeType::YieldFrom => "yield_from",
    }
  }

  fn from_string(s: &String) -> Option<Self> {
    match s.as_str() {
      "anonymous_function" => Some(NodeType::AnonymousFunction),
      "argument" => Some(NodeType::Argument),
      "array" => Some(NodeType::Array),
      "array_item" => Some(NodeType::ArrayItem),
      "array_lookup" => Some(NodeType::ArrayLookup),
      "arrow_function" => Some(NodeType::ArrowFunction),
      "assignment" => Some(NodeType::Assignment),
      "bin" => Some(NodeType::Bin),
      "block" => Some(NodeType::Block),
      "break" => Some(NodeType::Break),
      "call" => Some(NodeType::Call),
      "case" => Some(NodeType::Case),
      "cast" => Some(NodeType::Cast),
      "catch" => Some(NodeType::Catch),
      "class" => Some(NodeType::Class),
      "clone" => Some(NodeType::Clone),
      "comment_block" => Some(NodeType::CommentBlock),
      "comment_line" => Some(NodeType::CommentLine),
      "const" => Some(NodeType::Const),
      "const_property" => Some(NodeType::ConstProperty),
      "continue" => Some(NodeType::Continue),
      "declare" => Some(NodeType::Declare),
      "declare_argument" => Some(NodeType::DeclareArgument),
      "do_while" => Some(NodeType::DoWhile),
      "echo" => Some(NodeType::Echo),
      "encapsed" => Some(NodeType::Encapsed),
      "encapsed_part" => Some(NodeType::EncapsedPart),
      "enum" => Some(NodeType::Enum),
      "enum_item" => Some(NodeType::EnumItem),
      "eval" => Some(NodeType::Eval),
      "exit" => Some(NodeType::Exit),
      "for" => Some(NodeType::For),
      "foreach" => Some(NodeType::Foreach),
      "function" => Some(NodeType::Function),
      "global" => Some(NodeType::Global),
      "goto" => Some(NodeType::Goto),
      "identifier" => Some(NodeType::Identifier),
      "if" => Some(NodeType::If),
      "include" => Some(NodeType::Include),
      "instanceof" => Some(NodeType::InstanceOf),
      "interface" => Some(NodeType::Interface),
      "label" => Some(NodeType::Label),
      "list" => Some(NodeType::List),
      "magic" => Some(NodeType::Magic),
      "match" => Some(NodeType::Match),
      "match_arm" => Some(NodeType::MatchArm),
      "method" => Some(NodeType::Method),
      "namespace" => Some(NodeType::Namespace),
      "new" => Some(NodeType::New),
      "number" => Some(NodeType::Number),
      "object_access" => Some(NodeType::ObjectAccess),
      "parameter" => Some(NodeType::Parameter),
      "parent" => Some(NodeType::Parent),
      "parenthesis" => Some(NodeType::Parenthesis),
      "post" => Some(NodeType::Post),
      "pre" => Some(NodeType::Pre),
      "print" => Some(NodeType::Print),
      "program" => Some(NodeType::Program),
      "property" => Some(NodeType::Property),
      "property_item" => Some(NodeType::PropertyItem),
      "return" => Some(NodeType::Return),
      "static" => Some(NodeType::Static),
      "static_lookup" => Some(NodeType::StaticLookup),
      "string" => Some(NodeType::String),
      "switch" => Some(NodeType::Switch),
      "ternary" => Some(NodeType::Ternary),
      "trait" => Some(NodeType::Trait),
      "trait_use" => Some(NodeType::TraitUse),
      "trait_use_alias" => Some(NodeType::TraitUseAlias),
      "trait_use_precedence" => Some(NodeType::TraitUsePrecedence),
      "throw" => Some(NodeType::Throw),
      "try" => Some(NodeType::Try),
      "type" => Some(NodeType::Type),
      "use" => Some(NodeType::Use),
      "variable" => Some(NodeType::Variable),
      "while" => Some(NodeType::While),
      "yield" => Some(NodeType::Yield),
      "yield_from" => Some(NodeType::YieldFrom),
      _ => None,
    }
  }
}

pub trait NodeTrait: Debug + DynClone + Any {
  fn get_type(&self) -> NodeType;
  fn as_any(self: Box<Self>) -> Box<dyn Any>;

  fn add_leading_comments(&mut self, comments: Node);
  fn add_trailing_comments(&mut self, comments: Node);

  unsafe fn to_napi(&self, env: napi::sys::napi_env) -> napi::Result<napi::sys::napi_value>;
  fn from_napi(env: napi::sys::napi_env, val: napi::JsObject) -> Box<Self> where Self: Sized;
}
dyn_clone::clone_trait_object!(NodeTrait);

pub type Node = Box<dyn NodeTrait>;
pub type Nodes = Vec<Node>;

pub trait NodeTraitCast {
  fn cast<T: 'static>(self) -> Result<Box<T>, Box<dyn Any>> where T: NodeTrait;
}

impl NodeTraitCast for Node {
  fn cast<T: 'static>(self) -> Result<Box<T>, Box<dyn Any>> where T: NodeTrait {
    self.as_any().downcast::<T>()
  }
}

impl ToNapiValue for Node {
  unsafe fn to_napi_value(
    env: napi::sys::napi_env,
    val: Self
  ) -> napi::Result<napi::sys::napi_value> {
    val.to_napi(env)
  }
}

impl FromNapiValue for Node {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value
  ) -> napi::Result<Self> {
    let val = napi::JsObject::from_napi_value(env, napi_val)?;
    let obj_type = guard!(val.get("type")?, String::new());
    if let Some(node_type) = NodeType::from_string(&obj_type) {
      let node: Node = match node_type {
        NodeType::AnonymousFunction => AnonymousFunctionNode::from_napi(env, val),
        NodeType::Argument => ArgumentNode::from_napi(env, val),
        NodeType::Array => ArrayNode::from_napi(env, val),
        NodeType::ArrayItem => ArrayItemNode::from_napi(env, val),
        NodeType::ArrayLookup => ArrayLookupNode::from_napi(env, val),
        NodeType::ArrowFunction => ArrowFunctionNode::from_napi(env, val),
        NodeType::Assignment => AssignmentNode::from_napi(env, val),
        NodeType::Bin => BinNode::from_napi(env, val),
        NodeType::Block => BlockNode::from_napi(env, val),
        NodeType::Break => BreakNode::from_napi(env, val),
        NodeType::Call => CallNode::from_napi(env, val),
        NodeType::Case => CaseNode::from_napi(env, val),
        NodeType::Cast => CastNode::from_napi(env, val),
        NodeType::Catch => CatchNode::from_napi(env, val),
        NodeType::Class => ClassNode::from_napi(env, val),
        NodeType::Clone => CloneNode::from_napi(env, val),
        NodeType::CommentBlock => CommentBlockNode::from_napi(env, val),
        NodeType::CommentLine => CommentLineNode::from_napi(env, val),
        NodeType::Const => ConstNode::from_napi(env, val),
        NodeType::ConstProperty => ConstPropertyNode::from_napi(env, val),
        NodeType::Continue => ContinueNode::from_napi(env, val),
        NodeType::Declare => DeclareNode::from_napi(env, val),
        NodeType::DeclareArgument => DeclareArgumentNode::from_napi(env, val),
        NodeType::DoWhile => DoWhileNode::from_napi(env, val),
        NodeType::Echo => EchoNode::from_napi(env, val),
        NodeType::Encapsed => EncapsedNode::from_napi(env, val),
        NodeType::EncapsedPart => EncapsedPartNode::from_napi(env, val),
        NodeType::Enum => EnumNode::from_napi(env, val),
        NodeType::EnumItem => EnumItemNode::from_napi(env, val),
        NodeType::Eval => EvalNode::from_napi(env, val),
        NodeType::Exit => ExitNode::from_napi(env, val),
        NodeType::For => ForNode::from_napi(env, val),
        NodeType::Foreach => ForeachNode::from_napi(env, val),
        NodeType::Function => FunctionNode::from_napi(env, val),
        NodeType::Global => GlobalNode::from_napi(env, val),
        NodeType::Goto => GotoNode::from_napi(env, val),
        NodeType::Identifier => IdentifierNode::from_napi(env, val),
        NodeType::If => IfNode::from_napi(env, val),
        NodeType::Include => IncludeNode::from_napi(env, val),
        NodeType::InstanceOf => InstanceOfNode::from_napi(env, val),
        NodeType::Interface => InterfaceNode::from_napi(env, val),
        NodeType::Label => LabelNode::from_napi(env, val),
        NodeType::List => ListNode::from_napi(env, val),
        NodeType::Magic => MagicNode::from_napi(env, val),
        NodeType::Match => MatchNode::from_napi(env, val),
        NodeType::MatchArm => MatchArmNode::from_napi(env, val),
        NodeType::Method => MethodNode::from_napi(env, val),
        NodeType::Namespace => NamespaceNode::from_napi(env, val),
        NodeType::New => NewNode::from_napi(env, val),
        NodeType::Number => NumberNode::from_napi(env, val),
        NodeType::ObjectAccess => ObjectAccessNode::from_napi(env, val),
        NodeType::Parameter => ParameterNode::from_napi(env, val),
        NodeType::Parent => ParentNode::from_napi(env, val),
        NodeType::Parenthesis => ParenthesisNode::from_napi(env, val),
        NodeType::Post => PostNode::from_napi(env, val),
        NodeType::Pre => PreNode::from_napi(env, val),
        NodeType::Print => PrintNode::from_napi(env, val),
        NodeType::Program => ProgramNode::from_napi(env, val),
        NodeType::Property => PropertyNode::from_napi(env, val),
        NodeType::PropertyItem => PropertyItemNode::from_napi(env, val),
        NodeType::Return => ReturnNode::from_napi(env, val),
        NodeType::Static => StaticNode::from_napi(env, val),
        NodeType::StaticLookup => StaticLookupNode::from_napi(env, val),
        NodeType::String => StringNode::from_napi(env, val),
        NodeType::Switch => SwitchNode::from_napi(env, val),
        NodeType::Ternary => TernaryNode::from_napi(env, val),
        NodeType::Trait => TraitNode::from_napi(env, val),
        NodeType::TraitUse => TraitUseNode::from_napi(env, val),
        NodeType::TraitUseAlias => TraitUseAliasNode::from_napi(env, val),
        NodeType::TraitUsePrecedence => TraitUsePrecedenceNode::from_napi(env, val),
        NodeType::Throw => ThrowNode::from_napi(env, val),
        NodeType::Try => TryNode::from_napi(env, val),
        NodeType::Type => TypeNode::from_napi(env, val),
        NodeType::Use => UseNode::from_napi(env, val),
        NodeType::Variable => VariableNode::from_napi(env, val),
        NodeType::While => WhileNode::from_napi(env, val),
        NodeType::Yield => YieldNode::from_napi(env, val),
        NodeType::YieldFrom => YieldFromNode::from_napi(env, val),
      };
      return Ok(node);
    }
    Err(
      napi::Error::new(
        napi::Status::InvalidArg,
        format!("Invalid node type: {}", obj_type).as_str()
      )
    )
  }
}

#[napi]
#[derive(Debug, PartialEq)]
pub enum BodyType {
  Basic,
  Short,
  Empty,
}
