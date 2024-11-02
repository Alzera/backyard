use std::{ any::Any, fmt::Debug };

use dyn_clone::DynClone;
use napi::{ Env, JsObject };

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
  Identifier,
  Magic,
  Class,
  Trait,
  Interface,
  Function,
  Method,
  ConstProperty,
  Const,
  Property,
  PropertyItem,
  AnonymousFunction,
  ArrowFunction,
  Parameter,
  Variable,
  Block,
  Parenthesis,
  Cast,
  Type,
  Assignment,
  List,
  Bin,
  Number,
  String,
  Encapsed,
  EncapsedPart,
  Array,
  Call,
  Argument,
  Declare,
  DeclareArgument,
  Include,
  Namespace,
  Use,
  TraitUse,
  TraitUseAlias,
  TraitUsePrecedence,
  ObjectAccess,
  Global,
  Post,
  Pre,
  Ternary,
  If,
  Switch,
  Case,
  Break,
  Return,
  Continue,
  Print,
  Echo,
  Yield,
  YieldFrom,
  Foreach,
  While,
  DoWhile,
  Try,
  Catch,
  For,
  Label,
  Goto,
  New,
  Throw,
  Eval,
  Exit,
  InstanceOf,
  Match,
  MatchArm,
  StaticLookup,
  ArrayLookup,
}

pub trait NodeTrait: Debug + DynClone + Any {
  fn get_type(&self) -> NodeType;
  fn to_object(&self, env: Env) -> JsObject;
  fn as_any(self: Box<Self>) -> Box<dyn Any>;
}
dyn_clone::clone_trait_object!(NodeTrait);

pub trait NodeTraitCast {
  fn cast<T: 'static>(self) -> Result<Box<T>, Box<dyn Any>>;
}

impl NodeTraitCast for Box<dyn NodeTrait> {
  fn cast<T: 'static>(self) -> Result<Box<T>, Box<dyn Any>> {
    self.as_any().downcast::<T>()
  }
}

// fn cast<T: 'static>(le: Box<dyn NodeTrait>) -> Option<Box<T>> {
//   if let Ok(cast) = le.as_any().downcast::<T>() {
//     Some(cast);
//   }
//   None
// }

// fn get_type<T>(_: &T) -> &'static str {
//   type_name::<T>()
// }
// trait ToJsValue {
//   fn convert(&self, env: Env) -> Result<JsUnknown>;
// }
// impl ToJsValue for String {
//   fn convert(&self, env: Env) -> Result<JsUnknown> {
//     let s = env.create_string(self);
//     if let Ok(s) = s {
//       return Ok(s.into_unknown());
//     }
//     Err(Error::new(Status::Unknown, "Failed to convert rust type `String` into napi value"))
//   }
// }
// impl ToJsValue for bool {
//   fn convert(&self, env: Env) -> Result<JsUnknown> {
//     // let b = JsBoolean::
//   }
// }
// impl ToJsValue for Node {
//   fn convert(&self, env: Env) -> Result<JsUnknown> {
//     Ok(self.to_object(env).into_unknown())
//   }
// }
// impl ToJsValue for Vec<Node> {
//   fn convert(&self, env: Env) -> Result<JsUnknown> {
//     let mut arr = env.create_array(self.len() as u32)?;
//     for (i, value) in self.iter().enumerate() {
//       arr.set(i as u32, value.to_object(env));
//     }
//     Ok(arr.into_unknown())
//   }
// }
// macro_rules! type_based_macro {
//   ($value:expr; $ftype:ty) => {
//       <$ftype as TypeSpecificFunctionality>::execute();
//   };
// }
// #[macro_export]
// macro_rules! impl_node_trait {
//   ($struct_name:ident, $node_type:expr, { $($field:ident: $ftype:ty),* $(,)? }) => {
//     pub struct $struct_name {
//       $(pub $field: $ftype),*
//     }

//     impl NodeTrait for $struct_name {
//       fn get_type(&self) -> NodeType {
//         $node_type
//       }

//       fn to_object(&self, env: Env) -> JsObject {
//         let mut obj = env.create_object().unwrap();

//         obj.set("type", stringify!($node_type).to_lowercase());
//         $(obj.set(stringify!($field), &self.$field);)*

//         obj
//       }
//     }
//   };
// }

pub type Node = Box<dyn NodeTrait>;
pub type Nodes = Vec<Node>;

#[derive(Debug, Clone)]
pub struct ArrayLookupNode {
  pub target: Node,
  pub on: Node,
}

impl NodeTrait for ArrayLookupNode {
  fn get_type(&self) -> NodeType {
    NodeType::ArrayLookup
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "array_lookup");
    let _ = obj.set("target", self.target.to_object(env));
    let _ = obj.set("on", self.on.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct StaticLookupNode {
  pub target: Node,
  pub on: Node,
}

impl NodeTrait for StaticLookupNode {
  fn get_type(&self) -> NodeType {
    NodeType::StaticLookup
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "static_lookup");
    let _ = obj.set("target", self.target.to_object(env));
    let _ = obj.set("on", self.on.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ArrayNode {
  pub is_ellipsis: bool,
  pub values: Nodes,
}

impl NodeTrait for ArrayNode {
  fn get_type(&self) -> NodeType {
    NodeType::Array
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "array");
    let _ = obj.set("is_ellipsis", self.is_ellipsis);
    let _ = obj.set(
      "values",
      self.values
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    obj
  }
}

#[derive(Debug, Clone)]
pub struct IdentifierNode {
  pub name: String,
}

impl NodeTrait for IdentifierNode {
  fn get_type(&self) -> NodeType {
    NodeType::Identifier
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "identifier");
    let _ = obj.set("name", self.name.to_owned());
    obj
  }
}

#[derive(Debug, Clone)]
pub struct AssignmentNode {
  pub left: Node,
  pub operator: String,
  pub right: Node,
}

impl NodeTrait for AssignmentNode {
  fn get_type(&self) -> NodeType {
    NodeType::Assignment
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "assignment");
    let _ = obj.set("left", self.left.to_object(env));
    let _ = obj.set("operator", self.operator.to_owned());
    let _ = obj.set("right", self.right.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct BinNode {
  pub left: Node,
  pub operator: String,
  pub right: Node,
}

impl NodeTrait for BinNode {
  fn get_type(&self) -> NodeType {
    NodeType::Bin
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }
  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "bin");
    let _ = obj.set("left", self.left.to_object(env));
    let _ = obj.set("operator", self.operator.to_owned());
    let _ = obj.set("right", self.right.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct BlockNode {
  pub statements: Nodes,
}

impl NodeTrait for BlockNode {
  fn get_type(&self) -> NodeType {
    NodeType::Block
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "block");
    let _ = obj.set(
      "statements",
      self.statements
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    obj
  }
}

#[derive(Debug, Clone)]
pub struct CallNode {
  pub name: Node,
  pub arguments: Nodes,
}

impl NodeTrait for CallNode {
  fn get_type(&self) -> NodeType {
    NodeType::Call
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "call");
    let _ = obj.set("name", self.name.to_object(env));
    let _ = obj.set(
      "arguments",
      self.arguments
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ArgumentNode {
  pub name: Option<Node>,
  pub value: Node,
}

impl NodeTrait for ArgumentNode {
  fn get_type(&self) -> NodeType {
    NodeType::Argument
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "argument");
    let _ = obj.set("name", match &self.name {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set("value", self.value.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ClassNode {
  pub modifier: String,
  pub name: Node,
  pub extends: Option<Node>,
  pub implements: Nodes,
  pub body: Node,
}

impl NodeTrait for ClassNode {
  fn get_type(&self) -> NodeType {
    NodeType::Class
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "class");
    let _ = obj.set("name", self.name.to_object(env));
    let _ = obj.set("modifier", self.modifier.to_owned());
    let _ = obj.set("extends", match &self.extends {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set(
      "implements",
      self.implements
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set("body", self.body.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ConstNode {
  pub consts: Nodes,
}

impl NodeTrait for ConstNode {
  fn get_type(&self) -> NodeType {
    NodeType::Const
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "const");
    let _ = obj.set(
      "consts",
      self.consts
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ConstPropertyNode {
  pub visibility: String,
  pub consts: Nodes,
}

impl NodeTrait for ConstPropertyNode {
  fn get_type(&self) -> NodeType {
    NodeType::ConstProperty
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "const_property");
    let _ = obj.set("visibility", self.visibility.to_string());
    let _ = obj.set(
      "consts",
      self.consts
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    obj
  }
}

#[derive(Debug, Clone)]
pub struct DeclareNode {
  pub arguments: Nodes,
  pub body: Option<Node>,
  pub body_type: BodyType,
}

impl NodeTrait for DeclareNode {
  fn get_type(&self) -> NodeType {
    NodeType::Declare
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "declare");
    let _ = obj.set(
      "arguments",
      self.arguments
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set("body", match &self.body {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set("body_type", self.body_type.to_object());
    obj
  }
}

#[derive(Debug, Clone)]
pub struct DeclareArgumentNode {
  pub name: Node,
  pub value: Node,
}

impl NodeTrait for DeclareArgumentNode {
  fn get_type(&self) -> NodeType {
    NodeType::DeclareArgument
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "declare_argument");
    let _ = obj.set("name", self.name.to_object(env));
    let _ = obj.set("value", self.value.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct DoWhileNode {
  pub condition: Node,
  pub body: Node,
}

impl NodeTrait for DoWhileNode {
  fn get_type(&self) -> NodeType {
    NodeType::DoWhile
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "do_while");
    let _ = obj.set("condition", self.condition.to_object(env));
    let _ = obj.set("body", self.body.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct EvalNode {
  pub argument: Node,
}

impl NodeTrait for EvalNode {
  fn get_type(&self) -> NodeType {
    NodeType::Eval
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "eval");
    let _ = obj.set("argument", self.argument.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ExitNode {
  pub argument: Node,
}

impl NodeTrait for ExitNode {
  fn get_type(&self) -> NodeType {
    NodeType::Exit
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "exit");
    let _ = obj.set("argument", self.argument.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ForNode {
  pub inits: Nodes,
  pub tests: Nodes,
  pub increments: Nodes,
  pub body: Option<Node>,
  pub body_type: BodyType,
}

impl NodeTrait for ForNode {
  fn get_type(&self) -> NodeType {
    NodeType::For
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "for");
    let _ = obj.set(
      "inits",
      self.inits
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set(
      "tests",
      self.tests
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set(
      "increments",
      self.increments
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set("body", match &self.body {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set("body_type", self.body_type.to_object());
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ForeachNode {
  pub source: Node,
  pub key: Option<Node>,
  pub value: Node,
  pub body: Node,
  pub is_short: bool,
}

impl NodeTrait for ForeachNode {
  fn get_type(&self) -> NodeType {
    NodeType::Foreach
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "foreach");
    let _ = obj.set("source", self.source.to_object(env));
    let _ = obj.set("key", match &self.key {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set("value", self.value.to_object(env));
    let _ = obj.set("body", self.body.to_object(env));
    let _ = obj.set("is_short", self.is_short);
    obj
  }
}

#[derive(Debug, Clone)]
pub struct FunctionNode {
  pub is_ref: bool,
  pub name: Node,
  pub arguments: Nodes,
  pub return_type: Option<Node>,
  pub body: Option<Node>,
}

impl NodeTrait for FunctionNode {
  fn get_type(&self) -> NodeType {
    NodeType::Function
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "function");
    let _ = obj.set("is_ref", self.is_ref);
    let _ = obj.set("name", self.name.to_object(env));
    let _ = obj.set(
      "arguments",
      self.arguments
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set("return_type", match &self.return_type {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set("body", match &self.body {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ArrowFunctionNode {
  pub is_ref: bool,
  pub arguments: Nodes,
  pub return_type: Option<Node>,
  pub body: Node,
}

impl NodeTrait for ArrowFunctionNode {
  fn get_type(&self) -> NodeType {
    NodeType::ArrowFunction
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "arrow_function");
    let _ = obj.set("is_ref", self.is_ref);
    let _ = obj.set(
      "arguments",
      self.arguments
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set("return_type", match &self.return_type {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set("body", self.body.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct AnonymousFunctionNode {
  pub is_ref: bool,
  pub arguments: Nodes,
  pub uses: Nodes,
  pub return_type: Option<Node>,
  pub body: Node,
}

impl NodeTrait for AnonymousFunctionNode {
  fn get_type(&self) -> NodeType {
    NodeType::AnonymousFunction
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "anonymous_function");
    let _ = obj.set("is_ref", self.is_ref);
    let _ = obj.set(
      "arguments",
      self.arguments
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set(
      "uses",
      self.uses
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set("return_type", match &self.return_type {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set("body", self.body.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ParameterNode {
  pub variable_type: Option<Node>,
  pub is_ref: bool,
  pub is_ellipsis: bool,
  pub name: Node,
  pub value: Option<Node>,
}

impl NodeTrait for ParameterNode {
  fn get_type(&self) -> NodeType {
    NodeType::Parameter
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "parameter");
    let _ = obj.set("variable_type", match &self.variable_type {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set("is_ref", self.is_ref);
    let _ = obj.set("is_ellipsis", self.is_ellipsis);
    let _ = obj.set("name", self.name.to_object(env));
    let _ = obj.set("value", match &self.value {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    obj
  }
}

#[derive(Debug, Clone)]
pub struct GlobalNode {
  pub variable: Node,
}

impl NodeTrait for GlobalNode {
  fn get_type(&self) -> NodeType {
    NodeType::Global
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "global");
    let _ = obj.set("variable", self.variable.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct GotoNode {
  pub label: Node,
}

impl NodeTrait for GotoNode {
  fn get_type(&self) -> NodeType {
    NodeType::Goto
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "goto");
    let _ = obj.set("label", self.label.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct IfNode {
  pub condition: Node,
  pub valid: Node,
  pub invalid: Option<Node>,
  pub is_short: bool,
}

impl NodeTrait for IfNode {
  fn get_type(&self) -> NodeType {
    NodeType::If
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "if");
    let _ = obj.set("condition", self.condition.to_object(env));
    let _ = obj.set("valid", self.valid.to_object(env));
    let _ = obj.set("invalid", match &self.invalid {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set("is_short", self.is_short);
    obj
  }
}

#[derive(Debug, Clone)]
pub struct IncludeNode {
  pub is_require: bool,
  pub is_once: bool,
  pub argument: Node,
}

impl NodeTrait for IncludeNode {
  fn get_type(&self) -> NodeType {
    NodeType::Include
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "include");
    let _ = obj.set("is_require", self.is_require);
    let _ = obj.set("is_once", self.is_once);
    let _ = obj.set("argument", self.argument.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct InstanceOfNode {
  pub left: Node,
  pub right: Node,
}

impl NodeTrait for InstanceOfNode {
  fn get_type(&self) -> NodeType {
    NodeType::InstanceOf
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "instanceof");
    let _ = obj.set("left", self.left.to_object(env));
    let _ = obj.set("right", self.right.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct InterfaceNode {
  pub name: Node,
  pub implements: Nodes,
  pub body: Node,
}

impl NodeTrait for InterfaceNode {
  fn get_type(&self) -> NodeType {
    NodeType::Interface
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "interface");
    let _ = obj.set("name", self.name.to_object(env));
    let _ = obj.set(
      "implements",
      self.implements
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set("body", self.body.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct LabelNode {
  pub label: Node,
}

impl NodeTrait for LabelNode {
  fn get_type(&self) -> NodeType {
    NodeType::Label
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "label");
    let _ = obj.set("label", self.label.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ListNode {
  pub values: Nodes,
}

impl NodeTrait for ListNode {
  fn get_type(&self) -> NodeType {
    NodeType::List
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "list");
    let _ = obj.set(
      "values",
      self.values
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    obj
  }
}

#[derive(Debug, Clone)]
pub struct MagicNode {
  pub name: String,
}

impl NodeTrait for MagicNode {
  fn get_type(&self) -> NodeType {
    NodeType::Magic
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "magic");
    let _ = obj.set("name", self.name.to_string());
    obj
  }
}

#[derive(Debug, Clone)]
pub struct MatchNode {
  pub condition: Node,
  pub arms: Nodes,
}

impl NodeTrait for MatchNode {
  fn get_type(&self) -> NodeType {
    NodeType::Match
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "match");
    let _ = obj.set("condition", self.condition.to_object(env));
    let _ = obj.set(
      "arms",
      self.arms
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    obj
  }
}

#[derive(Debug, Clone)]
pub struct MatchArmNode {
  pub conditions: Nodes,
  pub body: Node,
}

impl NodeTrait for MatchArmNode {
  fn get_type(&self) -> NodeType {
    NodeType::MatchArm
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "match_arm");
    let _ = obj.set(
      "conditions",
      self.conditions
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set("body", self.body.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct MethodNode {
  pub visibility: String,
  pub modifier: String,
  pub is_static: bool,
  pub function: Node,
}

impl NodeTrait for MethodNode {
  fn get_type(&self) -> NodeType {
    NodeType::Method
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "method");
    let _ = obj.set("visibility", self.visibility.to_string());
    let _ = obj.set("modifier", self.modifier.to_string());
    let _ = obj.set("is_static", self.is_static);
    let _ = obj.set("function", self.function.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct NamespaceNode {
  pub name: Nodes,
  pub body: Node,
  pub is_bracket: bool,
}

impl NodeTrait for NamespaceNode {
  fn get_type(&self) -> NodeType {
    NodeType::Namespace
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "namespace");
    let _ = obj.set(
      "name",
      self.name
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set("body", self.body.to_object(env));
    let _ = obj.set("is_bracket", self.is_bracket);
    obj
  }
}

#[derive(Debug, Clone)]
pub struct NumberNode {
  pub value: String,
}

impl NodeTrait for NumberNode {
  fn get_type(&self) -> NodeType {
    NodeType::Number
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "number");
    let _ = obj.set("value", self.value.to_string());
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ObjectAccessNode {
  pub object: Node,
  pub property: Node,
}

impl NodeTrait for ObjectAccessNode {
  fn get_type(&self) -> NodeType {
    NodeType::ObjectAccess
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "objectaccess");
    let _ = obj.set("object", self.object.to_object(env));
    let _ = obj.set("property", self.property.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ParenthesisNode {
  pub statement: Node,
}

impl NodeTrait for ParenthesisNode {
  fn get_type(&self) -> NodeType {
    NodeType::Parenthesis
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "parenthesis");
    let _ = obj.set("statement", self.statement.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct PostNode {
  pub variable: Node,
  pub operator: String,
}

impl NodeTrait for PostNode {
  fn get_type(&self) -> NodeType {
    NodeType::Post
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "post");
    let _ = obj.set("variable", self.variable.to_object(env));
    let _ = obj.set("operator", self.operator.to_string());
    obj
  }
}

#[derive(Debug, Clone)]
pub struct PreNode {
  pub variable: Node,
  pub operator: String,
}

impl NodeTrait for PreNode {
  fn get_type(&self) -> NodeType {
    NodeType::Pre
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "pre");
    let _ = obj.set("variable", self.variable.to_object(env));
    let _ = obj.set("operator", self.operator.to_string());
    obj
  }
}

#[derive(Debug, Clone)]
pub struct PropertyItemNode {
  pub name: Node,
  pub variable_type: Option<Node>,
  pub value: Option<Node>,
}

impl NodeTrait for PropertyItemNode {
  fn get_type(&self) -> NodeType {
    NodeType::PropertyItem
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "property_item");
    let _ = obj.set("name", self.name.to_object(env));
    let _ = obj.set("variable_type", match &self.variable_type {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set("value", match &self.value {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    obj
  }
}

#[derive(Debug, Clone)]
pub struct PropertyNode {
  pub visibility: String,
  pub is_static: bool,
  pub items: Nodes,
}

impl NodeTrait for PropertyNode {
  fn get_type(&self) -> NodeType {
    NodeType::Property
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "property");
    let _ = obj.set("visibility", self.visibility.to_string());
    let _ = obj.set("is_static", self.is_static);
    let _ = obj.set(
      "items",
      self.items
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    obj
  }
}

#[derive(Debug, Clone)]
pub struct StringNode {
  pub value: String,
}

impl NodeTrait for StringNode {
  fn get_type(&self) -> NodeType {
    NodeType::String
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "string");
    let _ = obj.set("value", self.value.to_string());
    obj
  }
}

#[derive(Debug, Clone)]
pub struct EncapsedNode {
  pub values: Nodes,
}

impl NodeTrait for EncapsedNode {
  fn get_type(&self) -> NodeType {
    NodeType::Encapsed
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "encapsed");
    let _ = obj.set(
      "values",
      self.values
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    obj
  }
}

#[derive(Debug, Clone)]
pub struct EncapsedPartNode {
  pub is_advanced: bool,
  pub value: Node,
}

impl NodeTrait for EncapsedPartNode {
  fn get_type(&self) -> NodeType {
    NodeType::EncapsedPart
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "encapsedpart");
    let _ = obj.set("is_advanced", self.is_advanced);
    let _ = obj.set("value", self.value.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct TernaryNode {
  pub condition: Node,
  pub valid: Node,
  pub invalid: Node,
}

impl NodeTrait for TernaryNode {
  fn get_type(&self) -> NodeType {
    NodeType::Ternary
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "ternary");
    let _ = obj.set("condition", self.condition.to_object(env));
    let _ = obj.set("valid", self.valid.to_object(env));
    let _ = obj.set("invalid", self.invalid.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct TraitNode {
  pub name: Node,
  pub body: Node,
}

impl NodeTrait for TraitNode {
  fn get_type(&self) -> NodeType {
    NodeType::Trait
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "trait");
    let _ = obj.set("name", self.name.to_object(env));
    let _ = obj.set("body", self.body.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct TraitUseNode {
  pub traits: Nodes,
  pub adaptations: Option<Node>,
}

impl NodeTrait for TraitUseNode {
  fn get_type(&self) -> NodeType {
    NodeType::TraitUse
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "trait_use");
    let _ = obj.set(
      "traits",
      self.traits
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set("adaptations", match &self.adaptations {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    obj
  }
}

#[derive(Debug, Clone)]
pub struct TraitUseAliasNode {
  pub trait_name: Option<Node>,
  pub method: Node,
  pub alias: Node,
  pub visibility: String,
}

impl NodeTrait for TraitUseAliasNode {
  fn get_type(&self) -> NodeType {
    NodeType::TraitUseAlias
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "trait_use_alias");
    let _ = obj.set("trait_name", match &self.trait_name {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set("method", self.method.to_object(env));
    let _ = obj.set("alias", self.alias.to_object(env));
    let _ = obj.set("visibility", self.visibility.to_string());
    obj
  }
}

#[derive(Debug, Clone)]
pub struct TraitUsePrecedenceNode {
  pub trait_name: Option<Node>,
  pub method: Node,
  pub instead: Node,
}

impl NodeTrait for TraitUsePrecedenceNode {
  fn get_type(&self) -> NodeType {
    NodeType::TraitUsePrecedence
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "trait_use_precedence");
    let _ = obj.set("trait_name", match &self.trait_name {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set("method", self.method.to_object(env));
    let _ = obj.set("instead", self.instead.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct TryNode {
  pub body: Node,
  pub catches: Nodes,
  pub finally: Option<Node>,
}

impl NodeTrait for TryNode {
  fn get_type(&self) -> NodeType {
    NodeType::Try
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "try");
    let _ = obj.set("body", self.body.to_object(env));
    let _ = obj.set(
      "catches",
      self.catches
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set("finally", match &self.finally {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    obj
  }
}

#[derive(Debug, Clone)]
pub struct CatchNode {
  pub types: Nodes,
  pub variable: Node,
  pub body: Node,
}

impl NodeTrait for CatchNode {
  fn get_type(&self) -> NodeType {
    NodeType::Catch
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "catch");
    let _ = obj.set(
      "types",
      self.types
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set("variable", self.variable.to_object(env));
    let _ = obj.set("body", self.body.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct TypeNode {
  pub is_nullable: bool,
  pub name: String,
}

impl NodeTrait for TypeNode {
  fn get_type(&self) -> NodeType {
    NodeType::Type
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "type");
    let _ = obj.set("is_nullable", self.is_nullable);
    let _ = obj.set("name", self.name.to_string());
    obj
  }
}

#[derive(Debug, Clone)]
pub struct UsesNode {
  pub modifier: String,
  pub name: Nodes,
  pub items: Nodes,
}

impl NodeTrait for UsesNode {
  fn get_type(&self) -> NodeType {
    NodeType::Use
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "uses");
    let _ = obj.set("modifier", self.modifier.to_string());
    let _ = obj.set(
      "name",
      self.name
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set(
      "items",
      self.items
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    obj
  }
}

#[derive(Debug, Clone)]
pub struct VariableNode {
  pub is_ref: bool,
  pub name: Node,
}

impl NodeTrait for VariableNode {
  fn get_type(&self) -> NodeType {
    NodeType::Variable
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "variable");
    let _ = obj.set("is_ref", self.is_ref);
    let _ = obj.set("name", self.name.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct WhileNode {
  pub condition: Node,
  pub body: Node,
  pub is_short: bool,
}

impl NodeTrait for WhileNode {
  fn get_type(&self) -> NodeType {
    NodeType::While
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "while");
    let _ = obj.set("condition", self.condition.to_object(env));
    let _ = obj.set("body", self.body.to_object(env));
    let _ = obj.set("is_short", self.is_short);
    obj
  }
}

#[derive(Debug, Clone)]
pub struct YieldFromNode {
  pub value: Node,
}

impl NodeTrait for YieldFromNode {
  fn get_type(&self) -> NodeType {
    NodeType::YieldFrom
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "yield_from");
    let _ = obj.set("value", self.value.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct YieldNode {
  pub key: Option<Node>,
  pub value: Node,
}

impl NodeTrait for YieldNode {
  fn get_type(&self) -> NodeType {
    NodeType::Yield
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "yield");
    let _ = obj.set("key", match &self.key {
      Some(x) => Some(x.to_object(env)),
      None => None,
    });
    let _ = obj.set("value", self.value.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct BreakNode {
  pub argument: Option<Node>,
}

impl NodeTrait for BreakNode {
  fn get_type(&self) -> NodeType {
    NodeType::Break
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "break");
    let _ = obj.set("argument", match &self.argument {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ContinueNode {
  pub argument: Option<Node>,
}

impl NodeTrait for ContinueNode {
  fn get_type(&self) -> NodeType {
    NodeType::Continue
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "continue");
    let _ = obj.set("argument", match &self.argument {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ReturnNode {
  pub argument: Option<Node>,
}

impl NodeTrait for ReturnNode {
  fn get_type(&self) -> NodeType {
    NodeType::Return
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "return");
    let _ = obj.set("argument", match &self.argument {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    obj
  }
}

#[derive(Debug, Clone)]
pub struct EchoNode {
  pub argument: Node,
}

impl NodeTrait for EchoNode {
  fn get_type(&self) -> NodeType {
    NodeType::Echo
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "echo");
    let _ = obj.set("argument", self.argument.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct NewNode {
  pub argument: Node,
}

impl NodeTrait for NewNode {
  fn get_type(&self) -> NodeType {
    NodeType::New
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "new");
    let _ = obj.set("argument", self.argument.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct ThrowNode {
  pub argument: Node,
}

impl NodeTrait for ThrowNode {
  fn get_type(&self) -> NodeType {
    NodeType::Throw
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "throw");
    let _ = obj.set("argument", self.argument.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct PrintNode {
  pub argument: Node,
}

impl NodeTrait for PrintNode {
  fn get_type(&self) -> NodeType {
    NodeType::Print
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "print");
    let _ = obj.set("argument", self.argument.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct SwitchNode {
  pub condition: Node,
  pub body: Node,
  pub is_short: bool,
}

impl NodeTrait for SwitchNode {
  fn get_type(&self) -> NodeType {
    NodeType::Switch
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "switch");
    let _ = obj.set("condition", self.condition.to_object(env));
    let _ = obj.set("body", self.body.to_object(env));
    let _ = obj.set("is_short", self.is_short);
    obj
  }
}

#[derive(Debug, Clone)]
pub struct CaseNode {
  pub condition: Option<Node>,
  pub body: Node,
}

impl NodeTrait for CaseNode {
  fn get_type(&self) -> NodeType {
    NodeType::Case
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "case");
    let _ = obj.set("condition", match &self.condition {
      Some(x) => Some(x.to_object(env)),
      _ => None,
    });
    let _ = obj.set("body", self.body.to_object(env));
    obj
  }
}

#[derive(Debug, Clone)]
pub struct UseNode {
  pub modifier: String,
  pub name: Nodes,
  pub items: Nodes,
}

impl NodeTrait for UseNode {
  fn get_type(&self) -> NodeType {
    NodeType::Use
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "use");
    let _ = obj.set(
      "name",
      self.name
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    let _ = obj.set("modifier", self.modifier.to_owned());
    let _ = obj.set(
      "items",
      self.items
        .iter()
        .map(|x| x.to_object(env))
        .collect::<Vec<JsObject>>()
    );
    obj
  }
}

#[derive(Debug, Clone)]
pub struct CastNode {
  pub target: Node,
  pub expression: Node,
}

impl NodeTrait for CastNode {
  fn get_type(&self) -> NodeType {
    NodeType::Cast
  }

  fn as_any(self: Box<Self>) -> Box<dyn Any> {
    self
  }

  fn to_object(&self, env: Env) -> JsObject {
    let mut obj = env.create_object().unwrap();
    let _ = obj.set("type", "cast");
    let _ = obj.set("target", self.target.to_object(env));
    let _ = obj.set("expression", self.expression.to_object(env));
    obj
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BodyType {
  Basic,
  Short,
  Empty,
}

impl BodyType {
  pub fn to_object(&self) -> String {
    (
      match self {
        BodyType::Basic => "basic",
        BodyType::Short => "short",
        BodyType::Empty => "empty",
      }
    ).to_string()
  }
}
