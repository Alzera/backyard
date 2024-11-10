use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{
    node::{ BodyType, Node, NodeTraitCast, NodeType },
    nodes::declare::{ DeclareArgumentNode, DeclareNode },
  },
};

use super::{ block::BlockGenerator, identifier::IdentifierGenerator };

pub struct DeclareGenerator {}

impl DeclareGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<DeclareNode>(), {
      return;
    });
    builder.push("declare");
    let mut arguments = generator.generate_nodes_new(
      &node.arguments,
      &mut GeneratorArgument::for_parameter(&[(NodeType::DeclareArgument, Self::generate_argument)])
    );
    builder.push("(");
    if
      Generator::check_nodes_has_comments(&node.arguments) ||
      2 + builder.last_len() + arguments.total_len_with_separator(" ") > generator.max_length
    {
      arguments.indent();
      builder.extend(&arguments);
      builder.new_line();
    } else {
      builder.push(&arguments.to_string(" "));
    }
    builder.push(")");
    match node.body_type {
      BodyType::Basic => {
        if let Some(n) = &node.body {
          BlockGenerator::generate(generator, builder, &n, None);
        }
      }
      BodyType::Short => {
        if let Some(n) = &node.body {
          BlockGenerator::generate(generator, builder, &n, Some("enddeclare;"));
        }
      }
      BodyType::Empty => {
        builder.push(";");
      }
    }
  }

  pub fn generate_argument(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<DeclareArgumentNode>(), {
      return;
    });
    IdentifierGenerator::generate(generator, builder, &node.name);
    builder.push(" = ");
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("declare(strict_types = 1);");
    test("declare(ticks = 1):\nenddeclare;");
    test("declare(ticks = 1, ticks = 1) {\n}");
  }
}
