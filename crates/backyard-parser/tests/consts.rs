use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "const A = 1, B = 2;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_type() {
  let asts = parse(true, "class A { const string|int BAR = 'bar'; }").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn property() {
  let asts = parse(true, "class A { public private(set) const A = 1, B = 2; }").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
