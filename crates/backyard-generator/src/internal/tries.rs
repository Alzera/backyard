use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::block::BlockGenerator;

pub struct TryGenerator;

impl TryGenerator {
  pub fn generate<'arena, 'a>(
    generator: &mut Generator<'arena, 'a>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Try, &node.node);
    builder.push("try");
    BlockGenerator::generate(generator, builder, &node.body, None);
    for catch in &node.catches {
      if catch.node_type == NodeType::Finally {
        let node = cast_node!(Finally, &catch.node);
        builder.push(" finally");
        BlockGenerator::generate(generator, builder, &node.body, None);
      } else {
        Self::generate_catch(generator, builder, catch);
      }
    }
  }

  pub fn generate_catch<'arena, 'a>(
    generator: &mut Generator<'arena, 'a>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Catch, &node.node);
    builder.push(" catch (");
    let types = generator.generate_nodes_new(&node.types, &mut GeneratorArgument::default());
    builder.push(&types.print(" | "));
    if let Some(variable) = &node.variable {
      builder.push(" ");
      generator.generate_node(builder, variable, &mut GeneratorArgument::default());
    }
    builder.push(")");
    BlockGenerator::generate(generator, builder, &node.body, None);
  }
}
