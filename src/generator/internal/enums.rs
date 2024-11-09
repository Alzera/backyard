use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::enums::{ EnumItemNode, EnumNode } },
};

use super::identifier::IdentifierGenerator;

pub struct EnumGenerator {}

impl EnumGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<EnumNode>(), {
      return;
    });
    builder.push("enum ");
    IdentifierGenerator::generate(generator, builder, &node.name);
    let mut items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&[(NodeType::EnumItem, Self::generate_item)])
    );
    builder.push(" {");
    if
      Generator::check_nodes_has_comments(&node.items) ||
      2 + builder.last_len() + items.total_len_with_separator(", ") > generator.max_length
    {
      items.indent();
      builder.extend(&items);
      builder.new_line();
    } else {
      builder.push(&items.to_string(" "));
    }
    builder.push("}");
  }

  pub fn generate_item(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<EnumItemNode>(), {
      return;
    });
    builder.push("case ");
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
  }
}
