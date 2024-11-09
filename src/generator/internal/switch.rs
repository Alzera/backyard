use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::switch::{ CaseNode, SwitchNode } },
};

use super::block::BlockGenerator;

pub struct SwitchGenerator {}

impl SwitchGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<SwitchNode>(), {
      return;
    });

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

  pub fn generate_case(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<CaseNode>(), {
      return;
    });

    if let Some(n) = &node.condition {
      builder.push("case ");
      generator.generate_node(builder, &n, &mut GeneratorArgument::default());
      builder.push(":");
    } else {
      builder.push("default:");
    }
    let mut body = BlockGenerator::generate_base(generator, &node.body, &DEFAULT_GENERATORS);
    body.indent();
    builder.extend(&body);
  }
}
