use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct ObjectAccessGenerator;

impl ObjectAccessGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::ObjectAccess, &node.node);
    generator.generate_node(builder, &node.object, &mut GeneratorArgument::default());
    if node.nullsafe {
      builder.push("?");
    }
    builder.push("->");
    if node.bracket {
      builder.push("{");
      generator.generate_node(builder, &node.property, &mut GeneratorArgument::default());
      builder.push("}");
    } else {
      generator.generate_node(builder, &node.property, &mut GeneratorArgument::default());
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("$this->from;");
    test_eval("$this->$from;");
    test_eval("$a?->{\"b\"};");
    test_eval("$this->setTimezone(date_default_timezone_get());");
    test_eval(
      "while ($i <= 10) {\n  $this->subSecond();\n  $value += static::MICROSECONDS_PER_SECOND;\n}"
    );
  }
}
