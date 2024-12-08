use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("label:").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn get() {
  let asts = parse_eval("get:").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
