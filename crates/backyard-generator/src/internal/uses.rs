use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::identifier::IdentifierGenerator;

pub struct UseGenerator;

impl UseGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Use, &node.node);
    builder.push("use ");

    let mut items = generator.generate_nodes_new(
      &node.items,
      &mut GeneratorArgument::for_parameter(&[(NodeType::UseItem, Self::generate_item)])
    );

    if let Some(name) = &node.name {
      builder.push(name);

      builder.push("{");
      if
        Generator::check_nodes_has_comments(&node.items) ||
        1 + builder.last_len() + items.total_len_with_separator(" ") > generator.max_length
      {
        items.indent();
        builder.extend(&items);
        builder.new_line();
      } else {
        builder.push(&items.print(" "));
      }
      builder.push("}");
    } else if
      Generator::check_nodes_has_comments(&node.items) ||
      1 + builder.last_len() + items.total_len_with_separator(" ") > generator.max_length
    {
      items.indent();
      builder.extend_first_line(&items);
    } else {
      builder.push(&items.print(" "));
    }
  }

  pub fn generate_item(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::UseItem, &node.node);
    if let Some(n) = &node.modifier {
      builder.push(format!("{} ", n).as_str());
    }
    builder.push(&node.name);

    if let Some(alias) = &node.alias {
      builder.push(" as ");
      IdentifierGenerator::generate(generator, builder, alias);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("use const App\\Models\\User;");
    test_eval(
      "use App\\Models\\{
  const User\\UserTesting as UserTestingA,
  User\\UserTestingB as UserTestingB,
  function UserTestingC
};"
    );
    test_eval(
      "use Illuminate\\Foundation\\Auth\\User as Authenticatable,
  Illuminate\\Foundation\\Auth\\User as Authenticatable;"
    );
  }
}
