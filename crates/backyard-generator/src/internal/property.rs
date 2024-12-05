use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::{ block::BlockGenerator, function::FunctionGenerator, identifier::IdentifierGenerator };

pub struct PropertyGenerator;

impl PropertyGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Property, &node.node);
    for visibility in &node.visibilities {
      builder.push(&format!("{} ", visibility));
    }
    if let Some(n) = &node.modifier {
      builder.push(&format!("{} ", n));
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
      builder.extend_first_line(items);
    } else {
      builder.push(&items.print(" "));
    }
    if !node.hooks.is_empty() {
      let mut hooks = generator.generate_nodes_new(
        &node.hooks,
        &mut GeneratorArgument::generator(&[(NodeType::PropertyHook, Self::generate_hook)])
      );
      hooks.indent();
      builder.push(" {");
      builder.extend(hooks);
      builder.new_line();
      builder.push("}");
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

  pub fn generate_hook(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::PropertyHook, &node.node);
    if node.is_ref {
      builder.push("&");
    }
    if node.is_get {
      builder.push("get");
    } else {
      builder.push("set");
    }
    if !node.parameters.is_empty() {
      let mut parameters = FunctionGenerator::get_parameters(generator, &node.parameters);
      builder.push("(");
      if
        Generator::check_nodes_has_comments(&node.parameters) ||
        3 + builder.last_len() + parameters.total_len_with_separator(" ") > generator.max_length
      {
        parameters.indent();
        builder.extend(parameters);
        builder.new_line();
      } else {
        builder.push(&parameters.print(" "));
      }
      builder.push(")");
    }
    if node.body.node_type == NodeType::Block {
      builder.push(" ");
      BlockGenerator::generate(generator, builder, &node.body, None);
    } else {
      builder.push(" => ");
      generator.generate_node(builder, &node.body, &mut GeneratorArgument::default());
    }
  }
}
