use std::any::Any;

use napi::{ Env, JsObject };

use crate::parser::node::{ NodeTrait, NodeType, Nodes };

#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::Number)]
pub struct NumberNode {
  pub value: String,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl NodeTrait for NumberNode {
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
//     NodeType::Number
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   fn to_object(&self, env: Env) -> JsObject {
//     let mut obj = env.create_object().unwrap();
//     let _ = obj.set("type", "number");
//     let _ = obj.set("value", self.value.to_string());
//     obj
//   }
// }
