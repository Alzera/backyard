use backyard_parser::{ parse, parse_eval };

#[test]
fn test_break() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "break;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_break_with_argument() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "break 2;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_continue() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "continue;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_continue_with_argument() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "continue 2;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_goto() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "goto label;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_new() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "new Exception();").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_print() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "print \"Hello\";").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_return() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "return;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_return_with_argument() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "return 4;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_throw() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "throw $a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_parent() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "parent::a").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_static() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "static::a").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_clone() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "clone $a").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_this() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$this").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_true() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "true").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_false() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "false").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_null() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "null").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_self() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "self::a").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_inline() {
  let arena = bumpalo::Bump::new();
  let asts = parse(&arena, "Hello <?= $world ?>").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
