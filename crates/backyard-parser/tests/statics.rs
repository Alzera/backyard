use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "static $title;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn value() {
  let asts = parse(true, "static $title = 1;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn multiple() {
  let asts = parse(true, "static $title = 1, $hook_suffix, $current_screen;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
