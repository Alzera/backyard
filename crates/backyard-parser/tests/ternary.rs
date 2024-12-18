use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "$a = false ? 1 : 2;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
