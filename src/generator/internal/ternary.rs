use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::ternary::TernaryNode },
};

pub struct TernaryGenerator {}

impl TernaryGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<TernaryNode>(), {
      return;
    });

    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(" ? ");
    generator.generate_node(builder, &node.valid, &mut GeneratorArgument::default());
    builder.push(" : ");
    generator.generate_node(builder, &node.invalid, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("$a = false ? 1 : 2;");
  }
}
