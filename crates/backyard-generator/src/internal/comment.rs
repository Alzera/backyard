use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

pub struct CommentGenerator {}

impl CommentGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::CommentLine, &node.node);
    builder.push("//");
    builder.push(&node.comment);
  }

  pub fn generate_block(_: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::CommentBlock, &node.node);
    builder.push("/*");
    builder.push(&node.comment);
    builder.push("*/");
  }

  pub fn generate_doc(_: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::CommentDoc, &node.node);
    builder.push("/**");
    builder.push(&node.comment);
    builder.push("*/");
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test;

  #[test]
  fn basic() {
    test(
      "///////////////////////////////////////////////
/////////////////// Testing ///////////////////
///////////////////////////////////////////////"
    );
    test("/*
 testing leading
 */
function a() {
  /** 
   * testing inside
   */
}
// testing trailing");
  }
}
