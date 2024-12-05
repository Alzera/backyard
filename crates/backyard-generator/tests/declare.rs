use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("declare(strict_types = 1);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_body() {
  let asts = parse_eval("declare(ticks = 1, ticks = 1) {\n}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_short_body() {
  let asts = parse_eval("declare(ticks = 1):\nenddeclare;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
