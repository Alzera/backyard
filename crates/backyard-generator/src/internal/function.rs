use backyard_nodes::{ cast_node, Node, NodeType, NodeWrapper };

use crate::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS };

use super::{ block::BlockGenerator, identifier::IdentifierGenerator };

pub struct FunctionGenerator;

impl FunctionGenerator {
  pub fn get_parameters<'arena>(
    generator: &mut Generator<'arena, '_>,
    parameters: &[Node<'arena>]
  ) -> Builder {
    generator.generate_nodes_new(
      parameters,
      &mut GeneratorArgument::for_parameter(&[(NodeType::Parameter, Self::generate_parameter)])
    )
  }

  pub fn get_return_type<'arena>(
    generator: &mut Generator<'arena, '_>,
    node: &Option<&Node<'arena>>
  ) -> (Option<Builder>, usize) {
    let return_type = node.as_ref().map(|n| generator.generate_node_new(n));
    let return_type_len = if let Some(n) = &return_type {
      n.total_len_with_separator(" ")
    } else {
      0
    };
    (return_type, return_type_len)
  }

  pub fn generate<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Function, &node.wrapper);
    builder.push("function ");
    if node.is_ref {
      builder.push("&");
    }
    let mut parameters = if node.name.node_type == NodeType::Magic {
      let name = cast_node!(Magic, &node.name.wrapper);
      builder.push(&name.name);
      if name.name == "__construct" {
        generator.generate_nodes_new(
          &node.parameters,
          &mut GeneratorArgument::for_parameter(
            &[(NodeType::ConstructorParameter, Self::generate_constructor_parameter)]
          )
        )
      } else {
        Self::get_parameters(generator, &node.parameters)
      }
    } else {
      let name = cast_node!(Identifier, &node.name.wrapper);
      builder.push(&name.name);
      Self::get_parameters(generator, &node.parameters)
    };
    let (return_type, return_type_len) = Self::get_return_type(
      generator,
      &node.return_type.as_deref()
    );

    builder.push("(");
    if
      Generator::check_nodes_has_comments(&node.parameters) ||
      3 + builder.last_len() + parameters.total_len_with_separator(" ") + return_type_len >
        generator.max_length
    {
      parameters.indent();
      builder.extend(parameters);
      builder.new_line();
    } else {
      builder.push(&parameters.print(" "));
    }
    builder.push(")");

    if let Some(n) = return_type {
      builder.push(": ");
      builder.extend_first_line(n);
    }
    if let Some(n) = &node.body {
      BlockGenerator::generate(generator, builder, n, None);
    } else {
      builder.push(";");
    }
  }

  pub fn generate_anonymous<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(AnonymousFunction, &node.wrapper);
    builder.push("function ");
    if node.is_ref {
      builder.push("&");
    }
    let mut parameters = Self::get_parameters(generator, &node.parameters);
    let mut uses = generator.generate_nodes_new(
      &node.uses,
      &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
    );
    let uses_len = if node.uses.is_empty() { 0 } else { uses.total_len_with_separator(" ") + 7 };
    let (return_type, return_type_len) = Self::get_return_type(
      generator,
      &node.return_type.as_deref()
    );

    builder.push("(");
    if
      Generator::check_nodes_has_comments(&node.parameters) ||
      3 +
        builder.last_len() +
        parameters.total_len_with_separator(" ") +
        uses_len +
        return_type_len > generator.max_length
    {
      parameters.indent();
      builder.extend(parameters);
      builder.new_line();
    } else {
      builder.push(&parameters.print(" "));
    }
    builder.push(")");

    if !node.uses.is_empty() {
      builder.push(" use (");
      if
        Generator::check_nodes_has_comments(&node.uses) ||
        builder.last_len() + uses_len + return_type_len > generator.max_length
      {
        uses.indent();
        builder.extend(uses);
        builder.new_line();
      } else {
        builder.push(&uses.print(" "));
      }
      builder.push(")");
    }

    if let Some(n) = return_type {
      builder.push(": ");
      builder.extend_first_line(n);
    }

    BlockGenerator::generate(generator, builder, &node.body, None);
  }

  pub fn generate_arrow<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(ArrowFunction, &node.wrapper);
    builder.push("fn ");
    if node.is_ref {
      builder.push("&");
    }
    let mut parameters = Self::get_parameters(generator, &node.parameters);
    let (return_type, return_type_len) = Self::get_return_type(
      generator,
      &node.return_type.as_deref()
    );

    builder.push("(");
    if
      Generator::check_nodes_has_comments(&node.parameters) ||
      3 + builder.last_len() + parameters.total_len_with_separator(" ") + return_type_len >
        generator.max_length
    {
      parameters.indent();
      builder.extend(parameters);
      builder.new_line();
    } else {
      builder.push(&parameters.print(" "));
    }
    builder.push(")");

    if let Some(n) = return_type {
      builder.push(": ");
      builder.extend_first_line(n);
    }

    builder.push(" => ");
    generator.generate_node(builder, &node.body, &mut GeneratorArgument::default());
  }

  pub fn generate_constructor_parameter<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(ConstructorParameter, &node.wrapper);
    for visibility in &node.visibilities {
      builder.push(&format!("{} ", visibility));
    }
    if let Some(modifier) = &node.modifier {
      builder.push(&format!("{} ", modifier));
    }
    Self::generate_parameter(generator, builder, &node.parameter);
  }

  pub fn generate_parameter<'arena>(
    generator: &mut Generator<'arena, '_>,
    builder: &mut Builder,
    node: &Node<'arena>
  ) {
    let node = cast_node!(Parameter, &node.wrapper);
    if let Some(n) = &node.variable_type {
      generator.generate_node(builder, n, &mut GeneratorArgument::default());
      builder.push(" ");
    }
    if node.is_ref {
      builder.push("&");
    }
    if node.is_ellipsis {
      builder.push("...");
    }
    builder.push("$");
    IdentifierGenerator::generate(generator, builder, &node.name);
    if let Some(n) = &node.value {
      builder.push(" = ");
      generator.generate_node(builder, n, &mut GeneratorArgument::default());
    };
  }
}
