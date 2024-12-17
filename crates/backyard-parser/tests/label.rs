use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "label:").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn get() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "get:").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
