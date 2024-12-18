use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn line_basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "// test").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn line_long() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "///////////// TEST /////////////").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn block() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "/*\ntest\n*/").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn doc() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "/**\n * test\n */").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn before_block() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "if (false) // test
{
}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn after_block() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "do {
}
// test
while(false);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn at_block_end() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "if (false) {
}
// test").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn at_statement_end() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = 5 /* test */;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
