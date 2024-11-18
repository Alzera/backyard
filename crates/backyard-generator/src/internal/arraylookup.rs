use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ArrayLookupGenerator {}

impl ArrayLookupGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::ArrayLookup, &node.node);
    generator.generate_node(builder, &node.target, &mut GeneratorArgument::default());
    builder.push("[");
    generator.generate_node(builder, &node.on, &mut GeneratorArgument::default());
    builder.push("]");
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("[][0];");
    test("$a[0];");
  }
}
