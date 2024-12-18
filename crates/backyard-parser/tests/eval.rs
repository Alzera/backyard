use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "eval(\"\");").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
