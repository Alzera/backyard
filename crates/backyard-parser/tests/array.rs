use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("array(1, 2);").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn short() {
  let asts = parse_eval("[1, 2];").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_key() {
  let asts = parse_eval("['a' => 1, 'b' => 2];").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
