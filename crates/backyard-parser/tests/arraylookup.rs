use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("array(1, 2)[1];").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn no_expr() {
  let asts = parse_eval("$a[];").unwrap();
  insta::assert_yaml_snapshot!(asts);
}