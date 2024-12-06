use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct AssignmentGenerator;

impl AssignmentGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(Assignment, &node.node);
    generator.generate_node(builder, &node.left, &mut GeneratorArgument::default());
    builder.push(format!(" {} ", node.operator).as_str());
    let mut right = generator.generate_node_new(&node.right);
    if builder.last_len() + right.total_len_with_separator(" ") > generator.max_length {
      right.indent();
      builder.extend(right);
    } else {
      builder.extend_first_line(right);
    }
  }
}
