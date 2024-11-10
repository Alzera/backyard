use crate::{
  generator::generator::{ Builder, Generator },
  guard,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::traits::TraitNode },
};

use super::{
  block::BlockGenerator,
  consts::ConstGenerator,
  identifier::IdentifierGenerator,
  method::MethodGenerator,
  property::PropertyGenerator,
  traituse::TraitUseGenerator,
};

pub struct TraitGenerator {}

impl TraitGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<TraitNode>());
    builder.push("trait ");
    IdentifierGenerator::generate(generator, builder, &node.name);
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
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test(
      "trait B {
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
