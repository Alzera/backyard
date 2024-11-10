use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::yields::{ YieldFromNode, YieldNode } },
};

pub struct YieldGenerator {}

impl YieldGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<YieldNode>());

    builder.push("yield ");
    if let Some(key) = node.key {
      generator.generate_node(builder, &key, &mut GeneratorArgument::default());
      builder.push(" => ");
    }
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
  }

  pub fn generate_from(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<YieldFromNode>());

    builder.push("yield from ");
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("yield \"key\" => \"value\";");
    test("yield \"another_value\";");
    test("yield from [1, 2, 3];");
  }
}
