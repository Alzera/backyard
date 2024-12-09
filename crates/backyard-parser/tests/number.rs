use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("$a = 0;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn decimal() {
  let asts = parse_eval("$a = 0.1;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn decimal_only() {
  let asts = parse_eval("$a = .1;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn hex() {
  let asts = parse_eval("$a = 0x0f9bc0;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn binary() {
  let asts = parse_eval("$a = 0b1101_1010;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
