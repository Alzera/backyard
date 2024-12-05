use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("include(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn without_parenthesis() {
  let asts = parse_eval("include \"a\";").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn once() {
  let asts = parse_eval("include_once(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn require() {
  let asts = parse_eval("require(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn require_once() {
  let asts = parse_eval("require_once(\"a\");").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
