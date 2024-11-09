use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{
    node::{ Node, NodeTraitCast, NodeType },
    nodes::property::{ PropertyItemNode, PropertyNode },
  },
};

use super::identifier::IdentifierGenerator;

pub struct PropertyGenerator {}

impl PropertyGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<PropertyNode>(), {
      return;
    });
    if node.visibility.len() > 0 {
      builder.push(format!("{} ", node.visibility).as_str());
    }
    if node.modifier.len() > 0 {
      builder.push(format!("{} ", node.modifier).as_str());
    }

    let mut items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&[(NodeType::PropertyItem, Self::generate_item)])
    );
    if
      Generator::check_nodes_has_comments(&node.items) ||
      2 + builder.last_len() + items.total_len_with_separator(", ") > generator.max_length
    {
      items.indent();
      builder.extend_first_line(&items);
    } else {
      builder.push(&items.to_string(" "));
    }
  }

  pub fn generate_item(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<PropertyItemNode>(), {
      return;
    });
    if let Some(variable_type) = &node.variable_type {
      generator.generate_node(builder, variable_type, &mut GeneratorArgument::default());
      builder.push(" ");
    }
    builder.push("$");
    IdentifierGenerator::generate(generator, builder, &node.name);
    if let Some(value) = &node.value {
      builder.push(" = ");
      generator.generate_node(builder, value, &mut GeneratorArgument::default());
    }
  }
}
