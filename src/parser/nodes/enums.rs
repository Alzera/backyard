use std::any::Any;

use napi::bindgen_prelude::ToNapiValue;
use napi::bindgen_prelude::FromNapiRef;

use crate::parser::node::{ NodeType, Node, Nodes };
#[napi]
#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::Enum)]
pub struct EnumNode {
  pub name: Node,
  pub items: Nodes,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}
#[napi]
#[derive(Debug, Clone, macros::ImplementNodeTrait)]
#[implement_node_trait(NodeType::EnumItem)]
pub struct EnumItemNode {
  pub value: Node,

  pub leading_comments: Nodes,
  pub trailing_comments: Nodes,
}
