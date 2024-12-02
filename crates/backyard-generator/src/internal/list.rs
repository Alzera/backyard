use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS };

pub struct ListGenerator;

impl ListGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::List, &node.node);
    builder.push("list(");
    let mut values = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
    );
    if
      Generator::check_nodes_has_comments(&node.items) ||
      2 + builder.last_len() + values.total_len_with_separator(" ") > generator.max_length
    {
      values.indent();
      builder.extend(&values);
      builder.new_line();
    } else {
      builder.push(&values.print(" "));
    }
    builder.push(")");
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("list($a, $b) = [0, 1];");
    test_eval(
      "list(
  $an_unneccessary_very_long_variable_name,
  $another_unneccessary_very_long_variable_name,
  $still_another_unneccessary_very_long_variable_name
) = [0, 1];"
    );
  }
}
