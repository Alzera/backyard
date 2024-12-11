use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "self::a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn bracket() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "self::{$a};").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_class() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "self::class;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn variable() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "self::$a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
