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
    let mut values = generator.generate_nodes_new(
      &node.values,
      &mut GeneratorArgument::for_parameter(&[(NodeType::ArrayItem, Self::generate_item)])
    );
    builder.push("[");
    if
      Generator::check_nodes_has_comments(&node.values) ||
      2 + builder.last_len() + values.total_len_with_separator(", ") > generator.max_length
    {
      values.indent();
      builder.extend(&values);
      builder.new_line();
    } else {
      builder.push(&values.to_string(" "));
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
