use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "array(1, 2);").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn short() {
  let asts = parse(true, "[1, 2];").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_key() {
  let asts = parse(true, "['a' => 1, 'b' => 2];").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn complex() {
  let asts = parse(
    true,
    "$a = [$key = (is_int($key) ? $value : $key) => $value === true ? $key : $value];"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}
