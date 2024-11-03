use std::any::Any;

use napi::{ Env, JsObject };

use crate::parser::node::{ NodeTrait, NodeType, Node, Nodes };

#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::Ternary)]
pub struct TernaryNode {
  pub condition: Node,
  pub valid: Node,
  pub invalid: Node,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl NodeTrait for TernaryNode {
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
//     NodeType::Ternary
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   fn to_object(&self, env: Env) -> JsObject {
//     let mut obj = env.create_object().unwrap();
//     let _ = obj.set("type", "ternary");
//     let _ = obj.set("condition", self.condition.to_object(env));
//     let _ = obj.set("valid", self.valid.to_object(env));
//     let _ = obj.set("invalid", self.invalid.to_object(env));
//     obj
//   }
// }
