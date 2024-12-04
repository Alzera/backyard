use backyard_lexer::lex;

#[test]
fn unterminated() {
  let tokens = lex("<?php // test ?><div>test</div>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn inline_start() {
  let tokens = lex("<div>test</div><?php // test").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn inline_end() {
  let tokens = lex("<?php // test ?><div>test</div>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn inline_echo() {
  let tokens = lex("<div><?= $a ?></div>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
