use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "$a = false;\n__halt_compiler();\nThis text is invalid").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
