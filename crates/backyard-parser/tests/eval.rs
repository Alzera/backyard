use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("eval(\"\");").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
