use crate::{
  generator::generator::{ Builder, Generator },
  guard,
  parser::{
    node::{ Node, NodeTraitCast },
    nodes::comment::{ CommentBlockNode, CommentDocNode, CommentLineNode },
  },
};

pub struct CommentGenerator {}

impl CommentGenerator {
  pub fn generate(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<CommentLineNode>());
    builder.push("//");
    builder.push(&node.comment);
  }

  pub fn generate_block(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<CommentBlockNode>());
    builder.push("/*");
    builder.push(&node.comment);
    builder.push("*/");
  }

  pub fn generate_doc(_: &mut Generator, builder: &mut Builder, node: &Node) {
    let node = guard!(node.to_owned().cast::<CommentDocNode>());
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
