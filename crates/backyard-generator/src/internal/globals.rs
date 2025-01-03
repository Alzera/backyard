use backyard_nodes::{ cast_node, Node, NodeType, NodeWrapper };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::variable::VariableGenerator;

pub struct GlobalGenerator;

impl GlobalGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Global, &node.wrapper);

    builder.push("global ");
    let mut items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&[(NodeType::Variable, VariableGenerator::generate)])
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
