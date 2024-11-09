use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::arraylookup::ArrayLookupNode },
};

pub struct ArrayLookupGenerator {}

impl ArrayLookupGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ArrayLookupNode>(), {
      return;
    });
    generator.generate_node(builder, &node.target, &mut GeneratorArgument::default());
    builder.push("[");
    generator.generate_node(builder, &node.on, &mut GeneratorArgument::default());
    builder.push("]");
  }
}
