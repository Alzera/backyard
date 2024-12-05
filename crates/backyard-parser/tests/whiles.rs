use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("while ($i <= 10) {};").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn no_block() {
  let asts = parse_eval("while (true) $a = 4;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_key() {
  let asts = parse_eval("while (true): endwhile;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
