use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "echo \"Hello\";").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn multiple() {
  let asts = parse(true, "echo \"Hello\", \"World\";").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
