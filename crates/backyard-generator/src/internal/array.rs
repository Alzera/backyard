use backyard_nodes::{ cast_node, node::{ ArrayNode, Node, NodeType, NodeWrapper } };
use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ArrayGenerator;

impl ArrayGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Array, &node.node);

    let items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&[(NodeType::ArrayItem, Self::generate_item)])
    );
    if node.is_short {
      builder.push("[");
      Self::print_values(generator, builder, items, node);
      builder.push("]");
    } else {
      builder.push("array(");
      Self::print_values(generator, builder, items, node);
      builder.push(")");
    }
  }

  fn print_values(
    generator: &mut Generator,
    builder: &mut Builder,
    mut items: Builder,
    node: &ArrayNode
  ) {
    if
      Generator::check_nodes_has_comments(&node.items) ||
      2 + builder.last_len() + items.total_len_with_separator(" ") > generator.max_length
    {
      items.indent();
      builder.extend(items);
      builder.new_line();
    } else {
      builder.push(&items.print(" "));
    }
  }

  pub fn generate_item<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(ArrayItem, &node.node);
    if let Some(key) = &node.key {
      generator.generate_node(builder, key, &mut GeneratorArgument::default());
      builder.push(" => ");
    }
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
  }
}
