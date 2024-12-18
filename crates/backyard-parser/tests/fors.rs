use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "for ($i = 1; $i <= 10; $i++) {\n}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn short() {
  let asts = parse(true, "for (;;):\nendfor;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn no_body() {
  let asts = parse(true, "for ($i = 1, $j = 0; $i <= 10; $j += $i, print $i, $i++);").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
