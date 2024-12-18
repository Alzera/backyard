use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "$a = 5;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
