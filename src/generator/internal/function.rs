use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{
    node::{ Node, NodeTraitCast, NodeType, Nodes },
    nodes::function::{ AnonymousFunctionNode, ArrowFunctionNode, FunctionNode, ParameterNode },
  },
};

use super::{ block::BlockGenerator, identifier::IdentifierGenerator };

pub struct FunctionGenerator {}

impl FunctionGenerator {
  pub fn get_parameters(
    generator: &mut Generator,
    parameters: &Nodes,
    args: &mut GeneratorArgument
  ) -> Builder {
    generator.generate_nodes_new(
      parameters,
      &mut GeneratorArgument::new(
        &[(NodeType::Parameter, Self::generate_parameter)],
        args.max_length
      )
    )
  }

  pub fn get_return_type(
    generator: &mut Generator,
    node: &Option<Node>,
    args: &mut GeneratorArgument
  ) -> (Option<Builder>, usize) {
    let return_type = if let Some(n) = &node {
      Some(generator.generate_node_new(n, args))
    } else {
      None
    };
    let return_type_len = if let Some(n) = &return_type {
      n.total_len_with_separator(" ")
    } else {
      0
    };
    (return_type, return_type_len)
  }

  pub fn fill_body(
    generator: &mut Generator,
    builder: &mut Builder,
    node: &Node,
    args: &mut GeneratorArgument
  ) {
    builder.push(" {");
    let mut block = Builder::new();
    BlockGenerator::generate(generator, &mut block, &node, args);
    block.indent();
    builder.extend(&block);
    builder.new_line();
    builder.push("}");
  }

  pub fn generate(
    generator: &mut Generator,
    builder: &mut Builder,
    node: &Node,
    args: &mut GeneratorArgument
  ) {
    let node = guard_ok!(node.to_owned().cast::<FunctionNode>(), {
      return;
    });
    builder.push("function ");
    if node.is_ref {
      builder.push("&");
    }
    IdentifierGenerator::generate(generator, builder, &node.name, args);
    let mut parameters = Self::get_parameters(generator, &node.parameters, args);
    let (return_type, return_type_len) = Self::get_return_type(generator, &node.return_type, args);

    builder.push("(");
    if 3 + builder.last_len() + parameters.first_len() + return_type_len > args.max_length {
      parameters.indent();
      parameters.push_all_lines(",");
      builder.extend(&parameters);
      builder.new_line();
    } else {
      builder.push(&parameters.to_string(", "));
    }
    builder.push(")");

    if let Some(n) = &return_type {
      builder.push(": ");
      builder.extend_first_line(n);
    }
    Self::fill_body(generator, builder, &node.body, args);
  }

  pub fn generate_anonymous(
    generator: &mut Generator,
    builder: &mut Builder,
    node: &Node,
    args: &mut GeneratorArgument
  ) {
    let node = guard_ok!(node.to_owned().cast::<AnonymousFunctionNode>(), {
      return;
    });
    builder.push("function ");
    if node.is_ref {
      builder.push("&");
    }
    let mut parameters = Self::get_parameters(generator, &node.parameters, args);
    let mut uses = generator.generate_nodes_new(&node.uses, args);
    let uses_len = if node.uses.is_empty() { 0 } else { uses.total_len_with_separator(", ") + 7 };
    let (return_type, return_type_len) = Self::get_return_type(generator, &node.return_type, args);

    builder.push("(");
    if
      3 + builder.last_len() + parameters.first_len() + uses_len + return_type_len > args.max_length
    {
      parameters.indent();
      parameters.push_all_lines(",");
      builder.extend(&parameters);
      builder.new_line();
    } else {
      builder.push(&parameters.to_string(", "));
    }
    builder.push(")");

    if !node.uses.is_empty() {
      builder.push(" use (");
      if builder.last_len() + uses_len + return_type_len > args.max_length {
        uses.push_all_lines(",");
        builder.extend(&uses);
        builder.new_line();
      } else {
        builder.push(&uses.to_string(", "));
      }
      builder.push(")");
    }

    if let Some(n) = &return_type {
      builder.push(": ");
      builder.extend_first_line(n);
    }
    Self::fill_body(generator, builder, &node.body, args);
  }

  pub fn generate_arrow(
    generator: &mut Generator,
    builder: &mut Builder,
    node: &Node,
    args: &mut GeneratorArgument
  ) {
    let node = guard_ok!(node.to_owned().cast::<ArrowFunctionNode>(), {
      return;
    });
    builder.push("fn ");
    if node.is_ref {
      builder.push("&");
    }
    let mut parameters = Self::get_parameters(generator, &node.parameters, args);
    let (return_type, return_type_len) = Self::get_return_type(generator, &node.return_type, args);

    builder.push("(");
    if 3 + builder.last_len() + parameters.first_len() + return_type_len > args.max_length {
      parameters.indent();
      parameters.push_all_lines(",");
      builder.extend(&parameters);
      builder.new_line();
    } else {
      builder.push(&parameters.to_string(", "));
    }
    builder.push(")");

    if let Some(n) = &return_type {
      builder.push(": ");
      builder.extend_first_line(n);
    }

    builder.push(" => ");
    generator.generate_node(builder, &node.body, args);
  }

  pub fn generate_parameter(
    generator: &mut Generator,
    builder: &mut Builder,
    node: &Node,
    args: &mut GeneratorArgument
  ) {
    let node = guard_ok!(node.to_owned().cast::<ParameterNode>(), {
      return;
    });
    if let Some(n) = &node.variable_type {
      generator.generate_node(builder, n, &mut GeneratorArgument::default());
    }
    if node.is_ref {
      builder.push("&");
    }
    if node.is_ellipsis {
      builder.push("...");
    }
    builder.push("$");
    IdentifierGenerator::generate(generator, builder, &node.name, args);
    if let Some(n) = &node.value {
      generator.generate_node(builder, n, &mut GeneratorArgument::default());
    };
  }
}
