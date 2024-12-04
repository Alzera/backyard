use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::block::BlockGenerator;

pub struct IfGenerator;

impl IfGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::If, &node.node);

    builder.push("if (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(")");

    if node.is_short {
      if let Some(n) = &node.invalid {
        BlockGenerator::generate(generator, builder, &node.valid, Some(""));
        Self::generate_else(generator, builder, n);
      } else {
        BlockGenerator::generate(generator, builder, &node.valid, Some("endif;"));
      }
    } else {
      if node.valid.node_type == NodeType::Block {
        BlockGenerator::generate(generator, builder, &node.valid, None);
      } else {
        builder.push(" ");
        generator.generate_node(builder, &node.valid, &mut GeneratorArgument::for_block());
      }
      if let Some(n) = &node.invalid {
        if node.valid.node_type == NodeType::Block {
          builder.push(" ");
        } else {
          builder.new_line();
        }
        generator.generate_node(builder, n, &mut GeneratorArgument::default());
      }
    }
  }

  pub fn generate_else(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Else, &node.node);

    builder.push("else");
    match node.body.node_type {
      NodeType::If => {
        Self::generate(generator, builder, &node.body);
      }
      NodeType::Block => {
        BlockGenerator::generate(generator, builder, &node.body, node.is_short.then_some("endif;"));
      }
      _ => {
        builder.push(" ");
        generator.generate_node(builder, &node.body, &mut GeneratorArgument::for_block());
      }
    }
  }
}
