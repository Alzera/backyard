use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn test_break() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "break;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_break_with_argument() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "break 2;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_continue() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "continue;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_continue_with_argument() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "continue 2;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_goto() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "goto label;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_new() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "new Exception();").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_print() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "print \"Hello\";").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_return() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "return;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_return_with_argument() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "return 4;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_throw() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "throw $a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_parent() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "parent::a").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_static() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "static::a").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_clone() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "clone $a").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_this() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$this").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_true() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "true").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_false() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "false").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_null() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "null").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_self() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "self::a").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn test_inline() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, false, "Hello <?= $world ?>").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
