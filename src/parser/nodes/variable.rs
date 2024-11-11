use std::any::Any;

use napi::bindgen_prelude::ToNapiValue;
use napi::bindgen_prelude::FromNapiRef;

use crate::parser::node::{ NodeType, Node, Nodes };
#[napi]
#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::Variable)]
pub struct VariableNode {
  pub is_ref: bool,
  pub name: Node,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl VariableNode {
//   pub fn new(is_ref: bool, name: Node) -> Box<Self> {
//     Box::new(Self {
//       is_ref,
//       name,
//       leading_comments: vec![],
//       trailing_comments: vec![],
//     })
//   }
// }

// impl ToNapiValue for VariableNode {
//   unsafe fn to_napi_value(
//     env: napi::sys::napi_env,
//     val: Self
//   ) -> napi::Result<napi::sys::napi_value> {
//     let unraw_env = napi::Env::from_raw(env);
//     let mut obj = unraw_env.create_object()?;
//     let _ = obj.set("type", NodeType::Variable.to_string());

//     let _ = obj.set("is_ref", val.is_ref);
//     let _ = obj.set("name", val.name);

//     if val.leading_comments.len() > 0 {
//       let _ = obj.set("leading_comments", val.leading_comments);
//     }
//     if val.trailing_comments.len() > 0 {
//       let _ = obj.set("trailing_comments", val.trailing_comments);
//     }

//     napi::bindgen_prelude::Object::to_napi_value(env, obj)
//   }
// }

// impl crate::parser::node::NodeTrait for VariableNode {
//   fn add_leading_comments(&mut self, comments: crate::parser::node::Node) {
//     self.leading_comments.push(comments);
//   }

//   fn add_trailing_comments(&mut self, comments: crate::parser::node::Node) {
//     self.trailing_comments.push(comments);
//   }

//   fn get_type(&self) -> crate::parser::node::NodeType {
//     NodeType::Variable
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   unsafe fn as_napi(&self, env: napi::sys::napi_env) -> napi::Result<napi::sys::napi_value> {
//     VariableNode::to_napi_value(env, self.clone())
//   }
// }

// impl NodeTrait for VariableNode {
//   fn add_leading_comments(&mut self, comments: crate::parser::node::Node) {
//     self.leading_comments.push(comments);
//   }

//   fn add_trailing_comments(&mut self, comments: crate::parser::node::Node) {
//     self.trailing_comments.push(comments);
//   }

//   fn add_inner_comments(&mut self, comments: crate::parser::node::Node) {
//     self.inner_comments.push(comments);
//   }

//   fn get_type(&self) -> NodeType {
//     NodeType::Variable
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   fn to_object(&self, env: Env) -> JsObject {
//     let mut obj = env.create_object().unwrap();
//     let _ = obj.set("type", "variable");
//     let _ = obj.set("is_ref", self.is_ref);
//     let _ = obj.set("name", self.name.to_object(env));
//     obj
//   }
// }
