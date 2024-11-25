use backyard_nodes::{ cast_node, node::{ Node, NodeType, NodeWrapper } };

use crate::generator::{ Builder, Generator, GeneratorArgument };

use super::block::BlockGenerator;

pub struct IfGenerator {}

impl IfGenerator {
  pub fn generate(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::If, &node.node);

    builder.push("if (");
    generator.generate_node(builder, &node.condition, &mut GeneratorArgument::default());
    builder.push(")");

    if node.is_short {
      if let Some(n) = &node.invalid {
        BlockGenerator::generate(generator, builder, &node.valid, Some(""));
        Self::generate_else(generator, builder, &n);
      } else {
        BlockGenerator::generate(generator, builder, &node.valid, Some("endif;"));
      }
    } else {
      BlockGenerator::generate(generator, builder, &node.valid, None);
      if let Some(n) = &node.invalid {
        builder.push(" ");
        generator.generate_node(builder, n, &mut GeneratorArgument::default());
      }
    }
  }

  pub fn generate_else(generator: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Else, &node.node);

    builder.push("else");
    if node.body.node_type == NodeType::If {
      Self::generate(generator, builder, &node.body);
    } else {
      BlockGenerator::generate(
        generator,
        builder,
        &node.body,
        node.is_short.then(|| "endif;")
      );
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval(
      "try {
  if (1) {
  } else {
  }
} catch (AssertionFailedError $e) {
  if (2) {
  } elseif (3) {
  }
} catch (AssertionError $e) {
} catch (Throwable $e) {
}"
    );
    test_eval("if (true):
elseif (false):
else:
endif;");
    test_eval("if (isset($var1)) {
} elseif (empty([])) {
} else {
}");
    test_eval("if (1) {
  if (4) {
  } else {
  }
} else {
}");
    test_eval(
      "if ($request->user()->hasVerifiedEmail()) {
  return redirect()->intended(route('dashboard', absolute: false) . '?verified=1');
}
if ($request->user()->markEmailAsVerified()) {
  event(new Verified($request->user()));
}"
    );
  }
}
