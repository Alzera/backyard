use backyard_nodes::{ cast_node, node::{ Node, NodeWrapper } };

use crate::generator::{ Builder, Generator };

pub struct CommentGenerator;

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
    let comments = node.comment.split('\n');
    let comments_last_index = comments.clone().count() - 1;
    let comments = comments.enumerate().filter_map(|(index, i)| {
      let trimmed = i.trim_start();
      if (index == 0 || index == comments_last_index) && trimmed.is_empty() {
        None
      } else {
        Some(format!(" {}", trimmed))
      }
    });
    for i in comments {
      builder.push(&i);
      builder.new_line();
    }
    builder.push(" */");
  }

  pub fn generate_doc(_: &mut Generator, builder: &mut Builder, node: &Box<Node>) {
    let node = cast_node!(NodeWrapper::CommentDoc, &node.node);
    builder.push("/**");
    builder.new_line();
    let comments = node.comment.split('\n');
    let comments_last_index = comments.clone().count() - 1;
    let comments = comments.enumerate().filter_map(|(index, i)| {
      let trimmed = i.trim_start();
      if (index == 0 || index == comments_last_index) && trimmed.is_empty() {
        None
      } else {
        Some(format!(" {}", trimmed))
      }
    });
    for i in comments {
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
    test_eval("if (false !== 'getaddrinfo'
// DNS lookup failed
 || false !== 'getaddrinfo') {
}");
    test_eval(
      "///////////////////////////////////////////////
/////////////////// Testing ///////////////////
///////////////////////////////////////////////
$a;"
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
