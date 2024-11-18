use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct PostGenerator {}

impl PostGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Post, &node.node);
    generator.generate_node(builder, &node.variable, &mut GeneratorArgument::default());
    builder.push(&node.operator.as_str());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("$a = ++($a++);");
    test("$a = --($a--);");
  }
}