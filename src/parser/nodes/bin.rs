use std::any::Any;

use napi::bindgen_prelude::ToNapiValue;

use crate::parser::node::{ NodeType, Node, Nodes };

#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::Bin)]
pub struct BinNode {
  pub left: Node,
  pub operator: String,
  pub right: Node,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl NodeTrait for BinNode {
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
//     NodeType::Bin
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }
//   fn to_object(&self, env: Env) -> JsObject {
//     let mut obj = env.create_object().unwrap();
//     let _ = obj.set("type", "bin");
//     let _ = obj.set("left", self.left.to_object(env));
//     let _ = obj.set("operator", self.operator.to_owned());
//     let _ = obj.set("right", self.right.to_object(env));
//     obj
//   }
// }
