use std::{ any::Any, fmt::Debug };

use dyn_clone::DynClone;
use napi::{ Env, JsObject };

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
}

pub trait NodeTrait: Debug + DynClone + Any {
  fn get_type(&self) -> NodeType;
  fn to_object(&self, env: Env) -> JsObject;
  fn as_any(self: Box<Self>) -> Box<dyn Any>;

  fn add_leading_comments(&mut self, comments: Node);
  fn add_trailing_comments(&mut self, comments: Node);
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
//   fn convert<T>(&self, env: Env) -> Option<T>;
// }
// impl ToJsValue for String {
//   fn convert<JsString>(&self, env: Env) -> Option<JsString> {
//     let s = env.create_string(self);
//     if let Ok(s) = s {
//       return Some(s);
//     }
//     None
//   }
// }
// // impl ToJsValue for bool {
// //   fn convert<JsBoolean>(&self, env: Env) -> Option<JsBoolean> {
// //     b
// //   }
// // }
// impl ToJsValue for Node {
//   fn convert<JsObject>(&self, env: Env) -> Option<JsObject> {
//     Some(self.to_object(env))
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
// macro_rules! parse_node_value {
//   ("String", $value:expr) => { $value };
//   (bool, $value:expr) => { $value };
//   (Node, $value:expr) => {
//     $value.to_object(env)
//   };
//   (Nodes, $value:expr) => {
//     $value
//       .iter()
//       .map(|x| x.to_object(env))
//       .collect::<Vec<JsObject>>()
//   };
//   (BodyType, $value:expr) => {
//     $value.to_object(env)
//   };
// }

// #[macro_export]
// macro_rules! impl_node_trait {
//   ($struct_name:ident, $node_type:expr, { $($field:ident: $ftype:ty),* $(,)? }) => {
//     #[derive(Debug, Clone)]
//     pub struct $struct_name {
//       $(pub $field: $ftype,)*
//     }

//     impl NodeTrait for $struct_name {
//       fn get_type(&self) -> NodeType {
//         $node_type
//       }

//       fn as_any(self: Box<Self>) -> Box<dyn Any> {
//         self
//       }

//       fn to_object(&self, env: Env) -> JsObject {
//         let mut obj = env.create_object().unwrap();

//         obj.set("type", stringify!($node_type).to_lowercase());
//         $(obj.set(stringify!($field), parse_node_value!(stringify!($ftype), &self.$field));)*

//         obj
//       }
//     }
//   };
// }

// impl_node_trait!(TestNode, NodeType::Test, { test: String });

pub type Node = Box<dyn NodeTrait>;
pub type Nodes = Vec<Node>;

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

// macro_rules! parse_node_value {
//   ("string", $value:expr) => { $value };
//   ("bool", $value:expr) => { $value };
//   ("node", $value:expr) => {
//     $value
//         .iter()
//         .map(|x| x.to_object(env))
//         .collect::<Vec<JsObject>>()
//   };
// }

// macro_rules! test {
//   (
//     $struct_name:ident,
//     $struct_type:expr,
//     { $($field_name:ident | $field_type:ty | $render_type:block),* $(,)? }
//   ) => {
//     #[derive(Debug, Clone)]
//     pub struct $struct_name {
//       $(pub $field_name: $field_type,)*
//     }

//     impl NodeTrait for $struct_name {
//       fn get_type(&self) -> NodeType {
//         $struct_type
//       }

//       fn as_any(self: Box<Self>) -> Box<dyn Any> {
//         self
//       }

//       fn to_object(&self, env: Env) -> JsObject {
//         let mut obj = env.create_object().unwrap();

//         obj.set("type", stringify!($node_type).to_lowercase());
//         $(obj.set(stringify!($field), parse_node_value($render_type, &self.$field_name));)*

//         obj
//       }
//     }
//   };
// }

// test!(MyStruct, NodeType::Test, {
//   a | String | "string",
//   b | bool | "bool",
// });
