use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "static $title;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn value() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "static $title = 1;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn multiple() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "static $title = 1, $hook_suffix, $current_screen;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
