use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = 0;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn decimal() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = 0.1;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn decimal_only() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = .1;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn hex() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = 0x0f9bc0;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn binary() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = 0b1101_1010;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
