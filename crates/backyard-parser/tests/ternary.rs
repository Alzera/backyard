use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = false ? 1 : 2;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
