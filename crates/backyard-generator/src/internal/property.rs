use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::identifier::IdentifierGenerator;

pub struct PropertyGenerator {}

impl PropertyGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Property, &node.node);
    if node.visibility.len() > 0 {
      builder.push(format!("{} ", node.visibility).as_str());
    }
    if node.modifier.len() > 0 {
      builder.push(format!("{} ", node.modifier).as_str());
    }

    let mut items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&[(NodeType::PropertyItem, Self::generate_item)])
    );
    if
      Generator::check_nodes_has_comments(&node.items) ||
      2 + builder.last_len() + items.total_len_with_separator(" ") > generator.max_length
    {
      items.indent();
      builder.extend_first_line(&items);
    } else {
      builder.push(&items.to_string(" "));
    }
  }

  pub fn generate_item(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::PropertyItem, &node.node);
    if let Some(variable_type) = &node.variable_type {
      generator.generate_node(builder, variable_type, &mut GeneratorArgument::default());
      builder.push(" ");
    }
    builder.push("$");
    IdentifierGenerator::generate(generator, builder, &node.name);
    if let Some(value) = &node.value {
      builder.push(" = ");
      generator.generate_node(builder, value, &mut GeneratorArgument::default());
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval(
      "class A {
  public static ?A $instance = 4;
  public readonly A|callable|null $instance2 = 4;
}"
    );
  }
}
