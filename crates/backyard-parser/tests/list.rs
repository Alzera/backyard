use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("list($a, $b) = [0, 1];").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
