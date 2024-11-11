use std::any::Any;

use napi::bindgen_prelude::ToNapiValue;
use napi::bindgen_prelude::FromNapiRef;

use crate::parser::node::{ NodeType, Node, Nodes };
#[napi]
#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::PropertyItem)]
pub struct PropertyItemNode {
  pub name: Node,
  pub variable_type: Option<Node>,
  pub value: Option<Node>,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl NodeTrait for PropertyItemNode {
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
//     NodeType::PropertyItem
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   fn to_object(&self, env: Env) -> JsObject {
//     let mut obj = env.create_object().unwrap();
//     let _ = obj.set("type", "property_item");
//     let _ = obj.set("name", self.name.to_object(env));
//     let _ = obj.set("variable_type", match &self.variable_type {
//       Some(x) => Some(x.to_object(env)),
//       _ => None,
//     });
//     let _ = obj.set("value", match &self.value {
//       Some(x) => Some(x.to_object(env)),
//       _ => None,
//     });
//     obj
//   }
// }
#[napi]
#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::Property)]
pub struct PropertyNode {
  pub visibility: String,
  pub modifier: String,
  pub items: Nodes,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl NodeTrait for PropertyNode {
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
//     NodeType::Property
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   fn to_object(&self, env: Env) -> JsObject {
//     let mut obj = env.create_object().unwrap();
//     let _ = obj.set("type", "property");
//     let _ = obj.set("visibility", self.visibility.to_string());
//     let _ = obj.set("is_static", self.is_static);
//     let _ = obj.set(
//       "items",
//       self.items
//         .iter()
//         .map(|x| x.to_object(env))
//         .collect::<Vec<JsObject>>()
//     );
//     obj
//   }
// }
