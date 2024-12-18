use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "while ($i <= 10) {};").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn no_block() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "while (true) $a = 4;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_key() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "while (true): endwhile;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
