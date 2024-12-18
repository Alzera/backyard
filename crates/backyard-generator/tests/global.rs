use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "global $title, $hook_suffix, $current_screen;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
