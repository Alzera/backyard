use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, EndMode, Generator, GeneratorArgument };

use super::identifier::IdentifierGenerator;

pub struct TraitUseGenerator {}

impl TraitUseGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::TraitUse, &node.node);
    builder.push("use ");
    let mut traits = generator.generate_nodes_new(
      &node.traits,
      &mut GeneratorArgument::for_parameter(
        &[(NodeType::Identifier, IdentifierGenerator::generate)]
      )
    );
    if
      Generator::check_nodes_has_comments(&node.traits) ||
      2 + builder.last_len() + traits.total_len_with_separator(" ") > generator.max_length
    {
      traits.indent();
      builder.extend_first_line(&traits);
    } else {
      builder.push(&traits.to_string(" "));
    }
    if node.adaptations.len() > 0 {
      let mut adaptations_builder = generator.generate_nodes_new(
        &node.adaptations,
        &mut GeneratorArgument::new(
          EndMode::SemicolonDynamic,
          &[
            (NodeType::TraitUseAlias, Self::generate_alias),
            (NodeType::TraitUsePrecedence, Self::generate_precedence),
          ]
        )
      );
      builder.push(" {");
      if
        Generator::check_nodes_has_comments(&node.adaptations) ||
        2 + builder.last_len() + adaptations_builder.total_len_with_separator(" ") >
          generator.max_length
      {
        adaptations_builder.indent();
        builder.extend(&adaptations_builder);
        builder.new_line();
      } else {
        builder.push(&adaptations_builder.to_string(" "));
      }
      builder.push("}");
    } else {
      builder.push(";");
    }
  }

  pub fn generate_alias(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::TraitUseAlias, &node.node);
    if let Some(trait_name) = &node.trait_name {
      IdentifierGenerator::generate(generator, builder, trait_name);
      builder.push("::");
    }
    IdentifierGenerator::generate(generator, builder, &node.method);
    builder.push(" as ");
    if node.visibility.len() > 0 {
      builder.push(format!("{} ", node.visibility).as_str());
    }
    IdentifierGenerator::generate(generator, builder, &node.alias);
  }

  pub fn generate_precedence(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::TraitUsePrecedence, &node.node);
    IdentifierGenerator::generate(generator, builder, &node.trait_name);
    builder.push("::");
    IdentifierGenerator::generate(generator, builder, &node.method);
    builder.push(" insteadof ");
    IdentifierGenerator::generate(generator, builder, &node.instead);
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test(
      "class A {
  use Ale;
  use Loggable, Usable {
    log as private alias;
    Loggable::log as aliasLoggable;
    Usable::useResource insteadof Loggable;
  }
}"
    );
  }
}
