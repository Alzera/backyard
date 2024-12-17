use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "foreach ($a as &$b) {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn with_key() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "foreach ($a as $b => $c) {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn no_block() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "foreach ($a as $b => $c)\n\t$d = 5;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn short_block() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "foreach ($a as $b => $c):\n\tendforeach;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
