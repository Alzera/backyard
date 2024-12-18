pub mod error;
mod generator;
mod internal;

use backyard_nodes::{
  serde::node::{ SerializableNode, SerializableNodeWrapper },
  Node,
  NodeWrapper,
};
use bumpalo::Bump;
use error::GeneratorError;
use generator::Generator;

pub fn generate_serializable_node(node: &SerializableNode) -> Result<String, GeneratorError> {
  if let SerializableNodeWrapper::Program(_) = &node.wrapper {
    let arena = Bump::new();
    let node = node.deserialize_in(&arena);
    generate(&node)
  } else {
    Err(GeneratorError::NotAProgram)
  }
}

pub fn generate(node: &Node<'_>) -> Result<String, GeneratorError> {
  if let NodeWrapper::Program(program) = &node.wrapper {
    let mut generator = Generator::new(&program.children);
    Ok(generator.start())
  } else {
    Err(GeneratorError::NotAProgram)
  }
}
