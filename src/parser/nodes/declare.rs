use std::any::Any;

use napi::{ Env, JsObject };

use crate::parser::node::{ NodeTrait, NodeType, Node, Nodes, BodyType };

#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::Declare)]
pub struct DeclareNode {
  pub arguments: Nodes,
  pub body: Option<Node>,
  pub body_type: BodyType,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl NodeTrait for DeclareNode {
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
//     NodeType::Declare
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   fn to_object(&self, env: Env) -> JsObject {
//     let mut obj = env.create_object().unwrap();
//     let _ = obj.set("type", "declare");
//     let _ = obj.set(
//       "arguments",
//       self.arguments
//         .iter()
//         .map(|x| x.to_object(env))
//         .collect::<Vec<JsObject>>()
//     );
//     let _ = obj.set("body", match &self.body {
//       Some(x) => Some(x.to_object(env)),
//       _ => None,
//     });
//     let _ = obj.set("body_type", self.body_type.to_object());
//     obj
//   }
// }

#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::DeclareArgument)]
pub struct DeclareArgumentNode {
  pub name: Node,
  pub value: Node,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

// impl NodeTrait for DeclareArgumentNode {
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
//     NodeType::DeclareArgument
//   }

//   fn as_any(self: Box<Self>) -> Box<dyn Any> {
//     self
//   }

//   fn to_object(&self, env: Env) -> JsObject {
//     let mut obj = env.create_object().unwrap();
//     let _ = obj.set("type", "declare_argument");
//     let _ = obj.set("name", self.name.to_object(env));
//     let _ = obj.set("value", self.value.to_object(env));
//     obj
//   }
// }
