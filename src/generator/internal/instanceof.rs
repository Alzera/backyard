use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::instanceof::InstanceOfNode },
};

pub struct InstanceOfGenerator {}

impl InstanceOfGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<InstanceOfNode>());

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
