use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::tries::{ CatchNode, TryNode } },
};

use super::block::BlockGenerator;

pub struct TryGenerator {}

impl TryGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<TryNode>(), {
      return;
    });
    builder.push("try");
    BlockGenerator::generate(generator, builder, &node.body, None);
    generator.generate_nodes(
      builder,
      &node.catches,
      &mut GeneratorArgument::generator(&[(NodeType::Catch, Self::generate_catch)])
    );
    if let Some(finally) = &node.finally {
      builder.push(" finally");
      BlockGenerator::generate(generator, builder, &finally, None);
    }
  }

  pub fn generate_catch(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<CatchNode>(), {
      return;
    });
    builder.push(" catch (");
    let types = generator.generate_nodes_new(&node.types, &mut GeneratorArgument::default());
    builder.push(&types.to_string(" | "));
    builder.push(" ");
    generator.generate_node(builder, &node.variable, &mut GeneratorArgument::default());
    builder.push(")");
    BlockGenerator::generate(generator, builder, &node.body, None);
  }
}
