use backyard_parser::parse_eval;

#[test]
fn line_basic() {
  let asts = parse_eval("// test").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn line_long() {
  let asts = parse_eval("///////////// TEST /////////////").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn block() {
  let asts = parse_eval("/*\ntest\n*/").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn doc() {
  let asts = parse_eval("/**\n * test\n */").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn before_block() {
  let asts = parse_eval("if (false) // test
{
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn after_block() {
  let asts = parse_eval("do {
}
// test
while(false);").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn at_block_end() {
  let asts = parse_eval("if (false) {
}
// test").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn at_statement_end() {
  let asts = parse_eval("$a = 5 /* test */;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
