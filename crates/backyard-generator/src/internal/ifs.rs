use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::block::BlockGenerator;

pub struct IfGenerator {}

impl IfGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::If, &node.node);

    builder.push("if (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(")");

    if node.is_short {
      if let Some(n) = &node.invalid {
        BlockGenerator::generate(generator, builder, &node.valid, Some("else"));
        if n.node_type == NodeType::If {
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
        if n.node_type == NodeType::If {
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
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("if (true):\nelseif (false):\nelse:\nendif;");
    test_eval("if (isset($var1)) {\n} elseif (empty([])) {\n} else {\n}");
  }
}
