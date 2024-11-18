use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct InstanceOfGenerator {}

impl InstanceOfGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::InstanceOf, &node.node);

    generator.generate_node(builder, &node.left, &mut GeneratorArgument::default());
    builder.push(" instanceof ");
    if builder.last_len() > generator.max_length {
      builder.new_line();
    }
    generator.generate_node(builder, &node.right, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("$a = $a instanceof int;");
  }
}
