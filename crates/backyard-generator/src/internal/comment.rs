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
    builder.new_line();
    let n: Vec<&str> = node.comment
      .split('\n')
      .map(|i| i.trim_start())
      .collect();
    for i in n
      .iter()
      .enumerate()
      .filter_map(|(index, i)| (
        if (index == 0 || index == n.len() - 1) && i.is_empty() {
          None
        } else {
          let mut i = i.to_string();
          i.insert(0, ' ');
          Some(i)
        }
      )) {
      builder.push(&i);
      builder.new_line();
    }
    builder.push(" */");
  }

  pub fn generate_doc(_: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::CommentDoc, &node.node);
    builder.push("/**");
    builder.new_line();
    let n: Vec<&str> = node.comment
      .split('\n')
      .map(|i| i.trim_start())
      .collect();
    for i in n
      .iter()
      .enumerate()
      .filter_map(|(index, i)| (
        if (index == 0 || index == n.len() - 1) && i.is_empty() {
          None
        } else {
          let mut i = i.to_string();
          i.insert(0, ' ');
          Some(i)
        }
      )) {
      builder.push(&i);
      builder.new_line();
    }
    builder.push(" */");
  }
}

#[cfg(test)]
mod tests {
  use crate::test_utils::test_eval;

  #[test]
  fn basic() {
    test_eval(
      "///////////////////////////////////////////////
/////////////////// Testing ///////////////////
///////////////////////////////////////////////"
    );
    test_eval(
      "/*
 testing leading
 */
function a() {
  /**
   * testing inside
   */
}
// testing trailing"
    );
  }
}
