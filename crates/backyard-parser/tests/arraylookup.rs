use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "array(1, 2)[1];").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn no_expr() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a[];").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
