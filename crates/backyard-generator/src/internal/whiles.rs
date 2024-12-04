use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::block::BlockGenerator;

pub struct WhileGenerator;

impl WhileGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::While, &node.node);

    builder.push("while (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(")");
    if node.is_short {
      let end = if node.is_short { Some("endwhile;") } else { None };
      BlockGenerator::generate(generator, builder, &node.body, end);
    } else if node.body.node_type == NodeType::Block {
      BlockGenerator::generate(generator, builder, &node.body, None);
    } else {
      builder.push(" ");
      generator.generate_node(builder, &node.body, &mut GeneratorArgument::for_block());
    }
  }
}
