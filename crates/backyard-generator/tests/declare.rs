use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "declare(strict_types = 1);").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn with_body() {
  let asts = parse(true, "declare(ticks = 1, ticks = 1) {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn with_short_body() {
  let asts = parse(true, "declare(ticks = 1):\nenddeclare;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
