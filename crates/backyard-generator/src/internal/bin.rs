use backyard_nodes::{ cast_node, Node, NodeWrapper };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct BinGenerator;

impl BinGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Bin, &node.wrapper);
    generator.generate_node(builder, &node.left, &mut GeneratorArgument::default());
    let mut expr = generator.generate_node_new(&node.right);
    if builder.last_len() + expr.first_len() + node.operator.len() > generator.max_length {
      let mut t = Builder::new();
      t.new_line();
      t.push(format!("{} ", node.operator).as_str());
      t.indent();
      builder.extend(t);
      expr.indent();
      builder.extend_first_line(expr);
    } else {
      builder.push(format!(" {} ", node.operator).as_str());
      builder.extend_first_line(expr);
    }
  }
}
