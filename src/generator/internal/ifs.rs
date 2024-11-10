use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::ifs::IfNode },
};

use super::block::BlockGenerator;

pub struct IfGenerator {}

impl IfGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<IfNode>(), {
      return;
    });

    builder.push("if (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(")");

    if node.is_short {
      if let Some(n) = &node.invalid {
        BlockGenerator::generate(generator, builder, &node.valid, Some("else"));
        if n.get_type() == NodeType::If {
          IfGenerator::generate(generator, builder, &n);
        } else {
          BlockGenerator::generate(generator, builder, &n, Some("endif;"));
        }
      } else {
        BlockGenerator::generate(generator, builder, &node.valid, Some("endif;"));
      }
    } else {
      BlockGenerator::generate(generator, builder, &node.valid, None);
      if let Some(n) = &node.invalid {
        builder.push(" else");
        if n.get_type() == NodeType::If {
          IfGenerator::generate(generator, builder, &n);
        } else {
          BlockGenerator::generate(generator, builder, &n, None);
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("if (true):\nelseif (false):\nelse:\nendif;");
    test("if (isset($var1)) {\n} elseif (empty([])) {\n} else {\n}");
  }
}
