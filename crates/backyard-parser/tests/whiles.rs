use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "while ($i <= 10) {};").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn no_block() {
  let asts = parse(true, "while (true) $a = 4;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_key() {
  let asts = parse(true, "while (true): endwhile;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
