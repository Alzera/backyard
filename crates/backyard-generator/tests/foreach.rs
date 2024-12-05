use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("foreach ($a as &$b) {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_key() {
  let asts = parse_eval("foreach ($a as $b => $c) {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn no_block() {
  let asts = parse_eval("foreach ($a as $b => $c)\n\t$d = 5;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn short_block() {
  let asts = parse_eval("foreach ($a as $b => $c):\n\tendforeach;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
