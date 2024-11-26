use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct StaticLookupGenerator {}

impl StaticLookupGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::StaticLookup, &node.node);
    generator.generate_node(builder, &node.target, &mut GeneratorArgument::default());
    builder.push("::");
    if node.on.node_type == NodeType::ClassKeyword {
      builder.push("class");
    } else {
      if node.bracket {
        builder.push("{");
        generator.generate_node(builder, &node.on, &mut GeneratorArgument::default());
        builder.push("}");
      } else {
        generator.generate_node(builder, &node.on, &mut GeneratorArgument::default());
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("++A::b();");
    test_eval("A::class;");
    test_eval("$a = !(static::{'shouldOverflow' . $ucUnit}());");
  }
}
