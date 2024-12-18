use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "$a = 5 * (.5 + 0x2e45);").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn var() {
  let asts = parse(true, "$a = (int) $a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
