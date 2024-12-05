use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("$;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn bracket() {
  let asts = parse_eval("${$a} = 4;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn multiple() {
  let asts = parse_eval("$$a = 4;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
