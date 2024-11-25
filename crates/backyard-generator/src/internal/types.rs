use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

pub struct TypeGenerator {}

impl TypeGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::Type, &node.node);
    if node.is_nullable {
      builder.push("?");
    }
    builder.push(&node.name);
  }

  pub fn generate_union(_: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::UnionType, &node.node);
    builder.push(&node.types.join("|"));
  }

  pub fn generate_intersection(_: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::IntersectionType, &node.node);
    builder.push(&node.types.join("&"));
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval(
      "class A {
  public function __construct(
    \\Exception\\A|\\Exception\\B $exception,
    private array|\\Closure $suggestedValues = [],
    protected \\A&\\B $currentHandler
  ) {
  }
}"
    );
  }
}
