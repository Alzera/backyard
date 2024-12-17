use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "exit;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn with_argument() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "exit(0);").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
