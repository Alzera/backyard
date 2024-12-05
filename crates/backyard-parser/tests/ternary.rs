use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("$a = false ? 1 : 2;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
