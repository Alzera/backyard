use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ExitGenerator {}

impl ExitGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Exit, &node.node);
    builder.push("exit(");
    if let Some(argument) = &node.argument {
      generator.generate_node(builder, &argument, &mut GeneratorArgument::default());
    }
    builder.push(")");
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("exit(0);");
  }
}
