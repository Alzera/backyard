use std::any::Any;

use napi::bindgen_prelude::ToNapiValue;
use napi::bindgen_prelude::FromNapiRef;

use crate::parser::node::{ NodeType, Node, Nodes };
#[napi]
#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::StaticLookup)]
pub struct StaticLookupNode {
  pub target: Node,
  pub on: Node,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl NodeTrait for StaticLookupNode {
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
//     NodeType::StaticLookup
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   fn to_object(&self, env: Env) -> JsObject {
//     let mut obj = env.create_object().unwrap();
//     let _ = obj.set("type", "static_lookup");
//     let _ = obj.set("target", self.target.to_object(env));
//     let _ = obj.set("on", self.on.to_object(env));
//     obj
//   }
// }
