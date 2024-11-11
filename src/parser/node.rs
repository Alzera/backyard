use std::{ any::Any, fmt::Debug };

use dyn_clone::DynClone;
use napi::{ bindgen_prelude::{ FromNapiValue, ToNapiValue }, JsUnknown };

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
  CommentDoc,
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

// impl NodeType {
// pub fn to_string(&self) -> &'static str {
//   match self {
//     NodeType::AnonymousFunction => "anonymous_function",
//     NodeType::Argument => "argument",
//     NodeType::Array => "array",
//     NodeType::ArrayItem => "array_item",
//     NodeType::ArrayLookup => "array_lookup",
//     NodeType::ArrowFunction => "arrow_function",
//     NodeType::Assignment => "assignment",
//     NodeType::Bin => "bin",
//     NodeType::Block => "block",
//     NodeType::Break => "break",
//     NodeType::Call => "call",
//     NodeType::Case => "case",
//     NodeType::Cast => "cast",
//     NodeType::Catch => "catch",
//     NodeType::Class => "class",
//     NodeType::Clone => "clone",
//     NodeType::CommentBlock => "comment_block",
//     NodeType::CommentDoc => "comment_doc",
//     NodeType::CommentLine => "comment_line",
//     NodeType::Const => "const",
//     NodeType::ConstProperty => "const_property",
//     NodeType::Continue => "continue",
//     NodeType::Declare => "declare",
//     NodeType::DeclareArgument => "declare_argument",
//     NodeType::DoWhile => "do_while",
//     NodeType::Echo => "echo",
//     NodeType::Encapsed => "encapsed",
//     NodeType::EncapsedPart => "encapsed_part",
//     NodeType::Enum => "enum",
//     NodeType::EnumItem => "enum_item",
//     NodeType::Eval => "eval",
//     NodeType::Exit => "exit",
//     NodeType::For => "for",
//     NodeType::Foreach => "foreach",
//     NodeType::Function => "function",
//     NodeType::Global => "global",
//     NodeType::Goto => "goto",
//     NodeType::Identifier => "identifier",
//     NodeType::If => "if",
//     NodeType::Include => "include",
//     NodeType::InstanceOf => "instanceof",
//     NodeType::Interface => "interface",
//     NodeType::Label => "label",
//     NodeType::List => "list",
//     NodeType::Magic => "magic",
//     NodeType::Match => "match",
//     NodeType::MatchArm => "match_arm",
//     NodeType::Method => "method",
//     NodeType::Namespace => "namespace",
//     NodeType::New => "new",
//     NodeType::Number => "number",
//     NodeType::ObjectAccess => "object_access",
//     NodeType::Parameter => "parameter",
//     NodeType::Parent => "parent",
//     NodeType::Parenthesis => "parenthesis",
//     NodeType::Post => "post",
//     NodeType::Pre => "pre",
//     NodeType::Print => "print",
//     NodeType::Program => "program",
//     NodeType::Property => "property",
//     NodeType::PropertyItem => "property_item",
//     NodeType::Return => "return",
//     NodeType::Static => "static",
//     NodeType::StaticLookup => "static_lookup",
//     NodeType::String => "string",
//     NodeType::Switch => "switch",
//     NodeType::Ternary => "ternary",
//     NodeType::Trait => "trait",
//     NodeType::TraitUse => "trait_use",
//     NodeType::TraitUseAlias => "trait_use_alias",
//     NodeType::TraitUsePrecedence => "trait_use_precedence",
//     NodeType::Throw => "throw",
//     NodeType::Try => "try",
//     NodeType::Type => "type",
//     NodeType::Use => "use",
//     NodeType::Variable => "variable",
//     NodeType::While => "while",
//     NodeType::Yield => "yield",
//     NodeType::YieldFrom => "yield_from",
//   }
// }

// fn from_string(s: &String) -> Option<Self> {
//   match s.as_str() {
//     "anonymous_function" => Some(NodeType::AnonymousFunction),
//     "argument" => Some(NodeType::Argument),
//     "array" => Some(NodeType::Array),
//     "array_item" => Some(NodeType::ArrayItem),
//     "array_lookup" => Some(NodeType::ArrayLookup),
//     "arrow_function" => Some(NodeType::ArrowFunction),
//     "assignment" => Some(NodeType::Assignment),
//     "bin" => Some(NodeType::Bin),
//     "block" => Some(NodeType::Block),
//     "break" => Some(NodeType::Break),
//     "call" => Some(NodeType::Call),
//     "case" => Some(NodeType::Case),
//     "cast" => Some(NodeType::Cast),
//     "catch" => Some(NodeType::Catch),
//     "class" => Some(NodeType::Class),
//     "clone" => Some(NodeType::Clone),
//     "comment_block" => Some(NodeType::CommentBlock),
//     "comment_doc" => Some(NodeType::CommentDoc),
//     "comment_line" => Some(NodeType::CommentLine),
//     "const" => Some(NodeType::Const),
//     "const_property" => Some(NodeType::ConstProperty),
//     "continue" => Some(NodeType::Continue),
//     "declare" => Some(NodeType::Declare),
//     "declare_argument" => Some(NodeType::DeclareArgument),
//     "do_while" => Some(NodeType::DoWhile),
//     "echo" => Some(NodeType::Echo),
//     "encapsed" => Some(NodeType::Encapsed),
//     "encapsed_part" => Some(NodeType::EncapsedPart),
//     "enum" => Some(NodeType::Enum),
//     "enum_item" => Some(NodeType::EnumItem),
//     "eval" => Some(NodeType::Eval),
//     "exit" => Some(NodeType::Exit),
//     "for" => Some(NodeType::For),
//     "foreach" => Some(NodeType::Foreach),
//     "function" => Some(NodeType::Function),
//     "global" => Some(NodeType::Global),
//     "goto" => Some(NodeType::Goto),
//     "identifier" => Some(NodeType::Identifier),
//     "if" => Some(NodeType::If),
//     "include" => Some(NodeType::Include),
//     "instanceof" => Some(NodeType::InstanceOf),
//     "interface" => Some(NodeType::Interface),
//     "label" => Some(NodeType::Label),
//     "list" => Some(NodeType::List),
//     "magic" => Some(NodeType::Magic),
//     "match" => Some(NodeType::Match),
//     "match_arm" => Some(NodeType::MatchArm),
//     "method" => Some(NodeType::Method),
//     "namespace" => Some(NodeType::Namespace),
//     "new" => Some(NodeType::New),
//     "number" => Some(NodeType::Number),
//     "object_access" => Some(NodeType::ObjectAccess),
//     "parameter" => Some(NodeType::Parameter),
//     "parent" => Some(NodeType::Parent),
//     "parenthesis" => Some(NodeType::Parenthesis),
//     "post" => Some(NodeType::Post),
//     "pre" => Some(NodeType::Pre),
//     "print" => Some(NodeType::Print),
//     "program" => Some(NodeType::Program),
//     "property" => Some(NodeType::Property),
//     "property_item" => Some(NodeType::PropertyItem),
//     "return" => Some(NodeType::Return),
//     "static" => Some(NodeType::Static),
//     "static_lookup" => Some(NodeType::StaticLookup),
//     "string" => Some(NodeType::String),
//     "switch" => Some(NodeType::Switch),
//     "ternary" => Some(NodeType::Ternary),
//     "trait" => Some(NodeType::Trait),
//     "trait_use" => Some(NodeType::TraitUse),
//     "trait_use_alias" => Some(NodeType::TraitUseAlias),
//     "trait_use_precedence" => Some(NodeType::TraitUsePrecedence),
//     "throw" => Some(NodeType::Throw),
//     "try" => Some(NodeType::Try),
//     "type" => Some(NodeType::Type),
//     "use" => Some(NodeType::Use),
//     "variable" => Some(NodeType::Variable),
//     "while" => Some(NodeType::While),
//     "yield" => Some(NodeType::Yield),
//     "yield_from" => Some(NodeType::YieldFrom),
//     _ => None,
//   }
// }

// fn try_napi_parse(&self, env: napi::sys::napi_env, val: napi::JsObject) -> Node {
//   match self {
//     NodeType::AnonymousFunction => AnonymousFunctionNode::from_napi(env, val),
//     NodeType::Argument => ArgumentNode::from_napi(env, val),
//     NodeType::Array => ArrayNode::from_napi(env, val),
//     NodeType::ArrayItem => ArrayItemNode::from_napi(env, val),
//     NodeType::ArrayLookup => ArrayLookupNode::from_napi(env, val),
//     NodeType::ArrowFunction => ArrowFunctionNode::from_napi(env, val),
//     NodeType::Assignment => AssignmentNode::from_napi(env, val),
//     NodeType::Bin => BinNode::from_napi(env, val),
//     NodeType::Block => BlockNode::from_napi(env, val),
//     NodeType::Break => BreakNode::from_napi(env, val),
//     NodeType::Call => CallNode::from_napi(env, val),
//     NodeType::Case => CaseNode::from_napi(env, val),
//     NodeType::Cast => CastNode::from_napi(env, val),
//     NodeType::Catch => CatchNode::from_napi(env, val),
//     NodeType::Class => ClassNode::from_napi(env, val),
//     NodeType::Clone => CloneNode::from_napi(env, val),
//     NodeType::CommentBlock => CommentBlockNode::from_napi(env, val),
//     NodeType::CommentDoc => CommentDocNode::from_napi(env, val),
//     NodeType::CommentLine => CommentLineNode::from_napi(env, val),
//     NodeType::Const => ConstNode::from_napi(env, val),
//     NodeType::ConstProperty => ConstPropertyNode::from_napi(env, val),
//     NodeType::Continue => ContinueNode::from_napi(env, val),
//     NodeType::Declare => DeclareNode::from_napi(env, val),
//     NodeType::DeclareArgument => DeclareArgumentNode::from_napi(env, val),
//     NodeType::DoWhile => DoWhileNode::from_napi(env, val),
//     NodeType::Echo => EchoNode::from_napi(env, val),
//     NodeType::Encapsed => EncapsedNode::from_napi(env, val),
//     NodeType::EncapsedPart => EncapsedPartNode::from_napi(env, val),
//     NodeType::Enum => EnumNode::from_napi(env, val),
//     NodeType::EnumItem => EnumItemNode::from_napi(env, val),
//     NodeType::Eval => EvalNode::from_napi(env, val),
//     NodeType::Exit => ExitNode::from_napi(env, val),
//     NodeType::For => ForNode::from_napi(env, val),
//     NodeType::Foreach => ForeachNode::from_napi(env, val),
//     NodeType::Function => FunctionNode::from_napi(env, val),
//     NodeType::Global => GlobalNode::from_napi(env, val),
//     NodeType::Goto => GotoNode::from_napi(env, val),
//     NodeType::Identifier => IdentifierNode::from_napi(env, val),
//     NodeType::If => IfNode::from_napi(env, val),
//     NodeType::Include => IncludeNode::from_napi(env, val),
//     NodeType::InstanceOf => InstanceOfNode::from_napi(env, val),
//     NodeType::Interface => InterfaceNode::from_napi(env, val),
//     NodeType::Label => LabelNode::from_napi(env, val),
//     NodeType::List => ListNode::from_napi(env, val),
//     NodeType::Magic => MagicNode::from_napi(env, val),
//     NodeType::Match => MatchNode::from_napi(env, val),
//     NodeType::MatchArm => MatchArmNode::from_napi(env, val),
//     NodeType::Method => MethodNode::from_napi(env, val),
//     NodeType::Namespace => NamespaceNode::from_napi(env, val),
//     NodeType::New => NewNode::from_napi(env, val),
//     NodeType::Number => NumberNode::from_napi(env, val),
//     NodeType::ObjectAccess => ObjectAccessNode::from_napi(env, val),
//     NodeType::Parameter => ParameterNode::from_napi(env, val),
//     NodeType::Parent => ParentNode::from_napi(env, val),
//     NodeType::Parenthesis => ParenthesisNode::from_napi(env, val),
//     NodeType::Post => PostNode::from_napi(env, val),
//     NodeType::Pre => PreNode::from_napi(env, val),
//     NodeType::Print => PrintNode::from_napi(env, val),
//     NodeType::Program => ProgramNode::from_napi(env, val),
//     NodeType::Property => PropertyNode::from_napi(env, val),
//     NodeType::PropertyItem => PropertyItemNode::from_napi(env, val),
//     NodeType::Return => ReturnNode::from_napi(env, val),
//     NodeType::Static => StaticNode::from_napi(env, val),
//     NodeType::StaticLookup => StaticLookupNode::from_napi(env, val),
//     NodeType::String => StringNode::from_napi(env, val),
//     NodeType::Switch => SwitchNode::from_napi(env, val),
//     NodeType::Ternary => TernaryNode::from_napi(env, val),
//     NodeType::Trait => TraitNode::from_napi(env, val),
//     NodeType::TraitUse => TraitUseNode::from_napi(env, val),
//     NodeType::TraitUseAlias => TraitUseAliasNode::from_napi(env, val),
//     NodeType::TraitUsePrecedence => TraitUsePrecedenceNode::from_napi(env, val),
//     NodeType::Throw => ThrowNode::from_napi(env, val),
//     NodeType::Try => TryNode::from_napi(env, val),
//     NodeType::Type => TypeNode::from_napi(env, val),
//     NodeType::Use => UseNode::from_napi(env, val),
//     NodeType::Variable => VariableNode::from_napi(env, val),
//     NodeType::While => WhileNode::from_napi(env, val),
//     NodeType::Yield => YieldNode::from_napi(env, val),
//     NodeType::YieldFrom => YieldFromNode::from_napi(env, val),
//   }
// }
// }

// #[cfg(test)]
// mod tests {
//   use super::NodeType;

//   #[test]
//   fn basic() {
//     [
//       NodeType::AnonymousFunction,
//       NodeType::Argument,
//       NodeType::Array,
//       NodeType::ArrayItem,
//       NodeType::ArrayLookup,
//       NodeType::ArrowFunction,
//       NodeType::Assignment,
//       NodeType::Bin,
//       NodeType::Block,
//       NodeType::Break,
//       NodeType::Call,
//       NodeType::Case,
//       NodeType::Cast,
//       NodeType::Catch,
//       NodeType::Class,
//       NodeType::Clone,
//       NodeType::CommentBlock,
//       NodeType::CommentDoc,
//       NodeType::CommentLine,
//       NodeType::Const,
//       NodeType::ConstProperty,
//       NodeType::Continue,
//       NodeType::Declare,
//       NodeType::DeclareArgument,
//       NodeType::DoWhile,
//       NodeType::Echo,
//       NodeType::Encapsed,
//       NodeType::EncapsedPart,
//       NodeType::Enum,
//       NodeType::EnumItem,
//       NodeType::Eval,
//       NodeType::Exit,
//       NodeType::For,
//       NodeType::Foreach,
//       NodeType::Function,
//       NodeType::Global,
//       NodeType::Goto,
//       NodeType::Identifier,
//       NodeType::If,
//       NodeType::Include,
//       NodeType::InstanceOf,
//       NodeType::Interface,
//       NodeType::Label,
//       NodeType::List,
//       NodeType::Magic,
//       NodeType::Match,
//       NodeType::MatchArm,
//       NodeType::Method,
//       NodeType::Namespace,
//       NodeType::New,
//       NodeType::Number,
//       NodeType::ObjectAccess,
//       NodeType::Parameter,
//       NodeType::Parent,
//       NodeType::Parenthesis,
//       NodeType::Post,
//       NodeType::Pre,
//       NodeType::Print,
//       NodeType::Program,
//       NodeType::Property,
//       NodeType::PropertyItem,
//       NodeType::Return,
//       NodeType::Static,
//       NodeType::StaticLookup,
//       NodeType::String,
//       NodeType::Switch,
//       NodeType::Ternary,
//       NodeType::Trait,
//       NodeType::TraitUse,
//       NodeType::TraitUseAlias,
//       NodeType::TraitUsePrecedence,
//       NodeType::Throw,
//       NodeType::Try,
//       NodeType::Type,
//       NodeType::Use,
//       NodeType::Variable,
//       NodeType::While,
//       NodeType::Yield,
//       NodeType::YieldFrom,
//     ]
//       .iter()
//       .for_each(|i| {
//         let s = i.to_string();
//         let newi = NodeType::from_string(&s.to_string());
//         assert_eq!(*i, newi.unwrap());
//       });
//   }
// }

pub trait NodeTrait: Debug + DynClone + Any {
  fn get_type(&self) -> NodeType;
  fn as_any(self: Box<Self>) -> Box<dyn Any>;

  fn add_leading_comments(&mut self, comments: Node);
  fn add_trailing_comments(&mut self, comments: Node);
  fn get_leading_comments(&self) -> &Nodes;
  fn get_trailing_comments(&self) -> &Nodes;

  unsafe fn to_napi(&self, env: napi::sys::napi_env) -> napi::Result<napi::sys::napi_value>;
  unsafe fn from_napi(env: napi::sys::napi_env, val: napi::sys::napi_value) -> Box<Self>
    where Self: Sized;
}
dyn_clone::clone_trait_object!(NodeTrait);

pub type Node = Box<dyn NodeTrait>;
pub type Nodes = Vec<Node>;

pub trait NodeTraitCast {
  fn cast<T: 'static>(self) -> Option<Box<T>> where T: NodeTrait;
}

impl NodeTraitCast for Node {
  fn cast<T: 'static>(self) -> Option<Box<T>> where T: NodeTrait {
    if let Ok(t) = self.as_any().downcast::<T>() {
      return Some(t);
    }
    None
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
    let napi_env = napi::Env::from_raw(env);
    let val = JsUnknown::from_napi_value(env, napi_val)?;

    if AnonymousFunctionNode::instance_of(napi_env, &val)? {
      return Ok(AnonymousFunctionNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ArgumentNode::instance_of(napi_env, &val)? {
      return Ok(ArgumentNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ArrayNode::instance_of(napi_env, &val)? {
      return Ok(ArrayNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ArrayItemNode::instance_of(napi_env, &val)? {
      return Ok(ArrayItemNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ArrayLookupNode::instance_of(napi_env, &val)? {
      return Ok(ArrayLookupNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ArrowFunctionNode::instance_of(napi_env, &val)? {
      return Ok(ArrowFunctionNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if AssignmentNode::instance_of(napi_env, &val)? {
      return Ok(AssignmentNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if BinNode::instance_of(napi_env, &val)? {
      return Ok(BinNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if BlockNode::instance_of(napi_env, &val)? {
      return Ok(BlockNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if BreakNode::instance_of(napi_env, &val)? {
      return Ok(BreakNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if CallNode::instance_of(napi_env, &val)? {
      return Ok(CallNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if CaseNode::instance_of(napi_env, &val)? {
      return Ok(CaseNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if CastNode::instance_of(napi_env, &val)? {
      return Ok(CastNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if CatchNode::instance_of(napi_env, &val)? {
      return Ok(CatchNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ClassNode::instance_of(napi_env, &val)? {
      return Ok(ClassNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if CloneNode::instance_of(napi_env, &val)? {
      return Ok(CloneNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if CommentBlockNode::instance_of(napi_env, &val)? {
      return Ok(CommentBlockNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if CommentDocNode::instance_of(napi_env, &val)? {
      return Ok(CommentDocNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if CommentLineNode::instance_of(napi_env, &val)? {
      return Ok(CommentLineNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ConstNode::instance_of(napi_env, &val)? {
      return Ok(ConstNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ConstPropertyNode::instance_of(napi_env, &val)? {
      return Ok(ConstPropertyNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ContinueNode::instance_of(napi_env, &val)? {
      return Ok(ContinueNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if DeclareNode::instance_of(napi_env, &val)? {
      return Ok(DeclareNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if DeclareArgumentNode::instance_of(napi_env, &val)? {
      return Ok(DeclareArgumentNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if DoWhileNode::instance_of(napi_env, &val)? {
      return Ok(DoWhileNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if EchoNode::instance_of(napi_env, &val)? {
      return Ok(EchoNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if EncapsedNode::instance_of(napi_env, &val)? {
      return Ok(EncapsedNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if EncapsedPartNode::instance_of(napi_env, &val)? {
      return Ok(EncapsedPartNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if EnumNode::instance_of(napi_env, &val)? {
      return Ok(EnumNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if EnumItemNode::instance_of(napi_env, &val)? {
      return Ok(EnumItemNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if EvalNode::instance_of(napi_env, &val)? {
      return Ok(EvalNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ExitNode::instance_of(napi_env, &val)? {
      return Ok(ExitNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ForNode::instance_of(napi_env, &val)? {
      return Ok(ForNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ForeachNode::instance_of(napi_env, &val)? {
      return Ok(ForeachNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if FunctionNode::instance_of(napi_env, &val)? {
      return Ok(FunctionNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if GlobalNode::instance_of(napi_env, &val)? {
      return Ok(GlobalNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if GotoNode::instance_of(napi_env, &val)? {
      return Ok(GotoNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if IdentifierNode::instance_of(napi_env, &val)? {
      return Ok(IdentifierNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if IfNode::instance_of(napi_env, &val)? {
      return Ok(IfNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if IncludeNode::instance_of(napi_env, &val)? {
      return Ok(IncludeNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if InstanceOfNode::instance_of(napi_env, &val)? {
      return Ok(InstanceOfNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if InterfaceNode::instance_of(napi_env, &val)? {
      return Ok(InterfaceNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if LabelNode::instance_of(napi_env, &val)? {
      return Ok(LabelNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ListNode::instance_of(napi_env, &val)? {
      return Ok(ListNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if MagicNode::instance_of(napi_env, &val)? {
      return Ok(MagicNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if MatchNode::instance_of(napi_env, &val)? {
      return Ok(MatchNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if MatchArmNode::instance_of(napi_env, &val)? {
      return Ok(MatchArmNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if MethodNode::instance_of(napi_env, &val)? {
      return Ok(MethodNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if NamespaceNode::instance_of(napi_env, &val)? {
      return Ok(NamespaceNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if NewNode::instance_of(napi_env, &val)? {
      return Ok(NewNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if NumberNode::instance_of(napi_env, &val)? {
      return Ok(NumberNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ObjectAccessNode::instance_of(napi_env, &val)? {
      return Ok(ObjectAccessNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ParameterNode::instance_of(napi_env, &val)? {
      return Ok(ParameterNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ParentNode::instance_of(napi_env, &val)? {
      return Ok(ParentNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ParenthesisNode::instance_of(napi_env, &val)? {
      return Ok(ParenthesisNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if PostNode::instance_of(napi_env, &val)? {
      return Ok(PostNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if PreNode::instance_of(napi_env, &val)? {
      return Ok(PreNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if PrintNode::instance_of(napi_env, &val)? {
      return Ok(PrintNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ProgramNode::instance_of(napi_env, &val)? {
      return Ok(ProgramNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if PropertyNode::instance_of(napi_env, &val)? {
      return Ok(PropertyNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if PropertyItemNode::instance_of(napi_env, &val)? {
      return Ok(PropertyItemNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ReturnNode::instance_of(napi_env, &val)? {
      return Ok(ReturnNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if StaticNode::instance_of(napi_env, &val)? {
      return Ok(StaticNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if StaticLookupNode::instance_of(napi_env, &val)? {
      return Ok(StaticLookupNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if StringNode::instance_of(napi_env, &val)? {
      return Ok(StringNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if SwitchNode::instance_of(napi_env, &val)? {
      return Ok(SwitchNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if TernaryNode::instance_of(napi_env, &val)? {
      return Ok(TernaryNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if TraitNode::instance_of(napi_env, &val)? {
      return Ok(TraitNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if TraitUseNode::instance_of(napi_env, &val)? {
      return Ok(TraitUseNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if TraitUseAliasNode::instance_of(napi_env, &val)? {
      return Ok(TraitUseAliasNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if TraitUsePrecedenceNode::instance_of(napi_env, &val)? {
      return Ok(TraitUsePrecedenceNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if ThrowNode::instance_of(napi_env, &val)? {
      return Ok(ThrowNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if TryNode::instance_of(napi_env, &val)? {
      return Ok(TryNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if TypeNode::instance_of(napi_env, &val)? {
      return Ok(TypeNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if UseNode::instance_of(napi_env, &val)? {
      return Ok(UseNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if VariableNode::instance_of(napi_env, &val)? {
      return Ok(VariableNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if WhileNode::instance_of(napi_env, &val)? {
      return Ok(WhileNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if YieldNode::instance_of(napi_env, &val)? {
      return Ok(YieldNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    } else if YieldFromNode::instance_of(napi_env, &val)? {
      return Ok(YieldFromNode::from_napi(env, napi_val) as Box<dyn NodeTrait>);
    }
    Err(napi::Error::new(napi::Status::InvalidArg, format!("Invalid node type").as_str()))
  }
}

#[napi]
#[derive(Debug, PartialEq)]
pub enum BodyType {
  Basic,
  Short,
  Empty,
}
