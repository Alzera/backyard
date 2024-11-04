use std::any::Any;

use napi::{ Env, JsObject };

use crate::parser::node::{ NodeTrait, NodeType, Node, Nodes };

#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::Enum)]
pub struct EnumNode {
  pub name: Node,
  pub items: Nodes,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}

#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::EnumItem)]
pub struct EnumItemNode {
  pub value: Node,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}
