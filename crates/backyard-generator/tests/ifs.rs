use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "if (false) {}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_else() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "if (false) {} else {}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_elseif() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "if (false) {} elseif (true) {}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_else_if() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "if (false) {} else if (true) {}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn nested() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "if (false) {if (false) {} else (true) {}} else if (true) {}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn no_block() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "if (false) $e = 5; else if (true) $e = 6;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn short() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "if (true): elseif (false): else: endif;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
