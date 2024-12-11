use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn bracket() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "${$a} = 4;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn multiple() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$$a = 4;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
