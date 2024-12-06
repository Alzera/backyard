use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("static $title;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn value() {
  let asts = parse_eval("static $title = 1;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn multiple() {
  let asts = parse_eval("static $title = 1, $hook_suffix, $current_screen;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
