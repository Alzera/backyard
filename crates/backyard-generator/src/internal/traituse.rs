use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, EndMode, Generator, GeneratorArgument };

use super::identifier::IdentifierGenerator;

pub struct TraitUseGenerator;

impl TraitUseGenerator {
  pub fn generate<'arena, 'a>(
    generator: &mut Generator<'arena, 'a>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(TraitUse, &node.node);
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
      builder.extend_first_line(traits);
    } else {
      builder.push(&traits.print(" "));
    }
    if !node.adaptations.is_empty() {
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
      adaptations_builder.indent();
      builder.extend(adaptations_builder);
      builder.new_line();
      builder.push("}");
    } else {
      builder.push(";");
    }
  }

  pub fn generate_alias<'arena, 'a>(
    generator: &mut Generator<'arena, 'a>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(TraitUseAlias, &node.node);
    if let Some(trait_name) = &node.trait_name {
      IdentifierGenerator::generate(generator, builder, trait_name);
      builder.push("::");
    }
    IdentifierGenerator::generate(generator, builder, &node.method);
    builder.push(" as");
    if let Some(n) = &node.visibility {
      builder.push(" ");
      builder.push(format!("{}", n).as_str());
    }
    if let Some(alias) = &node.alias {
      builder.push(" ");
      IdentifierGenerator::generate(generator, builder, alias);
    }
  }

  pub fn generate_precedence<'arena, 'a>(
    generator: &mut Generator<'arena, 'a>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(TraitUsePrecedence, &node.node);
    if let Some(trait_name) = &node.trait_name {
      IdentifierGenerator::generate(generator, builder, trait_name);
      builder.push("::");
    }
    IdentifierGenerator::generate(generator, builder, &node.method);
    builder.push(" insteadof ");
    IdentifierGenerator::generate(generator, builder, &node.instead);
  }
}
