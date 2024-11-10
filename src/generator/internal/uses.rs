use crate::{
  generator::generator::{ Builder, EndMode, Generator, GeneratorArgument, DEFAULT_GENERATORS },
  guard,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::uses::UseNode },
};

use super::identifier::IdentifierGenerator;

pub struct UseGenerator {}

impl UseGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<UseNode>());
    builder.push("use ");
    if node.modifier.len() > 0 {
      builder.push(format!("{} ", node.modifier).as_str());
    }
    let names = generator.generate_nodes_new(
      &node.names,
      &mut GeneratorArgument::new(EndMode::None, &DEFAULT_GENERATORS)
    );
    builder.push(&names.to_string("\\"));
    if node.items.len() > 0 {
      let mut items = generator.generate_nodes_new(
        &node.items,
        &mut GeneratorArgument::for_parameter(
          &[(NodeType::Identifier, IdentifierGenerator::generate)]
        )
      );
      builder.push("\\{");
      if
        Generator::check_nodes_has_comments(&node.items) ||
        1 + builder.last_len() + items.total_len_with_separator(" ") > generator.max_length
      {
        items.indent();
        builder.extend(&items);
        builder.new_line();
      } else {
        builder.push(&items.to_string(" "));
      }
      builder.push("}");
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("use const App\\Models\\User;");
    test("use function App\\Models\\{User, Post, Comment};");
  }
}
