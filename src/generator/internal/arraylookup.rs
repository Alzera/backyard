use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::arraylookup::ArrayLookupNode },
};

pub struct ArrayLookupGenerator {}

impl ArrayLookupGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<ArrayLookupNode>());
    generator.generate_node(builder, &node.target, &mut GeneratorArgument::default());
    builder.push("[");
    generator.generate_node(builder, &node.on, &mut GeneratorArgument::default());
    builder.push("]");
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("[][0];");
    test("$a[0];");
  }
}
