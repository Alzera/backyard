use std::any::Any;

use napi::bindgen_prelude::ToNapiValue;
use napi::bindgen_prelude::FromNapiRef;

use crate::parser::node::{ NodeType, Nodes };
#[napi]
#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::List)]
pub struct ListNode {
  pub values: Nodes,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl NodeTrait for ListNode {
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
//     NodeType::List
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   fn to_object(&self, env: Env) -> JsObject {
//     let mut obj = env.create_object().unwrap();
//     let _ = obj.set("type", "list");
//     let _ = obj.set(
//       "values",
//       self.values
//         .iter()
//         .map(|x| x.to_object(env))
//         .collect::<Vec<JsObject>>()
//     );
//     obj
//   }
// }
