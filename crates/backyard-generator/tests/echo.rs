use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("echo \"Hello\";").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn multiple() {
  let asts = parse_eval("echo \"Hello\", \"World\";").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
