use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "global $title, $hook_suffix, $current_screen;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
