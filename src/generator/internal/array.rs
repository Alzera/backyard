use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::array::{ ArrayItemNode, ArrayNode } },
};

pub struct ArrayGenerator {}

impl ArrayGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ArrayNode>(), {
      return;
    });

    if node.is_ellipsis {
      builder.push("...");
    }
    let mut items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&[(NodeType::ArrayItem, Self::generate_item)])
    );
    builder.push("[");
    if
      Generator::check_nodes_has_comments(&node.items) ||
      2 + builder.last_len() + items.total_len_with_separator(" ") > generator.max_length
    {
      items.indent();
      builder.extend(&items);
      builder.new_line();
    } else {
      builder.push(&items.to_string(" "));
    }
    builder.push("]");
  }

  pub fn generate_item(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ArrayItemNode>(), {
      return;
    });
    if let Some(key) = &node.key {
      generator.generate_node(builder, &key, &mut GeneratorArgument::default());
      builder.push(" => ");
    }
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("[1, 2, 3];");
    test("[\"a\" => 1, \"b\" => 2, \"c\" => 3];");
  }
}
