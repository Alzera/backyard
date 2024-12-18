use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "$a = match($a) {
  'ucs2', 'utf-16' => 2,
  default => 1
};"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
