use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct CallGenerator {}

impl CallGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Call, &node.node);
    generator.generate_node(builder, &node.name, &mut GeneratorArgument::default());
    let mut arguments = generator.generate_nodes_new(
      &node.arguments,
      &mut GeneratorArgument::for_parameter(&[(NodeType::Argument, Self::generate_argument)])
    );
    builder.push("(");
    if
      Generator::check_nodes_has_comments(&node.arguments) ||
      2 + builder.last_len() + arguments.total_len_with_separator(" ") > generator.max_length
    {
      arguments.indent();
      builder.extend(&arguments);
      builder.new_line();
    } else {
      builder.push(&arguments.to_string(" "));
    }
    builder.push(")");
  }

  pub fn generate_argument(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Argument, &node.node);
    if let Some(name) = &node.name {
      generator.generate_node(builder, name, &mut GeneratorArgument::default());
      builder.push(": ");
    }
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("return new static($expression, $fieldFactory);");
    test("floor(((int) $this->rawFormat(\"u\")) / 1000);");
    test("\\call();");
    test("(fn () => 0)();");
    test("call(true);");
    test("call(a: 0, b: 0);");
    test(
      "call(
  an_unneccessary_very_long_variable_name: 0,
  another_unneccessary_very_long_variable_name: 0,
  still_another_unneccessary_very_long_variable_name: 0
);"
    );
  }
}
