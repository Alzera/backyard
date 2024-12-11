use backyard_lexer::lex;

#[test]
fn unterminated() {
  let arena = bumpalo::Bump::new();
  let tokens = lex(&arena, "<?php // test ?><div>test</div>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn inline_start() {
  let arena = bumpalo::Bump::new();
  let tokens = lex(&arena, "<div>test</div><?php // test").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn inline_end() {
  let arena = bumpalo::Bump::new();
  let tokens = lex(&arena, "<?php // test ?><div>test</div>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn inline_echo() {
  let arena = bumpalo::Bump::new();
  let tokens = lex(&arena, "<div><?= $a ?></div>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
