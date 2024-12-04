use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::property::PropertyGenerator;

pub struct GlobalGenerator;

impl GlobalGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Global, &node.node);

    builder.push("global ");
    let mut items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(
        &[(NodeType::PropertyItem, PropertyGenerator::generate_item)]
      )
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
