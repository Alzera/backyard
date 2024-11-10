use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::program::ProgramNode },
};

pub struct ProgramGenerator {}

impl ProgramGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<ProgramNode>());
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
