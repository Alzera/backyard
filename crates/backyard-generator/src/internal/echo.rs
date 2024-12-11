use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS };

pub struct EchoGenerator;

impl EchoGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Echo, &node.wrapper);

    builder.push("echo ");
    let mut items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
    );
    if
      Generator::check_nodes_has_comments(&node.items) ||
      2 + builder.last_len() + items.total_len_with_separator(" ") > generator.max_length
    {
      items.indent();
      builder.extend_first_line(items);
    } else {
      builder.push(&items.print(" "));
    }
  }
}
