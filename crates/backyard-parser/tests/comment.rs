use backyard_parser::parse_eval;

#[test]
fn line_basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "// test").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn line_long() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "///////////// TEST /////////////").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn block() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "/*\ntest\n*/").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn doc() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "/**\n * test\n */").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn before_block() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "if (false) // test
{
}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn after_block() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "do {
}
// test
while(false);").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn at_block_end() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "if (false) {
}
// test").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn at_statement_end() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "$a = 5 /* test */;").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}
