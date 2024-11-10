use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::assignment::AssignmentNode },
};

pub struct AssignmentGenerator {}

impl AssignmentGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<AssignmentNode>(), {
      return;
    });
    generator.generate_node(builder, &node.left, &mut GeneratorArgument::default());
    builder.push(format!(" {} ", node.operator).as_str());
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
    test("$a = 0;");
  }
}
