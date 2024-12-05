use backyard_lexer::lex_eval;

#[test]
fn basic() {
  let tokens = lex_eval("$a").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn this() {
  let tokens = lex_eval("$this").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn bracket() {
  let tokens = lex_eval("${$a}").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn multiple() {
  let tokens = lex_eval("$$$a").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
