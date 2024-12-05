use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("static $title, $hook_suffix, $current_screen;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}