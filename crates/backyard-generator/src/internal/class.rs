use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument, DEFAULT_GENERATORS };

use super::{
  block::BlockGenerator,
  consts::ConstGenerator,
  identifier::IdentifierGenerator,
  method::MethodGenerator,
  property::PropertyGenerator,
  traituse::TraitUseGenerator,
};

pub struct ClassGenerator;

impl ClassGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Class, &node.node);
    if node.is_readonly {
      builder.push("readonly ");
    }
    if let Some(n) = &node.inheritance {
      builder.push(format!("{} ", n).as_str());
    }
    builder.push("class");
    if let Some(n) = &node.name {
      builder.push(" ");
      IdentifierGenerator::generate(generator, builder, n);
    }
    if let Some(n) = &node.extends {
      builder.push(" extends ");
      IdentifierGenerator::generate(generator, builder, n);
    }
    if !node.implements.is_empty() {
      builder.push(" implements ");
      let implements = generator.generate_nodes_new(
        &node.implements,
        &mut GeneratorArgument::for_parameter(
          &[(NodeType::Identifier, IdentifierGenerator::generate)]
        )
      );
      builder.push(&implements.to_string(" "));
    }
    BlockGenerator::generate_specific(
      generator,
      builder,
      &node.body,
      None,
      &[
        (NodeType::TraitUse, TraitUseGenerator::generate),
        (NodeType::ConstProperty, ConstGenerator::generate_property),
        (NodeType::Property, PropertyGenerator::generate),
        (NodeType::Method, MethodGenerator::generate),
      ]
    );
  }

  pub fn generate_anonymous(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::AnonymousClass, &node.node);
    builder.push("class");
    if !node.parameters.is_empty() {
      builder.push("(");
      let parameters = generator.generate_nodes_new(
        &node.parameters,
        &mut GeneratorArgument::for_parameter(&DEFAULT_GENERATORS)
      );
      builder.push(&parameters.to_string(" "));
      builder.push(")");
    }
    if let Some(n) = &node.extends {
      builder.push(" extends ");
      IdentifierGenerator::generate(generator, builder, n);
    }
    if !node.implements.is_empty() {
      builder.push(" implements ");
      let implements = generator.generate_nodes_new(
        &node.implements,
        &mut GeneratorArgument::for_parameter(
          &[(NodeType::Identifier, IdentifierGenerator::generate)]
        )
      );
      builder.push(&implements.to_string(" "));
    }
    BlockGenerator::generate_specific(
      generator,
      builder,
      &node.body,
      None,
      &[
        (NodeType::TraitUse, TraitUseGenerator::generate),
        (NodeType::ConstProperty, ConstGenerator::generate_property),
        (NodeType::Property, PropertyGenerator::generate),
        (NodeType::Method, MethodGenerator::generate),
      ]
    );
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("abstract class A {
  private Extension\\Another $a;
}");
    test_eval("final class A extends \\PHPUnit\\Framework\\Exception {\n}");
    test_eval("return new class extends B {\n};");
    test_eval("return new class($a, .5) extends B {\n};");
    test_eval(
      "readonly class A implements C, D, E {
  use Ale;
  use Loggable, Usable {
    log as private alias;
    Loggable::log as aliasLoggable;
    Usable::useResource insteadof Loggable;
  }
  public const MY_CONST = \"constant\";
  public static ?A $instance = 4;
}"
    );
  }
}
