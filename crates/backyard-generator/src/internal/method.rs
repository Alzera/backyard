use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

use super::function::FunctionGenerator;

pub struct MethodGenerator;

impl MethodGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Method, &node.node);
    if let Some(n) = &node.visibility {
      builder.push(format!("{} ", n).as_str());
    }
    if let Some(n) = &node.inheritance {
      builder.push(format!("{} ", n).as_str());
    }
    if node.is_static {
      builder.push("static ");
    }
    FunctionGenerator::generate(generator, builder, &node.function);
  }
}
