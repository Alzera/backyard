use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::block::BlockGenerator;

pub struct ForeachGenerator;

impl ForeachGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Foreach, &node.node);

    builder.push("foreach (");
    generator.generate_node(builder, &node.source, &mut GeneratorArgument::default());
    builder.push(" as ");
    if let Some(key) = &node.key {
      generator.generate_node(builder, key, &mut GeneratorArgument::default());
      builder.push(" => ");
    }
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
    builder.push(")");
    if node.is_short {
      let end = if node.is_short { Some("endforeach;") } else { None };
      BlockGenerator::generate(generator, builder, &node.body, end);
    } else if node.body.node_type == NodeType::Block {
      BlockGenerator::generate(generator, builder, &node.body, None);
    } else {
      builder.push(" ");
      generator.generate_node(builder, &node.body, &mut GeneratorArgument::for_block());
    }
  }
}
