use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn line_basic() {
  let asts = parse(true, "// test").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn line_long() {
  let asts = parse(true, "///////////// TEST /////////////").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn block() {
  let asts = parse(true, "/*\ntest\n*/").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

// #[test]
// fn doc() {
//   let asts = parse(true, "/**\n * test\n */").unwrap();
//   insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
// }

#[test]
fn before_block() {
  let asts = parse(true, "if (false) // test
{
}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn after_block() {
  let asts = parse(true, "do {
}
// test
while(false);").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn at_block_end() {
  let asts = parse(true, "if (false) {
}
// test").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn at_statement_end() {
  let asts = parse(true, "$a = 5 /* test */;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
