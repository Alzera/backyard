use std::any::Any;

use napi::bindgen_prelude::ToNapiValue;
use napi::bindgen_prelude::FromNapiRef;

use crate::parser::node::{ Node, NodeType, Nodes };

#[napi]
#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::Array)]
pub struct ArrayNode {
  pub is_ellipsis: bool,
  pub items: Nodes,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// #[napi]
// impl ArrayNode {
//   #[napi]
//   pub fn create(is_ellipsis: bool, items: Nodes) -> Self {
//     Self {
//       is_ellipsis,
//       items,
//       leading_comments: vec![],
//       trailing_comments: vec![],
//     }
//   }
// }

#[napi]
#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::ArrayItem)]
pub struct ArrayItemNode {
  pub key: Option<Node>,
  pub value: Node,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl crate::parser::node::NodeTrait for ArrayNode {
//   fn add_leading_comments(&mut self, comments: crate::parser::node::Node) {
//     self.leading_comments.push(comments);
//   }

//   fn add_trailing_comments(&mut self, comments: crate::parser::node::Node) {
//     self.trailing_comments.push(comments);
//   }

//   fn get_type(&self) -> NodeType {
//     NodeType::Array
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   fn get_leading_comments(&self) -> &Nodes {
//     todo!()
//   }

//   fn get_trailing_comments(&self) -> &Nodes {
//     todo!()
//   }

//   unsafe fn to_napi(&self, env: napi::sys::napi_env) -> napi::Result<napi::sys::napi_value> {
//     todo!()
//   }

//   unsafe fn from_napi(env: napi::sys::napi_env, val: napi::sys::napi_value) -> Box<Self>
//     where Self: Sized
//   {
//     let node = Self::from_napi_ref(env, val).ok().unwrap();
//     Box::new(node.to_owned())
//   }
// }
