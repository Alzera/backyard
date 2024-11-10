use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::eval::EvalNode },
};

pub struct EvalGenerator {}

impl EvalGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<EvalNode>());
    builder.push("eval(");
    generator.generate_node(builder, &node.argument, &mut GeneratorArgument::default());
    builder.push(")");
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("eval(\"\");");
  }
}
