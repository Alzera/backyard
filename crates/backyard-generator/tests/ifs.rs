use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "if (false) {}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn with_else() {
  let asts = parse(true, "if (false) {} else {}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn with_elseif() {
  let asts = parse(true, "if (false) {} elseif (true) {}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn with_else_if() {
  let asts = parse(true, "if (false) {} else if (true) {}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn nested() {
  let asts = parse(true, "if (false) {if (false) {} else (true) {}} else if (true) {}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn no_block() {
  let asts = parse(true, "if (false) $e = 5; else if (true) $e = 6;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn short() {
  let asts = parse(true, "if (true): elseif (false): else: endif;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
