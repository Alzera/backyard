use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::exit::ExitNode },
};

pub struct ExitGenerator {}

impl ExitGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<ExitNode>());
    builder.push("exit(");
    generator.generate_node(builder, &node.argument, &mut GeneratorArgument::default());
    builder.push(")");
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("exit(0);");
  }
}
