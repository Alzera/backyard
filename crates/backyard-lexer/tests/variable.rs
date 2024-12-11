use backyard_lexer::lex_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn this() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$this").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn bracket() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "${$a}").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn multiple() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$$$a").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
