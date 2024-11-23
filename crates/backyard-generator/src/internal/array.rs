use backyard_nodes::{ cast_node, node::{ ArrayNode, Node, NodeType, NodeWrapper } };
use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ArrayGenerator {}

impl ArrayGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Array, &node.node);

    let mut items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&[(NodeType::ArrayItem, Self::generate_item)])
    );
    if node.is_short {
      builder.push("[");
      Self::print_values(generator, builder, &mut items, node);
      builder.push("]");
    } else {
      builder.push("array(");
      Self::print_values(generator, builder, &mut items, node);
      builder.push(")");
    }
  }

  fn print_values(
    generator: &mut Generator,
    builder: &mut Builder,
    items: &mut Builder,
    node: &ArrayNode
  ) {
    if
      Generator::check_nodes_has_comments(&node.items) ||
      2 + builder.last_len() + items.total_len_with_separator(" ") > generator.max_length
    {
      items.indent();
      builder.extend(&items);
      builder.new_line();
    } else {
      builder.push(&items.to_string(" "));
    }
  }

  pub fn generate_item(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::ArrayItem, &node.node);
    if let Some(key) = &node.key {
      generator.generate_node(builder, &key, &mut GeneratorArgument::default());
      builder.push(" => ");
    }
    generator.generate_node(builder, &node.value, &mut GeneratorArgument::default());
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("[1, 2, 3];");
    test_eval("[\"a\" => 1, \"b\" => 2, \"c\" => 3];");
    test_eval(
      "...[
  \"an_unneccessary_very_long_string\" => 1,
  \"another_unneccessary_very_long_string\" => 2,
  \"still_another_unneccessary_very_long_string\" => 3
];"
    );
    test_eval(
      "[
  // Unit with indexes starting at 1 (other units start at 0) 
  \"day\",
  \"week\",
  \"month\",
  \"quarter\"
];"
    );
    test_eval("array(1, 2, 3);");
  }
}
