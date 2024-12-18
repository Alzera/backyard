use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "call(1, 2);").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn named_argument() {
  let asts = parse(true, "$this(a: 1, b: 2);").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn parenthesis() {
  let asts = parse(true, "(fn() => 5)();").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
