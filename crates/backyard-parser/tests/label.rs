use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "label:").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn get() {
  let asts = parse(true, "get:").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
