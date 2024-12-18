use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "foreach ($a as &$b) {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_key() {
  let asts = parse(true, "foreach ($a as $b => $c) {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn no_block() {
  let asts = parse(true, "foreach ($a as $b => $c)\n\t$d = 5;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn short_block() {
  let asts = parse(true, "foreach ($a as $b => $c):\n\tendforeach;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
