use crate::{
  generator::generator::{ Builder, Generator },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::variable::VariableNode },
};

use super::identifier::IdentifierGenerator;

pub struct VariableGenerator {}

impl VariableGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<VariableNode>(), {
      return;
    });
    if node.is_ref {
      builder.push("&");
    }
    builder.push("$");
    if let NodeType::Identifier = node.name.get_type() {
      IdentifierGenerator::generate(generator, builder, &node.name);
    } else {
      builder.push("{");
      let mut expr = generator.generate_node_new(&node.name);
      if 1 + builder.last_len() + expr.first_len() > generator.max_length {
        expr.indent();
        builder.extend(&expr);
        builder.new_line();
      } else {
        builder.extend_first_line(&expr);
      }
      builder.push("}");
    }
  }
}