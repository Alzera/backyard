use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

pub struct BinGenerator;

impl BinGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Bin, &node.node);
    generator.generate_node(builder, &node.left, &mut GeneratorArgument::default());
    let mut expr = generator.generate_node_new(&node.right);
    if builder.last_len() + expr.first_len() + node.operator.len() > generator.max_length {
      let mut t = Builder::new();
      t.new_line();
      t.push(format!("{} ", node.operator).as_str());
      t.indent();
      builder.extend(&t);
      expr.indent();
      builder.extend_first_line(&expr);
    } else {
      builder.push(format!(" {} ", node.operator).as_str());
      builder.extend_first_line(&expr);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval("$platform instanceof SQLServerPlatform || $platform instanceof SQLitePlatform;");
    test_eval(
      "$this->callDiffAlias($unit, $parameters)
  ?? $this->callHumanDiffAlias($unit, $parameters) ?? $this->callRoundMethod($unit, $parameters)
    ?? $this->callGetOrSetMethod($method, $parameters) ?? $this->callMacroMethod($method, $parameters);"
    );
    [
      "+",
      "-",
      "*",
      "/",
      "%",
      "**",
      "&",
      "|",
      "^",
      "<<",
      ">>",
      "==",
      "===",
      "!=",
      "!==",
      "<",
      ">",
      "<=",
      ">=",
      "<=>",
      ".",
      "??",
      "&&",
      "||",
      "&",
      "|",
      "^",
    ]
      .iter()
      .for_each(|i| {
        test_eval(format!("$a {} 0;", i).as_str());
      });
    test_eval(
      "$an_unneccessary_very_long_variable_name
  . $another_unnecessary_very_long_variable_name_that_should_be_on_new_line;"
    );
  }
}
