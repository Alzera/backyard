use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("array(1, 2);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn short() {
  let asts = parse_eval("[1, 2];").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_key() {
  let asts = parse_eval("['a' => 1, 'b' => 2];").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn complex() {
  let asts = parse_eval(
    "$a = [$key = (is_int($key) ? $value : $key) => $value === true ? $key : $value];"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
