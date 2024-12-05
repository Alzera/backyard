use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("for ($i = 1; $i <= 10; $i++) {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn short() {
  let asts = parse_eval("for (;;):\nendfor;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn no_body() {
  let asts = parse_eval("for ($i = 1, $j = 0; $i <= 10; $j += $i, print $i, $i++);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
