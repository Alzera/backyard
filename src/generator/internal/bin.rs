use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::bin::BinNode },
};

pub struct BinGenerator {}

impl BinGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<BinNode>());
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
    [
      "+",
      "-",
      "*",
      "/",
      "%",
      "**",
      "&",
      "|",
      "^",
      "<<",
      ">>",
      "==",
      "===",
      "!=",
      "!==",
      "<",
      ">",
      "<=",
      ">=",
      "<=>",
      ".",
      "??",
      "&&",
      "||",
      "&",
      "|",
      "^",
    ]
      .iter()
      .for_each(|i| {
        test(format!("$a {} 0;", i).as_str());
      });
    test(
      "$an_unneccessary_very_long_variable_name
  . $another_unnecessary_very_long_variable_name_that_should_be_on_new_line;"
    );
  }
}
