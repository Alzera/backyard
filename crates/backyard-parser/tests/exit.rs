use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "exit;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_argument() {
  let asts = parse(true, "exit(0);").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
