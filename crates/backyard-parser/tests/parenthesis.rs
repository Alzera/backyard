use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = 5 * (.5 + 0x2e45);").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn var() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = (int) $a;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
