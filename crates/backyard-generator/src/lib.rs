use backyard_nodes::node::{ Node, NodeWrapper };
use error::GeneratorError;
use generator::Generator;

pub mod error;
mod generator;
mod internal;

pub fn generate(node: &Node) -> Result<String, GeneratorError> {
  if let NodeWrapper::Program(program) = &node.node {
    let mut generator = Generator::new(&program.children);
    Ok(generator.start())
  } else {
    Err(GeneratorError::NotAProgram)
  }
}
