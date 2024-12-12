use backyard_nodes::{ cast_node, Node, NodeType, NodeWrapper };

use crate::generator::{ Builder, Generator };

use super::identifier::IdentifierGenerator;

pub struct VariableGenerator;

impl VariableGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Variable, &node.wrapper);
    builder.push("$");
    if let NodeType::Identifier = node.name.node_type {
      IdentifierGenerator::generate(generator, builder, &node.name);
    } else {
      builder.push("{");
      let mut expr = generator.generate_node_new(&node.name);
      if 1 + builder.last_len() + expr.first_len() > generator.max_length {
        expr.indent();
        builder.extend(expr);
        builder.new_line();
      } else {
        builder.extend_first_line(expr);
      }
      builder.push("}");
    }
  }
}
