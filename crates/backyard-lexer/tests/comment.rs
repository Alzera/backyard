use backyard_lexer::{ lex, lex_eval };

#[test]
fn line() {
  let tokens = lex_eval("// test").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn line_new_line() {
  let tokens = lex_eval("// test\n$a").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn line_tag_close() {
  let tokens = lex("<?php // test ?>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn block() {
  let tokens = lex_eval("/* test */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc() {
  let tokens = lex_eval("/**\n * test\n */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
