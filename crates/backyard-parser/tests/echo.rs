use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("echo \"Hello\";").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn multiple() {
  let asts = parse_eval("echo \"Hello\", \"World\";").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
