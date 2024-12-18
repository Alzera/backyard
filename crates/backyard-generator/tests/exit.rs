use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "exit;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn with_argument() {
  let asts = parse(true, "exit(0);").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
