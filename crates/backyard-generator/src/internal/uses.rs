use backyard_nodes::{ cast_node, Node, NodeType, NodeWrapper };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::identifier::IdentifierGenerator;

pub struct UseGenerator;

impl UseGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Use, &node.wrapper);
    builder.push("use ");

    let mut items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&[(NodeType::UseItem, Self::generate_item)])
    );

    if let Some(name) = &node.name {
      builder.push(name);

      builder.push("{");
      if
        Generator::check_nodes_has_comments(&node.items) ||
        1 + builder.last_len() + items.total_len_with_separator(" ") > generator.max_length
      {
        items.indent();
        builder.extend(items);
        builder.new_line();
      } else {
        builder.push(&items.print(" "));
      }
      builder.push("}");
    } else if
      Generator::check_nodes_has_comments(&node.items) ||
      1 + builder.last_len() + items.total_len_with_separator(" ") > generator.max_length
    {
      items.indent();
      builder.extend_first_line(items);
    } else {
      builder.push(&items.print(" "));
    }
  }

  pub fn generate_item<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(UseItem, &node.wrapper);
    if let Some(n) = &node.modifier {
      builder.push(format!("{} ", n).as_str());
    }
    builder.push(&node.name);

    if let Some(alias) = &node.alias {
      builder.push(" as ");
      IdentifierGenerator::generate(generator, builder, alias);
    }
  }
}
