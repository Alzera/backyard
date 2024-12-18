use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "$;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn bracket() {
  let asts = parse(true, "${$a} = 4;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn multiple() {
  let asts = parse(true, "$$a = 4;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
