use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "const A = 1, B = 2;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_type() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "class A { const string|int BAR = 'bar'; }").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn property() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "class A { public private(set) const A = 1, B = 2; }").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
