use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ElvisGenerator {}

impl ElvisGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Elvis, &node.node);
    generator.generate_node(builder, &node.left, &mut GeneratorArgument::default());
    builder.push(" ?: ");
    let mut right = generator.generate_node_new(&node.right);
    if builder.last_len() + right.total_len_with_separator(" ") > generator.max_length {
      right.indent();
      builder.extend(&right);
    } else {
      builder.extend_first_line(&right);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("$a = $b ?: 0;");
  }
}
