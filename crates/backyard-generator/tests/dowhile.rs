use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "do {\n} while (false);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn separated_by_comment() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "do {\n}\n// this comment\nwhile (false);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
