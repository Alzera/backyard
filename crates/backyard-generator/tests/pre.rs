use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "++$a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn negate() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "!$a;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn silent() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "@$a();").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn ellipsis() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "...$a();").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn ambersand() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = &$b;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
