use std::any::Any;

use napi::bindgen_prelude::ToNapiValue;

use crate::parser::node::{ NodeType, Node, Nodes };

#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::While)]
pub struct WhileNode {
  pub condition: Node,
  pub body: Node,
  pub is_short: bool,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl NodeTrait for WhileNode {
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
//     NodeType::While
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   fn to_object(&self, env: Env) -> JsObject {
//     let mut obj = env.create_object().unwrap();
//     let _ = obj.set("type", "while");
//     let _ = obj.set("condition", self.condition.to_object(env));
//     let _ = obj.set("body", self.body.to_object(env));
//     let _ = obj.set("is_short", self.is_short);
//     obj
//   }
// }
