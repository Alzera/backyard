use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = 5 + 5;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn chained() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "$this->a($a)
  ?? $this->b($b) ?? $this->c($c)
    ?? $this->d($d) ?? $this->e($e);"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
