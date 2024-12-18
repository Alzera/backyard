use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "list($a, $b) = [0, 1];").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
