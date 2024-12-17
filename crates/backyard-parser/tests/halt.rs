use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = false;\n__halt_compiler();\nThis text is invalid").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
