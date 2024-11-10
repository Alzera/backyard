use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast }, nodes::bin::BinNode },
};

pub struct BinGenerator {}

impl BinGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<BinNode>(), {
      return;
    });
    generator.generate_node(builder, &node.left, &mut GeneratorArgument::default());
    let mut expr = generator.generate_node_new(&node.right);
    if builder.last_len() + expr.first_len() + node.operator.len() > generator.max_length {
      expr.shift(format!("{} ", node.operator).as_str());
      expr.indent();
      builder.extend(&expr);
    } else {
      builder.push(format!(" {} ", node.operator).as_str());
      builder.extend_first_line(&expr);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("$a . 0;");
  }
}
