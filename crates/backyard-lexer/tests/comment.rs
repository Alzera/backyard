use backyard_lexer::{ lex, lex_eval };

#[test]
fn line() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "// test").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn line_new_line() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "// test\n$a").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn line_tag_close() {
  let arena = bumpalo::Bump::new();
  let tokens = lex(&arena, "<?php // test ?>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn block() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "/* test */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "/**\n * test\n */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
