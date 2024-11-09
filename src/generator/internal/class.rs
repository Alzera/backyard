use crate::{
  generator::generator::{ Builder, Generator, GeneratorArgument },
  guard_ok,
  parser::{ node::{ Node, NodeTraitCast, NodeType }, nodes::class::ClassNode },
};

use super::{ block::BlockGenerator, identifier::IdentifierGenerator };

pub struct ClassGenerator {}

impl ClassGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard_ok!(node.to_owned().cast::<ClassNode>(), {
      return;
    });
    if node.modifier.len() > 0 {
      builder.push(format!("{} ", node.modifier).as_str());
    }
    builder.push("class ");
    IdentifierGenerator::generate(generator, builder, &node.name);
    if let Some(n) = &node.extends {
      builder.push(" extends ");
      IdentifierGenerator::generate(generator, builder, &n);
    }
    if node.implements.len() > 0 {
      builder.push(" implements ");
      let implements = generator.generate_nodes_new(
        &node.implements,
        &mut GeneratorArgument::for_parameter(
          &[(NodeType::Identifier, IdentifierGenerator::generate)]
        )
      );
      builder.push(&implements.to_string(" "));
    }
    builder.push(" {");
    let mut block = Builder::new();
    BlockGenerator::generate(generator, &mut block, &node.body);
    block.indent();
    builder.extend(&block);
    builder.new_line();
    builder.push("}");
  }
}
