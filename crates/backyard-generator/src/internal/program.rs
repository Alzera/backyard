use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ProgramGenerator {}

impl ProgramGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Program, &node.node);
    builder.push("<?php");
    generator.generate_nodes(builder, &node.children, &mut GeneratorArgument::for_block());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("<?php\n$a = ++($a++);");
  }
}
