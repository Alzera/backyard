use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard,
  parser::{ node::{ Node, NodeTraitCast }, nodes::assignment::AssignmentNode },
};

pub struct AssignmentGenerator {}

impl AssignmentGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<AssignmentNode>());
    generator.generate_node(builder, &node.left, &mut GeneratorArgument::default());
    builder.push(format!(" {} ", node.operator).as_str());
    let mut right = generator.generate_node_new(&node.right);
    if builder.last_len() + right.total_len_with_separator(" ") > generator.max_length {
      right.indent();
      builder.extend(&right);
    } else {
      builder.extend_first_line(&right);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    ["=", "&=", "??=", "%=", "^=", "**=", "*=", "/=", ".=", "|=", "-=", ">>=", "<<=", "+="]
      .iter()
      .for_each(|i| {
        test(format!("$a {} 0;", i).as_str());
      });
    test(
      "$an_unneccessary_very_long_variable_name = 
  $another_unnecessary_very_long_variable_name_that_should_be_on_new_line;"
    );
  }
}
