use backyard_lexer::lex;

#[test]
fn line() {
  let tokens = lex(true, "// test").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn line_new_line() {
  let tokens = lex(true, "// test\n$a").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn line_tag_close() {
  let tokens = lex(false, "<?php // test ?>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn block() {
  let tokens = lex(true, "/* test */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc() {
  let tokens = lex(true, "/**\n * test\n */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
