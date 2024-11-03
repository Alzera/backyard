use std::any::Any;

use napi::{ Env, JsObject };

use crate::parser::node::{ NodeTrait, NodeType, Node, Nodes };

#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::YieldFrom)]
pub struct YieldFromNode {
  pub value: Node,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl NodeTrait for YieldFromNode {
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
//     NodeType::YieldFrom
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   fn to_object(&self, env: Env) -> JsObject {
//     let mut obj = env.create_object().unwrap();
//     let _ = obj.set("type", "yield_from");
//     let _ = obj.set("value", self.value.to_object(env));
//     obj
//   }
// }

#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::Yield)]
pub struct YieldNode {
  pub key: Option<Node>,
  pub value: Node,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl NodeTrait for YieldNode {
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
//     NodeType::Yield
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   fn to_object(&self, env: Env) -> JsObject {
//     let mut obj = env.create_object().unwrap();
//     let _ = obj.set("type", "yield");
//     let _ = obj.set("key", match &self.key {
//       Some(x) => Some(x.to_object(env)),
//       None => None,
//     });
//     let _ = obj.set("value", self.value.to_object(env));
//     obj
//   }
// }
