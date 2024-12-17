use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "__DIR__ . __FILE__;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
