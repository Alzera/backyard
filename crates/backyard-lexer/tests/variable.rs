use backyard_lexer::lex;

#[test]
fn basic() {
  let tokens = lex(true, "$a").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn this() {
  let tokens = lex(true, "$this").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn bracket() {
  let tokens = lex(true, "${$a}").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn multiple() {
  let tokens = lex(true, "$$$a").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
