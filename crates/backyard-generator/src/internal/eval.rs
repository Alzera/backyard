use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct EvalGenerator;

impl EvalGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Eval, &node.node);
    builder.push("eval(");
    generator.generate_node(builder, &node.statement, &mut GeneratorArgument::default());
    builder.push(")");
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("eval(\"\");");
  }
}
