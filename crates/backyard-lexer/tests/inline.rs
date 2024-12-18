use backyard_lexer::lex;

#[test]
fn unterminated() {
  let tokens = lex(false, "<?php // test ?><div>test</div>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn inline_start() {
  let tokens = lex(false, "<div>test</div><?php // test").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn inline_end() {
  let tokens = lex(false, "<?php // test ?><div>test</div>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn inline_echo() {
  let tokens = lex(false, "<div><?= $a ?></div>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
