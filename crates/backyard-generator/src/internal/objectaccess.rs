use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ObjectAccessGenerator;

impl ObjectAccessGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = cast_node!(ObjectAccess, &node.node);
    generator.generate_node(builder, &node.object, &mut GeneratorArgument::default());
    if node.is_nullsafe {
      builder.push("?");
    }
    builder.push("->");
    if node.use_bracket {
      builder.push("{");
      generator.generate_node(builder, &node.property, &mut GeneratorArgument::default());
      builder.push("}");
    } else {
      generator.generate_node(builder, &node.property, &mut GeneratorArgument::default());
    }
  }
}
