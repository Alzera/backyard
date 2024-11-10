use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS },
  guard_ok,
  parser::{
    node::{ Node, NodeTraitCast, NodeType, Nodes },
    nodes::{
      function::{ AnonymousFunctionNode, ArrowFunctionNode, FunctionNode, ParameterNode },
      identifier::IdentifierNode,
    },
  },
};

use super::{ block::BlockGenerator, identifier::IdentifierGenerator, property::PropertyGenerator };

pub struct FunctionGenerator {}

impl FunctionGenerator {
  pub fn get_parameters(generator: &mut Generator, parameters: &Nodes) -> Builder {
    generator.generate_nodes_new(
      parameters,
      &mut GeneratorArgument::for_parameter(&[(NodeType::Parameter, Self::generate_parameter)])
    )
  }

  pub fn get_return_type(
    generator: &mut Generator,
    node: &Option<Node>
  ) -> (Option<Builder>, usize) {
    let return_type = if let Some(n) = &node { Some(generator.generate_node_new(n)) } else { None };
    let return_type_len = if let Some(n) = &return_type {
      n.total_len_with_separator(" ")
    } else {
      0
    };
    (return_type, return_type_len)
  }

  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<FunctionNode>(), {
      return;
    });
    builder.push("function ");
    if node.is_ref {
      builder.push("&");
    }
    let name = guard_ok!(node.name.to_owned().cast::<IdentifierNode>(), {
      return;
    });
    builder.push(&name.name);

    let mut parameters = if name.name == "__construct" {
      generator.generate_nodes_new(
        &node.parameters,
        &mut GeneratorArgument::for_parameter(&[(NodeType::Property, PropertyGenerator::generate)])
      )
    } else {
      Self::get_parameters(generator, &node.parameters)
    };
    let (return_type, return_type_len) = Self::get_return_type(generator, &node.return_type);

    builder.push("(");
    if
      Generator::check_nodes_has_comments(&node.parameters) ||
      3 + builder.last_len() + parameters.total_len_with_separator(" ") + return_type_len >
        generator.max_length
    {
      parameters.indent();
      builder.extend(&parameters);
      builder.new_line();
    } else {
      builder.push(&parameters.to_string(" "));
    }
    builder.push(")");

    if let Some(n) = &return_type {
      builder.push(": ");
      builder.extend_first_line(n);
    }
    if let Some(n) = &node.body {
      BlockGenerator::generate(generator, builder, &n, None);
    } else {
      builder.push(";");
    }
  }

  pub fn generate_anonymous(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<AnonymousFunctionNode>(), {
      return;
    });
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
    let (return_type, return_type_len) = Self::get_return_type(generator, &node.return_type);

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
      builder.extend(&parameters);
      builder.new_line();
    } else {
      builder.push(&parameters.to_string(" "));
    }
    builder.push(")");

    if !node.uses.is_empty() {
      builder.push(" use (");
      if
        Generator::check_nodes_has_comments(&node.uses) ||
        builder.last_len() + uses_len + return_type_len > generator.max_length
      {
        uses.indent();
        builder.extend(&uses);
        builder.new_line();
      } else {
        builder.push(&uses.to_string(" "));
      }
      builder.push(")");
    }

    if let Some(n) = &return_type {
      builder.push(": ");
      builder.extend_first_line(n);
    }

    BlockGenerator::generate(generator, builder, &node.body, None);
  }

  pub fn generate_arrow(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ArrowFunctionNode>(), {
      return;
    });
    builder.push("fn ");
    if node.is_ref {
      builder.push("&");
    }
    let mut parameters = Self::get_parameters(generator, &node.parameters);
    let (return_type, return_type_len) = Self::get_return_type(generator, &node.return_type);

    builder.push("(");
    if
      Generator::check_nodes_has_comments(&node.parameters) ||
      3 + builder.last_len() + parameters.total_len_with_separator(" ") + return_type_len >
        generator.max_length
    {
      parameters.indent();
      builder.extend(&parameters);
      builder.new_line();
    } else {
      builder.push(&parameters.to_string(" "));
    }
    builder.push(")");

    if let Some(n) = &return_type {
      builder.push(": ");
      builder.extend_first_line(n);
    }

    builder.push(" => ");
    generator.generate_node(builder, &node.body, &mut GeneratorArgument::default());
  }

  pub fn generate_parameter(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ParameterNode>(), {
      return;
    });
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

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test("class A {
  public function __construct(protected int $x, protected int $y = 0) {
  }
}");
    test("function &a(?int ...$b = 0, String &$c = [0.01, 0x12], bool $d): ?int {\n}");
    test("$a = fn &(int $x): ?int => null;");
    test("$a = function &(int $x, ?int $y) use ($arg2): static {\n};");
  }
}
