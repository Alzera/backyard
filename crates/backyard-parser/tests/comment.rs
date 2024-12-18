use backyard_parser::parse;

#[test]
fn line_basic() {
  let asts = parse(true, "// test").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn line_long() {
  let asts = parse(true, "///////////// TEST /////////////").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn block() {
  let asts = parse(true, "/*\ntest\n*/").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn doc() {
  let asts = parse(true, "/**\n * test\n */").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn before_block() {
  let asts = parse(true, "if (false) // test
{
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn after_block() {
  let asts = parse(true, "do {
}
// test
while(false);").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn at_block_end() {
  let asts = parse(true, "if (false) {
}
// test").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn at_statement_end() {
  let asts = parse(true, "$a = 5 /* test */;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
