use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "call(1, 2);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn named_argument() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$this(a: 1, b: 2);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn parenthesis() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "(fn() => 5)();").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
