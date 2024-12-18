use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "++$a;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn negate() {
  let asts = parse(true, "!$a;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn silent() {
  let asts = parse(true, "@$a();").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn ellipsis() {
  let asts = parse(true, "...$a();").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn ambersand() {
  let asts = parse(true, "$a = &$b;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
