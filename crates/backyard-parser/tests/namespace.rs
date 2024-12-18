use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "namespace MyApp\\A;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_body() {
  let asts = parse(true, "namespace MyApp\\B {}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
