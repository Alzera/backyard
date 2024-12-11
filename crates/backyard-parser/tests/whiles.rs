use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "while ($i <= 10) {};").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn no_block() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "while (true) $a = 4;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_key() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "while (true): endwhile;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
