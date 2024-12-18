use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "for ($i = 1; $i <= 10; $i++) {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn short() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "for (;;):\nendfor;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn no_body() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "for ($i = 1, $j = 0; $i <= 10; $j += $i, print $i, $i++);"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
