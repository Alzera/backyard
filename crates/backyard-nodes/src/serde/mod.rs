// For now, it's incredibly hard to serde deserialize bumpalo Vec and Box.
// So we just use this SerializableNode as work around to make deserialize possible.
pub mod deserialize;
pub mod serialize;
pub mod node;

use bumpalo::Bump;
use node::SerializableNode;
use serialize::Serializable;
use deserialize::Deserializable;

use crate::Node;

impl<'arena> Node<'arena> {
  pub fn serializable(&self) -> SerializableNode {
    self.to_serializable()
  }
}

impl SerializableNode {
  pub fn deserialize_in<'arena>(&self, arena: &'arena Bump) -> Node<'arena> {
    self.to_deserialize_in(arena)
  }
}
