use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::staticlookup::StaticLookupNode },
};

pub struct StaticLookupGenerator {}

impl StaticLookupGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<StaticLookupNode>(), {
      return;
    });
    generator.generate_node(builder, &node.target, &mut GeneratorArgument::default());
    builder.push("::");
    generator.generate_node(builder, &node.on, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("++A::b();");
  }
}
