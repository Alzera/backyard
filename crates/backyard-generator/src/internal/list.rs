use backyard_nodes::{ cast_node, Node, NodeWrapper };

use crate::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS };

pub struct ListGenerator;

impl ListGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(List, &node.wrapper);
    builder.push("list(");
    let mut values = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
    );
    if
      Generator::check_nodes_has_comments(&node.items) ||
      2 + builder.last_len() + values.total_len_with_separator(" ") > generator.max_length
    {
      values.indent();
      builder.extend(values);
      builder.new_line();
    } else {
      builder.push(&values.print(" "));
    }
    builder.push(")");
  }
}
