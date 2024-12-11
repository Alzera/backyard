use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS };

use super::block::BlockGenerator;

pub struct SwitchGenerator;

impl SwitchGenerator {
  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Switch, &node.wrapper);

    builder.push("switch (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(")");
    let end = if node.is_short { Some("endswitch;") } else { None };
    BlockGenerator::generate_specific(
      generator,
      builder,
      &node.body,
      end,
      &[(NodeType::Case, Self::generate_case)]
    );
  }

  pub fn generate_case<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Case, &node.wrapper);

    if let Some(n) = &node.condition {
      builder.push("case ");
      generator.generate_node(builder, n, &mut GeneratorArgument::default());
      builder.push(":");
    } else {
      builder.push("default:");
    }
    let mut body = BlockGenerator::generate_base(generator, &node.body, &DEFAULT_GENERATORS);
    body.indent();
    builder.extend(body);
  }
}
