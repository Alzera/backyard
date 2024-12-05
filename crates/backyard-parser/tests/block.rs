use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("{\n}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn nested() {
  let asts = parse_eval("{\n\t{\n\t}\n}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
