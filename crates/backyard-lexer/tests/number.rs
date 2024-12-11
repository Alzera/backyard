use backyard_lexer::lex_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "67").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn decimal() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "6.7").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn only_decimal() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, ".67").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn hex() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "0xff9abc").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn binary() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "0b1010_1010").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
