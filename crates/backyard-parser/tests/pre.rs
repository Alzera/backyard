use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("++$a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn negate() {
  let asts = parse_eval("!$a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn silent() {
  let asts = parse_eval("@$a();").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn ellipsis() {
  let asts = parse_eval("...$a();").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn ambersand() {
  let asts = parse_eval("$a = &$b;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
