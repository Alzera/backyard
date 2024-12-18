use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "{\n}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn nested() {
  let asts = parse(true, "{\n\t{\n\t}\n}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
