use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::tries::{ CatchNode, TryNode } },
};

use super::block::BlockGenerator;

pub struct TryGenerator {}

impl TryGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<TryNode>());
    builder.push("try");
    BlockGenerator::generate(generator, builder, &node.body, None);
    for catch in &node.catches {
      Self::generate_catch(generator, builder, catch);
    }
    if let Some(finally) = &node.finally {
      builder.push(" finally");
      BlockGenerator::generate(generator, builder, &finally, None);
    }
  }

  pub fn generate_catch(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<CatchNode>());
    builder.push(" catch (");
    let types = generator.generate_nodes_new(&node.types, &mut GeneratorArgument::default());
    builder.push(&types.to_string(" | "));
    builder.push(" ");
    generator.generate_node(builder, &node.variable, &mut GeneratorArgument::default());
    builder.push(")");
    BlockGenerator::generate(generator, builder, &node.body, None);
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test(
      "try {
  throw new Error(\"Custom error occurred\");
} catch (FooError $err) {
} catch (Foo2Error | BarError $err) {
} finally {
}"
    );
  }
}
