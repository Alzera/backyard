use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("exit;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_argument() {
  let asts = parse_eval("exit(0);").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
